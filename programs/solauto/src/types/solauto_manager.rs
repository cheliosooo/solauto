use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
};
use std::{ cmp::min, ops::{ Div, Mul } };

use super::{
    instruction::{ RebalanceArgs, SolautoAction, SolautoStandardAccounts, WithdrawParams },
    lending_protocol::{ LendingProtocolClient, LendingProtocolTokenAccounts },
    obligation_position::LendingProtocolObligationPosition,
    shared::{ DeserializedAccount, PositionAccount, SolautoError, SolautoRebalanceStep },
};
use crate::utils::*;

pub struct SolautoManagerAccounts<'a> {
    pub supply: Option<LendingProtocolTokenAccounts<'a>>,
    pub debt: Option<LendingProtocolTokenAccounts<'a>>,
    pub intermediary_ta: Option<&'a AccountInfo<'a>>,
}
impl<'a> SolautoManagerAccounts<'a> {
    pub fn from(
        supply_mint: Option<&'a AccountInfo<'a>>,
        position_supply_ta: Option<&'a AccountInfo<'a>>,
        bank_supply_ta: Option<&'a AccountInfo<'a>>,
        debt_mint: Option<&'a AccountInfo<'a>>,
        position_debt_ta: Option<&'a AccountInfo<'a>>,
        bank_debt_ta: Option<&'a AccountInfo<'a>>,
        intermediary_ta: Option<&'a AccountInfo<'a>>
    ) -> Result<Self, ProgramError> {
        let supply = LendingProtocolTokenAccounts::from(
            supply_mint,
            position_supply_ta,
            bank_supply_ta
        )?;
        let debt = LendingProtocolTokenAccounts::from(debt_mint, position_debt_ta, bank_debt_ta)?;
        Ok(Self {
            supply,
            debt,
            intermediary_ta,
        })
    }
}

pub struct SolautoManager<'a, 'b> {
    pub client: &'b dyn LendingProtocolClient<'a>,
    pub obligation_position: &'b mut LendingProtocolObligationPosition,
    pub accounts: SolautoManagerAccounts<'a>,
    pub std_accounts: SolautoStandardAccounts<'a>,
    pub solauto_fees_bps: solauto_utils::SolautoFeesBps,
}

impl<'a, 'b> SolautoManager<'a, 'b> {
    pub fn from(
        client: &'b dyn LendingProtocolClient<'a>,
        obligation_position: &'b mut LendingProtocolObligationPosition,
        accounts: SolautoManagerAccounts<'a>,
        std_accounts: SolautoStandardAccounts<'a>
    ) -> Result<Self, ProgramError> {
        client.validate(&std_accounts)?;
        let solauto_fees_bps = solauto_utils::SolautoFeesBps::from(
            !&std_accounts.referred_by_supply_ta.is_none()
        );
        Ok(Self {
            client,
            obligation_position,
            accounts,
            std_accounts,
            solauto_fees_bps,
        })
    }

    pub fn protocol_interaction(&mut self, action: SolautoAction) -> ProgramResult {
        match action {
            SolautoAction::Deposit(base_unit_amount) => {
                self.deposit(base_unit_amount)?;
            }
            SolautoAction::Borrow(base_unit_amount) => {
                self.borrow(
                    base_unit_amount,
                    self.accounts.debt.as_ref().unwrap().source_ta.account_info
                )?;
            }
            SolautoAction::Repay(base_unit_amount) => {
                self.repay(base_unit_amount)?;
            }
            SolautoAction::Withdraw(params) =>
                match params {
                    WithdrawParams::All => {
                        self.withdraw(
                            self.obligation_position.net_worth_base_amount(),
                            self.accounts.supply.as_ref().unwrap().source_ta.account_info
                        )?;
                    }
                    WithdrawParams::Partial(base_unit_amount) =>
                        self.withdraw(
                            base_unit_amount,
                            self.accounts.supply.as_ref().unwrap().source_ta.account_info
                        )?,
                }
        }

        if !self.std_accounts.solauto_position.data.self_managed {
            let repay_from_bps = self.std_accounts.solauto_position.data.position
                .as_ref()
                .unwrap().setting_params.repay_from_bps;
            if
                self.obligation_position.current_liq_utilization_rate_bps(None, None) >
                repay_from_bps
            {
                return Err(SolautoError::ExceededValidUtilizationRate.into());
            }
        } else if self.obligation_position.current_liq_utilization_rate_bps(None, None) > 9500 {
            return Err(SolautoError::ExceededValidUtilizationRate.into());
        }

        Ok(())
    }

    fn deposit(&mut self, base_unit_amount: u64) -> ProgramResult {
        self.client.deposit(base_unit_amount, &self.std_accounts)?;
        self.obligation_position.supply_lent_update(base_unit_amount as i64)
    }

    fn borrow(&mut self, base_unit_amount: u64, destination: &'a AccountInfo<'a>) -> ProgramResult {
        self.client.borrow(base_unit_amount, destination, &self.std_accounts)?;
        self.obligation_position.debt_borrowed_update(base_unit_amount as i64)
    }

