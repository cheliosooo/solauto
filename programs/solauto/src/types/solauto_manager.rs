use std::{ ops::{ Div, Mul }, cmp::min };
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
};
use spl_token::state::Account as TokenAccount;

use crate::utils::{
    math_utils::{ calculate_debt_adjustment_usd, to_base_unit },
    solana_utils::{ init_ata_if_needed, spl_token_transfer },
    solauto_utils::SolautoFeesBps,
};
use super::{
    instruction::{ SolautoAction, SolautoStandardAccounts, WithdrawParams },
    lending_protocol::{ LendingProtocolClient, LendingProtocolTokenAccounts },
    obligation_position::LendingProtocolObligationPosition,
    shared::{ DeserializedAccount, Position, SolautoError, SolautoRebalanceStep },
};

pub struct SolautoManagerAccounts<'a> {
    pub supply: Option<LendingProtocolTokenAccounts<'a>>,
    pub debt: Option<LendingProtocolTokenAccounts<'a>>,
    pub intermediary_ta: Option<&'a AccountInfo<'a>>,
}
impl<'a> SolautoManagerAccounts<'a> {
    pub fn from(
        supply_mint: Option<&'a AccountInfo<'a>>,
        source_supply_ta: Option<&'a AccountInfo<'a>>,
        bank_supply_ta: Option<&'a AccountInfo<'a>>,
        debt_mint: Option<&'a AccountInfo<'a>>,
        source_debt_ta: Option<&'a AccountInfo<'a>>,
        bank_debt_ta: Option<&'a AccountInfo<'a>>,
        intermediary_ta: Option<&'a AccountInfo<'a>>
    ) -> Self {
        let supply = LendingProtocolTokenAccounts::from(
            supply_mint,
            source_supply_ta,
            bank_supply_ta
        );
        let debt = LendingProtocolTokenAccounts::from(debt_mint, source_debt_ta, bank_debt_ta);
        Self {
            supply,
            debt,
            intermediary_ta,
        }
    }
}

pub struct SolautoManager<'a, 'b> {
    pub client: &'b dyn LendingProtocolClient<'a>,
    pub obligation_position: &'b mut LendingProtocolObligationPosition,
    pub accounts: SolautoManagerAccounts<'a>,
    pub std_accounts: SolautoStandardAccounts<'a>,
    pub solauto_fees_bps: SolautoFeesBps,
}

