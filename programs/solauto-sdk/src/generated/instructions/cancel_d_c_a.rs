//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CancelDCA {
    pub signer: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub ata_program: solana_program::pubkey::Pubkey,

    pub solauto_position: solana_program::pubkey::Pubkey,

    pub debt_mint: Option<solana_program::pubkey::Pubkey>,

    pub position_debt_ta: Option<solana_program::pubkey::Pubkey>,

    pub signer_debt_ta: Option<solana_program::pubkey::Pubkey>,
}

impl CancelDCA {
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
            self.ata_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.solauto_position,
            false,
        ));
        if let Some(debt_mint) = self.debt_mint {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                debt_mint, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(position_debt_ta) = self.position_debt_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                position_debt_ta,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(signer_debt_ta) = self.signer_debt_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                signer_debt_ta,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let data = CancelDCAInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::SOLAUTO_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct CancelDCAInstructionData {
    discriminator: u8,
}

impl CancelDCAInstructionData {
    fn new() -> Self {
        Self { discriminator: 7 }
    }
}

/// Instruction builder for `CancelDCA`.
///
/// ### Accounts:
///
///   0. `[signer]` signer
///   1. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   2. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   3. `[optional]` ata_program (default to `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`)
///   4. `[writable]` solauto_position
///   5. `[optional]` debt_mint
///   6. `[writable, optional]` position_debt_ta
///   7. `[writable, optional]` signer_debt_ta
#[derive(Default)]
pub struct CancelDCABuilder {
    signer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    ata_program: Option<solana_program::pubkey::Pubkey>,
    solauto_position: Option<solana_program::pubkey::Pubkey>,
    debt_mint: Option<solana_program::pubkey::Pubkey>,
    position_debt_ta: Option<solana_program::pubkey::Pubkey>,
    signer_debt_ta: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CancelDCABuilder {
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
    /// `[optional account, default to 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL']`
    #[inline(always)]
    pub fn ata_program(&mut self, ata_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ata_program = Some(ata_program);
        self
    }
    #[inline(always)]
    pub fn solauto_position(
        &mut self,
        solauto_position: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.solauto_position = Some(solauto_position);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn debt_mint(&mut self, debt_mint: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.debt_mint = debt_mint;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn position_debt_ta(
        &mut self,
        position_debt_ta: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.position_debt_ta = position_debt_ta;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn signer_debt_ta(
        &mut self,
        signer_debt_ta: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.signer_debt_ta = signer_debt_ta;
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
        let accounts = CancelDCA {
            signer: self.signer.expect("signer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            ata_program: self.ata_program.unwrap_or(solana_program::pubkey!(
                "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
            )),
            solauto_position: self.solauto_position.expect("solauto_position is not set"),
            debt_mint: self.debt_mint,
            position_debt_ta: self.position_debt_ta,
            signer_debt_ta: self.signer_debt_ta,
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `cancel_d_c_a` CPI accounts.
pub struct CancelDCACpiAccounts<'a, 'b> {
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub solauto_position: &'b solana_program::account_info::AccountInfo<'a>,

    pub debt_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub position_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub signer_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `cancel_d_c_a` CPI instruction.
pub struct CancelDCACpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub solauto_position: &'b solana_program::account_info::AccountInfo<'a>,

    pub debt_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub position_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub signer_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

impl<'a, 'b> CancelDCACpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CancelDCACpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            signer: accounts.signer,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            ata_program: accounts.ata_program,
            solauto_position: accounts.solauto_position,
            debt_mint: accounts.debt_mint,
            position_debt_ta: accounts.position_debt_ta,
            signer_debt_ta: accounts.signer_debt_ta,
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
            *self.ata_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.solauto_position.key,
            false,
        ));
        if let Some(debt_mint) = self.debt_mint {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *debt_mint.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(position_debt_ta) = self.position_debt_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *position_debt_ta.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(signer_debt_ta) = self.signer_debt_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *signer_debt_ta.key,
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
        let data = CancelDCAInstructionData::new().try_to_vec().unwrap();

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
        account_infos.push(self.ata_program.clone());
        account_infos.push(self.solauto_position.clone());
        if let Some(debt_mint) = self.debt_mint {
            account_infos.push(debt_mint.clone());
        }
        if let Some(position_debt_ta) = self.position_debt_ta {
            account_infos.push(position_debt_ta.clone());
        }
        if let Some(signer_debt_ta) = self.signer_debt_ta {
            account_infos.push(signer_debt_ta.clone());
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

/// Instruction builder for `CancelDCA` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` signer
///   1. `[]` system_program
///   2. `[]` token_program
///   3. `[]` ata_program
///   4. `[writable]` solauto_position
///   5. `[optional]` debt_mint
///   6. `[writable, optional]` position_debt_ta
///   7. `[writable, optional]` signer_debt_ta
pub struct CancelDCACpiBuilder<'a, 'b> {
    instruction: Box<CancelDCACpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CancelDCACpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CancelDCACpiBuilderInstruction {
            __program: program,
            signer: None,
            system_program: None,
            token_program: None,
            ata_program: None,
            solauto_position: None,
            debt_mint: None,
            position_debt_ta: None,
            signer_debt_ta: None,
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
    pub fn ata_program(
        &mut self,
        ata_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.ata_program = Some(ata_program);
        self
    }
    #[inline(always)]
    pub fn solauto_position(
        &mut self,
        solauto_position: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.solauto_position = Some(solauto_position);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn debt_mint(
        &mut self,
        debt_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.debt_mint = debt_mint;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn position_debt_ta(
        &mut self,
        position_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.position_debt_ta = position_debt_ta;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn signer_debt_ta(
        &mut self,
        signer_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.signer_debt_ta = signer_debt_ta;
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
        let instruction = CancelDCACpi {
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

            ata_program: self
                .instruction
                .ata_program
                .expect("ata_program is not set"),

            solauto_position: self
                .instruction
                .solauto_position
                .expect("solauto_position is not set"),

            debt_mint: self.instruction.debt_mint,

            position_debt_ta: self.instruction.position_debt_ta,

            signer_debt_ta: self.instruction.signer_debt_ta,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct CancelDCACpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ata_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    solauto_position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    debt_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    signer_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
