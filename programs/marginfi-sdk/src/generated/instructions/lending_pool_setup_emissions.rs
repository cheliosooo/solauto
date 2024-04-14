//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct LendingPoolSetupEmissions {
    pub marginfi_group: solana_program::pubkey::Pubkey,

    pub admin: solana_program::pubkey::Pubkey,

    pub bank: solana_program::pubkey::Pubkey,

    pub emissions_mint: solana_program::pubkey::Pubkey,

    pub emissions_auth: solana_program::pubkey::Pubkey,

    pub emissions_token_account: solana_program::pubkey::Pubkey,

    pub emissions_funding_account: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

impl LendingPoolSetupEmissions {
    pub fn instruction(
        &self,
        args: LendingPoolSetupEmissionsInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: LendingPoolSetupEmissionsInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.marginfi_group,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.admin, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.bank, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.emissions_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.emissions_auth,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.emissions_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.emissions_funding_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = LendingPoolSetupEmissionsInstructionData::new()
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
struct LendingPoolSetupEmissionsInstructionData {}

impl LendingPoolSetupEmissionsInstructionData {
    fn new() -> Self {
        Self {}
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LendingPoolSetupEmissionsInstructionArgs {
    pub flags: u64,
    pub rate: u64,
    pub total_emissions: u64,
}

/// Instruction builder for `LendingPoolSetupEmissions`.
///
/// ### Accounts:
///
///   0. `[]` marginfi_group
///   1. `[writable, signer]` admin
///   2. `[writable]` bank
///   3. `[]` emissions_mint
///   4. `[]` emissions_auth
///   5. `[writable]` emissions_token_account
///   6. `[writable]` emissions_funding_account
///   7. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   8. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Default)]
pub struct LendingPoolSetupEmissionsBuilder {
    marginfi_group: Option<solana_program::pubkey::Pubkey>,
    admin: Option<solana_program::pubkey::Pubkey>,
    bank: Option<solana_program::pubkey::Pubkey>,
    emissions_mint: Option<solana_program::pubkey::Pubkey>,
    emissions_auth: Option<solana_program::pubkey::Pubkey>,
    emissions_token_account: Option<solana_program::pubkey::Pubkey>,
    emissions_funding_account: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    flags: Option<u64>,
    rate: Option<u64>,
    total_emissions: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl LendingPoolSetupEmissionsBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn marginfi_group(&mut self, marginfi_group: solana_program::pubkey::Pubkey) -> &mut Self {
        self.marginfi_group = Some(marginfi_group);
        self
    }
    #[inline(always)]
    pub fn admin(&mut self, admin: solana_program::pubkey::Pubkey) -> &mut Self {
        self.admin = Some(admin);
        self
    }
    #[inline(always)]
    pub fn bank(&mut self, bank: solana_program::pubkey::Pubkey) -> &mut Self {
        self.bank = Some(bank);
        self
    }
    #[inline(always)]
    pub fn emissions_mint(&mut self, emissions_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.emissions_mint = Some(emissions_mint);
        self
    }
    #[inline(always)]
    pub fn emissions_auth(&mut self, emissions_auth: solana_program::pubkey::Pubkey) -> &mut Self {
        self.emissions_auth = Some(emissions_auth);
        self
    }
    #[inline(always)]
    pub fn emissions_token_account(
        &mut self,
        emissions_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.emissions_token_account = Some(emissions_token_account);
        self
    }
    #[inline(always)]
    pub fn emissions_funding_account(
        &mut self,
        emissions_funding_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.emissions_funding_account = Some(emissions_funding_account);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn flags(&mut self, flags: u64) -> &mut Self {
        self.flags = Some(flags);
        self
    }
    #[inline(always)]
    pub fn rate(&mut self, rate: u64) -> &mut Self {
        self.rate = Some(rate);
        self
    }
    #[inline(always)]
    pub fn total_emissions(&mut self, total_emissions: u64) -> &mut Self {
        self.total_emissions = Some(total_emissions);
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
        let accounts = LendingPoolSetupEmissions {
            marginfi_group: self.marginfi_group.expect("marginfi_group is not set"),
            admin: self.admin.expect("admin is not set"),
            bank: self.bank.expect("bank is not set"),
            emissions_mint: self.emissions_mint.expect("emissions_mint is not set"),
            emissions_auth: self.emissions_auth.expect("emissions_auth is not set"),
            emissions_token_account: self
                .emissions_token_account
                .expect("emissions_token_account is not set"),
            emissions_funding_account: self
                .emissions_funding_account
                .expect("emissions_funding_account is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = LendingPoolSetupEmissionsInstructionArgs {
            flags: self.flags.clone().expect("flags is not set"),
            rate: self.rate.clone().expect("rate is not set"),
            total_emissions: self
                .total_emissions
                .clone()
                .expect("total_emissions is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `lending_pool_setup_emissions` CPI accounts.
pub struct LendingPoolSetupEmissionsCpiAccounts<'a, 'b> {
    pub marginfi_group: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub bank: &'b solana_program::account_info::AccountInfo<'a>,

    pub emissions_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub emissions_auth: &'b solana_program::account_info::AccountInfo<'a>,

    pub emissions_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub emissions_funding_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `lending_pool_setup_emissions` CPI instruction.
pub struct LendingPoolSetupEmissionsCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub marginfi_group: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub bank: &'b solana_program::account_info::AccountInfo<'a>,

    pub emissions_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub emissions_auth: &'b solana_program::account_info::AccountInfo<'a>,

    pub emissions_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub emissions_funding_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: LendingPoolSetupEmissionsInstructionArgs,
}

impl<'a, 'b> LendingPoolSetupEmissionsCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: LendingPoolSetupEmissionsCpiAccounts<'a, 'b>,
        args: LendingPoolSetupEmissionsInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            marginfi_group: accounts.marginfi_group,
            admin: accounts.admin,
            bank: accounts.bank,
            emissions_mint: accounts.emissions_mint,
            emissions_auth: accounts.emissions_auth,
            emissions_token_account: accounts.emissions_token_account,
            emissions_funding_account: accounts.emissions_funding_account,
            token_program: accounts.token_program,
            system_program: accounts.system_program,
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
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.marginfi_group.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.admin.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.bank.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.emissions_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.emissions_auth.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.emissions_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.emissions_funding_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = LendingPoolSetupEmissionsInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MARGINFI_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.marginfi_group.clone());
        account_infos.push(self.admin.clone());
        account_infos.push(self.bank.clone());
        account_infos.push(self.emissions_mint.clone());
        account_infos.push(self.emissions_auth.clone());
        account_infos.push(self.emissions_token_account.clone());
        account_infos.push(self.emissions_funding_account.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.system_program.clone());
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

/// Instruction builder for `LendingPoolSetupEmissions` via CPI.
///
/// ### Accounts:
///
///   0. `[]` marginfi_group
///   1. `[writable, signer]` admin
///   2. `[writable]` bank
///   3. `[]` emissions_mint
///   4. `[]` emissions_auth
///   5. `[writable]` emissions_token_account
///   6. `[writable]` emissions_funding_account
///   7. `[]` token_program
///   8. `[]` system_program
pub struct LendingPoolSetupEmissionsCpiBuilder<'a, 'b> {
    instruction: Box<LendingPoolSetupEmissionsCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> LendingPoolSetupEmissionsCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(LendingPoolSetupEmissionsCpiBuilderInstruction {
            __program: program,
            marginfi_group: None,
            admin: None,
            bank: None,
            emissions_mint: None,
            emissions_auth: None,
            emissions_token_account: None,
            emissions_funding_account: None,
            token_program: None,
            system_program: None,
            flags: None,
            rate: None,
            total_emissions: None,
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
    pub fn admin(&mut self, admin: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.admin = Some(admin);
        self
    }
    #[inline(always)]
    pub fn bank(&mut self, bank: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.bank = Some(bank);
        self
    }
    #[inline(always)]
    pub fn emissions_mint(
        &mut self,
        emissions_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.emissions_mint = Some(emissions_mint);
        self
    }
    #[inline(always)]
    pub fn emissions_auth(
        &mut self,
        emissions_auth: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.emissions_auth = Some(emissions_auth);
        self
    }
    #[inline(always)]
    pub fn emissions_token_account(
        &mut self,
        emissions_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.emissions_token_account = Some(emissions_token_account);
        self
    }
    #[inline(always)]
    pub fn emissions_funding_account(
        &mut self,
        emissions_funding_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.emissions_funding_account = Some(emissions_funding_account);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn flags(&mut self, flags: u64) -> &mut Self {
        self.instruction.flags = Some(flags);
        self
    }
    #[inline(always)]
    pub fn rate(&mut self, rate: u64) -> &mut Self {
        self.instruction.rate = Some(rate);
        self
    }
    #[inline(always)]
    pub fn total_emissions(&mut self, total_emissions: u64) -> &mut Self {
        self.instruction.total_emissions = Some(total_emissions);
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
        let args = LendingPoolSetupEmissionsInstructionArgs {
            flags: self.instruction.flags.clone().expect("flags is not set"),
            rate: self.instruction.rate.clone().expect("rate is not set"),
            total_emissions: self
                .instruction
                .total_emissions
                .clone()
                .expect("total_emissions is not set"),
        };
        let instruction = LendingPoolSetupEmissionsCpi {
            __program: self.instruction.__program,

            marginfi_group: self
                .instruction
                .marginfi_group
                .expect("marginfi_group is not set"),

            admin: self.instruction.admin.expect("admin is not set"),

            bank: self.instruction.bank.expect("bank is not set"),

            emissions_mint: self
                .instruction
                .emissions_mint
                .expect("emissions_mint is not set"),

            emissions_auth: self
                .instruction
                .emissions_auth
                .expect("emissions_auth is not set"),

            emissions_token_account: self
                .instruction
                .emissions_token_account
                .expect("emissions_token_account is not set"),

            emissions_funding_account: self
                .instruction
                .emissions_funding_account
                .expect("emissions_funding_account is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct LendingPoolSetupEmissionsCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    marginfi_group: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bank: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    emissions_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    emissions_auth: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    emissions_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    emissions_funding_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    flags: Option<u64>,
    rate: Option<u64>,
    total_emissions: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
