use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::instructions::ID as ixs_sysvar_id,
};
use spl_associated_token_account::get_associated_token_address;
use std::ops::{ Div, Mul };

use crate::types::{
    instruction::{
        accounts::{
            Context,
            MarginfiProtocolInteractionAccounts,
            SolendProtocolInteractionAccounts,
        },
        SolautoAction,
        SolautoStandardAccounts,
    },
    shared::{
        DeserializedAccount,
        LendingPlatform,
        Position,
        SolautoAdminSettings,
        SolautoError,
        SolautoSettingsParameters,
    },
};

use crate::constants::{
    KAMINO_PROGRAM,
    MARGINFI_PROGRAM,
    SOLAUTO_ADMIN,
    SOLAUTO_ADMIN_SETTINGS_ACCOUNT_SEEDS,
    SOLEND_PROGRAM,
};

use super::solauto_utils::get_owner;

pub fn generic_instruction_validation(
    accounts: &SolautoStandardAccounts,
    authority_only_ix: bool,
    lending_platform: LendingPlatform
) -> ProgramResult {
    validate_signer(accounts.signer, &accounts.solauto_position, authority_only_ix)?;
    validate_program_account(accounts.lending_protocol, lending_platform)?;
    if !accounts.solauto_admin_settings.is_none() && !accounts.solauto_fees_receiver_ta.is_none() {
        validate_fees_receiver(
            accounts.solauto_admin_settings.unwrap(),
            accounts.solauto_fees_receiver_ta.unwrap()
        )?;
    }
    validate_referral_accounts(accounts)?;

    if !accounts.ixs_sysvar.is_none() && accounts.ixs_sysvar.unwrap().key != &ixs_sysvar_id {
        msg!("Incorrect ixs sysvar account provided");
        return Err(ProgramError::InvalidAccountData.into());
    }
    // We don't need to check other standard variables as shank handles system_program, token_program, ata_program, & rent
    // TODO verify this with a test by providing a different account in place of rent account

    Ok(())
}

pub fn validate_signer(
    signer: &AccountInfo,
    position_account: &Option<DeserializedAccount<Position>>,
    authority_only_ix: bool
) -> ProgramResult {
    if !signer.is_signer {
        msg!("Signer account is not a signer");
        return Err(ProgramError::MissingRequiredSignature.into());
    }

    if position_account.is_none() {
        return Ok(());
    }

    let position = position_account.as_ref().unwrap();
    let position_authority = position.data.authority;

    if authority_only_ix {
        if signer.key != &position_authority {
            msg!("Authority-only instruction, invalid signer for the specified instruction");
            return Err(ProgramError::InvalidAccountData.into());
        }

        let seeds = &[&[position.data.position_id], signer.key.as_ref()];
        let (pda, _bump) = Pubkey::find_program_address(seeds, &crate::ID);
        if &pda != position.account_info.key {
            msg!("Invalid position specified for the current signer");
            return Err(ProgramError::MissingRequiredSignature.into());
        }
    }

    Ok(())
}

pub fn validate_solauto_admin_signer(solauto_admin: &AccountInfo) -> ProgramResult {
    if !solauto_admin.is_signer || *solauto_admin.key != SOLAUTO_ADMIN {
        return Err(ProgramError::MissingRequiredSignature.into());
    }
    Ok(())
}

