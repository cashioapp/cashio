//! Validate accounts

use crate::*;
use anchor_lang::prelude::*;
use vipers::assert_keys_eq;
use vipers::validate::Validate;

macro_rules! assert_is_curator {
    ($self: ident) => {
        assert_keys_eq!($self.curator, $self.bank.curator, UnauthorizedNotCurator);
    };
}

macro_rules! assert_is_bankman {
    ($self: ident) => {
        assert_keys_eq!($self.bankman, $self.bank.bankman, UnauthorizedNotBankman);
    };
}

impl<'info> Validate<'info> for NewBank<'info> {
    fn validate(&self) -> Result<()> {
        // brrr_issue_authority does not need to be validated because
        // the Bank is created once
        // burn_withdraw_authority does not need to be validated because
        // the Bank is created once
        require!(self.crate_mint.supply == 0, NewBankSupplyMustBeZero);
        require!(
            self.crate_mint.decimals == CASH_DECIMALS,
            NewBankWrongDecimals
        );
        require!(self.crate_token.data_is_empty(), NewBankAlreadyInitialized);
        Ok(())
    }
}

impl<'info> Validate<'info> for AuthorizeCollateral<'info> {
    fn validate(&self) -> Result<()> {
        assert_is_curator!(self);
        Ok(())
    }
}

impl<'info> Validate<'info> for SetCurator<'info> {
    fn validate(&self) -> Result<()> {
        assert_is_bankman!(self);
        Ok(())
    }
}

impl<'info> Validate<'info> for SetBankman<'info> {
    fn validate(&self) -> Result<()> {
        assert_is_bankman!(self);
        Ok(())
    }
}

impl<'info> Validate<'info> for SetCollateralHardCap<'info> {
    fn validate(&self) -> Result<()> {
        assert_is_curator!(self);
        assert_keys_eq!(self.collateral.bank, self.bank);
        Ok(())
    }
}
