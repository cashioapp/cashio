use std::convert::TryInto;

use crate::*;
use anchor_lang::prelude::*;
use converter::CashSwap;
use vipers::{assert_keys_eq, unwrap_int, validate::Validate};

/// Prints $CASH.
pub fn print_cash(ctx: Context<PrintCash>, deposit_amount: u64) -> Result<()> {
    ctx.accounts.print_cash(deposit_amount)
}

impl<'info> PrintCash<'info> {
    fn print_cash(&self, deposit_amount: u64) -> Result<()> {
        let current_balance = self.common.crate_collateral_tokens.amount;
        require!(
            unwrap_int!(current_balance.checked_add(deposit_amount))
                <= self.common.collateral.hard_cap,
            CollateralHardCapHit
        );

        let swap: CashSwap = (&self.common.saber_swap).try_into()?;
        let print_amount = unwrap_int!(swap.calculate_cash_for_pool_tokens(deposit_amount));
        if print_amount == 0 {
            return Ok(());
        }

        // transfer LP tokens to the crate
        anchor_spl::token::transfer(
            CpiContext::new(
                self.common.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: self.depositor_source.to_account_info(),
                    to: self.common.crate_collateral_tokens.to_account_info(),
                    authority: self.depositor.to_account_info(),
                },
            ),
            deposit_amount,
        )?;

        // issue new crate tokens
        crate_token::cpi::issue(
            CpiContext::new_with_signer(
                self.common.crate_token_program.to_account_info(),
                crate_token::cpi::accounts::Issue {
                    crate_token: self.common.crate_token.to_account_info(),
                    crate_mint: self.common.crate_mint.to_account_info(),
                    issue_authority: self.issue_authority.to_account_info(),
                    mint_destination: self.mint_destination.to_account_info(),

                    // there are no author/protocol fees, so we pass in garbage here
                    author_fee_destination: self.mint_destination.to_account_info(),
                    protocol_fee_destination: self.mint_destination.to_account_info(),

                    token_program: self.common.token_program.to_account_info(),
                },
                ISSUE_AUTHORITY_SIGNER_SEEDS,
            ),
            print_amount,
        )?;

        emit!(PrintCashEvent {
            depositor: self.depositor.key(),
            collateral_mint: self.common.crate_collateral_tokens.mint,
            deposit_amount,
            print_amount,
            timestamp: Clock::get()?.unix_timestamp
        });

        Ok(())
    }
}

impl<'info> Validate<'info> for PrintCash<'info> {
    fn validate(&self) -> Result<()> {
        self.common.validate()?;
        assert_keys_eq!(self.depositor, self.depositor_source.owner);
        assert_keys_eq!(self.depositor_source.mint, self.common.collateral.mint);
        assert_keys_eq!(self.mint_destination.mint, self.common.crate_token.mint);
        assert_keys_eq!(self.issue_authority, ISSUE_AUTHORITY_ADDRESS);
        Ok(())
    }
}
