//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CropperSwap {
    pub token_swap_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub swap: solana_program::pubkey::Pubkey,

    pub swap_state: solana_program::pubkey::Pubkey,

    pub authority: solana_program::pubkey::Pubkey,

    pub user_transfer_authority: solana_program::pubkey::Pubkey,

    pub source: solana_program::pubkey::Pubkey,

    pub swap_source: solana_program::pubkey::Pubkey,

    pub swap_destination: solana_program::pubkey::Pubkey,

    pub destination: solana_program::pubkey::Pubkey,

    pub pool_mint: solana_program::pubkey::Pubkey,

    pub pool_fee: solana_program::pubkey::Pubkey,
}

impl CropperSwap {
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
            self.token_swap_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.swap, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.swap_state,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.user_transfer_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.source,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.swap_source,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.swap_destination,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.destination,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool_fee,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = CropperSwapInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CropperSwapInstructionData {
    discriminator: [u8; 8],
}

impl CropperSwapInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [230, 216, 47, 182, 165, 117, 210, 103],
        }
    }
}

/// Instruction builder for `CropperSwap`.
///
/// ### Accounts:
///
///   0. `[]` token_swap_program
///   1. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   2. `[]` swap
///   3. `[]` swap_state
///   4. `[]` authority
///   5. `[]` user_transfer_authority
///   6. `[writable]` source
///   7. `[writable]` swap_source
///   8. `[writable]` swap_destination
///   9. `[writable]` destination
///   10. `[writable]` pool_mint
///   11. `[writable]` pool_fee
#[derive(Default)]
pub struct CropperSwapBuilder {
    token_swap_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    swap: Option<solana_program::pubkey::Pubkey>,
    swap_state: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    user_transfer_authority: Option<solana_program::pubkey::Pubkey>,
    source: Option<solana_program::pubkey::Pubkey>,
    swap_source: Option<solana_program::pubkey::Pubkey>,
    swap_destination: Option<solana_program::pubkey::Pubkey>,
    destination: Option<solana_program::pubkey::Pubkey>,
    pool_mint: Option<solana_program::pubkey::Pubkey>,
    pool_fee: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CropperSwapBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn token_swap_program(
        &mut self,
        token_swap_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_swap_program = Some(token_swap_program);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn swap(&mut self, swap: solana_program::pubkey::Pubkey) -> &mut Self {
        self.swap = Some(swap);
        self
    }
    #[inline(always)]
    pub fn swap_state(&mut self, swap_state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.swap_state = Some(swap_state);
        self
    }
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    #[inline(always)]
    pub fn user_transfer_authority(
        &mut self,
        user_transfer_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.user_transfer_authority = Some(user_transfer_authority);
        self
    }
    #[inline(always)]
    pub fn source(&mut self, source: solana_program::pubkey::Pubkey) -> &mut Self {
        self.source = Some(source);
        self
    }
    #[inline(always)]
    pub fn swap_source(&mut self, swap_source: solana_program::pubkey::Pubkey) -> &mut Self {
        self.swap_source = Some(swap_source);
        self
    }
    #[inline(always)]
    pub fn swap_destination(
        &mut self,
        swap_destination: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.swap_destination = Some(swap_destination);
        self
    }
    #[inline(always)]
    pub fn destination(&mut self, destination: solana_program::pubkey::Pubkey) -> &mut Self {
        self.destination = Some(destination);
        self
    }
    #[inline(always)]
    pub fn pool_mint(&mut self, pool_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.pool_mint = Some(pool_mint);
        self
    }
    #[inline(always)]
    pub fn pool_fee(&mut self, pool_fee: solana_program::pubkey::Pubkey) -> &mut Self {
        self.pool_fee = Some(pool_fee);
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
        let accounts = CropperSwap {
            token_swap_program: self
                .token_swap_program
                .expect("token_swap_program is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            swap: self.swap.expect("swap is not set"),
            swap_state: self.swap_state.expect("swap_state is not set"),
            authority: self.authority.expect("authority is not set"),
            user_transfer_authority: self
                .user_transfer_authority
                .expect("user_transfer_authority is not set"),
            source: self.source.expect("source is not set"),
            swap_source: self.swap_source.expect("swap_source is not set"),
            swap_destination: self.swap_destination.expect("swap_destination is not set"),
            destination: self.destination.expect("destination is not set"),
            pool_mint: self.pool_mint.expect("pool_mint is not set"),
            pool_fee: self.pool_fee.expect("pool_fee is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `cropper_swap` CPI accounts.
pub struct CropperSwapCpiAccounts<'a, 'b> {
    pub token_swap_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub swap: &'b solana_program::account_info::AccountInfo<'a>,

    pub swap_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub source: &'b solana_program::account_info::AccountInfo<'a>,

    pub swap_source: &'b solana_program::account_info::AccountInfo<'a>,

    pub swap_destination: &'b solana_program::account_info::AccountInfo<'a>,

    pub destination: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_fee: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `cropper_swap` CPI instruction.
pub struct CropperSwapCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_swap_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub swap: &'b solana_program::account_info::AccountInfo<'a>,

    pub swap_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub source: &'b solana_program::account_info::AccountInfo<'a>,

    pub swap_source: &'b solana_program::account_info::AccountInfo<'a>,

    pub swap_destination: &'b solana_program::account_info::AccountInfo<'a>,

    pub destination: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool_fee: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> CropperSwapCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CropperSwapCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            token_swap_program: accounts.token_swap_program,
            token_program: accounts.token_program,
            swap: accounts.swap,
            swap_state: accounts.swap_state,
            authority: accounts.authority,
            user_transfer_authority: accounts.user_transfer_authority,
            source: accounts.source,
            swap_source: accounts.swap_source,
            swap_destination: accounts.swap_destination,
            destination: accounts.destination,
            pool_mint: accounts.pool_mint,
            pool_fee: accounts.pool_fee,
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
            *self.token_swap_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.swap.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.swap_state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.user_transfer_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.source.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.swap_source.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.swap_destination.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.destination.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool_fee.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = CropperSwapInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(12 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.token_swap_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.swap.clone());
        account_infos.push(self.swap_state.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.user_transfer_authority.clone());
        account_infos.push(self.source.clone());
        account_infos.push(self.swap_source.clone());
        account_infos.push(self.swap_destination.clone());
        account_infos.push(self.destination.clone());
        account_infos.push(self.pool_mint.clone());
        account_infos.push(self.pool_fee.clone());
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

/// Instruction builder for `CropperSwap` via CPI.
///
/// ### Accounts:
///
///   0. `[]` token_swap_program
///   1. `[]` token_program
///   2. `[]` swap
///   3. `[]` swap_state
///   4. `[]` authority
///   5. `[]` user_transfer_authority
///   6. `[writable]` source
///   7. `[writable]` swap_source
///   8. `[writable]` swap_destination
///   9. `[writable]` destination
///   10. `[writable]` pool_mint
///   11. `[writable]` pool_fee
pub struct CropperSwapCpiBuilder<'a, 'b> {
    instruction: Box<CropperSwapCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CropperSwapCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CropperSwapCpiBuilderInstruction {
            __program: program,
            token_swap_program: None,
            token_program: None,
            swap: None,
            swap_state: None,
            authority: None,
            user_transfer_authority: None,
            source: None,
            swap_source: None,
            swap_destination: None,
            destination: None,
            pool_mint: None,
            pool_fee: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn token_swap_program(
        &mut self,
        token_swap_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_swap_program = Some(token_swap_program);
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
    pub fn swap(&mut self, swap: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.swap = Some(swap);
        self
    }
    #[inline(always)]
    pub fn swap_state(
        &mut self,
        swap_state: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.swap_state = Some(swap_state);
        self
    }
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    #[inline(always)]
    pub fn user_transfer_authority(
        &mut self,
        user_transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_transfer_authority = Some(user_transfer_authority);
        self
    }
    #[inline(always)]
    pub fn source(
        &mut self,
        source: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.source = Some(source);
        self
    }
    #[inline(always)]
    pub fn swap_source(
        &mut self,
        swap_source: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.swap_source = Some(swap_source);
        self
    }
    #[inline(always)]
    pub fn swap_destination(
        &mut self,
        swap_destination: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.swap_destination = Some(swap_destination);
        self
    }
    #[inline(always)]
    pub fn destination(
        &mut self,
        destination: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.destination = Some(destination);
        self
    }
    #[inline(always)]
    pub fn pool_mint(
        &mut self,
        pool_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.pool_mint = Some(pool_mint);
        self
    }
    #[inline(always)]
    pub fn pool_fee(
        &mut self,
        pool_fee: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.pool_fee = Some(pool_fee);
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
        let instruction = CropperSwapCpi {
            __program: self.instruction.__program,

            token_swap_program: self
                .instruction
                .token_swap_program
                .expect("token_swap_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            swap: self.instruction.swap.expect("swap is not set"),

            swap_state: self.instruction.swap_state.expect("swap_state is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            user_transfer_authority: self
                .instruction
                .user_transfer_authority
                .expect("user_transfer_authority is not set"),

            source: self.instruction.source.expect("source is not set"),

            swap_source: self
                .instruction
                .swap_source
                .expect("swap_source is not set"),

            swap_destination: self
                .instruction
                .swap_destination
                .expect("swap_destination is not set"),

            destination: self
                .instruction
                .destination
                .expect("destination is not set"),

            pool_mint: self.instruction.pool_mint.expect("pool_mint is not set"),

            pool_fee: self.instruction.pool_fee.expect("pool_fee is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct CropperSwapCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    token_swap_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    swap: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    swap_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_transfer_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    source: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    swap_source: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    swap_destination: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    destination: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool_fee: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
