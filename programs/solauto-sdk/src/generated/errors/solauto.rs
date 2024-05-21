//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum SolautoError {
    /// 0 (0x0) - Missing or incorrect accounts provided for the given instruction
    #[error("Missing or incorrect accounts provided for the given instruction")]
    IncorrectAccounts,
    /// 1 (0x1) - Failed to deserialize account data, incorrect account was likely given
    #[error("Failed to deserialize account data, incorrect account was likely given")]
    FailedAccountDeserialization,
    /// 2 (0x2) - Invalid position settings given
    #[error("Invalid position settings given")]
    InvalidPositionSettings,
    /// 3 (0x3) - Invalid DCA settings given
    #[error("Invalid DCA settings given")]
    InvalidDCASettings,
    /// 4 (0x4) - Invalid automation data given
    #[error("Invalid automation data given")]
    InvalidAutomationData,
    /// 5 (0x5) - Stale protocol data. Refresh instruction must be invoked before taking a protocol action
    #[error(
        "Stale protocol data. Refresh instruction must be invoked before taking a protocol action"
    )]
    StaleProtocolData,
    /// 6 (0x6) - Unable to adjust position to the desired utilization rate
    #[error("Unable to adjust position to the desired utilization rate")]
    UnableToReposition,
    /// 7 (0x7) - Desired action brought the utilization rate to an unsafe amount
    #[error("Desired action brought the utilization rate to an unsafe amount")]
    ExceededValidUtilizationRate,
    /// 8 (0x8) - Invalid position condition to rebalance
    #[error("Invalid position condition to rebalance")]
    InvalidRebalanceCondition,
    /// 9 (0x9) - Unable to invoke instruciton through a CPI
    #[error("Unable to invoke instruciton through a CPI")]
    InstructionIsCPI,
    /// 10 (0xA) - Too many rebalance instruction invocations in the same transaction
    #[error("Too many rebalance instruction invocations in the same transaction")]
    RebalanceAbuse,
    /// 11 (0xB) - Incorrect set of instructions in the transaction
    #[error("Incorrect set of instructions in the transaction")]
    IncorrectInstructions,
}

impl solana_program::program_error::PrintProgramError for SolautoError {
    fn print<E>(&self) {
        solana_program::msg!(&self.to_string());
    }
}
