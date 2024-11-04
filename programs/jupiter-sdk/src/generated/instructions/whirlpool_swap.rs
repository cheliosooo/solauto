//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct WhirlpoolSwap {
    pub swap_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub token_authority: solana_program::pubkey::Pubkey,

    pub whirlpool: solana_program::pubkey::Pubkey,

    pub token_owner_account_a: solana_program::pubkey::Pubkey,

    pub token_vault_a: solana_program::pubkey::Pubkey,

    pub token_owner_account_b: solana_program::pubkey::Pubkey,

    pub token_vault_b: solana_program::pubkey::Pubkey,

    pub tick_array0: solana_program::pubkey::Pubkey,

    pub tick_array1: solana_program::pubkey::Pubkey,

    pub tick_array2: solana_program::pubkey::Pubkey,
    /// Oracle is currently unused and will be enabled on subsequent updates
    pub oracle: solana_program::pubkey::Pubkey,
}

impl WhirlpoolSwap {
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
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.whirlpool,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_owner_account_a,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_vault_a,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_owner_account_b,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_vault_b,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.tick_array0,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.tick_array1,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.tick_array2,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = WhirlpoolSwapInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct WhirlpoolSwapInstructionData {
    discriminator: [u8; 8],
}

impl WhirlpoolSwapInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [123, 229, 184, 63, 12, 0, 92, 145],
        }
    }
}

