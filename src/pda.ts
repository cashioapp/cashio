import { utils } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";

import { CASHIO_ADDRESSES } from ".";

/**
 * Generates the canonical Bank PDA.
 *
 * @param crateToken
 * @param programID
 * @returns
 */
export const generateBankAddress = (
  crateToken: PublicKey,
  programID: PublicKey = CASHIO_ADDRESSES.Bankman
): Promise<[PublicKey, number]> => {
  return PublicKey.findProgramAddress(
    [utils.bytes.utf8.encode("Bank"), crateToken.toBuffer()],
    programID
  );
};

/**
 * Generates the canonical Collateral PDA.
 *
 * @param bank Bank.
 * @param mint Mint of the collateral.
 * @param programID
 * @returns
 */
export const generateCollateralAddress = (
  bank: PublicKey,
  mint: PublicKey,
  programID: PublicKey = CASHIO_ADDRESSES.Bankman
): Promise<[PublicKey, number]> => {
  return PublicKey.findProgramAddress(
    [utils.bytes.utf8.encode("Collateral"), bank.toBuffer(), mint.toBuffer()],
    programID
  );
};
