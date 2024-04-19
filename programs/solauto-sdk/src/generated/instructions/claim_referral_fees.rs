//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct ClaimReferralFees {
    pub signer: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub referral_state: solana_program::pubkey::Pubkey,

    pub referral_fees_ta: solana_program::pubkey::Pubkey,

    pub referral_fees_mint: solana_program::pubkey::Pubkey,

    pub dest_ta: Option<solana_program::pubkey::Pubkey>,
}

impl ClaimReferralFees {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.signer,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.referral_state,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.referral_fees_ta,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.referral_fees_mint,
            false,
        ));
        if let Some(dest_ta) = self.dest_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                dest_ta, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let data = ClaimReferralFeesInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::SOLAUTO_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct ClaimReferralFeesInstructionData {
    discriminator: u8,
}

impl ClaimReferralFeesInstructionData {
    fn new() -> Self {
        Self { discriminator: 1 }
    }
}

/// Instruction builder for `ClaimReferralFees`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   2. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   3. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   4. `[]` referral_state
///   5. `[writable]` referral_fees_ta
///   6. `[writable]` referral_fees_mint
///   7. `[writable, optional]` dest_ta
#[derive(Default)]
pub struct ClaimReferralFeesBuilder {
    signer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    referral_state: Option<solana_program::pubkey::Pubkey>,
    referral_fees_ta: Option<solana_program::pubkey::Pubkey>,
    referral_fees_mint: Option<solana_program::pubkey::Pubkey>,
    dest_ta: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ClaimReferralFeesBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.signer = Some(signer);
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
    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }
    #[inline(always)]
    pub fn referral_state(&mut self, referral_state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.referral_state = Some(referral_state);
        self
    }
    #[inline(always)]
    pub fn referral_fees_ta(
        &mut self,
        referral_fees_ta: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.referral_fees_ta = Some(referral_fees_ta);
        self
    }
    #[inline(always)]
    pub fn referral_fees_mint(
        &mut self,
        referral_fees_mint: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.referral_fees_mint = Some(referral_fees_mint);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn dest_ta(&mut self, dest_ta: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.dest_ta = dest_ta;
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
        let accounts = ClaimReferralFees {
            signer: self.signer.expect("signer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            rent: self.rent.unwrap_or(solana_program::pubkey!(
                "SysvarRent111111111111111111111111111111111"
            )),
            referral_state: self.referral_state.expect("referral_state is not set"),
            referral_fees_ta: self.referral_fees_ta.expect("referral_fees_ta is not set"),
            referral_fees_mint: self
                .referral_fees_mint
                .expect("referral_fees_mint is not set"),
            dest_ta: self.dest_ta,
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `claim_referral_fees` CPI accounts.
pub struct ClaimReferralFeesCpiAccounts<'a, 'b> {
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral_fees_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral_fees_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub dest_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `claim_referral_fees` CPI instruction.
pub struct ClaimReferralFeesCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral_fees_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral_fees_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub dest_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

impl<'a, 'b> ClaimReferralFeesCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ClaimReferralFeesCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            signer: accounts.signer,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            rent: accounts.rent,
            referral_state: accounts.referral_state,
            referral_fees_ta: accounts.referral_fees_ta,
            referral_fees_mint: accounts.referral_fees_mint,
            dest_ta: accounts.dest_ta,
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
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.signer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.referral_state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.referral_fees_ta.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.referral_fees_mint.key,
            false,
        ));
        if let Some(dest_ta) = self.dest_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *dest_ta.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = ClaimReferralFeesInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SOLAUTO_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(8 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.signer.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.referral_state.clone());
        account_infos.push(self.referral_fees_ta.clone());
        account_infos.push(self.referral_fees_mint.clone());
        if let Some(dest_ta) = self.dest_ta {
            account_infos.push(dest_ta.clone());
        }
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

/// Instruction builder for `ClaimReferralFees` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[]` system_program
///   2. `[]` token_program
///   3. `[]` rent
///   4. `[]` referral_state
///   5. `[writable]` referral_fees_ta
///   6. `[writable]` referral_fees_mint
///   7. `[writable, optional]` dest_ta
pub struct ClaimReferralFeesCpiBuilder<'a, 'b> {
    instruction: Box<ClaimReferralFeesCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ClaimReferralFeesCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ClaimReferralFeesCpiBuilderInstruction {
            __program: program,
            signer: None,
            system_program: None,
            token_program: None,
            rent: None,
            referral_state: None,
            referral_fees_ta: None,
            referral_fees_mint: None,
            dest_ta: None,
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
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
        self
    }
    #[inline(always)]
    pub fn referral_state(
        &mut self,
        referral_state: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.referral_state = Some(referral_state);
        self
    }
    #[inline(always)]
    pub fn referral_fees_ta(
        &mut self,
        referral_fees_ta: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.referral_fees_ta = Some(referral_fees_ta);
        self
    }
    #[inline(always)]
    pub fn referral_fees_mint(
        &mut self,
        referral_fees_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.referral_fees_mint = Some(referral_fees_mint);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn dest_ta(
        &mut self,
        dest_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.dest_ta = dest_ta;
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
        let instruction = ClaimReferralFeesCpi {
            __program: self.instruction.__program,

            signer: self.instruction.signer.expect("signer is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            rent: self.instruction.rent.expect("rent is not set"),

            referral_state: self
                .instruction
                .referral_state
                .expect("referral_state is not set"),

            referral_fees_ta: self
                .instruction
                .referral_fees_ta
                .expect("referral_fees_ta is not set"),

            referral_fees_mint: self
                .instruction
                .referral_fees_mint
                .expect("referral_fees_mint is not set"),

            dest_ta: self.instruction.dest_ta,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct ClaimReferralFeesCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    referral_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    referral_fees_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    referral_fees_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    dest_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
