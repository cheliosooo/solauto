use solana_program::{ account_info::AccountInfo, entrypoint::ProgramResult };

use crate::{
    instructions::{ open_position, protocol_interaction, rebalance, refresh },
    types::{
        instruction::{
            accounts::{
                MarginfiOpenPositionAccounts,
                MarginfiProtocolInteractionAccounts,
                MarginfiRebalanceAccounts,
                MarginfiRefreshDataAccounts,
            },
            OptionalLiqUtilizationRateBps,
            PositionData,
            SolautoStandardAccounts,
        },
        shared::{ DeserializedAccount, LendingPlatform, Position, SolautoAction },
    },
    utils::*,
};

pub fn process_marginfi_open_position_instruction<'a>(
    accounts: &'a [AccountInfo<'a>],
    position_data: Option<PositionData>
) -> ProgramResult {
    let ctx = MarginfiOpenPositionAccounts::context(accounts)?;
    let solauto_position = solauto_utils::create_new_solauto_position(
        ctx.accounts.signer,
        ctx.accounts.solauto_position,
        position_data,
        LendingPlatform::Marginfi
    )?;
    let std_accounts = SolautoStandardAccounts {
        signer: ctx.accounts.signer,
        lending_protocol: ctx.accounts.marginfi_program,
        system_program: ctx.accounts.system_program,
        token_program: ctx.accounts.token_program,
        ata_program: ctx.accounts.ata_program,
        ix_sysvar: None,
        solauto_position,
        solauto_admin_settings: None,
        solauto_fees_receiver_ata: None,
    };
    validation_utils::generic_instruction_validation(&std_accounts, true, LendingPlatform::Marginfi)?;
    open_position::marginfi_open_position(ctx, std_accounts.solauto_position)
}

pub fn process_marginfi_refresh_data<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = MarginfiRefreshDataAccounts::context(accounts)?;
    let solauto_position = DeserializedAccount::<Position>::deserialize(
        ctx.accounts.solauto_position
    )?;
    validation_utils::validate_program_account(
        &ctx.accounts.marginfi_program,
        LendingPlatform::Marginfi
    )?;
    refresh::marginfi_refresh_accounts(ctx, solauto_position)
}

pub fn process_marginfi_interaction_instruction<'a>(
    accounts: &'a [AccountInfo<'a>],
    action: SolautoAction
) -> ProgramResult {
    let ctx = MarginfiProtocolInteractionAccounts::context(accounts)?;
    let solauto_position = DeserializedAccount::<Position>::deserialize(
        ctx.accounts.solauto_position
    )?;
    let std_accounts = SolautoStandardAccounts {
        signer: ctx.accounts.signer,
        lending_protocol: ctx.accounts.marginfi_program,
        system_program: ctx.accounts.system_program,
        token_program: ctx.accounts.token_program,
        ata_program: ctx.accounts.ata_program,
        ix_sysvar: None,
        solauto_position,
        solauto_admin_settings: None,
        solauto_fees_receiver_ata: None,
    };
    validation_utils::generic_instruction_validation(
        &std_accounts,
        true,
        LendingPlatform::Marginfi
    )?;
    validation_utils::validate_marginfi_protocol_interaction_ix(&ctx, &action)?;
    protocol_interaction::marginfi_interaction(ctx, std_accounts, action)
}

pub fn process_marginfi_rebalance<'a>(
    accounts: &'a [AccountInfo<'a>],
    target_liq_utilization_rate_bps: OptionalLiqUtilizationRateBps
) -> ProgramResult {
    let ctx = MarginfiRebalanceAccounts::context(accounts)?;
    let solauto_position = DeserializedAccount::<Position>::deserialize(
        ctx.accounts.solauto_position
    )?;
    let std_accounts = SolautoStandardAccounts {
        signer: ctx.accounts.signer,
        lending_protocol: ctx.accounts.marginfi_program,
        system_program: ctx.accounts.system_program,
        token_program: ctx.accounts.token_program,
        ata_program: ctx.accounts.ata_program,
        ix_sysvar: Some(ctx.accounts.ix_sysvar),
        solauto_position,
        solauto_admin_settings: Some(ctx.accounts.solauto_admin_settings),
        solauto_fees_receiver_ata: Some(ctx.accounts.solauto_fees_receiver_ata),
    };
    validation_utils::generic_instruction_validation(
        &std_accounts,
        false,
        LendingPlatform::Marginfi
    )?;
    rebalance::marginfi_rebalance(ctx, std_accounts, target_liq_utilization_rate_bps)
}
