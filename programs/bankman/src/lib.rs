//! Allowlist for $CASH collateral tokens.
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]

mod account_validators;
mod events;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use vipers::prelude::*;

pub use events::*;
use instructions::*;
pub use state::*;

/// Number of decimals of $CASH.
pub const CASH_DECIMALS: u8 = 6;

declare_id!("BANKhiCgEYd7QmcWwPLkqvTuuLN6qEwXDZgTe6HEbwv1");

/// [bankman] program.
#[program]
pub mod bankman {
    use super::*;

    /// Provisions a new [Bank].
    #[access_control(ctx.accounts.validate())]
    pub fn new_bank(ctx: Context<NewBank>, _bank_bump: u8, crate_bump: u8) -> Result<()> {
        crate_token::cpi::new_crate(
            CpiContext::new(
                ctx.accounts.crate_token_program.to_account_info(),
                crate_token::cpi::accounts::NewCrate {
                    crate_mint: ctx.accounts.crate_mint.to_account_info(),
                    crate_token: ctx.accounts.crate_token.to_account_info(),
                    fee_to_setter: ctx.accounts.bank.to_account_info(),
                    fee_setter_authority: ctx.accounts.bank.to_account_info(),
                    author_fee_to: ctx.accounts.bank.to_account_info(),
                    issue_authority: ctx.accounts.brrr_issue_authority.to_account_info(),
                    withdraw_authority: ctx.accounts.burn_withdraw_authority.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
            ),
            crate_bump,
        )?;

        let bank_bump = unwrap_bump!(ctx, "bank");
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"Bank".as_ref(),
            &ctx.accounts.crate_token.key().to_bytes(),
            &[bank_bump],
        ]];

        // Initial withdraw fee is 0.5% or 50 bps
        crate_token::cpi::set_withdraw_fee(
            CpiContext::new(
                ctx.accounts.crate_token_program.to_account_info(),
                crate_token::cpi::accounts::SetFees {
                    crate_token: ctx.accounts.crate_token.to_account_info(),
                    fee_setter: ctx.accounts.bank.to_account_info(),
                },
            )
            .with_signer(signer_seeds),
            50,
        )?;

        let bank = &mut ctx.accounts.bank;
        bank.crate_token = ctx.accounts.crate_token.key();
        bank.bump = bank_bump;

        bank.crate_mint = ctx.accounts.crate_mint.key();
        bank.curator = ctx.accounts.admin.key();
        bank.bankman = ctx.accounts.admin.key();

        emit!(NewBankEvent {
            bank: bank.key(),
            curator: bank.curator,
            timestamp: Clock::get()?.unix_timestamp
        });

        Ok(())
    }

    /// Adds a new collateral pool to a [Bank].
    #[access_control(ctx.accounts.validate())]
    pub fn authorize_collateral(ctx: Context<AuthorizeCollateral>, _bump: u8) -> Result<()> {
        let bank = &ctx.accounts.bank;

        let collateral = &mut ctx.accounts.collateral;
        collateral.bank = bank.key();
        collateral.mint = ctx.accounts.mint.key();
        collateral.bump = unwrap_bump!(ctx, "collateral");

        emit!(AddCollateralEvent {
            bank: bank.key(),
            collateral: collateral.key(),
            curator: bank.curator,
            mint: collateral.mint,
            timestamp: Clock::get()?.unix_timestamp
        });
        Ok(())
    }

    /// Adds a new collateral pool to a [Bank].
    #[access_control(ctx.accounts.validate())]
    pub fn set_collateral_hard_cap(
        ctx: Context<SetCollateralHardCap>,
        hard_cap: u64,
    ) -> Result<()> {
        let collateral = &mut ctx.accounts.collateral;
        collateral.hard_cap = hard_cap;

        emit!(SetCollateralHardCapEvent {
            bank: ctx.accounts.bank.key(),
            collateral: collateral.key(),
            hard_cap,
            timestamp: Clock::get()?.unix_timestamp
        });
        Ok(())
    }

    /// Sets the curator.
    #[access_control(ctx.accounts.validate())]
    pub fn set_curator(ctx: Context<SetCurator>) -> Result<()> {
        let bank = &mut ctx.accounts.bank;
        let previous_curator = bank.curator;
        bank.curator = ctx.accounts.next_curator.key();

        emit!(SetCuratorEvent {
            bank: bank.key(),
            previous_curator,
            curator: bank.curator,
            bankman: bank.bankman,
            timestamp: Clock::get()?.unix_timestamp
        });

        Ok(())
    }

    /// Sets the bankman.
    #[access_control(ctx.accounts.validate())]
    pub fn set_bankman(ctx: Context<SetBankman>) -> Result<()> {
        let bank = &mut ctx.accounts.bank;
        let previous_bankman = bank.bankman;
        bank.bankman = ctx.accounts.next_bankman.key();

        emit!(SetBankmanEvent {
            bank: bank.key(),
            previous_bankman,
            bankman: bank.bankman,
            timestamp: Clock::get()?.unix_timestamp
        });

        Ok(())
    }

    /// Withdraws the author fee to the specified location.
    #[access_control(ctx.accounts.validate())]
    pub fn withdraw_author_fee(ctx: Context<WithdrawAuthorFee>, amount: u64) -> Result<()> {
        instructions::withdraw_author_fee::handler(ctx, amount)
    }
}

