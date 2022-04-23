use anchor_lang::{prelude::*, solana_program::pubkey::PUBKEY_BYTES};

/// 🏦
///
/// Lets users print $CASH or redeem $CASH for its underlying.
#[account]
#[derive(Copy, Debug, Default, PartialEq, Eq)]
pub struct Bank {
    /// The [crate_token::CrateToken].
    pub crate_token: Pubkey,
    /// Bump.
    pub bump: u8,

    /// Mint of the [crate_token::CrateToken].
    pub crate_mint: Pubkey,
    /// Account that can choose what collateral is allowed.
    pub curator: Pubkey,
    /// Account that can change who the curator is.
    pub bankman: Pubkey,
}

impl Bank {
    pub const BYTES: usize = PUBKEY_BYTES + 1 + PUBKEY_BYTES * 3;
}

/// The collateral which has been authorized to mint $CASH.
#[account]
#[derive(Copy, Debug, Default, PartialEq, Eq)]
pub struct Collateral {
    /// The [Bank].
    pub bank: Pubkey,
    /// Mint of the collateral.
    pub mint: Pubkey,
    /// The bump.
    pub bump: u8,
    /// Hard cap on the number of collateral tokens that can be issued from this pool.
    pub hard_cap: u64,
}

impl Collateral {
    pub const BYTES: usize = PUBKEY_BYTES * 2 + 1 + 8;
}
