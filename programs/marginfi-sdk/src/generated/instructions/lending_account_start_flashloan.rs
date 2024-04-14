//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct LendingAccountStartFlashloan {
    pub marginfi_account: solana_program::pubkey::Pubkey,

    pub signer: solana_program::pubkey::Pubkey,

    pub ixs_sysvar: solana_program::pubkey::Pubkey,
}

impl LendingAccountStartFlashloan {
    pub fn instruction(
        &self,
        args: LendingAccountStartFlashloanInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: LendingAccountStartFlashloanInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.marginfi_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.ixs_sysvar,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = LendingAccountStartFlashloanInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::MARGINFI_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct LendingAccountStartFlashloanInstructionData {}

impl LendingAccountStartFlashloanInstructionData {
    fn new() -> Self {
        Self {}
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LendingAccountStartFlashloanInstructionArgs {
    pub end_index: u64,
}

/// Instruction builder for `LendingAccountStartFlashloan`.
///
/// ### Accounts:
///
///   0. `[writable]` marginfi_account
///   1. `[signer]` signer
///   2. `[]` ixs_sysvar
#[derive(Default)]
pub struct LendingAccountStartFlashloanBuilder {
    marginfi_account: Option<solana_program::pubkey::Pubkey>,
    signer: Option<solana_program::pubkey::Pubkey>,
    ixs_sysvar: Option<solana_program::pubkey::Pubkey>,
    end_index: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl LendingAccountStartFlashloanBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn marginfi_account(
        &mut self,
        marginfi_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.marginfi_account = Some(marginfi_account);
        self
    }
    #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.signer = Some(signer);
        self
    }
    #[inline(always)]
    pub fn ixs_sysvar(&mut self, ixs_sysvar: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ixs_sysvar = Some(ixs_sysvar);
        self
    }
    #[inline(always)]
    pub fn end_index(&mut self, end_index: u64) -> &mut Self {
        self.end_index = Some(end_index);
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
        let accounts = LendingAccountStartFlashloan {
            marginfi_account: self.marginfi_account.expect("marginfi_account is not set"),
            signer: self.signer.expect("signer is not set"),
            ixs_sysvar: self.ixs_sysvar.expect("ixs_sysvar is not set"),
        };
        let args = LendingAccountStartFlashloanInstructionArgs {
            end_index: self.end_index.clone().expect("end_index is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `lending_account_start_flashloan` CPI accounts.
pub struct LendingAccountStartFlashloanCpiAccounts<'a, 'b> {
    pub marginfi_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub ixs_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `lending_account_start_flashloan` CPI instruction.
pub struct LendingAccountStartFlashloanCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub marginfi_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub ixs_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: LendingAccountStartFlashloanInstructionArgs,
}

impl<'a, 'b> LendingAccountStartFlashloanCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: LendingAccountStartFlashloanCpiAccounts<'a, 'b>,
        args: LendingAccountStartFlashloanInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            marginfi_account: accounts.marginfi_account,
            signer: accounts.signer,
            ixs_sysvar: accounts.ixs_sysvar,
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
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.marginfi_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.signer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ixs_sysvar.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = LendingAccountStartFlashloanInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MARGINFI_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.marginfi_account.clone());
        account_infos.push(self.signer.clone());
        account_infos.push(self.ixs_sysvar.clone());
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

/// Instruction builder for `LendingAccountStartFlashloan` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` marginfi_account
///   1. `[signer]` signer
///   2. `[]` ixs_sysvar
pub struct LendingAccountStartFlashloanCpiBuilder<'a, 'b> {
    instruction: Box<LendingAccountStartFlashloanCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> LendingAccountStartFlashloanCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(LendingAccountStartFlashloanCpiBuilderInstruction {
            __program: program,
            marginfi_account: None,
            signer: None,
            ixs_sysvar: None,
            end_index: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn marginfi_account(
        &mut self,
        marginfi_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.marginfi_account = Some(marginfi_account);
        self
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
    pub fn ixs_sysvar(
        &mut self,
        ixs_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.ixs_sysvar = Some(ixs_sysvar);
        self
    }
    #[inline(always)]
    pub fn end_index(&mut self, end_index: u64) -> &mut Self {
        self.instruction.end_index = Some(end_index);
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
        let args = LendingAccountStartFlashloanInstructionArgs {
            end_index: self
                .instruction
                .end_index
                .clone()
                .expect("end_index is not set"),
        };
        let instruction = LendingAccountStartFlashloanCpi {
            __program: self.instruction.__program,

            marginfi_account: self
                .instruction
                .marginfi_account
                .expect("marginfi_account is not set"),

            signer: self.instruction.signer.expect("signer is not set"),

            ixs_sysvar: self.instruction.ixs_sysvar.expect("ixs_sysvar is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct LendingAccountStartFlashloanCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    marginfi_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ixs_sysvar: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    end_index: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
