//! Crate events
#![deny(missing_docs)]

use anchor_lang::prelude::*;

/// Emitted when $CASH is printed.
#[event]
pub struct PrintCashEvent {
    /// The user which deposited collateral.
    #[index]
    pub depositor: Pubkey,
    /// The mint of the collateral used to print.
    #[index]
    pub collateral_mint: Pubkey,

    /// Amount of $CASH printed.
    pub print_amount: u64,
    /// Amount of collateral tokens deposited.
    pub deposit_amount: u64,
    /// Timestamp of the event.
    pub timestamp: i64,
}

/// Emitted when $CASH is burned.
#[event]
pub struct BurnCashEvent {
    /// Burner
    #[index]
    pub burner: Pubkey,
    /// The mint of the collateral withdrawn.
    #[index]
    pub collateral_mint: Pubkey,

    /// Amount of $CASH burned.
    pub burn_amount: u64,
    /// Amount of collateral tokens withdrawn.
    pub withdraw_amount: u64,
    /// Timestamp of the event.
    pub timestamp: i64,
}
