//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct ConvertReferralFees {
    pub solauto_manager: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub ata_program: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub ixs_sysvar: solana_program::pubkey::Pubkey,

    pub referral_state: solana_program::pubkey::Pubkey,

    pub referral_fees_ta: solana_program::pubkey::Pubkey,

    pub intermediary_ta: solana_program::pubkey::Pubkey,
}

impl ConvertReferralFees {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.solauto_manager,
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
            self.ata_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.ixs_sysvar,
            false,
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
            self.intermediary_ta,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = ConvertReferralFeesInstructionData::new()
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
struct ConvertReferralFeesInstructionData {
    discriminator: u8,
}

impl ConvertReferralFeesInstructionData {
    fn new() -> Self {
        Self { discriminator: 1 }
    }
}

/// Instruction builder for `ConvertReferralFees`.
///
/// ### Accounts:
///
///   0. `[signer]` solauto_manager
///   1. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   2. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   3. `[optional]` ata_program (default to `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`)
///   4. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   5. `[]` ixs_sysvar
///   6. `[]` referral_state
///   7. `[writable]` referral_fees_ta
///   8. `[writable]` intermediary_ta
#[derive(Default)]
pub struct ConvertReferralFeesBuilder {
    solauto_manager: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    ata_program: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    ixs_sysvar: Option<solana_program::pubkey::Pubkey>,
    referral_state: Option<solana_program::pubkey::Pubkey>,
    referral_fees_ta: Option<solana_program::pubkey::Pubkey>,
    intermediary_ta: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ConvertReferralFeesBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn solauto_manager(
        &mut self,
        solauto_manager: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.solauto_manager = Some(solauto_manager);
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
    /// `[optional account, default to 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL']`
    #[inline(always)]
    pub fn ata_program(&mut self, ata_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ata_program = Some(ata_program);
        self
    }
    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }
    #[inline(always)]
    pub fn ixs_sysvar(&mut self, ixs_sysvar: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ixs_sysvar = Some(ixs_sysvar);
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
    pub fn intermediary_ta(
        &mut self,
        intermediary_ta: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.intermediary_ta = Some(intermediary_ta);
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
        let accounts = ConvertReferralFees {
            solauto_manager: self.solauto_manager.expect("solauto_manager is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            ata_program: self.ata_program.unwrap_or(solana_program::pubkey!(
                "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
            )),
            rent: self.rent.unwrap_or(solana_program::pubkey!(
                "SysvarRent111111111111111111111111111111111"
            )),
            ixs_sysvar: self.ixs_sysvar.expect("ixs_sysvar is not set"),
            referral_state: self.referral_state.expect("referral_state is not set"),
            referral_fees_ta: self.referral_fees_ta.expect("referral_fees_ta is not set"),
            intermediary_ta: self.intermediary_ta.expect("intermediary_ta is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `convert_referral_fees` CPI accounts.
pub struct ConvertReferralFeesCpiAccounts<'a, 'b> {
    pub solauto_manager: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub ixs_sysvar: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral_fees_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub intermediary_ta: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `convert_referral_fees` CPI instruction.
pub struct ConvertReferralFeesCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub solauto_manager: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub ixs_sysvar: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral_fees_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub intermediary_ta: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> ConvertReferralFeesCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ConvertReferralFeesCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            solauto_manager: accounts.solauto_manager,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            ata_program: accounts.ata_program,
            rent: accounts.rent,
            ixs_sysvar: accounts.ixs_sysvar,
            referral_state: accounts.referral_state,
            referral_fees_ta: accounts.referral_fees_ta,
            intermediary_ta: accounts.intermediary_ta,
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
            *self.solauto_manager.key,
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
            *self.ata_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ixs_sysvar.key,
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
            *self.intermediary_ta.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = ConvertReferralFeesInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SOLAUTO_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.solauto_manager.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.ata_program.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.ixs_sysvar.clone());
        account_infos.push(self.referral_state.clone());
        account_infos.push(self.referral_fees_ta.clone());
        account_infos.push(self.intermediary_ta.clone());
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

/// Instruction builder for `ConvertReferralFees` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` solauto_manager
///   1. `[]` system_program
///   2. `[]` token_program
///   3. `[]` ata_program
///   4. `[]` rent
///   5. `[]` ixs_sysvar
///   6. `[]` referral_state
///   7. `[writable]` referral_fees_ta
///   8. `[writable]` intermediary_ta
pub struct ConvertReferralFeesCpiBuilder<'a, 'b> {
    instruction: Box<ConvertReferralFeesCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ConvertReferralFeesCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ConvertReferralFeesCpiBuilderInstruction {
            __program: program,
            solauto_manager: None,
            system_program: None,
            token_program: None,
            ata_program: None,
            rent: None,
            ixs_sysvar: None,
            referral_state: None,
            referral_fees_ta: None,
            intermediary_ta: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn solauto_manager(
        &mut self,
        solauto_manager: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.solauto_manager = Some(solauto_manager);
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
    pub fn ata_program(
        &mut self,
        ata_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.ata_program = Some(ata_program);
        self
    }
    #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
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
    pub fn intermediary_ta(
        &mut self,
        intermediary_ta: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.intermediary_ta = Some(intermediary_ta);
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
        let instruction = ConvertReferralFeesCpi {
            __program: self.instruction.__program,

            solauto_manager: self
                .instruction
                .solauto_manager
                .expect("solauto_manager is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            ata_program: self
                .instruction
                .ata_program
                .expect("ata_program is not set"),

            rent: self.instruction.rent.expect("rent is not set"),

            ixs_sysvar: self.instruction.ixs_sysvar.expect("ixs_sysvar is not set"),

            referral_state: self
                .instruction
                .referral_state
                .expect("referral_state is not set"),

            referral_fees_ta: self
                .instruction
                .referral_fees_ta
                .expect("referral_fees_ta is not set"),

            intermediary_ta: self
                .instruction
                .intermediary_ta
                .expect("intermediary_ta is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct ConvertReferralFeesCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    solauto_manager: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ata_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ixs_sysvar: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    referral_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    referral_fees_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    intermediary_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
