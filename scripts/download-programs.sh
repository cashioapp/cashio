#!/usr/bin/env sh

cd $(dirname $0)/..

mkdir -p artifacts/programs/

# saber
solana program dump SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ \
    artifacts/programs/stable_swap.so --url mainnet-beta

# crate
solana program dump CRATwLpu6YZEeiVq9ajjxs61wPQ9f29s1UoQR9siJCRs \
    artifacts/programs/crate_token.so --url mainnet-beta

# arrow
solana program dump ARoWLTBWoWrKMvxEiaE2EH9DrWyV7mLpKywGDWxBGeq9 \
    artifacts/programs/arrow_sunny.so --url mainnet-beta

# sunny
solana program dump SPQR4kT3q2oUKEJes2L6NNSBCiPW9SfuhkuqC9bp6Sx \
    artifacts/programs/sunny.so --url mainnet-beta

# quarry
solana program dump QMNeHCGYnLVDn1icRAfQZpjPLBNkfGbSKRB83G5d8KB \
    artifacts/programs/quarry_mine.so --url mainnet-beta

solana program dump QMWoBmAyJLAsA1Lh9ugMTw2gciTihncciphzdNzdZYV \
    artifacts/programs/quarry_mint_wrapper.so --url mainnet-beta
