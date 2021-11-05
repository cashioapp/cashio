//! Crate events
#![deny(missing_docs)]

use anchor_lang::prelude::*;

/// Emitted when a [crate::Bank] is created.
#[event]
pub struct NewBankEvent {
    /// The [crate::Bank].
    #[index]
    pub bank: Pubkey,
    /// Curator.
    pub curator: Pubkey,
    /// Timestamp of the event.
    pub timestamp: i64,
}

/// Emitted when a [crate::Collateral] is created.
#[event]
pub struct AddCollateralEvent {
    /// crate::Bank
    #[index]
    pub bank: Pubkey,
    /// Stake pool
    #[index]
    pub collateral: Pubkey,

    /// The [crate::Bank::curator].
    pub curator: Pubkey,
    /// The [anchor_spl::token::Mint] of the collateral.
    pub mint: Pubkey,

    /// Timestamp of the event.
    pub timestamp: i64,
}

/// Emitted when an [crate::Bank]'s curator is modified.
#[event]
pub struct SetCuratorEvent {
    /// crate::Bank
    #[index]
    pub bank: Pubkey,

    /// The new [crate::Bank::curator].
    pub curator: Pubkey,
    /// The previous [crate::Bank::curator].
    pub previous_curator: Pubkey,
    /// The [crate::Bank::bankman].
    pub bankman: Pubkey,

    /// Timestamp of the event.
    pub timestamp: i64,
}

/// Emitted when an [crate::Bank]'s bankman is modified.
#[event]
pub struct SetBankmanEvent {
    /// crate::Bank
    #[index]
    pub bank: Pubkey,

    /// The new [crate::Bank::bankman].
    pub bankman: Pubkey,
    /// The previous [crate::Bank::bankman].
    pub previous_bankman: Pubkey,

    /// Timestamp of the event.
    pub timestamp: i64,
}

/// Emitted when a [crate::Collateral]'s hard cap is modified.
#[event]
pub struct SetCollateralHardCapEvent {
    /// crate::Bank
    #[index]
    pub bank: Pubkey,
    /// Stake pool
    #[index]
    pub collateral: Pubkey,

    /// Hard cap
    pub hard_cap: u64,
    /// Timestamp of the event.
    pub timestamp: i64,
}
