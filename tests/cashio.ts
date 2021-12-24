import { BN, EventParser } from "@project-serum/anchor";
import { expectTX } from "@saberhq/chai-solana";
import type { StableSwap } from "@saberhq/stableswap-sdk";
import { deployNewSwap, SWAP_PROGRAM_ID } from "@saberhq/stableswap-sdk";
import {
  createInitMintInstructions,
  createMint,
  getATAAddress,
  getTokenAccount,
  SPLToken,
  Token,
  TOKEN_PROGRAM_ID,
  TokenAmount,
  u64,
} from "@saberhq/token-utils";
import type { PublicKey } from "@solana/web3.js";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";

import type { AddCollateralEvent, CashioSDK } from "../src";
import { BANKMAN_CODER } from "../src";
import { createRewarderAndQuarry } from "./quarryUtils";
import { createSunnyPool } from "./sunnyUtils";
import { makeSDK } from "./workspace";

describe("Cashio", () => {
  let sdk: CashioSDK;
  let bank: PublicKey;
  let cashToken: Token;

  beforeEach(async () => {
    sdk = makeSDK();
    const mintKP = Keypair.generate();
    const { tx: createTX, bankKey } = await sdk.newBank({
      mintKP,
    });
    bank = bankKey;
    cashToken = Token.fromMint(mintKP.publicKey, 6, {
      name: "CASH",
    });
    await expectTX(createTX, "Create Crate Token").to.be.fulfilled;
  });

  it("add collateral pool", async () => {
    const collateralKP = Keypair.generate();
    const randomCollateral = await createInitMintInstructions({
      provider: sdk.provider,
      mintKP: collateralKP,
      decimals: 9,
    });
    await expectTX(randomCollateral).to.be.fulfilled;

    const { tx } = await sdk.authorizeCollateral({
      bankKey: bank,
      mint: collateralKP.publicKey,
    });
    const result = await tx.send();
    await expectTX(result).to.be.fulfilled;

    const parser = new EventParser(
      sdk.programs.Bankman.programId,
      BANKMAN_CODER
    );
    const logs = (await result.wait()).response.meta?.logMessages ?? [];

    parser.parseLogs(logs, (ev) => {
      const event: AddCollateralEvent = ev as AddCollateralEvent;

      expect(event.name).to.eq("AddCollateralEvent");
      expect(event.data.bank).to.eqAddress(bank);
      expect(event.data.curator).to.eqAddress(sdk.provider.wallet.publicKey);
      expect(event.data.mint).to.eqAddress(collateralKP.publicKey);
    });
  });

  describe("with saber collateral", () => {
    let swap: StableSwap;
    let lpToken: Token;
    let sunnyPool: PublicKey;
    let arrowToken: Token;

    beforeEach("prepare swap", async () => {
      const { provider } = sdk;

      const adminKP = Keypair.generate();
      await sdk.provider.connection.confirmTransaction(
        await sdk.provider.connection.requestAirdrop(
          adminKP.publicKey,
          10 * LAMPORTS_PER_SOL
        )
      );
      const mintA = await createMint(provider, adminKP.publicKey, 6);
      const mintB = await createMint(provider, adminKP.publicKey, 6);

      const tokenA = Token.fromMint(mintA, 6);
      const tokenB = Token.fromMint(mintB, 6);

      const deployResult = await deployNewSwap({
        provider,
        swapProgramID: SWAP_PROGRAM_ID,

        initialLiquidityProvider: sdk.provider.wallet.publicKey,
        useAssociatedAccountForInitialLP: true,
        tokenAMint: tokenA.mintAccount,
        tokenBMint: tokenB.mintAccount,
        adminAccount: adminKP.publicKey,
        ampFactor: new u64(1_000),

        seedPoolAccounts: ({ tokenAAccount, tokenBAccount }) => ({
          instructions: [
            SPLToken.createMintToInstruction(
              TOKEN_PROGRAM_ID,
              mintA,
              tokenAAccount,
              adminKP.publicKey,
              [],
              1_000_000
            ),
            SPLToken.createMintToInstruction(
              TOKEN_PROGRAM_ID,
              mintB,
              tokenBAccount,
              adminKP.publicKey,
              [],
              1_000_000
            ),
          ],
          signers: [adminKP],
        }),
      });

      swap = deployResult.swap;
      lpToken = Token.fromMint(deployResult.swap.state.poolTokenMint, 6);

      // set up the quarry and rewarder
      const rewarderAndQuarry = await createRewarderAndQuarry({
        connection: sdk.provider.connection,
        stakedToken: lpToken,
        annualRate: new u64(1_000_000_000),
      });

      // set up the sunny pool
      const sunnyPoolResult = await createSunnyPool({
        provider: sdk.provider,
        rewarder: rewarderAndQuarry.rewarder,
        quarry: rewarderAndQuarry.quarry,
      });
      sunnyPool = sunnyPoolResult.sunnyPool;
      const internalMint = sunnyPoolResult.internalMint;

      // set up the sunny quarry and rewarder
      const sunnyRewarder = await createRewarderAndQuarry({
        connection: sdk.provider.connection,
        stakedToken: Token.fromMint(internalMint, 6),
        annualRate: new u64(1_000_000_000),
      });

      const beneficiaryKP = Keypair.generate();
      const arrowMintKP = Keypair.generate();
      const { initTX, newArrowTX } = await sdk.arrow.newArrow({
        sunnyPool,
        beneficiary: beneficiaryKP.publicKey,
        mintKP: arrowMintKP,
        sunnyRewarderKey: sunnyRewarder.rewarder,
      });
      await expectTX(initTX, "init").to.be.fulfilled;
      await expectTX(newArrowTX, "new arrow").to.be.fulfilled;

      arrowToken = Token.fromMint(arrowMintKP.publicKey, 6);

      const { tx } = await sdk.authorizeCollateral({
        bankKey: bank,
        mint: arrowToken.mintAccount,
      });
      const result = await tx.send();
      await expectTX(result).to.be.fulfilled;

      const parser = new EventParser(
        sdk.programs.Bankman.programId,
        BANKMAN_CODER
      );
      const logs = (await result.wait()).response.meta?.logMessages ?? [];

      parser.parseLogs(logs, (ev) => {
        const event: AddCollateralEvent = ev as AddCollateralEvent;

        expect(event.name).to.eq("AddCollateralEvent");
        expect(event.data.bank, "bank").to.eqAddress(bank);
        expect(event.data.curator, "curator").to.eqAddress(
          sdk.provider.wallet.publicKey
        );
        expect(event.data.mint, "mint").to.eqAddress(arrowToken.mintAccount);
      });

      await expectTX(
        await sdk.setCollateralHardCap({
          bankKey: bank,
          hardCap: new TokenAmount(arrowToken, 1_000),
        }),
        "set collateral hard cap"
      ).to.be.fulfilled;
    });

    describe("print", () => {
      it("happy path", async () => {
        const { stakeTX, printTX } = await sdk.printCashFromLP({
          arrowMint: arrowToken.mintAccount,
          bankKey: bank,
          lpAmount: new TokenAmount(lpToken, 1_000),
          swap,
        });
        await expectTX(stakeTX, "stake").to.be.fulfilled;
        await expectTX(printTX, "print").to.be.fulfilled;
      });

      it("cannot print over hard cap", async () => {
        const { stakeTX, printTX } = await sdk.printCashFromLP({
          arrowMint: arrowToken.mintAccount,
          bankKey: bank,
          lpAmount: new TokenAmount(lpToken, 1_001),
          swap,
        });
        await expectTX(stakeTX, "stake").to.be.fulfilled;
        await expectTX(printTX, "print over hard cap").to.be.rejected;
      });
    });

    it("burn", async () => {
      const { stakeTX, printTX } = await sdk.printCashFromLP({
        arrowMint: arrowToken.mintAccount,
        bankKey: bank,
        lpAmount: new TokenAmount(lpToken, 1_000),
        swap,
      });
      await expectTX(stakeTX, "stake").to.be.fulfilled;
      await expectTX(printTX, "print").to.be.fulfilled;

      expect(
        (
          await getTokenAccount(
            sdk.provider,
            await getATAAddress({
              mint: cashToken.mintAccount,
              owner: sdk.provider.wallet.publicKey,
            })
          )
        ).amount,
        "cash in wallet"
      ).to.bignumber.eq(new BN(1_000));

      const burnTX = await sdk.burnCash({
        arrowMint: arrowToken.mintAccount,
        bankKey: bank,
        cashAmount: new TokenAmount(cashToken, 1_000),
        swap,
      });
      await expectTX(burnTX, "burn").to.be.fulfilled;

      expect(
        (
          await getTokenAccount(
            sdk.provider,
            await getATAAddress({
              mint: cashToken.mintAccount,
              owner: sdk.provider.wallet.publicKey,
            })
          )
        ).amount,
        "no more tokens"
      ).to.bignumber.eq("0");
    });
  });
});
