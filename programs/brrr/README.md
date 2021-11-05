# 🖨 brrr

[![Crates.io](https://img.shields.io/crates/v/brrr)](https://crates.io/crates/brrr)
[![Docs.rs](https://docs.rs/brrr/badge.svg)](https://docs.rs/brrr)
[![License](https://img.shields.io/badge/license-AGPL)](https://github.com/CashioApp/cashio/blob/master/LICENSE.txt)

Handles the printing and burning of $CASH, using [Saber LP](https://saber.so) [Arrows](https://arrowprotocol.com) as collateral.

## Addresses

The program address is the same on devnet, testnet, and mainnet-beta.

Program Address: [`BRRRot6ig147TBU6EGp7TMesmQrwu729CbG6qu2ZUHWm`](https://explorer.solana.com/address/BRRRot6ig147TBU6EGp7TMesmQrwu729CbG6qu2ZUHWm)

## Mechanism

There are two instructions that handle each of these actions: `print_cash` and `burn_cash`.

### `print_cash`

This instruction prints $CASH in exchange for Arrow Saber LP tokens.

The issue authority of `print_cash` is `BJ9L3jNu6tvrUxPHTMfwyA8Lgw2X6ky5bVNyDqiXSxgA`.

### `burn_cash`

This instruction burns $CASH in exchange for Arrow Saber LP tokens.

The withdraw authority of `burn_cash` is `7Twx9JYz3gB4rF3h2cyUMnQWj9QEtmwviTvVD7xjAGEw`.
