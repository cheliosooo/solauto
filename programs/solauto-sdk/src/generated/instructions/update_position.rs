//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::SolautoSettingsParameters;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct UpdatePosition {
    pub signer: solana_program::pubkey::Pubkey,

    pub solauto_position: solana_program::pubkey::Pubkey,
}

impl UpdatePosition {
    pub fn instruction(
        &self,
        args: UpdatePositionInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: UpdatePositionInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.signer,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.solauto_position,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = UpdatePositionInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::SOLAUTO_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct UpdatePositionInstructionData {
    discriminator: u8,
}

impl UpdatePositionInstructionData {
    fn new() -> Self {
        Self { discriminator: 3 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePositionInstructionArgs {
    pub solauto_settings_parameters: SolautoSettingsParameters,
}

/// Instruction builder for `UpdatePosition`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[writable]` solauto_position
#[derive(Default)]
pub struct UpdatePositionBuilder {
    signer: Option<solana_program::pubkey::Pubkey>,
    solauto_position: Option<solana_program::pubkey::Pubkey>,
    solauto_settings_parameters: Option<SolautoSettingsParameters>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpdatePositionBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.signer = Some(signer);
        self
    }
    #[inline(always)]
    pub fn solauto_position(
        &mut self,
        solauto_position: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.solauto_position = Some(solauto_position);
        self
    }
    #[inline(always)]
    pub fn solauto_settings_parameters(
        &mut self,
        solauto_settings_parameters: SolautoSettingsParameters,
    ) -> &mut Self {
        self.solauto_settings_parameters = Some(solauto_settings_parameters);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = UpdatePosition {
            signer: self.signer.expect("signer is not set"),
            solauto_position: self.solauto_position.expect("solauto_position is not set"),
        };
        let args = UpdatePositionInstructionArgs {
            solauto_settings_parameters: self
                .solauto_settings_parameters
                .clone()
                .expect("solauto_settings_parameters is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `update_position` CPI accounts.
pub struct UpdatePositionCpiAccounts<'a, 'b> {
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub solauto_position: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `update_position` CPI instruction.
pub struct UpdatePositionCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub solauto_position: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: UpdatePositionInstructionArgs,
}

impl<'a, 'b> UpdatePositionCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UpdatePositionCpiAccounts<'a, 'b>,
        args: UpdatePositionInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            signer: accounts.signer,
            solauto_position: accounts.solauto_position,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.signer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.solauto_position.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = UpdatePositionInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SOLAUTO_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(2 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.signer.clone());
        account_infos.push(self.solauto_position.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `UpdatePosition` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[writable]` solauto_position
pub struct UpdatePositionCpiBuilder<'a, 'b> {
    instruction: Box<UpdatePositionCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdatePositionCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpdatePositionCpiBuilderInstruction {
            __program: program,
            signer: None,
            solauto_position: None,
            solauto_settings_parameters: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn signer(
        &mut self,
        signer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.signer = Some(signer);
        self
    }
    #[inline(always)]
    pub fn solauto_position(
        &mut self,
        solauto_position: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.solauto_position = Some(solauto_position);
        self
    }
    #[inline(always)]
    pub fn solauto_settings_parameters(
        &mut self,
        solauto_settings_parameters: SolautoSettingsParameters,
    ) -> &mut Self {
        self.instruction.solauto_settings_parameters = Some(solauto_settings_parameters);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = UpdatePositionInstructionArgs {
            solauto_settings_parameters: self
                .instruction
                .solauto_settings_parameters
                .clone()
                .expect("solauto_settings_parameters is not set"),
        };
        let instruction = UpdatePositionCpi {
            __program: self.instruction.__program,

            signer: self.instruction.signer.expect("signer is not set"),

            solauto_position: self
                .instruction
                .solauto_position
                .expect("solauto_position is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct UpdatePositionCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    solauto_position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    solauto_settings_parameters: Option<SolautoSettingsParameters>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
