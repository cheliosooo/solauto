//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct MarinadeUnstake {
    pub marinade_finance_program: solana_program::pubkey::Pubkey,

    pub state: solana_program::pubkey::Pubkey,

    pub msol_mint: solana_program::pubkey::Pubkey,

    pub liq_pool_sol_leg_pda: solana_program::pubkey::Pubkey,

    pub liq_pool_msol_leg: solana_program::pubkey::Pubkey,

    pub treasury_msol_account: solana_program::pubkey::Pubkey,

    pub get_msol_from: solana_program::pubkey::Pubkey,

    pub get_msol_from_authority: solana_program::pubkey::Pubkey,

    pub transfer_sol_to: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub user_wsol_token_account: solana_program::pubkey::Pubkey,
}

impl MarinadeUnstake {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.marinade_finance_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.state, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.msol_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.liq_pool_sol_leg_pda,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.liq_pool_msol_leg,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.treasury_msol_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.get_msol_from,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.get_msol_from_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.transfer_sol_to,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_wsol_token_account,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = MarinadeUnstakeInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MarinadeUnstakeInstructionData {
    discriminator: [u8; 8],
}

impl MarinadeUnstakeInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [41, 120, 15, 0, 113, 219, 42, 1],
        }
    }
}

/// Instruction builder for `MarinadeUnstake`.
///
/// ### Accounts:
///
///   0. `[]` marinade_finance_program
///   1. `[writable]` state
///   2. `[writable]` msol_mint
///   3. `[writable]` liq_pool_sol_leg_pda
///   4. `[writable]` liq_pool_msol_leg
///   5. `[writable]` treasury_msol_account
///   6. `[writable]` get_msol_from
///   7. `[]` get_msol_from_authority
///   8. `[writable]` transfer_sol_to
///   9. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   10. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   11. `[writable]` user_wsol_token_account
#[derive(Default)]
pub struct MarinadeUnstakeBuilder {
    marinade_finance_program: Option<solana_program::pubkey::Pubkey>,
    state: Option<solana_program::pubkey::Pubkey>,
    msol_mint: Option<solana_program::pubkey::Pubkey>,
    liq_pool_sol_leg_pda: Option<solana_program::pubkey::Pubkey>,
    liq_pool_msol_leg: Option<solana_program::pubkey::Pubkey>,
    treasury_msol_account: Option<solana_program::pubkey::Pubkey>,
    get_msol_from: Option<solana_program::pubkey::Pubkey>,
    get_msol_from_authority: Option<solana_program::pubkey::Pubkey>,
    transfer_sol_to: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    user_wsol_token_account: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl MarinadeUnstakeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn marinade_finance_program(
        &mut self,
        marinade_finance_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.marinade_finance_program = Some(marinade_finance_program);
        self
    }
    #[inline(always)]
    pub fn state(&mut self, state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.state = Some(state);
        self
    }
    #[inline(always)]
    pub fn msol_mint(&mut self, msol_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.msol_mint = Some(msol_mint);
        self
    }
    #[inline(always)]
    pub fn liq_pool_sol_leg_pda(
        &mut self,
        liq_pool_sol_leg_pda: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.liq_pool_sol_leg_pda = Some(liq_pool_sol_leg_pda);
        self
    }
    #[inline(always)]
    pub fn liq_pool_msol_leg(
        &mut self,
        liq_pool_msol_leg: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.liq_pool_msol_leg = Some(liq_pool_msol_leg);
        self
    }
    #[inline(always)]
    pub fn treasury_msol_account(
        &mut self,
        treasury_msol_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.treasury_msol_account = Some(treasury_msol_account);
        self
    }
    #[inline(always)]
    pub fn get_msol_from(&mut self, get_msol_from: solana_program::pubkey::Pubkey) -> &mut Self {
        self.get_msol_from = Some(get_msol_from);
        self
    }
    #[inline(always)]
    pub fn get_msol_from_authority(
        &mut self,
        get_msol_from_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.get_msol_from_authority = Some(get_msol_from_authority);
        self
    }
    #[inline(always)]
    pub fn transfer_sol_to(
        &mut self,
        transfer_sol_to: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.transfer_sol_to = Some(transfer_sol_to);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn user_wsol_token_account(
        &mut self,
        user_wsol_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.user_wsol_token_account = Some(user_wsol_token_account);
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
        let accounts = MarinadeUnstake {
            marinade_finance_program: self
                .marinade_finance_program
                .expect("marinade_finance_program is not set"),
            state: self.state.expect("state is not set"),
            msol_mint: self.msol_mint.expect("msol_mint is not set"),
            liq_pool_sol_leg_pda: self
                .liq_pool_sol_leg_pda
                .expect("liq_pool_sol_leg_pda is not set"),
            liq_pool_msol_leg: self
                .liq_pool_msol_leg
                .expect("liq_pool_msol_leg is not set"),
            treasury_msol_account: self
                .treasury_msol_account
                .expect("treasury_msol_account is not set"),
            get_msol_from: self.get_msol_from.expect("get_msol_from is not set"),
            get_msol_from_authority: self
                .get_msol_from_authority
                .expect("get_msol_from_authority is not set"),
            transfer_sol_to: self.transfer_sol_to.expect("transfer_sol_to is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            user_wsol_token_account: self
                .user_wsol_token_account
                .expect("user_wsol_token_account is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `marinade_unstake` CPI accounts.
pub struct MarinadeUnstakeCpiAccounts<'a, 'b> {
    pub marinade_finance_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub msol_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub liq_pool_sol_leg_pda: &'b solana_program::account_info::AccountInfo<'a>,

    pub liq_pool_msol_leg: &'b solana_program::account_info::AccountInfo<'a>,

    pub treasury_msol_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub get_msol_from: &'b solana_program::account_info::AccountInfo<'a>,

    pub get_msol_from_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub transfer_sol_to: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_wsol_token_account: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `marinade_unstake` CPI instruction.
pub struct MarinadeUnstakeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub marinade_finance_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub state: &'b solana_program::account_info::AccountInfo<'a>,

    pub msol_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub liq_pool_sol_leg_pda: &'b solana_program::account_info::AccountInfo<'a>,

    pub liq_pool_msol_leg: &'b solana_program::account_info::AccountInfo<'a>,

    pub treasury_msol_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub get_msol_from: &'b solana_program::account_info::AccountInfo<'a>,

    pub get_msol_from_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub transfer_sol_to: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_wsol_token_account: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> MarinadeUnstakeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: MarinadeUnstakeCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            marinade_finance_program: accounts.marinade_finance_program,
            state: accounts.state,
            msol_mint: accounts.msol_mint,
            liq_pool_sol_leg_pda: accounts.liq_pool_sol_leg_pda,
            liq_pool_msol_leg: accounts.liq_pool_msol_leg,
            treasury_msol_account: accounts.treasury_msol_account,
            get_msol_from: accounts.get_msol_from,
            get_msol_from_authority: accounts.get_msol_from_authority,
            transfer_sol_to: accounts.transfer_sol_to,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            user_wsol_token_account: accounts.user_wsol_token_account,
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
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.marinade_finance_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.msol_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.liq_pool_sol_leg_pda.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.liq_pool_msol_leg.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.treasury_msol_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.get_msol_from.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.get_msol_from_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.transfer_sol_to.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_wsol_token_account.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = MarinadeUnstakeInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(12 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.marinade_finance_program.clone());
        account_infos.push(self.state.clone());
        account_infos.push(self.msol_mint.clone());
        account_infos.push(self.liq_pool_sol_leg_pda.clone());
        account_infos.push(self.liq_pool_msol_leg.clone());
        account_infos.push(self.treasury_msol_account.clone());
        account_infos.push(self.get_msol_from.clone());
        account_infos.push(self.get_msol_from_authority.clone());
        account_infos.push(self.transfer_sol_to.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.user_wsol_token_account.clone());
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

/// Instruction builder for `MarinadeUnstake` via CPI.
///
/// ### Accounts:
///
///   0. `[]` marinade_finance_program
///   1. `[writable]` state
///   2. `[writable]` msol_mint
///   3. `[writable]` liq_pool_sol_leg_pda
///   4. `[writable]` liq_pool_msol_leg
///   5. `[writable]` treasury_msol_account
///   6. `[writable]` get_msol_from
///   7. `[]` get_msol_from_authority
///   8. `[writable]` transfer_sol_to
///   9. `[]` system_program
///   10. `[]` token_program
///   11. `[writable]` user_wsol_token_account
pub struct MarinadeUnstakeCpiBuilder<'a, 'b> {
    instruction: Box<MarinadeUnstakeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> MarinadeUnstakeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(MarinadeUnstakeCpiBuilderInstruction {
            __program: program,
            marinade_finance_program: None,
            state: None,
            msol_mint: None,
            liq_pool_sol_leg_pda: None,
            liq_pool_msol_leg: None,
            treasury_msol_account: None,
            get_msol_from: None,
            get_msol_from_authority: None,
            transfer_sol_to: None,
            system_program: None,
            token_program: None,
            user_wsol_token_account: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn marinade_finance_program(
        &mut self,
        marinade_finance_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.marinade_finance_program = Some(marinade_finance_program);
        self
    }
    #[inline(always)]
    pub fn state(&mut self, state: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.state = Some(state);
        self
    }
    #[inline(always)]
    pub fn msol_mint(
        &mut self,
        msol_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.msol_mint = Some(msol_mint);
        self
    }
    #[inline(always)]
    pub fn liq_pool_sol_leg_pda(
        &mut self,
        liq_pool_sol_leg_pda: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.liq_pool_sol_leg_pda = Some(liq_pool_sol_leg_pda);
        self
    }
    #[inline(always)]
    pub fn liq_pool_msol_leg(
        &mut self,
        liq_pool_msol_leg: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.liq_pool_msol_leg = Some(liq_pool_msol_leg);
        self
    }
    #[inline(always)]
    pub fn treasury_msol_account(
        &mut self,
        treasury_msol_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.treasury_msol_account = Some(treasury_msol_account);
        self
    }
    #[inline(always)]
    pub fn get_msol_from(
        &mut self,
        get_msol_from: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.get_msol_from = Some(get_msol_from);
        self
    }
    #[inline(always)]
    pub fn get_msol_from_authority(
        &mut self,
        get_msol_from_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.get_msol_from_authority = Some(get_msol_from_authority);
        self
    }
    #[inline(always)]
    pub fn transfer_sol_to(
        &mut self,
        transfer_sol_to: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.transfer_sol_to = Some(transfer_sol_to);
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
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn user_wsol_token_account(
        &mut self,
        user_wsol_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_wsol_token_account = Some(user_wsol_token_account);
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
        let instruction = MarinadeUnstakeCpi {
            __program: self.instruction.__program,

            marinade_finance_program: self
                .instruction
                .marinade_finance_program
                .expect("marinade_finance_program is not set"),

            state: self.instruction.state.expect("state is not set"),

            msol_mint: self.instruction.msol_mint.expect("msol_mint is not set"),

            liq_pool_sol_leg_pda: self
                .instruction
                .liq_pool_sol_leg_pda
                .expect("liq_pool_sol_leg_pda is not set"),

            liq_pool_msol_leg: self
                .instruction
                .liq_pool_msol_leg
                .expect("liq_pool_msol_leg is not set"),

            treasury_msol_account: self
                .instruction
                .treasury_msol_account
                .expect("treasury_msol_account is not set"),

            get_msol_from: self
                .instruction
                .get_msol_from
                .expect("get_msol_from is not set"),

            get_msol_from_authority: self
                .instruction
                .get_msol_from_authority
                .expect("get_msol_from_authority is not set"),

            transfer_sol_to: self
                .instruction
                .transfer_sol_to
                .expect("transfer_sol_to is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            user_wsol_token_account: self
                .instruction
                .user_wsol_token_account
                .expect("user_wsol_token_account is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct MarinadeUnstakeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    marinade_finance_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    msol_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    liq_pool_sol_leg_pda: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    liq_pool_msol_leg: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    treasury_msol_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    get_msol_from: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    get_msol_from_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    transfer_sol_to: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_wsol_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}