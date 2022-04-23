use crate::*;
use anchor_spl::token::{self, Token, TokenAccount};

/// Accounts for [bankman::withdraw_author_fee].
#[derive(Accounts)]
pub struct WithdrawAuthorFee<'info> {
    /// The [Bank].
    #[account(has_one = bankman @ crate::ErrorCode::UnauthorizedNotBankman)]
    pub bank: Account<'info, Bank>,
    /// The [Bank::bankman].
    pub bankman: Signer<'info>,
    /// The [Collateral].
    #[account(has_one = bank)]
    pub collateral: Account<'info, Collateral>,
    /// Author fees.
    #[account(mut, constraint = author_fees.mint == collateral.mint)]
    pub author_fees: Account<'info, TokenAccount>,
    /// Account to send the author fees to.
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    /// The [Token] program.
    pub token_program: Program<'info, Token>,
}

impl<'info> Validate<'info> for WithdrawAuthorFee<'info> {
    fn validate(&self) -> Result<()> {
        assert_keys_neq!(self.author_fees, self.destination);
        assert_keys_eq!(self.author_fees.owner, self.bank);
        assert_keys_eq!(self.author_fees.mint, self.collateral.mint);
        Ok(())
    }
}

pub fn handler(ctx: Context<WithdrawAuthorFee>, amount: u64) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"Bank".as_ref(),
        &ctx.accounts.bank.crate_token.to_bytes(),
        &[ctx.accounts.bank.bump],
    ]];
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.author_fees.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.bank.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        amount,
    )?;
    Ok(())
}
