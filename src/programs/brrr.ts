import type { AnchorTypes } from "@saberhq/anchor-contrib";

import type { BrrrIDL } from "../idls/brrr";

export * from "../idls/brrr";

type BrrrTypes = AnchorTypes<BrrrIDL>;

export type BrrrProgram = BrrrTypes["Program"];

export type BurnCashEvent = BrrrTypes["Events"]["BurnCashEvent"];
export type PrintCashEvent = BrrrTypes["Events"]["PrintCashEvent"];
