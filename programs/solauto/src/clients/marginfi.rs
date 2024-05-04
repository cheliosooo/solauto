use anchor_lang::{accounts::signer::Signer, context::CpiContext};
use marginfi_sdk::generated::{
    accounts::{Bank, MarginfiAccount},
    types::RiskTier,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
};

use crate::{
    types::{
        instruction::{
            accounts::{Context, MarginfiOpenPositionAccounts},
            SolautoStandardAccounts,
        },
        lending_protocol::{LendingProtocolClient, LendingProtocolTokenAccounts},
        obligation_position::LendingProtocolObligationPosition,
        shared::{DeserializedAccount, PositionAccount, SolautoError},
    },
    utils::validation_utils::*,
};

pub struct MarginfiBankAccounts<'a> {
    pub bank: DeserializedAccount<'a, Bank>,
    pub vault_authority: Option<&'a AccountInfo<'a>>,
    pub token_accounts: LendingProtocolTokenAccounts<'a>,
}

pub struct MarginfiClient<'a> {
    signer: &'a AccountInfo<'a>,
    program: &'a AccountInfo<'a>,
    marginfi_account: DeserializedAccount<'a, MarginfiAccount>,
    marginfi_group: &'a AccountInfo<'a>,
    supply: Option<MarginfiBankAccounts<'a>>,
    debt: Option<MarginfiBankAccounts<'a>>,
}

impl<'a> MarginfiClient<'a> {
    pub fn initialize<'b>(
        ctx: &'b Context<'a, MarginfiOpenPositionAccounts>,
        solauto_position: &DeserializedAccount<PositionAccount>,
    ) -> ProgramResult {
        // validate_position_settings(solauto_position.as_ref().unwrap().data.setting_params, max_ltv, liq_threshold)
        // TODO
        Ok(())
    }

    pub fn from(
        signer: &'a AccountInfo<'a>,
        program: &'a AccountInfo<'a>,
        marginfi_group: &'a AccountInfo<'a>,
        marginfi_account: &'a AccountInfo<'a>,
        supply_bank: Option<&'a AccountInfo<'a>>,
        source_supply_ta: Option<&'a AccountInfo<'a>>,
        vault_supply_ta: Option<&'a AccountInfo<'a>>,
        supply_vault_authority: Option<&'a AccountInfo<'a>>,
        debt_bank: Option<&'a AccountInfo<'a>>,
        source_debt_ta: Option<&'a AccountInfo<'a>>,
        vault_debt_ta: Option<&'a AccountInfo<'a>>,
        debt_vault_authority: Option<&'a AccountInfo<'a>>,
    ) -> Result<(Self, LendingProtocolObligationPosition), ProgramError> {
        let (deserialized_marginfi_account, deserialized_supply_bank, deserialized_debt_bank) =
            MarginfiClient::deserialize_margfinfi_accounts(
                marginfi_account,
                supply_bank,
                debt_bank,
            )?;

        let obligation_position = MarginfiClient::get_obligation_position(
            &deserialized_marginfi_account.data,
            deserialized_supply_bank
                .as_ref()
                .map_or_else(|| None, |bank| Some(&bank.data)),
            deserialized_debt_bank
                .as_ref()
                .map_or_else(|| None, |bank| Some(&bank.data)),
        )?;

        let supply = if deserialized_supply_bank.is_some() {
            Some(MarginfiBankAccounts {
                bank: deserialized_supply_bank.unwrap(),
                vault_authority: supply_vault_authority,
                token_accounts: LendingProtocolTokenAccounts::from(
                    None,
                    source_supply_ta,
                    vault_supply_ta,
                )?
                .unwrap(),
            })
        } else {
            None
        };

        let debt = if deserialized_debt_bank.is_some() {
            Some(MarginfiBankAccounts {
                bank: deserialized_debt_bank.unwrap(),
                vault_authority: debt_vault_authority,
                token_accounts: LendingProtocolTokenAccounts::from(
                    None,
                    source_debt_ta,
                    vault_debt_ta,
                )?
                .unwrap(),
            })
        } else {
            None
        };

        let client = Self {
            signer,
            program,
            marginfi_account: deserialized_marginfi_account,
            marginfi_group,
            supply,
            debt,
        };

        return Ok((client, obligation_position));
    }

    pub fn deserialize_margfinfi_accounts(
        marginfi_account: &'a AccountInfo<'a>,
        supply_bank: Option<&'a AccountInfo<'a>>,
        debt_bank: Option<&'a AccountInfo<'a>>,
    ) -> Result<
        (
            DeserializedAccount<'a, MarginfiAccount>,
            Option<DeserializedAccount<'a, Bank>>,
            Option<DeserializedAccount<'a, Bank>>,
        ),
        ProgramError,
    > {
        Ok((
            DeserializedAccount::<MarginfiAccount>::anchor_deserialize(Some(marginfi_account))?
                .unwrap(),
            DeserializedAccount::<Bank>::anchor_deserialize(supply_bank)?,
            DeserializedAccount::<Bank>::anchor_deserialize(debt_bank)?,
        ))
    }

    pub fn get_max_ltv_and_liq_threshold(&self, supply_bank: &Box<Bank>) -> (f64, f64) {
        // TODO
        (0.0, 0.0)
    }

    pub fn get_obligation_position(
        marginfi_account: &Box<MarginfiAccount>,
        supply_bank: Option<&Box<Bank>>,
        debt_bank: Option<&Box<Bank>>,
    ) -> Result<LendingProtocolObligationPosition, ProgramError> {
        // TODO
        return Err(ProgramError::Custom(0));
    }
}

