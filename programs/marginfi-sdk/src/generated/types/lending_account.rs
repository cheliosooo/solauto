//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::Balance;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use bytemuck::Pod;
use bytemuck::Zeroable;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq, Copy, Pod, Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LendingAccount {
    pub balances: [Balance; 16],
    pub padding: [u64; 8],
}
