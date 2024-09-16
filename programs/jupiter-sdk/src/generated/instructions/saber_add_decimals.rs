//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct SaberAddDecimals {
    pub add_decimals_program: solana_program::pubkey::Pubkey,

    pub wrapper: solana_program::pubkey::Pubkey,

    pub wrapper_mint: solana_program::pubkey::Pubkey,

    pub wrapper_underlying_tokens: solana_program::pubkey::Pubkey,

    pub owner: solana_program::pubkey::Pubkey,

    pub user_underlying_tokens: solana_program::pubkey::Pubkey,

    pub user_wrapped_tokens: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl SaberAddDecimals {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.add_decimals_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.wrapper,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.wrapper_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.wrapper_underlying_tokens,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.owner, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_underlying_tokens,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_wrapped_tokens,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = SaberAddDecimalsInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SaberAddDecimalsInstructionData {
    discriminator: [u8; 8],
}

impl SaberAddDecimalsInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [36, 53, 231, 184, 7, 181, 5, 238],
        }
    }
}

/// Instruction builder for `SaberAddDecimals`.
///
/// ### Accounts:
///
///   0. `[]` add_decimals_program
///   1. `[]` wrapper
///   2. `[writable]` wrapper_mint
///   3. `[writable]` wrapper_underlying_tokens
///   4. `[]` owner
///   5. `[writable]` user_underlying_tokens
///   6. `[writable]` user_wrapped_tokens
///   7. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Default)]
pub struct SaberAddDecimalsBuilder {
    add_decimals_program: Option<solana_program::pubkey::Pubkey>,
    wrapper: Option<solana_program::pubkey::Pubkey>,
    wrapper_mint: Option<solana_program::pubkey::Pubkey>,
    wrapper_underlying_tokens: Option<solana_program::pubkey::Pubkey>,
    owner: Option<solana_program::pubkey::Pubkey>,
    user_underlying_tokens: Option<solana_program::pubkey::Pubkey>,
    user_wrapped_tokens: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl SaberAddDecimalsBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn add_decimals_program(
        &mut self,
        add_decimals_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.add_decimals_program = Some(add_decimals_program);
        self
    }
    #[inline(always)]
    pub fn wrapper(&mut self, wrapper: solana_program::pubkey::Pubkey) -> &mut Self {
        self.wrapper = Some(wrapper);
        self
    }
    #[inline(always)]
    pub fn wrapper_mint(&mut self, wrapper_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.wrapper_mint = Some(wrapper_mint);
        self
    }
    #[inline(always)]
    pub fn wrapper_underlying_tokens(
        &mut self,
        wrapper_underlying_tokens: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.wrapper_underlying_tokens = Some(wrapper_underlying_tokens);
        self
    }
    #[inline(always)]
    pub fn owner(&mut self, owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }
    #[inline(always)]
    pub fn user_underlying_tokens(
        &mut self,
        user_underlying_tokens: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.user_underlying_tokens = Some(user_underlying_tokens);
        self
    }
    #[inline(always)]
    pub fn user_wrapped_tokens(
        &mut self,
        user_wrapped_tokens: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.user_wrapped_tokens = Some(user_wrapped_tokens);
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
        let accounts = SaberAddDecimals {
            add_decimals_program: self
                .add_decimals_program
                .expect("add_decimals_program is not set"),
            wrapper: self.wrapper.expect("wrapper is not set"),
            wrapper_mint: self.wrapper_mint.expect("wrapper_mint is not set"),
            wrapper_underlying_tokens: self
                .wrapper_underlying_tokens
                .expect("wrapper_underlying_tokens is not set"),
            owner: self.owner.expect("owner is not set"),
            user_underlying_tokens: self
                .user_underlying_tokens
                .expect("user_underlying_tokens is not set"),
            user_wrapped_tokens: self
                .user_wrapped_tokens
                .expect("user_wrapped_tokens is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `saber_add_decimals` CPI accounts.
pub struct SaberAddDecimalsCpiAccounts<'a, 'b> {
    pub add_decimals_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub wrapper: &'b solana_program::account_info::AccountInfo<'a>,

    pub wrapper_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub wrapper_underlying_tokens: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_underlying_tokens: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_wrapped_tokens: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `saber_add_decimals` CPI instruction.
pub struct SaberAddDecimalsCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub add_decimals_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub wrapper: &'b solana_program::account_info::AccountInfo<'a>,

    pub wrapper_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub wrapper_underlying_tokens: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_underlying_tokens: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_wrapped_tokens: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> SaberAddDecimalsCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: SaberAddDecimalsCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            add_decimals_program: accounts.add_decimals_program,
            wrapper: accounts.wrapper,
            wrapper_mint: accounts.wrapper_mint,
            wrapper_underlying_tokens: accounts.wrapper_underlying_tokens,
            owner: accounts.owner,
            user_underlying_tokens: accounts.user_underlying_tokens,
            user_wrapped_tokens: accounts.user_wrapped_tokens,
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
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.add_decimals_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.wrapper.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.wrapper_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.wrapper_underlying_tokens.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.owner.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_underlying_tokens.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_wrapped_tokens.key,
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
        let data = SaberAddDecimalsInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(8 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.add_decimals_program.clone());
        account_infos.push(self.wrapper.clone());
        account_infos.push(self.wrapper_mint.clone());
        account_infos.push(self.wrapper_underlying_tokens.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.user_underlying_tokens.clone());
        account_infos.push(self.user_wrapped_tokens.clone());
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

/// Instruction builder for `SaberAddDecimals` via CPI.
///
/// ### Accounts:
///
///   0. `[]` add_decimals_program
///   1. `[]` wrapper
///   2. `[writable]` wrapper_mint
///   3. `[writable]` wrapper_underlying_tokens
///   4. `[]` owner
///   5. `[writable]` user_underlying_tokens
///   6. `[writable]` user_wrapped_tokens
///   7. `[]` token_program
pub struct SaberAddDecimalsCpiBuilder<'a, 'b> {
    instruction: Box<SaberAddDecimalsCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SaberAddDecimalsCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SaberAddDecimalsCpiBuilderInstruction {
            __program: program,
            add_decimals_program: None,
            wrapper: None,
            wrapper_mint: None,
            wrapper_underlying_tokens: None,
            owner: None,
            user_underlying_tokens: None,
            user_wrapped_tokens: None,
            token_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn add_decimals_program(
        &mut self,
        add_decimals_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.add_decimals_program = Some(add_decimals_program);
        self
    }
    #[inline(always)]
    pub fn wrapper(
        &mut self,
        wrapper: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.wrapper = Some(wrapper);
        self
    }
    #[inline(always)]
    pub fn wrapper_mint(
        &mut self,
        wrapper_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.wrapper_mint = Some(wrapper_mint);
        self
    }
    #[inline(always)]
    pub fn wrapper_underlying_tokens(
        &mut self,
        wrapper_underlying_tokens: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.wrapper_underlying_tokens = Some(wrapper_underlying_tokens);
        self
    }
    #[inline(always)]
    pub fn owner(&mut self, owner: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.owner = Some(owner);
        self
    }
    #[inline(always)]
    pub fn user_underlying_tokens(
        &mut self,
        user_underlying_tokens: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_underlying_tokens = Some(user_underlying_tokens);
        self
    }
    #[inline(always)]
    pub fn user_wrapped_tokens(
        &mut self,
        user_wrapped_tokens: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_wrapped_tokens = Some(user_wrapped_tokens);
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
        let instruction = SaberAddDecimalsCpi {
            __program: self.instruction.__program,

            add_decimals_program: self
                .instruction
                .add_decimals_program
                .expect("add_decimals_program is not set"),

            wrapper: self.instruction.wrapper.expect("wrapper is not set"),

            wrapper_mint: self
                .instruction
                .wrapper_mint
                .expect("wrapper_mint is not set"),

            wrapper_underlying_tokens: self
                .instruction
                .wrapper_underlying_tokens
                .expect("wrapper_underlying_tokens is not set"),

            owner: self.instruction.owner.expect("owner is not set"),

            user_underlying_tokens: self
                .instruction
                .user_underlying_tokens
                .expect("user_underlying_tokens is not set"),

            user_wrapped_tokens: self
                .instruction
                .user_wrapped_tokens
                .expect("user_wrapped_tokens is not set"),

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

struct SaberAddDecimalsCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    add_decimals_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    wrapper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    wrapper_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    wrapper_underlying_tokens: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_underlying_tokens: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_wrapped_tokens: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
