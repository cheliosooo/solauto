//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::RebalanceArgs;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct MarginfiRebalance {
    pub signer: solana_program::pubkey::Pubkey,

    pub marginfi_program: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub ata_program: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub ixs_sysvar: solana_program::pubkey::Pubkey,

    pub solauto_fees_supply_ta: solana_program::pubkey::Pubkey,

    pub authority_referral_state: solana_program::pubkey::Pubkey,

    pub referred_by_supply_ta: Option<solana_program::pubkey::Pubkey>,

    pub solauto_position: solana_program::pubkey::Pubkey,

    pub marginfi_group: solana_program::pubkey::Pubkey,

    pub marginfi_account: solana_program::pubkey::Pubkey,

    pub intermediary_ta: solana_program::pubkey::Pubkey,

    pub position_supply_ta: solana_program::pubkey::Pubkey,

    pub bank_supply_ta: solana_program::pubkey::Pubkey,

    pub position_debt_ta: solana_program::pubkey::Pubkey,

    pub bank_debt_ta: solana_program::pubkey::Pubkey,
}

impl MarginfiRebalance {
    pub fn instruction(
        &self,
        args: MarginfiRebalanceInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: MarginfiRebalanceInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(18 + remaining_accounts.len());
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.ixs_sysvar,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.solauto_fees_supply_ta,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority_referral_state,
            false,
        ));
        if let Some(referred_by_supply_ta) = self.referred_by_supply_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                referred_by_supply_ta,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.solauto_position,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.marginfi_group,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.marginfi_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.intermediary_ta,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.position_supply_ta,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.bank_supply_ta,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.position_debt_ta,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.bank_debt_ta,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = MarginfiRebalanceInstructionData::new()
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
struct MarginfiRebalanceInstructionData {
    discriminator: u8,
}

