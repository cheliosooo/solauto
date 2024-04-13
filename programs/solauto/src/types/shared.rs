use thiserror::Error;
use borsh;
use borsh::{ BorshDeserialize, BorshSerialize };
use shank::{ ShankAccount, ShankType };
use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    program_pack::{ IsInitialized, Pack },
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankType, PartialEq)]
#[borsh(use_discriminant = true)]
pub enum LendingPlatform {
    Marginfi = 0,
    Solend = 1,
    Kamino = 2,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankType)]
pub struct SolautoSettingsParameters {
    /// At which liquidation utilization rate or higher to begin a rebalance
    pub repay_from_bps: u16,
    /// At which liquidation utilization rate to finish a rebalance
    pub repay_to_bps: u16,
    /// At which liquidation utilization rate or lower to begin boosting leverage
    pub boost_from_bps: u16,
    /// At which liquidation utilization rate to boost leverage to
    pub boost_to_bps: u16,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankType)]
pub struct SolendPositionData {
    pub obligation: Pubkey,
    pub supply_reserve: Pubkey,
    pub debt_reserve: Option<Pubkey>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankType)]
pub struct MarginfiPositionData {
    pub marginfi_account: Pubkey,
    pub supply_bank: Pubkey,
    pub debt_bank: Option<Pubkey>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankType)]
pub struct KaminoPositionData {
    // TODO
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankType, Default)]
pub struct GeneralPositionData {
    pub liq_utilization_rate_bps: u16,
    pub net_worth_usd_base_amount: u64,
    pub base_amount_liquidity_net_worth: u64,
    pub base_amount_supplied: u64,
    pub base_amount_borrowed: u64,
}

pub const POSITION_ACCOUNT_SPACE: usize = 500;
#[derive(ShankAccount, BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Position {
    pub position_id: u8,
    pub authority: Pubkey,
    pub setting_params: SolautoSettingsParameters,
    pub general_data: GeneralPositionData,
    pub lending_platform: LendingPlatform,
    pub marginfi_data: Option<MarginfiPositionData>,
    pub solend_data: Option<SolendPositionData>,
    pub kamino_data: Option<KaminoPositionData>,
}

pub const REFERRAL_ACCOUNT_SPACE: usize = 300;
#[derive(ShankAccount, BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct RefferalState {
    pub authority: Pubkey,
    pub referred_by_ta: Option<Pubkey>,
    pub fees_mint: Pubkey,
    pub fees_ta: Pubkey,
}

pub const SOLAUTO_SETTINGS_ACCOUNT_SPACE: usize = 100;
#[derive(ShankAccount, BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SolautoAdminSettings {
    pub fees_wallet: Pubkey,
    pub fees_token_mint: Pubkey,
}

#[derive(PartialEq)]
pub enum SolautoRebalanceStep {
    BeginSolautoRebalanceSandwich,
    FinishSolautoRebalanceSandwich,
    /// Includes the base unit amount which was borrowed
    FinishFlashLoanSandwich(u64),
}

#[derive(Clone)]
pub struct DeserializedAccount<'a, T> {
    pub account_info: &'a AccountInfo<'a>,
    pub data: Box<T>,
}

impl<'a, T: BorshDeserialize> DeserializedAccount<'a, T> {
    pub fn deserialize(account: Option<&'a AccountInfo<'a>>) -> Result<Option<Self>, ProgramError> {
        match account {
            Some(account_info) => {
                let deserialized_data = T::try_from_slice(&account_info.data.borrow()).map_err(
                    |_| SolautoError::FailedAccountDeserialization
                )?;
                Ok(
                    Some(Self {
                        account_info,
                        data: Box::new(deserialized_data),
                    })
                )
            }
            None => Ok(None),
        }
    }
}

impl<'a, T: Pack + IsInitialized> DeserializedAccount<'a, T> {
    pub fn unpack(account: Option<&'a AccountInfo<'a>>) -> Result<Option<Self>, ProgramError> {
        match account {
            Some(account_info) => {
                let deserialized_data = T::unpack(&account_info.data.borrow()).map_err(
                    |_| SolautoError::FailedAccountDeserialization
                )?;
                Ok(
                    Some(Self {
                        account_info,
                        data: Box::new(deserialized_data),
                    })
                )
            }
            None => Ok(None),
        }
    }
}

#[derive(Error, Debug)]
pub enum SolautoError {
    #[error("Incorrect Solauto admin settings account")]
    IncorrectSolautoSettingsAccount,
    #[error("Incorrect fee receiver account provided")]
    IncorrectFeesReceiverAccount,
    #[error("Missing required accounts for the given instruction")]
    MissingRequiredAccounts,
    #[error("Failed to deserialize account data, incorrect account was likely given")]
    FailedAccountDeserialization,
    #[error("Invalid position data given")]
    InvalidPositionSettings,
    #[error(
        "Stale protocol data. Refresh instruction must be invoked before taking a protocol action"
    )]
    StaleProtocolData,
    #[error("Unable to adjust position to the desired utilization rate")]
    UnableToReposition,
    #[error("Desired action brought the utilization rate to an unsafe amount")]
    ExceededValidUtilizationRate,
    #[error("Invalid position condition to rebalance")]
    InvalidRebalanceCondition,
    #[error("Unable to invoke instruciton through a CPI")]
    InstructionIsCPI,
    #[error("Too many rebalance instruction invocations in the same transaction")]
    RebalanceAbuse,
    #[error("Incorrect set of rebalance instructions in the transaction")]
    IncorrectRebalanceInstructions,
}

impl From<SolautoError> for ProgramError {
    fn from(e: SolautoError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
