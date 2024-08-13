//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::WrappedI80F48;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use bytemuck::Pod;
use bytemuck::Zeroable;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq, Copy, Pod, Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterestRateConfig {
    pub optimal_utilization_rate: WrappedI80F48,
    pub plateau_interest_rate: WrappedI80F48,
    pub max_interest_rate: WrappedI80F48,
    pub insurance_fee_fixed_apr: WrappedI80F48,
    pub insurance_ir_fee: WrappedI80F48,
    pub protocol_fixed_fee_apr: WrappedI80F48,
    pub protocol_ir_fee: WrappedI80F48,
    pub padding: [[u64; 2]; 8],
}
