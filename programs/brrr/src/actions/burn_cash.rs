use std::convert::TryInto;

use crate::*;
use anchor_lang::prelude::*;
use converter::CashSwap;
use vipers::{assert_keys_eq, unwrap_int, validate::Validate};

/// Prints $CASH.
pub fn burn_cash(ctx: Context<BurnCash>, burn_amount: u64) -> Result<()> {
    ctx.accounts.burn_cash(burn_amount)
}

impl<'info> BurnCash<'info> {
    /// We like the $CASH.
    fn burn_cash(&self, burn_amount: u64) -> Result<()> {
        let swap: CashSwap = (&self.common.saber_swap).try_into()?;
        let withdraw_pool_token_amount =
            unwrap_int!(swap.calculate_pool_tokens_for_cash(burn_amount));
        if withdraw_pool_token_amount == 0 {
            return Ok(());
        }

        let current_balance = self.common.crate_collateral_tokens.amount;
        require!(
            current_balance >= withdraw_pool_token_amount,
            InsufficientFunds
        );

        // Burn the $CASH.
        anchor_spl::token::burn(
            CpiContext::new(
                self.common.token_program.to_account_info(),
                anchor_spl::token::Burn {
                    mint: self.common.crate_mint.to_account_info(),
                    from: self.burned_cash_source.to_account_info(),
                    authority: self.burner.to_account_info(),
                },
            ),
            burn_amount,
        )?;

        // Withdraw the LP tokens from the pool.
        crate_token::cpi::withdraw(
            CpiContext::new_with_signer(
                self.common.crate_token_program.to_account_info(),
                crate_token::cpi::accounts::Withdraw {
                    crate_token: self.common.crate_token.to_account_info(),
                    crate_underlying: self.common.crate_collateral_tokens.to_account_info(),
                    withdraw_authority: self.withdraw_authority.to_account_info(),
                    withdraw_destination: self.withdraw_destination.to_account_info(),
                    author_fee_destination: self.author_fee_destination.to_account_info(),
                    protocol_fee_destination: self.protocol_fee_destination.to_account_info(),
                    token_program: self.common.token_program.to_account_info(),
                },
                WITHDRAW_AUTHORITY_SIGNER_SEEDS,
            ),
            withdraw_pool_token_amount,
        )?;

        Ok(())
    }
}

impl<'info> Validate<'info> for BurnCash<'info> {
    fn validate(&self) -> Result<()> {
        self.common.validate()?;
        assert_keys_eq!(self.burner, self.burned_cash_source.owner);
        assert_keys_eq!(self.burned_cash_source.mint, self.common.crate_mint);

        assert_keys_eq!(self.withdraw_destination.mint, self.common.collateral.mint);
        // author_fee_destination is validated by Crate
        // protocol_fee_destination is validated by Crate
        assert_keys_eq!(self.withdraw_authority, WITHDRAW_AUTHORITY_ADDRESS);
        Ok(())
    }
}
