//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::SolautoAction;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct MarginfiProtocolInteraction {
    pub signer: solana_program::pubkey::Pubkey,

    pub marginfi_program: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub ata_program: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub solauto_position: solana_program::pubkey::Pubkey,

    pub supply_mint: Option<solana_program::pubkey::Pubkey>,

    pub authority_supply_ta: Option<solana_program::pubkey::Pubkey>,

    pub bank_supply_ta: Option<solana_program::pubkey::Pubkey>,

    pub debt_mint: Option<solana_program::pubkey::Pubkey>,

    pub authority_debt_ta: Option<solana_program::pubkey::Pubkey>,

    pub bank_debt_ta: Option<solana_program::pubkey::Pubkey>,
}

impl MarginfiProtocolInteraction {
    pub fn instruction(
        &self,
        args: MarginfiProtocolInteractionInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: MarginfiProtocolInteractionInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(13 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.signer,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.marginfi_program,
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.ata_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.solauto_position,
            false,
        ));
        if let Some(supply_mint) = self.supply_mint {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                supply_mint,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(authority_supply_ta) = self.authority_supply_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                authority_supply_ta,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(bank_supply_ta) = self.bank_supply_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                bank_supply_ta,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
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
        if let Some(authority_debt_ta) = self.authority_debt_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                authority_debt_ta,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(bank_debt_ta) = self.bank_debt_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                bank_debt_ta,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let mut data = MarginfiProtocolInteractionInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::SOLAUTO_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct MarginfiProtocolInteractionInstructionData {
    discriminator: u8,
}

impl MarginfiProtocolInteractionInstructionData {
    fn new() -> Self {
        Self { discriminator: 9 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarginfiProtocolInteractionInstructionArgs {
    pub solauto_action: SolautoAction,
}

/// Instruction builder for `MarginfiProtocolInteraction`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[]` marginfi_program
///   2. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   3. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   4. `[optional]` ata_program (default to `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`)
///   5. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   6. `[writable]` solauto_position
///   7. `[optional]` supply_mint
///   8. `[writable, optional]` authority_supply_ta
///   9. `[writable, optional]` bank_supply_ta
///   10. `[optional]` debt_mint
///   11. `[writable, optional]` authority_debt_ta
///   12. `[writable, optional]` bank_debt_ta
#[derive(Default)]
pub struct MarginfiProtocolInteractionBuilder {
    signer: Option<solana_program::pubkey::Pubkey>,
    marginfi_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    ata_program: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    solauto_position: Option<solana_program::pubkey::Pubkey>,
    supply_mint: Option<solana_program::pubkey::Pubkey>,
    authority_supply_ta: Option<solana_program::pubkey::Pubkey>,
    bank_supply_ta: Option<solana_program::pubkey::Pubkey>,
    debt_mint: Option<solana_program::pubkey::Pubkey>,
    authority_debt_ta: Option<solana_program::pubkey::Pubkey>,
    bank_debt_ta: Option<solana_program::pubkey::Pubkey>,
    solauto_action: Option<SolautoAction>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl MarginfiProtocolInteractionBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.signer = Some(signer);
        self
    }
    #[inline(always)]
    pub fn marginfi_program(
        &mut self,
        marginfi_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.marginfi_program = Some(marginfi_program);
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
    pub fn solauto_position(
        &mut self,
        solauto_position: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.solauto_position = Some(solauto_position);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn supply_mint(
        &mut self,
        supply_mint: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.supply_mint = supply_mint;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn authority_supply_ta(
        &mut self,
        authority_supply_ta: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.authority_supply_ta = authority_supply_ta;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn bank_supply_ta(
        &mut self,
        bank_supply_ta: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.bank_supply_ta = bank_supply_ta;
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
    pub fn authority_debt_ta(
        &mut self,
        authority_debt_ta: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.authority_debt_ta = authority_debt_ta;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn bank_debt_ta(
        &mut self,
        bank_debt_ta: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.bank_debt_ta = bank_debt_ta;
        self
    }
    #[inline(always)]
    pub fn solauto_action(&mut self, solauto_action: SolautoAction) -> &mut Self {
        self.solauto_action = Some(solauto_action);
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
        let accounts = MarginfiProtocolInteraction {
            signer: self.signer.expect("signer is not set"),
            marginfi_program: self.marginfi_program.expect("marginfi_program is not set"),
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
            solauto_position: self.solauto_position.expect("solauto_position is not set"),
            supply_mint: self.supply_mint,
            authority_supply_ta: self.authority_supply_ta,
            bank_supply_ta: self.bank_supply_ta,
            debt_mint: self.debt_mint,
            authority_debt_ta: self.authority_debt_ta,
            bank_debt_ta: self.bank_debt_ta,
        };
        let args = MarginfiProtocolInteractionInstructionArgs {
            solauto_action: self
                .solauto_action
                .clone()
                .expect("solauto_action is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `marginfi_protocol_interaction` CPI accounts.
pub struct MarginfiProtocolInteractionCpiAccounts<'a, 'b> {
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub marginfi_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub solauto_position: &'b solana_program::account_info::AccountInfo<'a>,

    pub supply_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub authority_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub bank_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub debt_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub authority_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub bank_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `marginfi_protocol_interaction` CPI instruction.
pub struct MarginfiProtocolInteractionCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub marginfi_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub solauto_position: &'b solana_program::account_info::AccountInfo<'a>,

    pub supply_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub authority_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub bank_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub debt_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub authority_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub bank_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The arguments for the instruction.
    pub __args: MarginfiProtocolInteractionInstructionArgs,
}

impl<'a, 'b> MarginfiProtocolInteractionCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: MarginfiProtocolInteractionCpiAccounts<'a, 'b>,
        args: MarginfiProtocolInteractionInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            signer: accounts.signer,
            marginfi_program: accounts.marginfi_program,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            ata_program: accounts.ata_program,
            rent: accounts.rent,
            solauto_position: accounts.solauto_position,
            supply_mint: accounts.supply_mint,
            authority_supply_ta: accounts.authority_supply_ta,
            bank_supply_ta: accounts.bank_supply_ta,
            debt_mint: accounts.debt_mint,
            authority_debt_ta: accounts.authority_debt_ta,
            bank_debt_ta: accounts.bank_debt_ta,
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
        let mut accounts = Vec::with_capacity(13 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.signer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.marginfi_program.key,
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ata_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.solauto_position.key,
            false,
        ));
        if let Some(supply_mint) = self.supply_mint {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *supply_mint.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(authority_supply_ta) = self.authority_supply_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *authority_supply_ta.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(bank_supply_ta) = self.bank_supply_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *bank_supply_ta.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
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
        if let Some(authority_debt_ta) = self.authority_debt_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *authority_debt_ta.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(bank_debt_ta) = self.bank_debt_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *bank_debt_ta.key,
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
        let mut data = MarginfiProtocolInteractionInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SOLAUTO_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(13 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.signer.clone());
        account_infos.push(self.marginfi_program.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.ata_program.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.solauto_position.clone());
        if let Some(supply_mint) = self.supply_mint {
            account_infos.push(supply_mint.clone());
        }
        if let Some(authority_supply_ta) = self.authority_supply_ta {
            account_infos.push(authority_supply_ta.clone());
        }
        if let Some(bank_supply_ta) = self.bank_supply_ta {
            account_infos.push(bank_supply_ta.clone());
        }
        if let Some(debt_mint) = self.debt_mint {
            account_infos.push(debt_mint.clone());
        }
        if let Some(authority_debt_ta) = self.authority_debt_ta {
            account_infos.push(authority_debt_ta.clone());
        }
        if let Some(bank_debt_ta) = self.bank_debt_ta {
            account_infos.push(bank_debt_ta.clone());
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

/// Instruction builder for `MarginfiProtocolInteraction` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[]` marginfi_program
///   2. `[]` system_program
///   3. `[]` token_program
///   4. `[]` ata_program
///   5. `[]` rent
///   6. `[writable]` solauto_position
///   7. `[optional]` supply_mint
///   8. `[writable, optional]` authority_supply_ta
///   9. `[writable, optional]` bank_supply_ta
///   10. `[optional]` debt_mint
///   11. `[writable, optional]` authority_debt_ta
///   12. `[writable, optional]` bank_debt_ta
pub struct MarginfiProtocolInteractionCpiBuilder<'a, 'b> {
    instruction: Box<MarginfiProtocolInteractionCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> MarginfiProtocolInteractionCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(MarginfiProtocolInteractionCpiBuilderInstruction {
            __program: program,
            signer: None,
            marginfi_program: None,
            system_program: None,
            token_program: None,
            ata_program: None,
            rent: None,
            solauto_position: None,
            supply_mint: None,
            authority_supply_ta: None,
            bank_supply_ta: None,
            debt_mint: None,
            authority_debt_ta: None,
            bank_debt_ta: None,
            solauto_action: None,
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
    pub fn marginfi_program(
        &mut self,
        marginfi_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.marginfi_program = Some(marginfi_program);
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
    pub fn solauto_position(
        &mut self,
        solauto_position: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.solauto_position = Some(solauto_position);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn supply_mint(
        &mut self,
        supply_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.supply_mint = supply_mint;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn authority_supply_ta(
        &mut self,
        authority_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.authority_supply_ta = authority_supply_ta;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn bank_supply_ta(
        &mut self,
        bank_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.bank_supply_ta = bank_supply_ta;
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
    pub fn authority_debt_ta(
        &mut self,
        authority_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.authority_debt_ta = authority_debt_ta;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn bank_debt_ta(
        &mut self,
        bank_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.bank_debt_ta = bank_debt_ta;
        self
    }
    #[inline(always)]
    pub fn solauto_action(&mut self, solauto_action: SolautoAction) -> &mut Self {
        self.instruction.solauto_action = Some(solauto_action);
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
        let args = MarginfiProtocolInteractionInstructionArgs {
            solauto_action: self
                .instruction
                .solauto_action
                .clone()
                .expect("solauto_action is not set"),
        };
        let instruction = MarginfiProtocolInteractionCpi {
            __program: self.instruction.__program,

            signer: self.instruction.signer.expect("signer is not set"),

            marginfi_program: self
                .instruction
                .marginfi_program
                .expect("marginfi_program is not set"),

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

            solauto_position: self
                .instruction
                .solauto_position
                .expect("solauto_position is not set"),

            supply_mint: self.instruction.supply_mint,

            authority_supply_ta: self.instruction.authority_supply_ta,

            bank_supply_ta: self.instruction.bank_supply_ta,

            debt_mint: self.instruction.debt_mint,

            authority_debt_ta: self.instruction.authority_debt_ta,

            bank_debt_ta: self.instruction.bank_debt_ta,
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct MarginfiProtocolInteractionCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    marginfi_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ata_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    solauto_position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    supply_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bank_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    debt_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bank_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    solauto_action: Option<SolautoAction>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
