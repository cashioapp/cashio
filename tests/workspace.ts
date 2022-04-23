import * as anchor from "@project-serum/anchor";
import { AnchorProvider } from "@project-serum/anchor";
import { chaiSolana } from "@saberhq/chai-solana";
import { SolanaProvider } from "@saberhq/solana-contrib";
import chai from "chai";

import { CashioSDK } from "../src";

chai.use(chaiSolana);

const anchorProvider = AnchorProvider.env();
anchor.setProvider(anchorProvider);

const provider = SolanaProvider.init({
  connection: anchorProvider.connection,
  wallet: anchorProvider.wallet,
  opts: anchorProvider.opts,
});

export const makeSDK = (): CashioSDK => {
  return CashioSDK.init(provider);
};