impl<'a, 'b> SolautoManager<'a, 'b> {
    pub fn from(
        client: &'b dyn LendingProtocolClient<'a>,
        obligation_position: &'b mut LendingProtocolObligationPosition,
        accounts: SolautoManagerAccounts<'a>,
        std_accounts: SolautoStandardAccounts<'a>
    ) -> Result<Self, ProgramError> {
        client.validate(&std_accounts)?;
        let solauto_fees_bps = SolautoFeesBps::from(!&std_accounts.referred_by_ta.is_none());
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
                self.borrow(base_unit_amount, self.accounts.debt.as_ref().unwrap().source_ta)?;
            }
            SolautoAction::Repay(base_unit_amount) => {
                self.repay(base_unit_amount)?;
            }
            SolautoAction::Withdraw(params) =>
                match params {
                    WithdrawParams::All => {
                        self.withdraw(
                            self.obligation_position.net_worth_base_amount(),
                            self.accounts.supply.as_ref().unwrap().source_ta
                        )?;
                    }
                    WithdrawParams::Partial(base_unit_amount) =>
                        self.withdraw(
                            base_unit_amount,
                            self.accounts.supply.as_ref().unwrap().source_ta
                        )?,
                }
        }

        if !self.std_accounts.solauto_position.data.self_managed {
            let repay_from_bps = self.std_accounts.solauto_position.data.position
                .as_ref()
                .unwrap().setting_params.repay_from_bps;
            if self.obligation_position.current_liq_utilization_rate_bps() > repay_from_bps {
                return Err(SolautoError::ExceededValidUtilizationRate.into());
            }
        } else if self.obligation_position.current_liq_utilization_rate_bps() > 9500 {
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
        target_liq_utilization_rate_bps: u16,
        max_price_slippage_bps: u16,
        rebalance_step: SolautoRebalanceStep
    ) -> ProgramResult {
        if
            rebalance_step == SolautoRebalanceStep::StartSolautoRebalanceSandwich ||
            rebalance_step == SolautoRebalanceStep::StartMarginfiFlashLoanSandwich
        {
            self.begin_rebalance(target_liq_utilization_rate_bps, max_price_slippage_bps)
        } else if
            rebalance_step == SolautoRebalanceStep::FinishSolautoRebalanceSandwich ||
            rebalance_step == SolautoRebalanceStep::FinishMarginfiFlashLoanSandwich
        {
            self.finish_rebalance()?;
            Ok(())
        } else {
            // TODO
            msg!("Rebalance currently unsupported for this");
            return Err(SolautoError::InvalidRebalanceCondition.into());
        }
    }

    fn begin_rebalance(
        &mut self,
        target_liq_utilization_rate_bps: u16,
        max_price_slippage_bps: u16
    ) -> ProgramResult {
        let increasing_leverage =
            self.obligation_position.current_liq_utilization_rate_bps() <
            target_liq_utilization_rate_bps;

        let mut debt_adjustment_usd = calculate_debt_adjustment_usd(
            self.obligation_position.liq_threshold,
            self.obligation_position.supply.as_ref().unwrap().amount_used.usd_value as f64,
            self.obligation_position.debt.as_ref().unwrap().amount_used.usd_value as f64,
            target_liq_utilization_rate_bps,
            Some(self.solauto_fees_bps.total)
        );
        debt_adjustment_usd += debt_adjustment_usd.mul(
            (max_price_slippage_bps as f64).div(10000.0)
        );

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
        init_ata_if_needed(
            self.std_accounts.token_program,
            self.std_accounts.system_program,
            self.std_accounts.rent,
            self.std_accounts.signer,
            self.std_accounts.signer,
            self.accounts.intermediary_ta.unwrap(),
            token_mint
        )?;

        let base_unit_amount = to_base_unit::<f64, u8, u64>(
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
        let supply_source_ta = TokenAccount::unpack(
            &self.accounts.supply.as_ref().unwrap().source_ta.data.borrow()
        )?;
        let debt_source_ta = TokenAccount::unpack(
            &self.accounts.debt.as_ref().unwrap().source_ta.data.borrow()
        )?;

        let supply_balance = supply_source_ta.amount;
        let debt_balance = debt_source_ta.amount;

        if supply_balance > 0 {
            self.payout_fees(self.accounts.supply.as_ref().unwrap().source_ta, &supply_source_ta)?;

            let new_supply_balance = TokenAccount::unpack(
                &self.accounts.supply.as_ref().unwrap().source_ta.data.borrow()
            )?.amount;
            self.deposit(new_supply_balance)?;
        } else if debt_balance > 0 {
            self.repay(debt_balance)?;
        } else {
            msg!("Missing required source liquidity to rebalance position");
            return Err(SolautoError::UnableToReposition.into());
        }

        Ok(())
    }

    fn payout_fees(
        &self,
        account_info: &'a AccountInfo<'a>,
        token_account: &'b TokenAccount
    ) -> ProgramResult {
        let balance = token_account.amount;

        let solauto_fees = (balance as f64).mul(
            (self.solauto_fees_bps.solauto as f64).div(10000.0)
        ) as u64;

        spl_token_transfer(
            self.std_accounts.token_program,
            account_info,
            self.std_accounts.solauto_position.account_info,
            self.std_accounts.solauto_fees_receiver_ta.unwrap(),
            solauto_fees,
            Some(vec![
                &[self.std_accounts.solauto_position.data.position_id],
                self.std_accounts.solauto_position.data.authority.as_ref()
            ])
        )?;

        let referrer_fees = (balance as f64).mul(
            (self.solauto_fees_bps.referrer as f64).div(10000.0)
        ) as u64;
        // TODO referrer_ta is not correct, we need to have signing authority outside this program to sign a jup swap\
        // OR, we use this for now, but we add a new instruction to convert to dest token where the transaction creates new ta with random wallet
        // and solauto transfers to that ta, then a jup swap occurs, and the dest ta is the referrer dest ta

        Ok(())
    }

    pub fn refresh_position(
        obligation_position: &LendingProtocolObligationPosition,
        solauto_position: &mut DeserializedAccount<Position>
    ) {
        if solauto_position.data.self_managed {
            return;
        }

        let position = solauto_position.data.position.as_mut().unwrap();

        position.state.net_worth_usd_base_amount = obligation_position.net_worth_usd_base_amount();
        position.state.base_amount_liquidity_net_worth =
            obligation_position.net_worth_base_amount();
        position.state.liq_utilization_rate_bps =
            obligation_position.current_liq_utilization_rate_bps();
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
    }
}