impl<'a> LendingProtocolClient<'a> for MarginfiClient<'a> {
    fn validate(&self, std_accounts: &SolautoStandardAccounts) -> ProgramResult {
        validate_lending_protocol_accounts(
            std_accounts.signer,
            &std_accounts.solauto_position,
            self.marginfi_account.account_info,
            self.supply
                .as_ref()
                .unwrap()
                .token_accounts
                .source_ta
                .account_info,
            self.debt
                .as_ref()
                .map_or_else(|| None, |debt| Some(debt.token_accounts.protocol_ta)),
        )?;

        if self.supply.is_some()
            && self.debt.is_some()
            && self.supply.as_ref().unwrap().bank.data.config.risk_tier == RiskTier::Isolated
        {
            msg!("Cannot use an isolated asset as collateral");
            return Err(SolautoError::IncorrectAccounts.into());
        }

        Ok(())
    }

    fn deposit<'b>(
        &self,
        base_unit_amount: u64,
        std_accounts: &'b SolautoStandardAccounts<'a>,
    ) -> ProgramResult {
        let supply = self.supply.as_ref().unwrap();
        // TODO

        // let cpi_accounts = marginfi_anchor::ix_accounts::LendingAccountDeposit {
        //     token_program: std_accounts.token_program.clone(),
        //     marginfi_group: self.marginfi_group.clone(),
        //     marginfi_account: self.marginfi_account.account_info.clone(),
        //     signer: std_accounts.solauto_position.account_info.clone(),
        //     bank: supply.bank.account_info.clone(),
        //     signer_token_account: supply.token_accounts.source_ta.account_info.clone(),
        //     bank_liquidity_vault: supply.token_accounts.protocol_ta.clone(),
        // };
        // // let cpi_accounts = marginfi_anchor::accounts::LendingAccountDeposit {
        // //     token_program: *std_accounts.token_program.key,
        // //     marginfi_group: *self.marginfi_group.key,
        // //     marginfi_account: *self.marginfi_account.account_info.key,
        // //     signer: *std_accounts.solauto_position.account_info.key,
        // //     bank: *supply.bank.account_info.key,
        // //     signer_token_account: *supply.token_accounts.source_ta.account_info.key,
        // //     bank_liquidity_vault: *supply.token_accounts.protocol_ta.key,
        // // };
        // // CpiContext::new_with_signer(self.program.clone(), cpi_accounts, &[&[]]);

        // // let cpi_ctx = CpiContext::<LendingAccountDeposit>::new(
        // //     self.program.clone(),

        // // );

        // marginfi_anchor::marginfi::lending_account_deposit(ctx, amount)
        Ok(())
    }

    fn withdraw<'b>(
        &self,
        base_unit_amount: u64,
        destination: &'a AccountInfo<'a>,
        std_accounts: &'b SolautoStandardAccounts<'a>,
    ) -> ProgramResult {
        // TODO
        // add 4 remaining accounts: supply bank, supply pyth price oracle, (if marginfi account has debt position): debt bank, debt pyth price oracle
        Ok(())
    }

    fn borrow<'b>(
        &self,
        base_unit_amount: u64,
        destination: &'a AccountInfo<'a>,
        std_accounts: &'b SolautoStandardAccounts<'a>,
    ) -> ProgramResult {
        // TODO
        // add 4 remaining accounts: supply bank, supply pyth price oracle, (if marginfi account has debt position): debt bank, debt pyth price oracle
        Ok(())
    }

    fn repay<'b>(
        &self,
        base_unit_amount: u64,
        std_accounts: &'b SolautoStandardAccounts<'a>,
    ) -> ProgramResult {
        // TODO
        Ok(())
    }
}