/// Instruction builder for `WhirlpoolSwap`.
///
/// ### Accounts:
///
///   0. `[]` swap_program
///   1. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   2. `[]` token_authority
///   3. `[writable]` whirlpool
///   4. `[writable]` token_owner_account_a
///   5. `[writable]` token_vault_a
///   6. `[writable]` token_owner_account_b
///   7. `[writable]` token_vault_b
///   8. `[writable]` tick_array0
///   9. `[writable]` tick_array1
///   10. `[writable]` tick_array2
///   11. `[]` oracle
#[derive(Default)]
pub struct WhirlpoolSwapBuilder {
    swap_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    token_authority: Option<solana_program::pubkey::Pubkey>,
    whirlpool: Option<solana_program::pubkey::Pubkey>,
    token_owner_account_a: Option<solana_program::pubkey::Pubkey>,
    token_vault_a: Option<solana_program::pubkey::Pubkey>,
    token_owner_account_b: Option<solana_program::pubkey::Pubkey>,
    token_vault_b: Option<solana_program::pubkey::Pubkey>,
    tick_array0: Option<solana_program::pubkey::Pubkey>,
    tick_array1: Option<solana_program::pubkey::Pubkey>,
    tick_array2: Option<solana_program::pubkey::Pubkey>,
    oracle: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl WhirlpoolSwapBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn swap_program(&mut self, swap_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.swap_program = Some(swap_program);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn token_authority(
        &mut self,
        token_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_authority = Some(token_authority);
        self
    }
    #[inline(always)]
    pub fn whirlpool(&mut self, whirlpool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.whirlpool = Some(whirlpool);
        self
    }
    #[inline(always)]
    pub fn token_owner_account_a(
        &mut self,
        token_owner_account_a: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_owner_account_a = Some(token_owner_account_a);
        self
    }
    #[inline(always)]
    pub fn token_vault_a(&mut self, token_vault_a: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_vault_a = Some(token_vault_a);
        self
    }
    #[inline(always)]
    pub fn token_owner_account_b(
        &mut self,
        token_owner_account_b: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_owner_account_b = Some(token_owner_account_b);
        self
    }
    #[inline(always)]
    pub fn token_vault_b(&mut self, token_vault_b: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_vault_b = Some(token_vault_b);
        self
    }
    #[inline(always)]
    pub fn tick_array0(&mut self, tick_array0: solana_program::pubkey::Pubkey) -> &mut Self {
        self.tick_array0 = Some(tick_array0);
        self
    }
    #[inline(always)]
    pub fn tick_array1(&mut self, tick_array1: solana_program::pubkey::Pubkey) -> &mut Self {
        self.tick_array1 = Some(tick_array1);
        self
    }
    #[inline(always)]
    pub fn tick_array2(&mut self, tick_array2: solana_program::pubkey::Pubkey) -> &mut Self {
        self.tick_array2 = Some(tick_array2);
        self
    }
    /// Oracle is currently unused and will be enabled on subsequent updates
    #[inline(always)]
    pub fn oracle(&mut self, oracle: solana_program::pubkey::Pubkey) -> &mut Self {
        self.oracle = Some(oracle);
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
        let accounts = WhirlpoolSwap {
            swap_program: self.swap_program.expect("swap_program is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            token_authority: self.token_authority.expect("token_authority is not set"),
            whirlpool: self.whirlpool.expect("whirlpool is not set"),
            token_owner_account_a: self
                .token_owner_account_a
                .expect("token_owner_account_a is not set"),
            token_vault_a: self.token_vault_a.expect("token_vault_a is not set"),
            token_owner_account_b: self
                .token_owner_account_b
                .expect("token_owner_account_b is not set"),
            token_vault_b: self.token_vault_b.expect("token_vault_b is not set"),
            tick_array0: self.tick_array0.expect("tick_array0 is not set"),
            tick_array1: self.tick_array1.expect("tick_array1 is not set"),
            tick_array2: self.tick_array2.expect("tick_array2 is not set"),
            oracle: self.oracle.expect("oracle is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `whirlpool_swap` CPI accounts.
pub struct WhirlpoolSwapCpiAccounts<'a, 'b> {
    pub swap_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_owner_account_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_owner_account_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub tick_array0: &'b solana_program::account_info::AccountInfo<'a>,

    pub tick_array1: &'b solana_program::account_info::AccountInfo<'a>,

    pub tick_array2: &'b solana_program::account_info::AccountInfo<'a>,
    /// Oracle is currently unused and will be enabled on subsequent updates
    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `whirlpool_swap` CPI instruction.
pub struct WhirlpoolSwapCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub swap_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_owner_account_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_owner_account_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub tick_array0: &'b solana_program::account_info::AccountInfo<'a>,

    pub tick_array1: &'b solana_program::account_info::AccountInfo<'a>,

    pub tick_array2: &'b solana_program::account_info::AccountInfo<'a>,
    /// Oracle is currently unused and will be enabled on subsequent updates
    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> WhirlpoolSwapCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: WhirlpoolSwapCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            swap_program: accounts.swap_program,
            token_program: accounts.token_program,
            token_authority: accounts.token_authority,
            whirlpool: accounts.whirlpool,
            token_owner_account_a: accounts.token_owner_account_a,
            token_vault_a: accounts.token_vault_a,
            token_owner_account_b: accounts.token_owner_account_b,
            token_vault_b: accounts.token_vault_b,
            tick_array0: accounts.tick_array0,
            tick_array1: accounts.tick_array1,
            tick_array2: accounts.tick_array2,
            oracle: accounts.oracle,
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
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.whirlpool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_owner_account_a.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_vault_a.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_owner_account_b.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_vault_b.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.tick_array0.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.tick_array1.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.tick_array2.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.oracle.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = WhirlpoolSwapInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(12 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.swap_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.token_authority.clone());
        account_infos.push(self.whirlpool.clone());
        account_infos.push(self.token_owner_account_a.clone());
        account_infos.push(self.token_vault_a.clone());
        account_infos.push(self.token_owner_account_b.clone());
        account_infos.push(self.token_vault_b.clone());
        account_infos.push(self.tick_array0.clone());
        account_infos.push(self.tick_array1.clone());
        account_infos.push(self.tick_array2.clone());
        account_infos.push(self.oracle.clone());
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

/// Instruction builder for `WhirlpoolSwap` via CPI.
///
/// ### Accounts:
///
///   0. `[]` swap_program
///   1. `[]` token_program
///   2. `[]` token_authority
///   3. `[writable]` whirlpool
///   4. `[writable]` token_owner_account_a
///   5. `[writable]` token_vault_a
///   6. `[writable]` token_owner_account_b
///   7. `[writable]` token_vault_b
///   8. `[writable]` tick_array0
///   9. `[writable]` tick_array1
///   10. `[writable]` tick_array2
///   11. `[]` oracle
pub struct WhirlpoolSwapCpiBuilder<'a, 'b> {
    instruction: Box<WhirlpoolSwapCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> WhirlpoolSwapCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(WhirlpoolSwapCpiBuilderInstruction {
            __program: program,
            swap_program: None,
            token_program: None,
            token_authority: None,
            whirlpool: None,
            token_owner_account_a: None,
            token_vault_a: None,
            token_owner_account_b: None,
            token_vault_b: None,
            tick_array0: None,
            tick_array1: None,
            tick_array2: None,
            oracle: None,
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
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn token_authority(
        &mut self,
        token_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_authority = Some(token_authority);
        self
    }
    #[inline(always)]
    pub fn whirlpool(
        &mut self,
        whirlpool: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.whirlpool = Some(whirlpool);
        self
    }
    #[inline(always)]
    pub fn token_owner_account_a(
        &mut self,
        token_owner_account_a: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_owner_account_a = Some(token_owner_account_a);
        self
    }
    #[inline(always)]
    pub fn token_vault_a(
        &mut self,
        token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_vault_a = Some(token_vault_a);
        self
    }
    #[inline(always)]
    pub fn token_owner_account_b(
        &mut self,
        token_owner_account_b: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_owner_account_b = Some(token_owner_account_b);
        self
    }
    #[inline(always)]
    pub fn token_vault_b(
        &mut self,
        token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_vault_b = Some(token_vault_b);
        self
    }
    #[inline(always)]
    pub fn tick_array0(
        &mut self,
        tick_array0: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.tick_array0 = Some(tick_array0);
        self
    }
    #[inline(always)]
    pub fn tick_array1(
        &mut self,
        tick_array1: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.tick_array1 = Some(tick_array1);
        self
    }
    #[inline(always)]
    pub fn tick_array2(
        &mut self,
        tick_array2: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.tick_array2 = Some(tick_array2);
        self
    }
    /// Oracle is currently unused and will be enabled on subsequent updates
    #[inline(always)]
    pub fn oracle(
        &mut self,
        oracle: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.oracle = Some(oracle);
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
        let instruction = WhirlpoolSwapCpi {
            __program: self.instruction.__program,

            swap_program: self
                .instruction
                .swap_program
                .expect("swap_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            token_authority: self
                .instruction
                .token_authority
                .expect("token_authority is not set"),

            whirlpool: self.instruction.whirlpool.expect("whirlpool is not set"),

            token_owner_account_a: self
                .instruction
                .token_owner_account_a
                .expect("token_owner_account_a is not set"),

            token_vault_a: self
                .instruction
                .token_vault_a
                .expect("token_vault_a is not set"),

            token_owner_account_b: self
                .instruction
                .token_owner_account_b
                .expect("token_owner_account_b is not set"),

            token_vault_b: self
                .instruction
                .token_vault_b
                .expect("token_vault_b is not set"),

            tick_array0: self
                .instruction
                .tick_array0
                .expect("tick_array0 is not set"),

            tick_array1: self
                .instruction
                .tick_array1
                .expect("tick_array1 is not set"),

            tick_array2: self
                .instruction
                .tick_array2
                .expect("tick_array2 is not set"),

            oracle: self.instruction.oracle.expect("oracle is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct WhirlpoolSwapCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    swap_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    whirlpool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_owner_account_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_vault_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_owner_account_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_vault_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    tick_array0: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    tick_array1: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    tick_array2: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    oracle: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
