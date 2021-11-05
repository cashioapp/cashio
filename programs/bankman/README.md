# `bankman`

[![Crates.io](https://img.shields.io/crates/v/bankman)](https://crates.io/crates/bankman)
[![Docs.rs](https://docs.rs/bankman/badge.svg)](https://docs.rs/bankman)
[![License](https://img.shields.io/badge/license-AGPL)](https://github.com/cashioapp/cashio/blob/master/LICENSE.txt)

Allowlist for $CASH collateral tokens.

The Bank manager, or `bankman` for short, keeps track of the collateral that is allowed to be used as a backing for the $CASH token.

This program itself does not have the ability to print or burn $CASH. Instead, it delegates this responsibility to the `brrr` program, which acts as the [Crate](https://crate.so) `issue_authority` and `withdraw_authority`.

## Addresses

The program address is the same on devnet, testnet, and mainnet-beta.

Program Address: [`BANKhiCgEYd7QmcWwPLkqvTuuLN6qEwXDZgTe6HEbwv1`](https://explorer.solana.com/address/BANKhiCgEYd7QmcWwPLkqvTuuLN6qEwXDZgTe6HEbwv1)