    fn withdraw(
        &mut self,
        base_unit_amount: u64,
        destination: &'a AccountInfo<'a>
    ) -> ProgramResult {
        self.client.withdraw(base_unit_amount, destination, &self.std_accounts)?;
        self.obligation_position.supply_lent_update((base_unit_amount as i64) * -1)
    }

    fn repay(&mut self, base_unit_amount: u64) -> ProgramResult {
        self.client.repay(base_unit_amount, &self.std_accounts)?;
        self.obligation_position.debt_borrowed_update((base_unit_amount as i64) * -1)
    }

    pub fn rebalance(
        &mut self,
        rebalance_args: RebalanceArgs,
        rebalance_step: SolautoRebalanceStep
    ) -> ProgramResult {
        if
            rebalance_step == SolautoRebalanceStep::StartSolautoRebalanceSandwich ||
            rebalance_step == SolautoRebalanceStep::StartMarginfiFlashLoanSandwich
        {
            self.begin_rebalance(&rebalance_args, &rebalance_step)
        } else if
            rebalance_step == SolautoRebalanceStep::FinishSolautoRebalanceSandwich ||
            rebalance_step == SolautoRebalanceStep::FinishMarginfiFlashLoanSandwich
        {
            self.finish_rebalance()
        } else {
            // TODO
            msg!("Rebalance currently unsupported for this");
            return Err(SolautoError::InvalidRebalanceCondition.into());
        }
    }

    fn get_debt_adjustment_usd(
        &self,
        rebalance_args: &RebalanceArgs,
        rebalance_step: &SolautoRebalanceStep
    ) -> Result<f64, ProgramError> {
        // TODO CHECK IF DCA
        // TODO WHAT DO WE SET AS TARGET BPS IF DCA?
        // TODO WE NEED TO HANDLE A DEPOSIT FIRST IF DCAing IN (in solauto manager)
        // WE NEED TO DELETE DCA DATA IN SOLAUTO POSITION IF DCAing HAS FINISHED

        let first_or_only_rebalance_ix =
            rebalance_step == &SolautoRebalanceStep::StartSolautoRebalanceSandwich ||
            rebalance_step == &SolautoRebalanceStep::StartMarginfiFlashLoanSandwich ||
            rebalance_step == &SolautoRebalanceStep::FinishStandardFlashLoanSandwich;

        let current_liq_utilization_rate_bps = if first_or_only_rebalance_ix {
            self.obligation_position.current_liq_utilization_rate_bps(None, None)
        } else {
            let position_supply_ta = &self.accounts.supply.as_ref().unwrap().source_ta.data;
            let position_debt_ta = &self.accounts.debt.as_ref().unwrap().source_ta.data;

            if position_supply_ta.amount > 0 {
                self.obligation_position.current_liq_utilization_rate_bps(
                    None,
                    Some((position_supply_ta.amount as i64).mul(-1))
                )
            } else {
                self.obligation_position.current_liq_utilization_rate_bps(
                    Some(position_debt_ta.amount as i64),
                    None
                )
            }
        };

        let target_liq_utilization_rate_bps = solauto_utils::get_target_liq_utilization_rate(
            &self.std_accounts,
            current_liq_utilization_rate_bps,
            self.obligation_position.max_ltv,
            self.obligation_position.liq_threshold,
            &rebalance_args
        )?;

        let max_price_slippage_bps = if !rebalance_args.max_price_slippage_bps.is_none() {
            rebalance_args.max_price_slippage_bps.unwrap()
        } else {
            300
        };

        let increasing_leverage =
            current_liq_utilization_rate_bps < target_liq_utilization_rate_bps;

        let adjustment_fee_bps = if increasing_leverage {
            Some(self.solauto_fees_bps.total)
        } else {
            None
        };

        let mut debt_adjustment_usd = math_utils::calculate_debt_adjustment_usd(
            self.obligation_position.liq_threshold,
            self.obligation_position.supply.as_ref().unwrap().amount_used.usd_value as f64,
            self.obligation_position.debt.as_ref().unwrap().amount_used.usd_value as f64,
            target_liq_utilization_rate_bps,
            adjustment_fee_bps
        );
        debt_adjustment_usd += debt_adjustment_usd.mul(
            (max_price_slippage_bps as f64).div(10000.0)
        );

        Ok(debt_adjustment_usd)
    }

