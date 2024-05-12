use solana_program::{ account_info::AccountInfo, entrypoint::ProgramResult, msg };
use spl_token::state::Account as TokenAccount;

use crate::{
    instructions::{close_position, update_position},
    types::{
        instruction::{
            accounts::{ ClosePositionAccounts, UpdatePositionAccounts },
            UpdatePositionData,
        },
        shared::{ DeserializedAccount, SolautoError, SolautoPosition },
    },
    utils::validation_utils,
};

pub fn process_update_position_instruction<'a>(
    accounts: &'a [AccountInfo<'a>],
    new_data: UpdatePositionData
) -> ProgramResult {
    let ctx = UpdatePositionAccounts::context(accounts)?;
    let solauto_position = DeserializedAccount::<SolautoPosition>
        ::deserialize(Some(ctx.accounts.solauto_position))?
        .unwrap();

    validation_utils::validate_signer(ctx.accounts.signer, &solauto_position, true)?;
    if solauto_position.data.self_managed {
        msg!("Cannot update a self-managed position");
        return Err(SolautoError::IncorrectAccounts.into());
    }

    update_position::update_position(ctx, solauto_position, new_data)
}

pub fn process_close_position_instruction<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = ClosePositionAccounts::context(accounts)?;
    let solauto_position = DeserializedAccount::<SolautoPosition>
        ::deserialize(Some(ctx.accounts.solauto_position))?
        .unwrap();
    let position_debt_liquidity_ta = DeserializedAccount::<TokenAccount>::unpack(
        ctx.accounts.position_debt_liquidity_ta
    )?;

    validation_utils::validate_signer(ctx.accounts.signer, &solauto_position, true)?;
    if solauto_position.data.self_managed {
        msg!("Cannot close a self-managed position");
        return Err(SolautoError::IncorrectAccounts.into());
    }

    validation_utils::validate_token_accounts(
        ctx.accounts.signer,
        &solauto_position,
        &DeserializedAccount::<TokenAccount>
        ::unpack(Some(ctx.accounts.position_supply_liquidity_ta))?.unwrap(),
        position_debt_liquidity_ta.as_ref()
    )?;

    close_position::close_position(ctx, solauto_position, position_debt_liquidity_ta)
}