/// Accounts for [bankman::new_bank].
#[derive(Accounts)]
pub struct NewBank<'info> {
    /// Information about the [Bank].
    #[account(
        init,
        seeds = [
            b"Bank".as_ref(),
            crate_token.key().to_bytes().as_ref()
        ],
        bump,
        space = 8 + Bank::BYTES,
        payer = payer
    )]
    pub bank: Account<'info, Bank>,

    /// [Mint] of the [crate_token::CrateToken].
    pub crate_mint: Account<'info, Mint>,

    /// The [crate_token::CrateToken] to be created.
    #[account(mut)]
    pub crate_token: SystemAccount<'info>,

    /// The `brrr_issue_authority`.
    /// CHECK: Arbitrary.
    pub brrr_issue_authority: UncheckedAccount<'info>,

    /// The `burn_withdraw_authority`.
    /// CHECK: Arbitrary.
    pub burn_withdraw_authority: UncheckedAccount<'info>,

    /// Payer of the crate initialization.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The admin, who becomes the curator and the curator setter.
    /// CHECK: Arbitrary.
    pub admin: UncheckedAccount<'info>,

    /// System program.
    pub system_program: Program<'info, System>,

    /// Crate token program.
    pub crate_token_program: Program<'info, crate_token::program::CrateToken>,
}

/// Accounts for [bankman::authorize_collateral].
#[derive(Accounts)]
pub struct AuthorizeCollateral<'info> {
    /// The [Bank].
    pub bank: Account<'info, Bank>,

    /// The [Collateral] to add.
    #[account(
        init,
        seeds = [
            b"Collateral".as_ref(),
            bank.key().to_bytes().as_ref(),
            mint.key().to_bytes().as_ref()
        ],
        bump,
        space = 8 + Collateral::BYTES,
        payer = payer
    )]
    pub collateral: Account<'info, Collateral>,

    /// [Mint] of the collateral.
    pub mint: Box<Account<'info, Mint>>,

    /// The [Bank::curator].
    pub curator: Signer<'info>,

    /// Payer of the crate initialization.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// System program.
    pub system_program: Program<'info, System>,
}

/// Accounts for [bankman::set_collateral_hard_cap].
#[derive(Accounts)]
pub struct SetCollateralHardCap<'info> {
    /// The [Bank].
    pub bank: Account<'info, Bank>,
    /// The [Collateral].
    #[account(mut)]
    pub collateral: Account<'info, Collateral>,
    /// The [Bank::curator].
    pub curator: Signer<'info>,
}

/// Accounts for [bankman::set_curator].
#[derive(Accounts)]
pub struct SetCurator<'info> {
    /// The [Bank].
    #[account(mut)]
    pub bank: Account<'info, Bank>,
    /// The [Bank::bankman].
    pub bankman: Signer<'info>,
    /// The [Bank::curator] to set.
    /// CHECK: Arbitrary.
    pub next_curator: UncheckedAccount<'info>,
}

/// Accounts for [bankman::set_bankman].
#[derive(Accounts)]
pub struct SetBankman<'info> {
    /// The [Bank].
    #[account(mut)]
    pub bank: Account<'info, Bank>,
    /// The [Bank::bankman].
    pub bankman: Signer<'info>,
    /// The [Bank::curator] to set.
    /// CHECK: Arbitrary.
    pub next_bankman: UncheckedAccount<'info>,
}

/// Errors.
#[error_code]
pub enum ErrorCode {
    #[msg("Must be curator.")]
    UnauthorizedNotCurator,
    #[msg("Must be the bankman.")]
    UnauthorizedNotBankman,

    #[msg("Pool not found in snapshot.", offset = 10)]
    PoolNotFoundInSnapshot,
    #[msg("Cannot add a pool that has already been added.")]
    PoolAlreadyAdded,

    #[msg("new_bank: supply must be zero", offset = 20)]
    NewBankSupplyMustBeZero,
    #[msg("new_bank: cash must have 6 decimals")]
    NewBankWrongDecimals,
    #[msg("new_bank: crate already initialized")]
    NewBankAlreadyInitialized,
}
