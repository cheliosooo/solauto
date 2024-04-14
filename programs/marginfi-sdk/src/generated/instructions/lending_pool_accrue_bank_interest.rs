//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct LendingPoolAccrueBankInterest {
    pub marginfi_group: solana_program::pubkey::Pubkey,

    pub bank: solana_program::pubkey::Pubkey,
}

impl LendingPoolAccrueBankInterest {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.marginfi_group,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.bank, false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = LendingPoolAccrueBankInterestInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::MARGINFI_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct LendingPoolAccrueBankInterestInstructionData {}

impl LendingPoolAccrueBankInterestInstructionData {
    fn new() -> Self {
        Self {}
    }
}

/// Instruction builder for `LendingPoolAccrueBankInterest`.
///
/// ### Accounts:
///
///   0. `[]` marginfi_group
///   1. `[writable]` bank
#[derive(Default)]
pub struct LendingPoolAccrueBankInterestBuilder {
    marginfi_group: Option<solana_program::pubkey::Pubkey>,
    bank: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl LendingPoolAccrueBankInterestBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn marginfi_group(&mut self, marginfi_group: solana_program::pubkey::Pubkey) -> &mut Self {
        self.marginfi_group = Some(marginfi_group);
        self
    }
    #[inline(always)]
    pub fn bank(&mut self, bank: solana_program::pubkey::Pubkey) -> &mut Self {
        self.bank = Some(bank);
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
        let accounts = LendingPoolAccrueBankInterest {
            marginfi_group: self.marginfi_group.expect("marginfi_group is not set"),
            bank: self.bank.expect("bank is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `lending_pool_accrue_bank_interest` CPI accounts.
pub struct LendingPoolAccrueBankInterestCpiAccounts<'a, 'b> {
    pub marginfi_group: &'b solana_program::account_info::AccountInfo<'a>,

    pub bank: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `lending_pool_accrue_bank_interest` CPI instruction.
pub struct LendingPoolAccrueBankInterestCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub marginfi_group: &'b solana_program::account_info::AccountInfo<'a>,

    pub bank: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> LendingPoolAccrueBankInterestCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: LendingPoolAccrueBankInterestCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            marginfi_group: accounts.marginfi_group,
            bank: accounts.bank,
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.marginfi_group.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.bank.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = LendingPoolAccrueBankInterestInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MARGINFI_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(2 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.marginfi_group.clone());
        account_infos.push(self.bank.clone());
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

/// Instruction builder for `LendingPoolAccrueBankInterest` via CPI.
///
/// ### Accounts:
///
///   0. `[]` marginfi_group
///   1. `[writable]` bank
pub struct LendingPoolAccrueBankInterestCpiBuilder<'a, 'b> {
    instruction: Box<LendingPoolAccrueBankInterestCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> LendingPoolAccrueBankInterestCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(LendingPoolAccrueBankInterestCpiBuilderInstruction {
            __program: program,
            marginfi_group: None,
            bank: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn marginfi_group(
        &mut self,
        marginfi_group: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.marginfi_group = Some(marginfi_group);
        self
    }
    #[inline(always)]
    pub fn bank(&mut self, bank: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.bank = Some(bank);
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
        let instruction = LendingPoolAccrueBankInterestCpi {
            __program: self.instruction.__program,

            marginfi_group: self
                .instruction
                .marginfi_group
                .expect("marginfi_group is not set"),

            bank: self.instruction.bank.expect("bank is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct LendingPoolAccrueBankInterestCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    marginfi_group: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bank: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