    fn begin_rebalance(
        &mut self,
        rebalance_args: &RebalanceArgs,
        rebalance_step: &SolautoRebalanceStep
    ) -> ProgramResult {
        let debt_adjustment_usd = self.get_debt_adjustment_usd(rebalance_args, rebalance_step)?;
        let increasing_leverage = debt_adjustment_usd > 0.0;

        let (token_mint, market_price, decimals) = if increasing_leverage {
            (
                self.accounts.debt.as_ref().unwrap().mint,
                self.obligation_position.debt.as_ref().unwrap().market_price,
                self.obligation_position.debt.as_ref().unwrap().decimals,
            )
        } else {
            (
                self.accounts.supply.as_ref().unwrap().mint,
                self.obligation_position.supply.as_ref().unwrap().market_price,
                self.obligation_position.supply.as_ref().unwrap().decimals,
            )
        };
        solana_utils::init_ata_if_needed(
            self.std_accounts.token_program,
            self.std_accounts.system_program,
            self.std_accounts.rent,
            self.std_accounts.signer,
            self.std_accounts.signer,
            self.accounts.intermediary_ta.unwrap(),
            token_mint
        )?;

        let base_unit_amount = math_utils::to_base_unit::<f64, u8, u64>(
            debt_adjustment_usd.div(market_price),
            decimals
        );

        if increasing_leverage {
            self.borrow(
                min(
                    base_unit_amount,
                    ((
                        self.obligation_position.debt
                            .as_ref()
                            .unwrap().amount_can_be_used.base_unit as f64
                    ) * 0.9) as u64
                ),
                self.accounts.intermediary_ta.unwrap()
            )
        } else {
            self.withdraw(
                min(
                    base_unit_amount,
                    ((
                        self.obligation_position.supply
                            .as_ref()
                            .unwrap().amount_can_be_used.base_unit as f64
                    ) * 0.9) as u64
                ),
                self.accounts.intermediary_ta.unwrap()
            )
        }
    }

    fn finish_rebalance(&mut self) -> ProgramResult {
        let position_supply_ta = &self.accounts.supply.as_ref().unwrap().source_ta.data;
        let position_debt_ta = &self.accounts.debt.as_ref().unwrap().source_ta.data;

        if position_supply_ta.amount > 0 {
            let amount_after_fees = self.payout_fees()?;
            self.deposit(amount_after_fees)?;
        } else if position_debt_ta.amount > 0 {
            self.repay(position_debt_ta.amount)?;
        } else {
            msg!("Missing required position liquidity to rebalance position");
            return Err(SolautoError::UnableToReposition.into());
        }

        Ok(())
    }

    fn payout_fees(&self) -> Result<u64, ProgramError> {
        if
            self.std_accounts.authority_referral_state.is_none() ||
            self.std_accounts.referred_by_supply_ta.is_none()
        {
            msg!(
                "Missing referral account(s) when we are boosting leverage. Referral accounts are required"
            );
            return Err(ProgramError::InvalidAccountData.into());
        }

        let position_supply_ta = &self.accounts.supply.as_ref().unwrap().source_ta;
        let balance = position_supply_ta.data.amount;

        let solauto_fees = (balance as f64).mul(
            (self.solauto_fees_bps.solauto as f64).div(10000.0)
        ) as u64;

        solana_utils::spl_token_transfer(
            self.std_accounts.token_program,
            position_supply_ta.account_info,
            self.std_accounts.solauto_position.account_info,
            self.std_accounts.solauto_fees_supply_ta.unwrap(),
            solauto_fees,
            Some(
                vec![
                    &[self.std_accounts.solauto_position.data.position_id],
                    self.std_accounts.solauto_position.data.authority.as_ref()
                ]
            )
        )?;

        let referrer_fees = (balance as f64).mul(
            (self.solauto_fees_bps.referrer as f64).div(10000.0)
        ) as u64;

        if referrer_fees > 0 {
            solana_utils::spl_token_transfer(
                self.std_accounts.token_program,
                position_supply_ta.account_info,
                self.std_accounts.solauto_position.account_info,
                self.std_accounts.referred_by_supply_ta.unwrap(),
                referrer_fees,
                Some(
                    vec![
                        &[self.std_accounts.solauto_position.data.position_id],
                        self.std_accounts.solauto_position.data.authority.as_ref()
                    ]
                )
            )?;
        }

        Ok(balance - solauto_fees)
    }

    pub fn refresh_position(
        obligation_position: &LendingProtocolObligationPosition,
        solauto_position: &mut DeserializedAccount<PositionAccount>
    ) {
        if solauto_position.data.self_managed {
            return;
        }

        let position = solauto_position.data.position.as_mut().unwrap();

        position.state.net_worth_usd_base_amount = obligation_position.net_worth_usd_base_amount();
        position.state.base_amount_liquidity_net_worth =
            obligation_position.net_worth_base_amount();
        position.state.liq_utilization_rate_bps =
            obligation_position.current_liq_utilization_rate_bps(None, None);
        position.state.base_amount_supplied = if !obligation_position.supply.is_none() {
            obligation_position.supply.as_ref().unwrap().amount_used.base_unit
        } else {
            0
        };
        position.state.base_amount_supplied = if !obligation_position.debt.is_none() {
            obligation_position.debt.as_ref().unwrap().amount_used.base_unit
        } else {
            0
        };

        position.state.max_ltv_bps = obligation_position.max_ltv.mul(10000.0) as u64;
        position.state.liq_threshold = obligation_position.liq_threshold.mul(10000.0) as u64;
    }
}
