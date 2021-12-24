//! Saber swap math helpers for $CASH
#![deny(missing_docs)]
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
#![deny(clippy::integer_arithmetic)]

use std::cmp::Ordering;

/// Number of decimals of $CASH.
pub const CASH_DECIMALS: u8 = 6;

pub use stable_swap_math::price::SaberSwap;

/// A Saber swap and number of decimals.
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct CashSwap {
    /// Decimals of the LP mint.
    /// This is used for $CASH conversion.
    pub lp_mint_decimals: u8,
    /// Saber.
    pub saber: SaberSwap,
}

impl CashSwap {
    /// Calculates the virtual price of the given amount of pool tokens.
    pub fn calculate_cash_for_pool_tokens(&self, pool_token_amount: u64) -> Option<u64> {
        self.scale_lp_to_cash_decimals(
            self.saber
                .calculate_virtual_price_of_pool_tokens(pool_token_amount)?,
        )
    }

    /// Calculates the virtual price of the given amount of pool tokens.
    pub fn calculate_pool_tokens_for_cash(&self, cash_amount: u64) -> Option<u64> {
        self.saber
            .calculate_pool_tokens_from_virtual_amount(self.scale_cash_to_lp_decimals(cash_amount)?)
    }

    fn scale_lp_to_cash_decimals(&self, amount: u64) -> Option<u64> {
        match CASH_DECIMALS.cmp(&self.lp_mint_decimals) {
            Ordering::Equal => amount.into(),
            Ordering::Less => amount.checked_mul(
                10u64.checked_pow(self.lp_mint_decimals.checked_sub(CASH_DECIMALS)?.into())?,
            ),
            Ordering::Greater => amount.checked_div(
                10u64.checked_pow(CASH_DECIMALS.checked_sub(self.lp_mint_decimals)?.into())?,
            ),
        }
    }

    fn scale_cash_to_lp_decimals(&self, amount: u64) -> Option<u64> {
        match CASH_DECIMALS.cmp(&self.lp_mint_decimals) {
            Ordering::Equal => amount.into(),
            Ordering::Less => amount.checked_div(
                10u64.checked_pow(self.lp_mint_decimals.checked_sub(CASH_DECIMALS)?.into())?,
            ),
            Ordering::Greater => amount.checked_mul(
                10u64.checked_pow(CASH_DECIMALS.checked_sub(self.lp_mint_decimals)?.into())?,
            ),
        }
    }
}
