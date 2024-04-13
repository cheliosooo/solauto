use std::str::FromStr;
use solana_program::{ account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey };
use spl_associated_token_account::get_associated_token_address;

use crate::{
    constants::WSOL_MINT_ADDRESS,
    types::{
        instruction::PositionData,
        shared::{
            DeserializedAccount,
            GeneralPositionData,
            LendingPlatform,
            Position,
            RefferalState,
            REFERRAL_ACCOUNT_SPACE,
        },
    },
};

use super::{
    ix_utils,
    solana_utils::{ account_is_rent_exempt, init_ata_if_needed, init_new_account },
};

pub fn get_owner<'a, 'b>(
    solauto_position: &'b Option<DeserializedAccount<'a, Position>>,
    signer: &'a AccountInfo<'a>
) -> &'a AccountInfo<'a> {
    if !solauto_position.is_none() {
        solauto_position.as_ref().unwrap().account_info
    } else {
        signer
    }
}

pub fn create_new_solauto_position<'a>(
    signer: &AccountInfo<'a>,
    solauto_position: Option<&'a AccountInfo<'a>>,
    new_position_data: Option<PositionData>,
    lending_platform: LendingPlatform
) -> Result<Option<DeserializedAccount<'a, Position>>, ProgramError> {
    let data = if !new_position_data.is_none() {
        let data = new_position_data.as_ref().unwrap();
        Some(Position {
            position_id: data.position_id,
            authority: *signer.key,
            setting_params: data.setting_params.clone(),
            general_data: GeneralPositionData::default(),
            lending_platform,
            marginfi_data: data.marginfi_data.clone(),
            solend_data: data.solend_data.clone(),
            kamino_data: data.kamino_data.clone(),
        })
    } else {
        None
    };

    if !data.is_none() {
        Ok(
            Some(DeserializedAccount::<Position> {
                account_info: solauto_position.unwrap(),
                data: Box::new(data.unwrap()),
            })
        )
    } else {
        Ok(None)
    }
}

pub fn get_or_create_referral_state<'a>(
    system_program: &'a AccountInfo<'a>,
    token_program: &'a AccountInfo<'a>,
    rent: &'a AccountInfo<'a>,
    signer: &'a AccountInfo<'a>,
    authority: &'a AccountInfo<'a>,
    referral_state: &'a AccountInfo<'a>,
    referral_fees_mint: &'a AccountInfo<'a>,
    referral_state_ta: &'a AccountInfo<'a>,
    referred_by_state: Option<&'a AccountInfo<'a>>,
    referred_by_ta: Option<&'a AccountInfo<'a>>
) -> Result<DeserializedAccount<'a, RefferalState>, ProgramError> {
    let wsol_mint = Pubkey::from_str(WSOL_MINT_ADDRESS).expect(
        "Failed to create pubkey from WSOL mint address"
    );
    let validate_correct_token_account = |wallet: &AccountInfo, token_account: &AccountInfo| {
        let token_account_pubkey = get_associated_token_address(wallet.key, &wsol_mint);
        if &token_account_pubkey != token_account.key {
            msg!("Token account is not correct for the given token mint & wallet");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    };

    let referral_state_seeds = get_referral_account_seeds(authority);
    let (referral_state_pda, _bump) = Pubkey::find_program_address(
        referral_state_seeds.as_slice(),
        &crate::ID
    );
    if &referral_state_pda != referral_state.key {
        msg!("Invalid referral position account given for the provided authority");
        return Err(ProgramError::InvalidAccountData.into());
    }

    validate_correct_token_account(referral_state, referral_state_ta)?;
    if !referred_by_state.is_none() && !referred_by_ta.is_none() {
        validate_correct_token_account(referral_state, referral_state_ta)?;
        if referred_by_state.unwrap().owner != &crate::ID {
            msg!("Referred by position account is not owned by Solauto");
            return Err(ProgramError::InvalidAccountData.into());
        }
    }

    if account_is_rent_exempt(rent, referral_state)? {
        let mut referral_state_account = Some(
            DeserializedAccount::<RefferalState>::deserialize(Some(referral_state))?.unwrap()
        );

        if
            referral_state_account.as_ref().unwrap().data.referred_by_ta.is_none() &&
            !referred_by_ta.is_none()
        {
            referral_state_account.as_mut().unwrap().data.referred_by_ta = Some(
                referred_by_ta.unwrap().key.clone()
            );
        }

        ix_utils::update_data(&mut referral_state_account)?;
        Ok(referral_state_account.unwrap())
    } else {
        init_new_account(
            system_program,
            rent,
            signer,
            referral_state,
            &crate::ID,
            referral_state_seeds[..].to_vec(),
            REFERRAL_ACCOUNT_SPACE
        )?;

        let fees_mint = referral_fees_mint.key;
        if fees_mint != &wsol_mint {
            msg!(format!("Referral fees mint must be wSOL {}", WSOL_MINT_ADDRESS).as_str());
            return Err(ProgramError::InvalidAccountData.into());
        }

        init_ata_if_needed(
            token_program,
            system_program,
            rent,
            signer,
            referral_state_ta,
            referral_state_ta,
            referral_fees_mint
        )?;

        if !referred_by_state.is_none() && !referred_by_ta.is_none() {
            init_ata_if_needed(
                token_program,
                system_program,
                rent,
                signer,
                referred_by_state.unwrap(),
                referred_by_ta.unwrap(),
                referral_fees_mint
            )?;
        }

        let data = Box::new(RefferalState {
            authority: authority.key.clone(),
            referred_by_ta: referred_by_ta.map_or(None, |r| Some(r.key.clone())),
            fees_ta: referral_state_ta.key.clone(),
            fees_mint: wsol_mint.clone(),
        });

        let deserialized_account = DeserializedAccount {
            account_info: referral_state,
            data,
        };

        Ok(deserialized_account)
    }
}

pub fn get_referral_account_seeds<'a>(authority: &'a AccountInfo<'a>) -> Vec<&[u8]> {
    vec![authority.key.as_ref(), b"referrals"]
}