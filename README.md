# cashio

[![License](https://img.shields.io/crates/l/bankman)](https://github.com/cashioapp/cashio/blob/master/LICENSE.md)
[![Build Status](https://img.shields.io/github/workflow/status/cashioapp/cashio/E2E/master)](https://github.com/cashioapp/cashio/actions/workflows/programs-e2e.yml?query=branch%3Amaster)
[![Contributors](https://img.shields.io/github/contributors/cashioapp/cashio)](https://github.com/cashioapp/cashio/graphs/contributors)
[![Chat](https://img.shields.io/badge/chat-on%20keybase-success)](https://keybase.io/team/cashiochat)

![Cashio](/images/cashio.png)

cashio is a decentralized stablecoin made for the people, by the people.

We're in active development. For the latest updates, please join our community:

- Twitter: https://twitter.com/cashioapp
- Discord: https://discord.gg/5Mvhfc8vnX

## About

Cashio is a decentralized stablecoin fully backed by interest-bearing [Saber](https://saber.so) USD liquidity provider tokens. Cashio specifically chooses USD LPs that are backed by safer USD assets, attempting to capture the risk-free rate of the Solana stablecoin ecosystem.

Using [Arrow Protocol](https://arrowprotocol.com), Cashio stakes LP tokens into [Sunny Aggregator](https://sunny.ag), earning $SBR and $SUNNY tokens to the Cashio DAO. Cashio also uses [Crate Protocol](https://crateprotocol.com) to build its USD-pegged stablecoin, which can be thought of as a basket of stablecoin LPs.

_Currently, protocol profits accrue to a program-owned account known as the Bank. We intend to create a mechanism to have these cash flows accrue value to users of the Cashio Protocol. More information on this will be available soon._

## Packages

| Package          | Description                                                                     | Version                                                                                                 | Docs                                                                                   |
| :--------------- | :------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------- |
| `converter`      | Math helpers for converting $CASH to/from Saber LP tokens.                      | [![Crates.io](https://img.shields.io/crates/v/converter)](https://crates.io/crates/converter)           | [![Docs.rs](https://docs.rs/converter/badge.svg)](https://docs.rs/converter)           |
| `brrr`           | Handles the printing and burning of $CASH, using Saber LP Arrows as collateral. | [![Crates.io](https://img.shields.io/crates/v/brrr)](https://crates.io/crates/brrr)                     | [![Docs.rs](https://docs.rs/brrr/badge.svg)](https://docs.rs/brrr)                     |
| `bankman`        | Allowlist for $CASH collateral tokens.                                          | [![Crates.io](https://img.shields.io/crates/v/bankman)](https://crates.io/crates/bankman)               | [![Docs.rs](https://docs.rs/bankman/badge.svg)](https://docs.rs/cashio)                |
| `@cashio/cashio` | TypeScript SDK for Cashio                                                       | [![npm](https://img.shields.io/npm/v/@cashio/cashio.svg)](https://www.npmjs.com/package/@cashio/cashio) | [![Docs](https://img.shields.io/badge/docs-typedoc-blue)](https://docs.cashio.app/ts/) |

## Note

- **Cashio is in active development, so all APIs are subject to change.**
- **This code is unaudited. Use at your own risk.**

## Contribution

Thank you for your interest in contributing to Cashio Protocol! All contributions are welcome no
matter how big or small. This includes (but is not limited to) filing issues,
adding documentation, fixing bugs, creating examples, and implementing features.

If you'd like to contribute, please claim an issue by commenting, forking, and
opening a pull request, even if empty. This allows the maintainers to track who
is working on what issue as to not overlap work.

For simple documentation changes, feel free to just open a pull request.

If you're considering larger changes or self motivated features, please file an issue
and engage with the maintainers by joining the development channel on [Keybase](https://keybase.io/team/cashiochat).

## License

Cashio Protocol is licensed under [the Affero GPL 3.0 license](/LICENSE.txt).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Cashio Protocol by you, as defined in the AGPL-3.0 license, shall be licensed as above, without any additional terms or conditions.
