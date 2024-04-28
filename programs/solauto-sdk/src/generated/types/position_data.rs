//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::DCASettings;
use crate::generated::types::LendingPlatform;
use crate::generated::types::LendingProtocolPositionData;
use crate::generated::types::PositionState;
use crate::generated::types::SolautoSettingsParameters;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PositionData {
    pub state: PositionState,
    pub lending_platform: LendingPlatform,
    pub protocol_data: LendingProtocolPositionData,
    pub setting_params: SolautoSettingsParameters,
    pub active_dca: Option<DCASettings>,
    pub supply_balance: u64,
    pub debt_balance: u64,
}
