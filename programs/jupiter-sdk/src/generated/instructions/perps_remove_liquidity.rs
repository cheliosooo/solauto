//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct PerpsRemoveLiquidity {
    pub swap_program: solana_program::pubkey::Pubkey,

    pub owner: solana_program::pubkey::Pubkey,

    pub funding_or_receiving_account: solana_program::pubkey::Pubkey,

    pub lp_token_account: solana_program::pubkey::Pubkey,

    pub transfer_authority: solana_program::pubkey::Pubkey,

    pub perpetuals: solana_program::pubkey::Pubkey,

    pub pool: solana_program::pubkey::Pubkey,

    pub custody: solana_program::pubkey::Pubkey,

    pub custody_oracle_account: solana_program::pubkey::Pubkey,

    pub custody_token_account: solana_program::pubkey::Pubkey,

    pub lp_token_mint: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub event_authority: solana_program::pubkey::Pubkey,

    pub program: solana_program::pubkey::Pubkey,
}

impl PerpsRemoveLiquidity {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(14 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.swap_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.owner, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.funding_or_receiving_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.lp_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.transfer_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.perpetuals,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.custody,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.custody_oracle_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.custody_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.lp_token_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.event_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = PerpsRemoveLiquidityInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct PerpsRemoveLiquidityInstructionData {
    discriminator: [u8; 8],
}

impl PerpsRemoveLiquidityInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [79, 211, 232, 140, 8, 78, 220, 34],
        }
    }
}

