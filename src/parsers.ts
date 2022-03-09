import { BorshCoder } from "@project-serum/anchor";
import type { KeyedAccountInfo } from "@solana/web3.js";

import type { BankData, CollateralData } from ".";
import { BankmanJSON } from "./idls/bankman";

export const BANKMAN_CODER = new BorshCoder(BankmanJSON);

export const parseBank = (data: KeyedAccountInfo): BankData =>
  BANKMAN_CODER.accounts.decode<BankData>("Bank", data.accountInfo.data);

export const parseCollateral = (data: KeyedAccountInfo): CollateralData =>
  BANKMAN_CODER.accounts.decode<CollateralData>(
    "Collateral",
    data.accountInfo.data
  );
