use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, hash::hash, instruction::Instruction,
    program::invoke, program_error::ProgramError, pubkey::Pubkey,
    sysvar::instructions::load_instruction_at_checked,
};

use super::{solana_utils::invoke_signed_with_seed, solauto_utils::get_solauto_position_seeds};
use crate::types::shared::{DeserializedAccount, SolautoPosition};

pub fn update_data<T: BorshSerialize>(account: &mut DeserializedAccount<T>) -> ProgramResult {
    account
        .data
        .serialize(&mut &mut account.account_info.data.borrow_mut()[..])?;
    Ok(())
}

pub fn invoke_instruction(
    instruction: Instruction,
    account_infos: &[AccountInfo],
    solauto_position: &DeserializedAccount<SolautoPosition>,
) -> ProgramResult {
    if solauto_position.data.self_managed {
        invoke(&instruction, account_infos)?;
    } else {
        invoke_signed_with_seed(
            &instruction,
            account_infos,
            get_solauto_position_seeds(&solauto_position)
                .iter()
                .map(|v| v.as_slice())
                .collect(),
        )?;
    }
    Ok(())
}

pub fn get_relative_instruction(
    ixs_sysvar: &AccountInfo,
    current_ix_idx: u16,
    relative_idx: i16,
    total_ix_in_tx: u16,
) -> Result<Option<Instruction>, ProgramError> {
    if (current_ix_idx as i16) + relative_idx > 0
        && (current_ix_idx as i16) + relative_idx < (total_ix_in_tx as i16)
    {
        Ok(Some(load_instruction_at_checked(
            ((current_ix_idx as i16) + relative_idx) as usize,
            ixs_sysvar,
        )?))
    } else {
        Ok(None)
    }
}

pub fn get_anchor_ix_discriminator(instruction_name: &str) -> u64 {
    let concatenated = format!("global:{}", instruction_name.to_lowercase());
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&hash(concatenated.as_bytes()).to_bytes()[..8]);
    println!("{:?}", sighash);
    u64::from_le_bytes(sighash)
}

pub struct InstructionChecker {
    program_id: Pubkey,
    ix_discriminators: Option<Vec<u64>>,
}
impl InstructionChecker {
    pub fn from(program_id: Pubkey, ix_discriminators: Option<Vec<u64>>) -> Self {
        Self {
            program_id,
            ix_discriminators,
        }
    }
    pub fn from_anchor(program_id: Pubkey, ix_names: Vec<&str>) -> Self {
        let mut ix_discriminators: Vec<u64> = Vec::new();
        for name in ix_names.iter() {
            ix_discriminators.push(get_anchor_ix_discriminator(name));
        }
        Self {
            program_id,
            ix_discriminators: Some(ix_discriminators),
        }
    }
    pub fn matches(&self, ix: &Option<Instruction>) -> bool {
        if ix.is_none() {
            return false;
        }

        let instruction = ix.as_ref().unwrap();
        if instruction.program_id == self.program_id {
            if instruction.data.len() >= 8 {
                let discriminator: [u8; 8] = instruction.data[0..8]
                    .try_into()
                    .expect("Slice with incorrect length");

                if self.ix_discriminators.is_none()
                    || self
                        .ix_discriminators
                        .as_ref()
                        .unwrap()
                        .iter()
                        .any(|&x| x == u64::from_le_bytes(discriminator))
                {
                    return true;
                }
            }
        }

        false
    }
}