impl MarginfiRebalanceInstructionData {
    fn new() -> Self {
        Self { discriminator: 11 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarginfiRebalanceInstructionArgs {
    pub rebalance_args: RebalanceArgs,
}

/// Instruction builder for `MarginfiRebalance`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[]` marginfi_program
///   2. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   3. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   4. `[optional]` ata_program (default to `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`)
///   5. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   6. `[]` ixs_sysvar
///   7. `[writable]` solauto_fees_supply_ta
///   8. `[]` authority_referral_state
///   9. `[writable, optional]` referred_by_supply_ta
///   10. `[writable]` solauto_position
///   11. `[]` marginfi_group
///   12. `[writable]` marginfi_account
///   13. `[writable]` intermediary_ta
///   14. `[writable]` position_supply_ta
///   15. `[writable]` bank_supply_ta
///   16. `[writable]` position_debt_ta
///   17. `[writable]` bank_debt_ta
#[derive(Default)]
pub struct MarginfiRebalanceBuilder {
    signer: Option<solana_program::pubkey::Pubkey>,
    marginfi_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    ata_program: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    ixs_sysvar: Option<solana_program::pubkey::Pubkey>,
    solauto_fees_supply_ta: Option<solana_program::pubkey::Pubkey>,
    authority_referral_state: Option<solana_program::pubkey::Pubkey>,
    referred_by_supply_ta: Option<solana_program::pubkey::Pubkey>,
    solauto_position: Option<solana_program::pubkey::Pubkey>,
    marginfi_group: Option<solana_program::pubkey::Pubkey>,
    marginfi_account: Option<solana_program::pubkey::Pubkey>,
    intermediary_ta: Option<solana_program::pubkey::Pubkey>,
    position_supply_ta: Option<solana_program::pubkey::Pubkey>,
    bank_supply_ta: Option<solana_program::pubkey::Pubkey>,
    position_debt_ta: Option<solana_program::pubkey::Pubkey>,
    bank_debt_ta: Option<solana_program::pubkey::Pubkey>,
    rebalance_args: Option<RebalanceArgs>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl MarginfiRebalanceBuilder {
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
    pub fn ixs_sysvar(&mut self, ixs_sysvar: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ixs_sysvar = Some(ixs_sysvar);
        self
    }
    #[inline(always)]
    pub fn solauto_fees_supply_ta(
        &mut self,
        solauto_fees_supply_ta: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.solauto_fees_supply_ta = Some(solauto_fees_supply_ta);
        self
    }
    #[inline(always)]
    pub fn authority_referral_state(
        &mut self,
        authority_referral_state: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.authority_referral_state = Some(authority_referral_state);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn referred_by_supply_ta(
        &mut self,
        referred_by_supply_ta: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.referred_by_supply_ta = referred_by_supply_ta;
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
    #[inline(always)]
    pub fn marginfi_group(&mut self, marginfi_group: solana_program::pubkey::Pubkey) -> &mut Self {
        self.marginfi_group = Some(marginfi_group);
        self
    }
    #[inline(always)]
    pub fn marginfi_account(
        &mut self,
        marginfi_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.marginfi_account = Some(marginfi_account);
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
    #[inline(always)]
    pub fn position_supply_ta(
        &mut self,
        position_supply_ta: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.position_supply_ta = Some(position_supply_ta);
        self
    }
    #[inline(always)]
    pub fn bank_supply_ta(&mut self, bank_supply_ta: solana_program::pubkey::Pubkey) -> &mut Self {
        self.bank_supply_ta = Some(bank_supply_ta);
        self
    }
    #[inline(always)]
    pub fn position_debt_ta(
        &mut self,
        position_debt_ta: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.position_debt_ta = Some(position_debt_ta);
        self
    }
    #[inline(always)]
    pub fn bank_debt_ta(&mut self, bank_debt_ta: solana_program::pubkey::Pubkey) -> &mut Self {
        self.bank_debt_ta = Some(bank_debt_ta);
        self
    }
    #[inline(always)]
    pub fn rebalance_args(&mut self, rebalance_args: RebalanceArgs) -> &mut Self {
        self.rebalance_args = Some(rebalance_args);
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
        let accounts = MarginfiRebalance {
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
            ixs_sysvar: self.ixs_sysvar.expect("ixs_sysvar is not set"),
            solauto_fees_supply_ta: self
                .solauto_fees_supply_ta
                .expect("solauto_fees_supply_ta is not set"),
            authority_referral_state: self
                .authority_referral_state
                .expect("authority_referral_state is not set"),
            referred_by_supply_ta: self.referred_by_supply_ta,
            solauto_position: self.solauto_position.expect("solauto_position is not set"),
            marginfi_group: self.marginfi_group.expect("marginfi_group is not set"),
            marginfi_account: self.marginfi_account.expect("marginfi_account is not set"),
            intermediary_ta: self.intermediary_ta.expect("intermediary_ta is not set"),
            position_supply_ta: self
                .position_supply_ta
                .expect("position_supply_ta is not set"),
            bank_supply_ta: self.bank_supply_ta.expect("bank_supply_ta is not set"),
            position_debt_ta: self.position_debt_ta.expect("position_debt_ta is not set"),
            bank_debt_ta: self.bank_debt_ta.expect("bank_debt_ta is not set"),
        };
        let args = MarginfiRebalanceInstructionArgs {
            rebalance_args: self
                .rebalance_args
                .clone()
                .expect("rebalance_args is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `marginfi_rebalance` CPI accounts.
pub struct MarginfiRebalanceCpiAccounts<'a, 'b> {
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub marginfi_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub ixs_sysvar: &'b solana_program::account_info::AccountInfo<'a>,

    pub solauto_fees_supply_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority_referral_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub referred_by_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub solauto_position: &'b solana_program::account_info::AccountInfo<'a>,

    pub marginfi_group: &'b solana_program::account_info::AccountInfo<'a>,

    pub marginfi_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub intermediary_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_supply_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub bank_supply_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_debt_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub bank_debt_ta: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `marginfi_rebalance` CPI instruction.
pub struct MarginfiRebalanceCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub marginfi_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub ixs_sysvar: &'b solana_program::account_info::AccountInfo<'a>,

    pub solauto_fees_supply_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority_referral_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub referred_by_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub solauto_position: &'b solana_program::account_info::AccountInfo<'a>,

    pub marginfi_group: &'b solana_program::account_info::AccountInfo<'a>,

    pub marginfi_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub intermediary_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_supply_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub bank_supply_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_debt_ta: &'b solana_program::account_info::AccountInfo<'a>,

    pub bank_debt_ta: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: MarginfiRebalanceInstructionArgs,
}

impl<'a, 'b> MarginfiRebalanceCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: MarginfiRebalanceCpiAccounts<'a, 'b>,
        args: MarginfiRebalanceInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            signer: accounts.signer,
            marginfi_program: accounts.marginfi_program,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            ata_program: accounts.ata_program,
            rent: accounts.rent,
            ixs_sysvar: accounts.ixs_sysvar,
            solauto_fees_supply_ta: accounts.solauto_fees_supply_ta,
            authority_referral_state: accounts.authority_referral_state,
            referred_by_supply_ta: accounts.referred_by_supply_ta,
            solauto_position: accounts.solauto_position,
            marginfi_group: accounts.marginfi_group,
            marginfi_account: accounts.marginfi_account,
            intermediary_ta: accounts.intermediary_ta,
            position_supply_ta: accounts.position_supply_ta,
            bank_supply_ta: accounts.bank_supply_ta,
            position_debt_ta: accounts.position_debt_ta,
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
        let mut accounts = Vec::with_capacity(18 + remaining_accounts.len());
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ixs_sysvar.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.solauto_fees_supply_ta.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority_referral_state.key,
            false,
        ));
        if let Some(referred_by_supply_ta) = self.referred_by_supply_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *referred_by_supply_ta.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.solauto_position.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.marginfi_group.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.marginfi_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.intermediary_ta.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position_supply_ta.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.bank_supply_ta.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position_debt_ta.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.bank_debt_ta.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = MarginfiRebalanceInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SOLAUTO_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(18 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.signer.clone());
        account_infos.push(self.marginfi_program.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.ata_program.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.ixs_sysvar.clone());
        account_infos.push(self.solauto_fees_supply_ta.clone());
        account_infos.push(self.authority_referral_state.clone());
        if let Some(referred_by_supply_ta) = self.referred_by_supply_ta {
            account_infos.push(referred_by_supply_ta.clone());
        }
        account_infos.push(self.solauto_position.clone());
        account_infos.push(self.marginfi_group.clone());
        account_infos.push(self.marginfi_account.clone());
        account_infos.push(self.intermediary_ta.clone());
        account_infos.push(self.position_supply_ta.clone());
        account_infos.push(self.bank_supply_ta.clone());
        account_infos.push(self.position_debt_ta.clone());
        account_infos.push(self.bank_debt_ta.clone());
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

/// Instruction builder for `MarginfiRebalance` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[]` marginfi_program
///   2. `[]` system_program
///   3. `[]` token_program
///   4. `[]` ata_program
///   5. `[]` rent
///   6. `[]` ixs_sysvar
///   7. `[writable]` solauto_fees_supply_ta
///   8. `[]` authority_referral_state
///   9. `[writable, optional]` referred_by_supply_ta
///   10. `[writable]` solauto_position
///   11. `[]` marginfi_group
///   12. `[writable]` marginfi_account
///   13. `[writable]` intermediary_ta
///   14. `[writable]` position_supply_ta
///   15. `[writable]` bank_supply_ta
///   16. `[writable]` position_debt_ta
///   17. `[writable]` bank_debt_ta
pub struct MarginfiRebalanceCpiBuilder<'a, 'b> {
    instruction: Box<MarginfiRebalanceCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> MarginfiRebalanceCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(MarginfiRebalanceCpiBuilderInstruction {
            __program: program,
            signer: None,
            marginfi_program: None,
            system_program: None,
            token_program: None,
            ata_program: None,
            rent: None,
            ixs_sysvar: None,
            solauto_fees_supply_ta: None,
            authority_referral_state: None,
            referred_by_supply_ta: None,
            solauto_position: None,
            marginfi_group: None,
            marginfi_account: None,
            intermediary_ta: None,
            position_supply_ta: None,
            bank_supply_ta: None,
            position_debt_ta: None,
            bank_debt_ta: None,
            rebalance_args: None,
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
    pub fn ixs_sysvar(
        &mut self,
        ixs_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.ixs_sysvar = Some(ixs_sysvar);
        self
    }
    #[inline(always)]
    pub fn solauto_fees_supply_ta(
        &mut self,
        solauto_fees_supply_ta: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.solauto_fees_supply_ta = Some(solauto_fees_supply_ta);
        self
    }
    #[inline(always)]
    pub fn authority_referral_state(
        &mut self,
        authority_referral_state: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority_referral_state = Some(authority_referral_state);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn referred_by_supply_ta(
        &mut self,
        referred_by_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.referred_by_supply_ta = referred_by_supply_ta;
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
    #[inline(always)]
    pub fn marginfi_group(
        &mut self,
        marginfi_group: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.marginfi_group = Some(marginfi_group);
        self
    }
    #[inline(always)]
    pub fn marginfi_account(
        &mut self,
        marginfi_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.marginfi_account = Some(marginfi_account);
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
    #[inline(always)]
    pub fn position_supply_ta(
        &mut self,
        position_supply_ta: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position_supply_ta = Some(position_supply_ta);
        self
    }
    #[inline(always)]
    pub fn bank_supply_ta(
        &mut self,
        bank_supply_ta: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.bank_supply_ta = Some(bank_supply_ta);
        self
    }
    #[inline(always)]
    pub fn position_debt_ta(
        &mut self,
        position_debt_ta: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position_debt_ta = Some(position_debt_ta);
        self
    }
    #[inline(always)]
    pub fn bank_debt_ta(
        &mut self,
        bank_debt_ta: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.bank_debt_ta = Some(bank_debt_ta);
        self
    }
    #[inline(always)]
    pub fn rebalance_args(&mut self, rebalance_args: RebalanceArgs) -> &mut Self {
        self.instruction.rebalance_args = Some(rebalance_args);
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
        let args = MarginfiRebalanceInstructionArgs {
            rebalance_args: self
                .instruction
                .rebalance_args
                .clone()
                .expect("rebalance_args is not set"),
        };
        let instruction = MarginfiRebalanceCpi {
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

            ixs_sysvar: self.instruction.ixs_sysvar.expect("ixs_sysvar is not set"),

            solauto_fees_supply_ta: self
                .instruction
                .solauto_fees_supply_ta
                .expect("solauto_fees_supply_ta is not set"),

            authority_referral_state: self
                .instruction
                .authority_referral_state
                .expect("authority_referral_state is not set"),

            referred_by_supply_ta: self.instruction.referred_by_supply_ta,

            solauto_position: self
                .instruction
                .solauto_position
                .expect("solauto_position is not set"),

            marginfi_group: self
                .instruction
                .marginfi_group
                .expect("marginfi_group is not set"),

            marginfi_account: self
                .instruction
                .marginfi_account
                .expect("marginfi_account is not set"),

            intermediary_ta: self
                .instruction
                .intermediary_ta
                .expect("intermediary_ta is not set"),

            position_supply_ta: self
                .instruction
                .position_supply_ta
                .expect("position_supply_ta is not set"),

            bank_supply_ta: self
                .instruction
                .bank_supply_ta
                .expect("bank_supply_ta is not set"),

            position_debt_ta: self
                .instruction
                .position_debt_ta
                .expect("position_debt_ta is not set"),

            bank_debt_ta: self
                .instruction
                .bank_debt_ta
                .expect("bank_debt_ta is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct MarginfiRebalanceCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    marginfi_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ata_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ixs_sysvar: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    solauto_fees_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority_referral_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    referred_by_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    solauto_position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    marginfi_group: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    marginfi_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    intermediary_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bank_supply_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bank_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rebalance_args: Option<RebalanceArgs>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