/// Instruction builder for `PerpsRemoveLiquidity`.
///
/// ### Accounts:
///
///   0. `[]` swap_program
///   1. `[writable]` owner
///   2. `[writable]` funding_or_receiving_account
///   3. `[writable]` lp_token_account
///   4. `[]` transfer_authority
///   5. `[]` perpetuals
///   6. `[writable]` pool
///   7. `[writable]` custody
///   8. `[]` custody_oracle_account
///   9. `[writable]` custody_token_account
///   10. `[writable]` lp_token_mint
///   11. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   12. `[]` event_authority
///   13. `[]` program
#[derive(Default)]
pub struct PerpsRemoveLiquidityBuilder {
    swap_program: Option<solana_program::pubkey::Pubkey>,
    owner: Option<solana_program::pubkey::Pubkey>,
    funding_or_receiving_account: Option<solana_program::pubkey::Pubkey>,
    lp_token_account: Option<solana_program::pubkey::Pubkey>,
    transfer_authority: Option<solana_program::pubkey::Pubkey>,
    perpetuals: Option<solana_program::pubkey::Pubkey>,
    pool: Option<solana_program::pubkey::Pubkey>,
    custody: Option<solana_program::pubkey::Pubkey>,
    custody_oracle_account: Option<solana_program::pubkey::Pubkey>,
    custody_token_account: Option<solana_program::pubkey::Pubkey>,
    lp_token_mint: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    event_authority: Option<solana_program::pubkey::Pubkey>,
    program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl PerpsRemoveLiquidityBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn swap_program(&mut self, swap_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.swap_program = Some(swap_program);
        self
    }
    #[inline(always)]
    pub fn owner(&mut self, owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }
    #[inline(always)]
    pub fn funding_or_receiving_account(
        &mut self,
        funding_or_receiving_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.funding_or_receiving_account = Some(funding_or_receiving_account);
        self
    }
    #[inline(always)]
    pub fn lp_token_account(
        &mut self,
        lp_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.lp_token_account = Some(lp_token_account);
        self
    }
    #[inline(always)]
    pub fn transfer_authority(
        &mut self,
        transfer_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.transfer_authority = Some(transfer_authority);
        self
    }
    #[inline(always)]
    pub fn perpetuals(&mut self, perpetuals: solana_program::pubkey::Pubkey) -> &mut Self {
        self.perpetuals = Some(perpetuals);
        self
    }
    #[inline(always)]
    pub fn pool(&mut self, pool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.pool = Some(pool);
        self
    }
    #[inline(always)]
    pub fn custody(&mut self, custody: solana_program::pubkey::Pubkey) -> &mut Self {
        self.custody = Some(custody);
        self
    }
    #[inline(always)]
    pub fn custody_oracle_account(
        &mut self,
        custody_oracle_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.custody_oracle_account = Some(custody_oracle_account);
        self
    }
    #[inline(always)]
    pub fn custody_token_account(
        &mut self,
        custody_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.custody_token_account = Some(custody_token_account);
        self
    }
    #[inline(always)]
    pub fn lp_token_mint(&mut self, lp_token_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lp_token_mint = Some(lp_token_mint);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.event_authority = Some(event_authority);
        self
    }
    #[inline(always)]
    pub fn program(&mut self, program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.program = Some(program);
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
        let accounts = PerpsRemoveLiquidity {
            swap_program: self.swap_program.expect("swap_program is not set"),
            owner: self.owner.expect("owner is not set"),
            funding_or_receiving_account: self
                .funding_or_receiving_account
                .expect("funding_or_receiving_account is not set"),
            lp_token_account: self.lp_token_account.expect("lp_token_account is not set"),
            transfer_authority: self
                .transfer_authority
                .expect("transfer_authority is not set"),
            perpetuals: self.perpetuals.expect("perpetuals is not set"),
            pool: self.pool.expect("pool is not set"),
            custody: self.custody.expect("custody is not set"),
            custody_oracle_account: self
                .custody_oracle_account
                .expect("custody_oracle_account is not set"),
            custody_token_account: self
                .custody_token_account
                .expect("custody_token_account is not set"),
            lp_token_mint: self.lp_token_mint.expect("lp_token_mint is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `perps_remove_liquidity` CPI accounts.
pub struct PerpsRemoveLiquidityCpiAccounts<'a, 'b> {
    pub swap_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub funding_or_receiving_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub lp_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub lp_token_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `perps_remove_liquidity` CPI instruction.
pub struct PerpsRemoveLiquidityCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub swap_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub funding_or_receiving_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub lp_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub lp_token_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> PerpsRemoveLiquidityCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: PerpsRemoveLiquidityCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            swap_program: accounts.swap_program,
            owner: accounts.owner,
            funding_or_receiving_account: accounts.funding_or_receiving_account,
            lp_token_account: accounts.lp_token_account,
            transfer_authority: accounts.transfer_authority,
            perpetuals: accounts.perpetuals,
            pool: accounts.pool,
            custody: accounts.custody,
            custody_oracle_account: accounts.custody_oracle_account,
            custody_token_account: accounts.custody_token_account,
            lp_token_mint: accounts.lp_token_mint,
            token_program: accounts.token_program,
            event_authority: accounts.event_authority,
            program: accounts.program,
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
        let mut accounts = Vec::with_capacity(14 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.swap_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.owner.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.funding_or_receiving_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lp_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.transfer_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.perpetuals.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.custody.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.custody_oracle_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.custody_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lp_token_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.event_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = PerpsRemoveLiquidityInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(14 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.swap_program.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.funding_or_receiving_account.clone());
        account_infos.push(self.lp_token_account.clone());
        account_infos.push(self.transfer_authority.clone());
        account_infos.push(self.perpetuals.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.custody.clone());
        account_infos.push(self.custody_oracle_account.clone());
        account_infos.push(self.custody_token_account.clone());
        account_infos.push(self.lp_token_mint.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.event_authority.clone());
        account_infos.push(self.program.clone());
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

/// Instruction builder for `PerpsRemoveLiquidity` via CPI.
///
/// ### Accounts:
///
///   0. `[]` swap_program
///   1. `[writable]` owner
///   2. `[writable]` funding_or_receiving_account
///   3. `[writable]` lp_token_account
///   4. `[]` transfer_authority
///   5. `[]` perpetuals
///   6. `[writable]` pool
///   7. `[writable]` custody
///   8. `[]` custody_oracle_account
///   9. `[writable]` custody_token_account
///   10. `[writable]` lp_token_mint
///   11. `[]` token_program
///   12. `[]` event_authority
///   13. `[]` program
pub struct PerpsRemoveLiquidityCpiBuilder<'a, 'b> {
    instruction: Box<PerpsRemoveLiquidityCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> PerpsRemoveLiquidityCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(PerpsRemoveLiquidityCpiBuilderInstruction {
            __program: program,
            swap_program: None,
            owner: None,
            funding_or_receiving_account: None,
            lp_token_account: None,
            transfer_authority: None,
            perpetuals: None,
            pool: None,
            custody: None,
            custody_oracle_account: None,
            custody_token_account: None,
            lp_token_mint: None,
            token_program: None,
            event_authority: None,
            program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn swap_program(
        &mut self,
        swap_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.swap_program = Some(swap_program);
        self
    }
    #[inline(always)]
    pub fn owner(&mut self, owner: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.owner = Some(owner);
        self
    }
    #[inline(always)]
    pub fn funding_or_receiving_account(
        &mut self,
        funding_or_receiving_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.funding_or_receiving_account = Some(funding_or_receiving_account);
        self
    }
    #[inline(always)]
    pub fn lp_token_account(
        &mut self,
        lp_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lp_token_account = Some(lp_token_account);
        self
    }
    #[inline(always)]
    pub fn transfer_authority(
        &mut self,
        transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.transfer_authority = Some(transfer_authority);
        self
    }
    #[inline(always)]
    pub fn perpetuals(
        &mut self,
        perpetuals: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.perpetuals = Some(perpetuals);
        self
    }
    #[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.pool = Some(pool);
        self
    }
    #[inline(always)]
    pub fn custody(
        &mut self,
        custody: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.custody = Some(custody);
        self
    }
    #[inline(always)]
    pub fn custody_oracle_account(
        &mut self,
        custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.custody_oracle_account = Some(custody_oracle_account);
        self
    }
    #[inline(always)]
    pub fn custody_token_account(
        &mut self,
        custody_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.custody_token_account = Some(custody_token_account);
        self
    }
    #[inline(always)]
    pub fn lp_token_mint(
        &mut self,
        lp_token_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lp_token_mint = Some(lp_token_mint);
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
    pub fn event_authority(
        &mut self,
        event_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.event_authority = Some(event_authority);
        self
    }
    #[inline(always)]
    pub fn program(
        &mut self,
        program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program = Some(program);
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
        let instruction = PerpsRemoveLiquidityCpi {
            __program: self.instruction.__program,

            swap_program: self
                .instruction
                .swap_program
                .expect("swap_program is not set"),

            owner: self.instruction.owner.expect("owner is not set"),

            funding_or_receiving_account: self
                .instruction
                .funding_or_receiving_account
                .expect("funding_or_receiving_account is not set"),

            lp_token_account: self
                .instruction
                .lp_token_account
                .expect("lp_token_account is not set"),

            transfer_authority: self
                .instruction
                .transfer_authority
                .expect("transfer_authority is not set"),

            perpetuals: self.instruction.perpetuals.expect("perpetuals is not set"),

            pool: self.instruction.pool.expect("pool is not set"),

            custody: self.instruction.custody.expect("custody is not set"),

            custody_oracle_account: self
                .instruction
                .custody_oracle_account
                .expect("custody_oracle_account is not set"),

            custody_token_account: self
                .instruction
                .custody_token_account
                .expect("custody_token_account is not set"),

            lp_token_mint: self
                .instruction
                .lp_token_mint
                .expect("lp_token_mint is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            event_authority: self
                .instruction
                .event_authority
                .expect("event_authority is not set"),

            program: self.instruction.program.expect("program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct PerpsRemoveLiquidityCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    swap_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    funding_or_receiving_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lp_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    transfer_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    perpetuals: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    custody: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    custody_oracle_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    custody_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lp_token_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
