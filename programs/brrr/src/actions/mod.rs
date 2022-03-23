//! Processes actions related to the money printer

use crate::BrrrCommon;
use anchor_lang::prelude::*;
use vipers::{assert_keys_eq, validate::Validate};

pub(crate) mod burn_cash;
pub(crate) mod print_cash;

impl<'info> Validate<'info> for BrrrCommon<'info> {
    fn validate(&self) -> Result<()> {
        assert_keys_eq!(self.bank, self.collateral.bank);
        assert_keys_eq!(self.bank.crate_mint, self.crate_mint);
        assert_keys_eq!(self.crate_token, self.crate_collateral_tokens.owner);
        assert_keys_eq!(self.crate_mint, self.crate_token.mint);
        assert_keys_eq!(self.crate_collateral_tokens.mint, self.collateral.mint);

        // saber swap
        self.saber_swap.validate()?;
        assert_keys_eq!(self.collateral.mint, self.saber_swap.arrow.mint);

        Ok(())
    }
}