pub fn validate_position_settings(
    settings: &SolautoSettingsParameters,
    max_ltv: f64,
    liq_threshold: f64
) -> ProgramResult {
    let invalid_params = |error_msg| {
        msg!(error_msg);
        Err(SolautoError::InvalidPositionSettings.into())
    };

    if
        settings.repay_from_bps != 0 &&
        settings.repay_to_bps != 0 &&
        settings.boost_from_bps != 0 &&
        settings.boost_to_bps != 0
    {
        if settings.repay_from_bps <= settings.repay_to_bps {
            return invalid_params("repay_from_bps value must be greater than repay_to_bps value");
        }
        if settings.boost_from_bps >= settings.boost_to_bps {
            return invalid_params("boost_from_bps value must be less than boost_to_bps value");
        }
        if settings.repay_from_bps - settings.repay_to_bps < 50 {
            return invalid_params(
                "Minimum difference between repay_from_bps and repay_to_bps must be 50 or greater"
            );
        }
        if settings.boost_to_bps - settings.boost_from_bps < 50 {
            return invalid_params(
                "Minimum difference between boost_to_bps to boost_from_bps must be 50 or greater"
            );
        }
        if settings.repay_from_bps > 9500 {
            return invalid_params("repay_from_bps must be lower or equal to 9500");
        }

        let maximum_repay_to_bps = get_maximum_repay_to_bps_param(max_ltv, liq_threshold, None);
        if settings.repay_to_bps > maximum_repay_to_bps {
            return invalid_params(
                format!("For the given max_ltv and liq_threshold of the supplied asset, repay_to_bps must be lower or equal to {} in order to bring the utilization rate to an allowed position", maximum_repay_to_bps).as_str()
            );
        }
    } else {
        let params = vec![
            settings.repay_from_bps,
            settings.repay_to_bps,
            settings.boost_from_bps,
            settings.boost_to_bps
        ];
        if params.iter().any(|&x| x != 0) {
            return invalid_params("Either all setting parameters should be 0, or none");
        }
    }

    Ok(())
}

pub fn get_maximum_repay_to_bps_param(
    max_ltv: f64,
    liq_threshold: f64,
    max_price_slippage: Option<u16>
) -> u16 {
    // price slippage buffer room to account for any unexpected price slippage when swapping tokens
    let price_slippage_buffer_room = if max_price_slippage.is_none() {
        3.0
    } else {
        max_price_slippage.unwrap() as f64
    };

    (max_ltv - price_slippage_buffer_room).div(liq_threshold).mul(10000.0) as u16
}

pub fn validate_program_account(
    program: &AccountInfo,
    lending_platform: LendingPlatform
) -> ProgramResult {
    match lending_platform {
        LendingPlatform::Solend => {
            if *program.key != SOLEND_PROGRAM {
                msg!("Incorrect Solend program account");
                return Err(ProgramError::InvalidAccountData.into());
            }
        }
        LendingPlatform::Marginfi => {
            if *program.key != MARGINFI_PROGRAM {
                msg!("Incorrect Marginfi program account");
                return Err(ProgramError::InvalidAccountData.into());
            }
        }
        LendingPlatform::Kamino => {
            if *program.key != KAMINO_PROGRAM {
                msg!("Incorrect Kamino program account");
                return Err(ProgramError::InvalidAccountData.into());
            }
        }
    }
    // We don't need to check more than this, as lending protocols have their own account checks and will fail during CPI if there is an issue with the provided accounts
    Ok(())
}

pub fn validate_fees_receiver<'a>(
    solauto_admin_settings: &'a AccountInfo<'a>,
    fee_receiver_ata: &'a AccountInfo<'a>
) -> ProgramResult {
    let seeds = &[SOLAUTO_ADMIN_SETTINGS_ACCOUNT_SEEDS];
    let (pda, _bump) = Pubkey::find_program_address(seeds, &crate::ID);
    if &pda != solauto_admin_settings.key {
        return Err(SolautoError::IncorrectSolautoSettingsAccount.into());
    }

    let solauto_admin_settings = DeserializedAccount::<SolautoAdminSettings>
        ::deserialize(Some(solauto_admin_settings))?
        .unwrap();

    let associated_token_account = get_associated_token_address(
        &solauto_admin_settings.data.fees_wallet,
        &solauto_admin_settings.data.fees_token_mint
    );

    if &associated_token_account != fee_receiver_ata.key {
        Err(SolautoError::IncorrectFeesReceiverAccount.into())
    } else {
        Ok(())
    }
}

pub fn require_accounts(accounts: &[Option<&AccountInfo>]) -> ProgramResult {
    for acc in accounts.into_iter() {
        if acc.is_none() {
            return Err(SolautoError::MissingRequiredAccounts.into());
        }
    }
    Ok(())
}

