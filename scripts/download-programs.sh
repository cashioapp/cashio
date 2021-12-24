#!/usr/bin/env sh

cd $(dirname $0)/..

mkdir -p artifacts/programs/

# saber
solana program dump SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ \
    artifacts/programs/stable_swap.so --url mainnet-beta

# crate
curl -L https://github.com/CrateProtocol/crate/releases/download/v0.4.0/crate_token.so > \
    artifacts/programs/crate_token.so

# arrow
curl -L https://github.com/ArrowProtocol/arrow/releases/download/v0.1.8/arrow_sunny.so > \
    artifacts/programs/arrow_sunny.so

# sunny
solana program dump SPQR4kT3q2oUKEJes2L6NNSBCiPW9SfuhkuqC9bp6Sx \
    artifacts/programs/sunny.so --url mainnet-beta

# quarry
curl -L https://github.com/QuarryProtocol/quarry/releases/download/v1.11.3/quarry_mine.so > \
    artifacts/programs/quarry_mine.so

curl -L https://github.com/QuarryProtocol/quarry/releases/download/v1.11.3/quarry_mint_wrapper.so > \
    artifacts/programs/quarry_mint_wrapper.so
