//! Handles the printing and burning of $CASH, using
//! [Saber LP](https://saber.so) [Arrows](https://arrowprotocol.com) as collateral.
//!
//! Printing is done in exchange for Arrow Saber LP tokens.
//! Burning allows the redemption of any single Saber LP.
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]

mod actions;
mod addresses;
mod events;
mod saber;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use arrow_sunny::Arrow;
use bankman::{Bank, Collateral};
use stable_swap_anchor::SwapInfo;
use vipers::validate::Validate;

pub use addresses::*;
pub use events::*;

declare_id!("BRRRot6ig147TBU6EGp7TMesmQrwu729CbG6qu2ZUHWm");

/// [brrr] program.
#[program]
pub mod brrr {
    use super::*;

    /// Prints $CASH.
    ///
    /// $CASH can be printed by depositing Saber LP tokens.
    /// The amount of $CASH created is based on the virtual price of the
    /// Saber LP token; for example, if one deposits 1 USDC-USDT LP
    /// but that LP's virtual price is 1.02, one will receive 1.02 $CASH
    /// for each 1 USDC-USDT LP deposited.
    #[access_control(ctx.accounts.validate())]
    pub fn print_cash(ctx: Context<PrintCash>, deposit_amount: u64) -> Result<()> {
        vipers::invariant!(false, "temporarily disabled");
        actions::print_cash::print_cash(ctx, deposit_amount)
    }

    /// Burns $CASH.
    ///
    /// $CASH may be burned for any of the underlying LP tokens.
    /// This means that $CASHs's underlying value is the value of its cheapest ("floor")
    /// LP token, minus the burn fee.
    #[access_control(ctx.accounts.validate())]
    pub fn burn_cash(ctx: Context<BurnCash>, burn_amount: u64) -> Result<()> {
        vipers::invariant!(false, "temporarily disabled");
        actions::burn_cash::burn_cash(ctx, burn_amount)
    }
}

/// Accounts related to the Saber pool.
#[derive(Accounts)]
pub struct SaberSwapAccounts<'info> {
    /// The [Arrow] used as collateral.
    pub arrow: Box<Account<'info, Arrow>>,
    /// The Saber [SwapInfo] of the collateral.
    pub saber_swap: Box<Account<'info, SwapInfo>>,
    /// Mint of the pool.
    pub pool_mint: Box<Account<'info, Mint>>,
    /// Reserve of token A.
    pub reserve_a: Box<Account<'info, TokenAccount>>,
    /// Reserve of token B.
    pub reserve_b: Box<Account<'info, TokenAccount>>,
}

/// Accounts for printing $CASH.
#[derive(Accounts)]
pub struct PrintCash<'info> {
    /// Common accounts.
    pub common: BrrrCommon<'info>,

    /// The depositor into the pool.
    #[account(mut)]
    pub depositor: Signer<'info>,

    /// The source of the deposited [Collateral] tokens.
    #[account(mut)]
    pub depositor_source: Box<Account<'info, TokenAccount>>,

    /// Destination of the issued $CASH.
    #[account(mut)]
    pub mint_destination: Box<Account<'info, TokenAccount>>,

    /// The [ISSUE_AUTHORITY_ADDRESS].
    /// CHECK: this is handled by Vipers.
    pub issue_authority: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct BrrrCommon<'info> {
    /// Information about the bank.
    pub bank: Box<Account<'info, Bank>>,

    /// The [Collateral].
    pub collateral: Box<Account<'info, Collateral>>,

    /// Information about the crate.
    pub crate_token: Box<Account<'info, crate_token::CrateToken>>,

    /// [Mint] of the [crate_token::CrateToken].
    #[account(mut)]
    pub crate_mint: Box<Account<'info, Mint>>,

    /// [TokenAccount] holding the [Collateral] tokens of the [crate_token::CrateToken].
    #[account(mut)]
    pub crate_collateral_tokens: Box<Account<'info, TokenAccount>>,

    /// Saber swap accounts.
    pub saber_swap: SaberSwapAccounts<'info>,

    /// [Token] program.
    pub token_program: Program<'info, Token>,

    /// [crate_token::program::CrateToken] program.
    pub crate_token_program: Program<'info, crate_token::program::CrateToken>,
}

/// Accounts for burning $CASH.
#[derive(Accounts)]
pub struct BurnCash<'info> {
    /// Common accounts.
    pub common: BrrrCommon<'info>,

    /// The depositor into the pool.
    #[account(mut)]
    pub burner: Signer<'info>,

    /// The source of the burned $CASH.
    #[account(mut)]
    pub burned_cash_source: Box<Account<'info, TokenAccount>>,

    /// Destination of the issued tokens.
    #[account(mut)]
    pub withdraw_destination: Box<Account<'info, TokenAccount>>,

    /// Author fee token destination
    #[account(mut)]
    pub author_fee_destination: Account<'info, TokenAccount>,

    /// Protocol fee token destination
    #[account(mut)]
    pub protocol_fee_destination: Account<'info, TokenAccount>,

    /// The [WITHDRAW_AUTHORITY_ADDRESS].
    /// CHECK: this is handled by Vipers.
    pub withdraw_authority: UncheckedAccount<'info>,
}

/// Errors.
#[error_code]
pub enum ErrorCode {
    #[msg("Too many of this LP token are being used as collateral.")]
    CollateralHardCapHit,
    #[msg("Insufficient pool funds.")]
    InsufficientFunds,
}