pub fn validate_marginfi_protocol_interaction_ix(
    ctx: &Context<MarginfiProtocolInteractionAccounts>,
    action: &SolautoAction
) -> ProgramResult {
    let require_supply_accounts = || {
        return require_accounts(
            &(
                [
                    // TODO
                ]
            )
        );
    };

    let require_debt_accounts = || {
        return require_accounts(
            &(
                [
                    // TODO
                ]
            )
        );
    };

    match action {
        SolautoAction::Deposit(_) => {
            require_supply_accounts()?;
        }
        SolautoAction::Withdraw(_) => {
            require_supply_accounts()?;
        }
        SolautoAction::Borrow(_) => {
            require_debt_accounts()?;
        }
        SolautoAction::Repay(_) => {
            require_debt_accounts()?;
        }
    }

    Ok(())
}

pub fn validate_solend_protocol_interaction_ix(
    ctx: &Context<SolendProtocolInteractionAccounts>,
    action: &SolautoAction
) -> ProgramResult {
    let require_supply_accounts = || {
        return require_accounts(
            &[
                ctx.accounts.supply_reserve,
                ctx.accounts.supply_reserve_pyth_price_oracle,
                ctx.accounts.supply_reserve_switchboard_oracle,
                ctx.accounts.supply_liquidity_mint,
                ctx.accounts.source_supply_liquidity_ta,
                ctx.accounts.reserve_supply_liquidity_ta,
                ctx.accounts.supply_collateral_mint,
                ctx.accounts.supply_collateral_mint,
                ctx.accounts.source_supply_collateral_ta,
                ctx.accounts.reserve_supply_collateral_ta,
            ]
        );
    };

    let require_debt_accounts = || {
        return require_accounts(
            &[
                ctx.accounts.debt_reserve,
                ctx.accounts.debt_reserve_fee_receiver_ta,
                ctx.accounts.debt_liquidity_mint,
                ctx.accounts.source_debt_liquidity_ta,
                ctx.accounts.reserve_debt_liquidity_ta,
            ]
        );
    };

    match action {
        SolautoAction::Deposit(_) => {
            require_supply_accounts()?;
        }
        SolautoAction::Withdraw(_) => {
            require_supply_accounts()?;
        }
        SolautoAction::Borrow(_) => {
            require_debt_accounts()?;
        }
        SolautoAction::Repay(_) => {
            require_debt_accounts()?;
        }
    }

    Ok(())
}

pub fn validate_referral_accounts(std_accounts: &SolautoStandardAccounts) -> ProgramResult {
    if std_accounts.authority_referral_state.is_none() {
        return Ok(());
    }

    let authority = if !std_accounts.solauto_position.is_none() {
        &std_accounts.solauto_position.as_ref().unwrap().data.authority
    } else {
        std_accounts.signer.key
    };

    let referral_state_seeds = &[authority.as_ref(), b"referrals"];
    let (referral_state_pda, _bump) = Pubkey::find_program_address(
        referral_state_seeds,
        &crate::ID
    );
    if
        &referral_state_pda !=
        std_accounts.authority_referral_state.as_ref().unwrap().account_info.key
    {
        msg!("Invalid referral position account given for the provided authority");
        return Err(ProgramError::InvalidAccountData.into());
    }

    let authority_referred_by_ta = std_accounts.authority_referral_state
        .as_ref()
        .unwrap().data.referred_by_ta;
    if !authority_referred_by_ta.is_none() && std_accounts.referred_by_ta.is_none() {
        msg!("Missing referred_by token account when this authority account has been referred");
        return Err(ProgramError::InvalidAccountData.into());
    }
    if &authority_referred_by_ta.unwrap() != std_accounts.referred_by_ta.unwrap().key {
        msg!(
            "Provided incorrect referred_by_ta according to the given authority referral position"
        );
        return Err(ProgramError::InvalidAccountData.into());
    }

    Ok(())
}

pub fn validate_source_token_account(
    std_accounts: &SolautoStandardAccounts,
    source_token_account: &AccountInfo,
    token_mint: &AccountInfo
) -> ProgramResult {
    let obligation_owner = get_owner(&std_accounts.solauto_position, std_accounts.signer);
    if
        source_token_account.key !=
        &get_associated_token_address(obligation_owner.key, token_mint.key)
    {
        msg!("Invalid source token account provided for the given obligation owner & token mint");
        return Err(ProgramError::InvalidAccountData.into());
    }
    Ok(())
}
