use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    processors::{
        general::*,
        marginfi::*,
        solend::{
            process_solend_interaction_instruction,
            process_solend_open_position_instruction,
            process_solend_rebalance,
            process_solend_refresh_data,
        },
    },
    types::instruction::Instruction,
};

entrypoint!(process_instruction);

fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8]
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(data)?;
    match instruction {
        Instruction::UpdateSolautoAdminSettings =>
            process_update_solauto_admin_settings_instruction(accounts),

        Instruction::MarginfiOpenPosition(args) =>
            process_marginfi_open_position_instruction(accounts, args),
        Instruction::SolendOpenPosition(args) =>
            process_solend_open_position_instruction(accounts, args),

        Instruction::UpdatePosition(_settings) => process_update_position_instruction(),
        Instruction::ClosePosition => process_close_position_instruction(),

        Instruction::MarginfiRefreshData => process_marginfi_refresh_data(accounts),
        Instruction::SolendRefreshData => process_solend_refresh_data(accounts),

        Instruction::MarginfiProtocolInteraction(action) =>
            process_marginfi_interaction_instruction(accounts, action),
        Instruction::SolendProtocolInteraction(action) =>
            process_solend_interaction_instruction(accounts, action),

        Instruction::MarginfiRebalance(target_liq_utilization_rate_bps) =>
            process_marginfi_rebalance(accounts, target_liq_utilization_rate_bps),
        Instruction::SolendRebalance(target_liq_utilization_rate_bps) =>
            process_solend_rebalance(accounts, target_liq_utilization_rate_bps),
    }
}
