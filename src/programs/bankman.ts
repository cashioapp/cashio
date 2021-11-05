import type { AnchorTypes } from "@saberhq/anchor-contrib";

import type { BankmanIDL } from "../idls/bankman";

export * from "../idls/bankman";

type BankmanTypes = AnchorTypes<
  BankmanIDL,
  {
    bank: BankData;
    collateral: CollateralData;
  }
>;

export type BankData = BankmanTypes["Accounts"]["Bank"];
export type CollateralData = BankmanTypes["Accounts"]["Collateral"];

export type BankmanProgram = BankmanTypes["Program"];

export type NewBankEvent = BankmanTypes["Events"]["NewBankEvent"];
export type AddCollateralEvent = BankmanTypes["Events"]["AddCollateralEvent"];
export type SetCuratorEvent = BankmanTypes["Events"]["SetCuratorEvent"];
export type SetCollateralHardCapEvent =
  BankmanTypes["Events"]["SetCollateralHardCapEvent"];
