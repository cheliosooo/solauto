//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct OneIntroSwap {
    pub swap_program: solana_program::pubkey::Pubkey,

    pub metadata_state: solana_program::pubkey::Pubkey,

    pub pool_state: solana_program::pubkey::Pubkey,

    pub pool_auth_pda: solana_program::pubkey::Pubkey,

    pub pool_token_in_account: solana_program::pubkey::Pubkey,

    pub pool_token_out_account: solana_program::pubkey::Pubkey,

    pub user: solana_program::pubkey::Pubkey,

    pub user_token_in_account: solana_program::pubkey::Pubkey,

    pub user_token_out_account: solana_program::pubkey::Pubkey,

    pub metadata_swap_fee_account: solana_program::pubkey::Pubkey,

    pub referral_token_account: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl OneIntroSwap {
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
            self.swap_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.metadata_state,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool_state,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.pool_auth_pda,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool_token_in_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool_token_out_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_token_in_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_token_out_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.metadata_swap_fee_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.referral_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = OneIntroSwapInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OneIntroSwapInstructionData {
    discriminator: [u8; 8],
}

impl OneIntroSwapInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [208, 212, 80, 169, 36, 148, 209, 35],
        }
    }
}

/// Instruction builder for `OneIntroSwap`.
///
/// ### Accounts:
///
///   0. `[]` swap_program
///   1. `[]` metadata_state
///   2. `[writable]` pool_state
///   3. `[]` pool_auth_pda
///   4. `[writable]` pool_token_in_account
///   5. `[writable]` pool_token_out_account
///   6. `[writable]` user
///   7. `[writable]` user_token_in_account
///   8. `[writable]` user_token_out_account
///   9. `[writable]` metadata_swap_fee_account
///   10. `[writable]` referral_token_account
///   11. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Default)]
pub struct OneIntroSwapBuilder {
    swap_program: Option<solana_program::pubkey::Pubkey>,
    metadata_state: Option<solana_program::pubkey::Pubkey>,
    pool_state: Option<solana_program::pubkey::Pubkey>,
    pool_auth_pda: Option<solana_program::pubkey::Pubkey>,
    pool_token_in_account: Option<solana_program::pubkey::Pubkey>,
    pool_token_out_account: Option<solana_program::pubkey::Pubkey>,
    user: Option<solana_program::pubkey::Pubkey>,
    user_token_in_account: Option<solana_program::pubkey::Pubkey>,
    user_token_out_account: Option<solana_program::pubkey::Pubkey>,
    metadata_swap_fee_account: Option<solana_program::pubkey::Pubkey>,
    referral_token_account: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl OneIntroSwapBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn swap_program(&mut self, swap_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.swap_program = Some(swap_program);
        self
    }
    #[inline(always)]
    pub fn metadata_state(&mut self, metadata_state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.metadata_state = Some(metadata_state);
        self
    }
    #[inline(always)]
    pub fn pool_state(&mut self, pool_state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.pool_state = Some(pool_state);
        self
    }
    #[inline(always)]
    pub fn pool_auth_pda(&mut self, pool_auth_pda: solana_program::pubkey::Pubkey) -> &mut Self {
        self.pool_auth_pda = Some(pool_auth_pda);
        self
    }
    #[inline(always)]
    pub fn pool_token_in_account(
        &mut self,
        pool_token_in_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.pool_token_in_account = Some(pool_token_in_account);
        self
    }
    #[inline(always)]
    pub fn pool_token_out_account(
        &mut self,
        pool_token_out_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.pool_token_out_account = Some(pool_token_out_account);
        self
    }
    #[inline(always)]
    pub fn user(&mut self, user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user = Some(user);
        self
    }
    #[inline(always)]
    pub fn user_token_in_account(
        &mut self,
        user_token_in_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.user_token_in_account = Some(user_token_in_account);
        self
    }
    #[inline(always)]
    pub fn user_token_out_account(
        &mut self,
        user_token_out_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.user_token_out_account = Some(user_token_out_account);
        self
    }
    #[inline(always)]
    pub fn metadata_swap_fee_account(
        &mut self,
        metadata_swap_fee_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.metadata_swap_fee_account = Some(metadata_swap_fee_account);
        self
    }
    #[inline(always)]
    pub fn referral_token_account(
        &mut self,
        referral_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.referral_token_account = Some(referral_token_account);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
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
        let accounts = OneIntroSwap {
            swap_program: self.swap_program.expect("swap_program is not set"),
            metadata_state: self.metadata_state.expect("metadata_state is not set"),
            pool_state: self.pool_state.expect("pool_state is not set"),
            pool_auth_pda: self.pool_auth_pda.expect("pool_auth_pda is not set"),
            pool_token_in_account: self
                .pool_token_in_account
                .expect("pool_token_in_account is not set"),
            pool_token_out_account: self
                .pool_token_out_account
                .expect("pool_token_out_account is not set"),
            user: self.user.expect("user is not set"),
            user_token_in_account: self
                .user_token_in_account
                .expect("user_token_in_account is not set"),
            user_token_out_account: self
                .user_token_out_account
                .expect("user_token_out_account is not set"),
            metadata_swap_fee_account: self
                .metadata_swap_fee_account
                .expect("metadata_swap_fee_account is not set"),
            referral_token_account: self
                .referral_token_account
                .expect("referral_token_account is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `one_intro_swap` CPI accounts.
pub struct OneIntroSwapCpiAccounts<'a, 'b> {
    pub swap_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub metadata_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_auth_pda: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_token_in_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_token_out_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_in_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_out_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub metadata_swap_fee_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `one_intro_swap` CPI instruction.
pub struct OneIntroSwapCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub swap_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub metadata_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_auth_pda: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_token_in_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_token_out_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_in_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_out_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub metadata_swap_fee_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> OneIntroSwapCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: OneIntroSwapCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            swap_program: accounts.swap_program,
            metadata_state: accounts.metadata_state,
            pool_state: accounts.pool_state,
            pool_auth_pda: accounts.pool_auth_pda,
            pool_token_in_account: accounts.pool_token_in_account,
            pool_token_out_account: accounts.pool_token_out_account,
            user: accounts.user,
            user_token_in_account: accounts.user_token_in_account,
            user_token_out_account: accounts.user_token_out_account,
            metadata_swap_fee_account: accounts.metadata_swap_fee_account,
            referral_token_account: accounts.referral_token_account,
            token_program: accounts.token_program,
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
            *self.swap_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.metadata_state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool_state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.pool_auth_pda.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool_token_in_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool_token_out_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_token_in_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_token_out_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.metadata_swap_fee_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.referral_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = OneIntroSwapInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(12 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.swap_program.clone());
        account_infos.push(self.metadata_state.clone());
        account_infos.push(self.pool_state.clone());
        account_infos.push(self.pool_auth_pda.clone());
        account_infos.push(self.pool_token_in_account.clone());
        account_infos.push(self.pool_token_out_account.clone());
        account_infos.push(self.user.clone());
        account_infos.push(self.user_token_in_account.clone());
        account_infos.push(self.user_token_out_account.clone());
        account_infos.push(self.metadata_swap_fee_account.clone());
        account_infos.push(self.referral_token_account.clone());
        account_infos.push(self.token_program.clone());
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

/// Instruction builder for `OneIntroSwap` via CPI.
///
/// ### Accounts:
///
///   0. `[]` swap_program
///   1. `[]` metadata_state
///   2. `[writable]` pool_state
///   3. `[]` pool_auth_pda
///   4. `[writable]` pool_token_in_account
///   5. `[writable]` pool_token_out_account
///   6. `[writable]` user
///   7. `[writable]` user_token_in_account
///   8. `[writable]` user_token_out_account
///   9. `[writable]` metadata_swap_fee_account
///   10. `[writable]` referral_token_account
///   11. `[]` token_program
pub struct OneIntroSwapCpiBuilder<'a, 'b> {
    instruction: Box<OneIntroSwapCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> OneIntroSwapCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(OneIntroSwapCpiBuilderInstruction {
            __program: program,
            swap_program: None,
            metadata_state: None,
            pool_state: None,
            pool_auth_pda: None,
            pool_token_in_account: None,
            pool_token_out_account: None,
            user: None,
            user_token_in_account: None,
            user_token_out_account: None,
            metadata_swap_fee_account: None,
            referral_token_account: None,
            token_program: None,
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
    pub fn metadata_state(
        &mut self,
        metadata_state: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.metadata_state = Some(metadata_state);
        self
    }
    #[inline(always)]
    pub fn pool_state(
        &mut self,
        pool_state: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.pool_state = Some(pool_state);
        self
    }
    #[inline(always)]
    pub fn pool_auth_pda(
        &mut self,
        pool_auth_pda: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.pool_auth_pda = Some(pool_auth_pda);
        self
    }
    #[inline(always)]
    pub fn pool_token_in_account(
        &mut self,
        pool_token_in_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.pool_token_in_account = Some(pool_token_in_account);
        self
    }
    #[inline(always)]
    pub fn pool_token_out_account(
        &mut self,
        pool_token_out_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.pool_token_out_account = Some(pool_token_out_account);
        self
    }
    #[inline(always)]
    pub fn user(&mut self, user: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.user = Some(user);
        self
    }
    #[inline(always)]
    pub fn user_token_in_account(
        &mut self,
        user_token_in_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_token_in_account = Some(user_token_in_account);
        self
    }
    #[inline(always)]
    pub fn user_token_out_account(
        &mut self,
        user_token_out_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_token_out_account = Some(user_token_out_account);
        self
    }
    #[inline(always)]
    pub fn metadata_swap_fee_account(
        &mut self,
        metadata_swap_fee_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.metadata_swap_fee_account = Some(metadata_swap_fee_account);
        self
    }
    #[inline(always)]
    pub fn referral_token_account(
        &mut self,
        referral_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.referral_token_account = Some(referral_token_account);
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
        let instruction = OneIntroSwapCpi {
            __program: self.instruction.__program,

            swap_program: self
                .instruction
                .swap_program
                .expect("swap_program is not set"),

            metadata_state: self
                .instruction
                .metadata_state
                .expect("metadata_state is not set"),

            pool_state: self.instruction.pool_state.expect("pool_state is not set"),

            pool_auth_pda: self
                .instruction
                .pool_auth_pda
                .expect("pool_auth_pda is not set"),

            pool_token_in_account: self
                .instruction
                .pool_token_in_account
                .expect("pool_token_in_account is not set"),

            pool_token_out_account: self
                .instruction
                .pool_token_out_account
                .expect("pool_token_out_account is not set"),

            user: self.instruction.user.expect("user is not set"),

            user_token_in_account: self
                .instruction
                .user_token_in_account
                .expect("user_token_in_account is not set"),

            user_token_out_account: self
                .instruction
                .user_token_out_account
                .expect("user_token_out_account is not set"),

            metadata_swap_fee_account: self
                .instruction
                .metadata_swap_fee_account
                .expect("metadata_swap_fee_account is not set"),

            referral_token_account: self
                .instruction
                .referral_token_account
                .expect("referral_token_account is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct OneIntroSwapCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    swap_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    metadata_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool_auth_pda: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool_token_in_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool_token_out_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_token_in_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_token_out_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    metadata_swap_fee_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    referral_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}