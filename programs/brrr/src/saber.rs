//! Saber adapters

use std::convert::TryFrom;

use anchor_lang::prelude::*;
use converter::{CashSwap, SaberSwap};
use vipers::{assert_keys_eq, validate::Validate};

use crate::SaberSwapAccounts;

impl<'info> TryFrom<&SaberSwapAccounts<'info>> for CashSwap {
    type Error = anchor_lang::error::Error;

    fn try_from(accounts: &SaberSwapAccounts<'info>) -> Result<Self> {
        Ok(Self {
            lp_mint_decimals: accounts.pool_mint.decimals,
            saber: SaberSwap {
                initial_amp_factor: accounts.saber_swap.initial_amp_factor,
                target_amp_factor: accounts.saber_swap.target_amp_factor,
                current_ts: Clock::get()?.unix_timestamp,
                start_ramp_ts: accounts.saber_swap.start_ramp_ts,
                stop_ramp_ts: accounts.saber_swap.stop_ramp_ts,

                lp_mint_supply: accounts.pool_mint.supply,
                token_a_reserve: accounts.reserve_a.amount,
                token_b_reserve: accounts.reserve_b.amount,
            },
        })
    }
}

impl<'info> Validate<'info> for SaberSwapAccounts<'info> {
    fn validate(&self) -> Result<()> {
        assert_keys_eq!(self.arrow.vendor_miner.mint, self.pool_mint);
        assert_keys_eq!(self.saber_swap.pool_mint, self.pool_mint);
        assert_keys_eq!(self.saber_swap.token_a.reserves, self.reserve_a);
        assert_keys_eq!(self.saber_swap.token_b.reserves, self.reserve_b);
        Ok(())
    }
}
