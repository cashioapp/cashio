import { Arrow, generateArrowAddress } from "@arrowprotocol/arrow";
import {
  CRATE_ADDRESSES,
  CRATE_FEE_OWNER,
  CrateSDK,
  generateCrateAddress,
} from "@crateprotocol/crate-sdk";
import type { AccountClient } from "@project-serum/anchor";
import { AnchorProvider, Program } from "@project-serum/anchor";
import type { AugmentedProvider, Provider } from "@saberhq/solana-contrib";
import {
  SignerWallet,
  SolanaAugmentedProvider,
  SolanaProvider,
  TransactionEnvelope,
} from "@saberhq/solana-contrib";
import type { StableSwap } from "@saberhq/stableswap-sdk";
import {
  createInitMintInstructions,
  getATAAddress,
  getATAAddresses,
  getOrCreateATA,
  getOrCreateATAs,
  Token,
  TOKEN_PROGRAM_ID,
  TokenAmount,
} from "@saberhq/token-utils";
import type { PublicKey, Signer } from "@solana/web3.js";
import { Keypair, SystemProgram } from "@solana/web3.js";

import {
  BRRR_ISSUE_AUTHORITY,
  BURN_WITHDRAW_AUTHORITY,
  CASH_DECIMALS,
  CASHIO_ADDRESSES,
  generateCollateralAddress,
} from ".";
import { generateBankAddress } from "./pda";
import type { BankData, BankmanIDL, BankmanProgram } from "./programs/bankman";
import { BankmanJSON } from "./programs/bankman";
import type { BrrrProgram } from "./programs/brrr";
import { BrrrJSON } from "./programs/brrr";

export interface CashioPrograms {
  Brrr: BrrrProgram;
  Bankman: BankmanProgram;
}

/**
 * Javascript SDK for interacting with Cashio.
 */
export class CashioSDK {
  /**
   * Reference to the Crate SDK.
   */
  readonly crate: CrateSDK;

  /**
   * Reference to Arrow Protocol.
   */
  readonly arrow: Arrow;

  constructor(
    readonly provider: AugmentedProvider,
    readonly programs: CashioPrograms
  ) {
    this.crate = CrateSDK.init(provider);
    this.arrow = Arrow.init(provider);
  }

  /**
   * Initialize from a Provider
   * @param provider
   * @returns
   */
  static init(provider: Provider): CashioSDK {
    const anchorProvider = new AnchorProvider(
      provider.connection,
      provider.wallet,
      provider.opts
    );
    return new CashioSDK(new SolanaAugmentedProvider(provider), {
      Brrr: new Program(
        BrrrJSON,
        CASHIO_ADDRESSES.Brrr,
        anchorProvider
      ) as unknown as BrrrProgram,
      Bankman: new Program(
        BankmanJSON,
        CASHIO_ADDRESSES.Bankman,
        anchorProvider
      ) as unknown as BankmanProgram,
    });
  }

  /**
   * Creates a new instance of the SDK with the given keypair.
   */
  withSigner(signer: Signer): CashioSDK {
    return CashioSDK.init(
      new SolanaProvider(
        this.provider.connection,
        this.provider.broadcaster,
        new SignerWallet(signer),
        this.provider.opts
      )
    );
  }

  /**
   * Creates a new Bank.
   * @returns
   */
  async newBank({
    mintKP = Keypair.generate(),
    admin = this.provider.wallet.publicKey,
    payer = this.provider.wallet.publicKey,
  }: {
    mintKP?: Keypair;
    admin?: PublicKey;
    payer?: PublicKey;
  } = {}): Promise<{
    tx: TransactionEnvelope;
    bankKey: PublicKey;
    crateKey: PublicKey;
  }> {
    const [crateKey, crateBump] = await generateCrateAddress(mintKP.publicKey);
    const [bankKey, aggBump] = await generateBankAddress(crateKey);
    const initMintTX = await createInitMintInstructions({
      provider: this.provider,
      mintKP,
      decimals: CASH_DECIMALS,
      mintAuthority: crateKey,
      freezeAuthority: crateKey,
    });
    const newBankTX = new TransactionEnvelope(this.provider, [
      this.programs.Bankman.instruction.newBank(aggBump, crateBump, {
        accounts: {
          crateMint: mintKP.publicKey,
          payer,
          bank: bankKey,
          crateToken: crateKey,
          brrrIssueAuthority: BRRR_ISSUE_AUTHORITY,
          burnWithdrawAuthority: BURN_WITHDRAW_AUTHORITY,
          admin,
          systemProgram: SystemProgram.programId,
          crateTokenProgram: CRATE_ADDRESSES.CrateToken,
        },
      }),
    ]);
    return { tx: initMintTX.combine(newBankTX), bankKey, crateKey };
  }

  /**
   * Authorizes a new pool as collateral.
   * @returns
   */
  async authorizeCollateral({
    bankKey,
    mint,
    curator = this.provider.wallet.publicKey,
    payer = this.provider.wallet.publicKey,
  }: {
    bankKey: PublicKey;
    mint: PublicKey;
    curator?: PublicKey;
    payer?: PublicKey;
  }): Promise<{ tx: TransactionEnvelope; collateralKey: PublicKey }> {
    const [collateralKey, bump] = await generateCollateralAddress(
      bankKey,
      mint,
      this.programs.Bankman.programId
    );

    const bank: BankData | null = await (
      this.programs.Bankman.account as unknown as {
        bank: AccountClient<BankmanIDL>;
      }
    ).bank.fetchNullable(bankKey);
    if (!bank) {
      throw new Error("No bank found.");
    }

    // create the ATA for the bank
    const bankATA = await getOrCreateATA({
      provider: this.provider,
      mint,
      owner: bankKey,
    });

    // create the ATA for collateral
    const createATA = await getOrCreateATA({
      provider: this.provider,
      mint,
      owner: bank.crateToken,
    });

    const feeATA = await getOrCreateATA({
      provider: this.provider,
      mint,
      owner: CRATE_FEE_OWNER,
    });

    const newStakePoolTX = new TransactionEnvelope(this.provider, [
      ...(bankATA.instruction ? [bankATA.instruction] : []),
      ...(createATA.instruction ? [createATA.instruction] : []),
      ...(feeATA.instruction ? [feeATA.instruction] : []),
      this.programs.Bankman.instruction.authorizeCollateral(bump, {
        accounts: {
          bank: bankKey,
          collateral: collateralKey,
          mint,
          curator,
          payer,
          systemProgram: SystemProgram.programId,
        },
      }),
    ]);
    return { tx: newStakePoolTX, collateralKey };
  }

  /**
   * Set collateral hard cap
   * @returns
   */
  async setCollateralHardCap({
    bankKey,
    hardCap,
    curator = this.provider.wallet.publicKey,
  }: {
    bankKey: PublicKey;
    hardCap: TokenAmount;
    curator?: PublicKey;
  }): Promise<TransactionEnvelope> {
    const [collateralKey] = await generateCollateralAddress(
      bankKey,
      hardCap.token.mintAccount,
      this.programs.Bankman.programId
    );
    return new TransactionEnvelope(this.provider, [
      this.programs.Bankman.instruction.setCollateralHardCap(hardCap.toU64(), {
        accounts: {
          bank: bankKey,
          collateral: collateralKey,
          curator,
        },
      }),
    ]);
  }

  /**
   * Helper for withdrawing author fees.
   * @returns
   */
  async withdrawAuthorFees({
    bankKey,
    amount,
    bankman = this.provider.wallet.publicKey,
    recipient = this.provider.wallet.publicKey,
  }: {
    bankKey: PublicKey;
    amount: TokenAmount;
    bankman?: PublicKey;
    recipient?: PublicKey;
  }): Promise<TransactionEnvelope> {
    const [collateralKey] = await generateCollateralAddress(
      bankKey,
      amount.token.mintAccount,
      this.programs.Bankman.programId
    );
    const authorFees = await getATAAddress({
      mint: amount.token.mintAccount,
      owner: bankKey,
    });
    const destination = await getOrCreateATA({
      provider: this.provider,
      mint: amount.token.mintAccount,
      owner: recipient,
    });
    return this.provider.newTX([
      destination.instruction,
      this.programs.Bankman.instruction.withdrawAuthorFee(amount.toU64(), {
        accounts: {
          bank: bankKey,
          bankman,
          collateral: collateralKey,
          authorFees,
          destination: destination.address,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
      }),
    ]);
  }

  /**
   * Prints $CASH from Saber LP tokens, via Arrow.
   * @returns
   */
  async printCashFromLP({
    arrowMint,
    bankKey,
    lpAmount,
    swap,
    depositor = this.provider.wallet.publicKey,
  }: {
    bankKey: PublicKey;
    swap: StableSwap;
    lpAmount: TokenAmount;
    arrowMint: PublicKey;
    depositor?: PublicKey;
  }): Promise<{ stakeTX: TransactionEnvelope; printTX: TransactionEnvelope }> {
    const stakeTX = await this.arrow.stake({
      arrowMint,
      amount: lpAmount,
      depositor,
    });
    const printTX = await this.printCash({
      bankKey,
      collateralAmount: new TokenAmount(
        Token.fromMint(arrowMint, lpAmount.token.decimals),
        lpAmount.raw
      ),
      swap,
      depositor,
    });
    return { stakeTX, printTX };
  }

  /**
   * Prints $CASH.
   * @returns
   */
  async printCash({
    bankKey,
    collateralAmount,
    swap,
    depositor = this.provider.wallet.publicKey,
  }: {
    bankKey: PublicKey;
    swap: StableSwap;
    collateralAmount: TokenAmount;
    depositor?: PublicKey;
  }): Promise<TransactionEnvelope> {
    const bank: BankData | null = await (
      this.programs.Bankman.account as unknown as {
        bank: AccountClient<BankmanIDL>;
      }
    ).bank.fetchNullable(bankKey);
    if (!bank) {
      throw new Error("No bank found.");
    }

    const depositorATAs = await getOrCreateATAs({
      provider: this.provider,
      mints: {
        collateral: collateralAmount.token.mintAccount,
        cash: bank.crateMint,
      },
      owner: depositor,
    });

    // the collateral account should never be necessary
    // but we don't call it here so we can create the ATA
    // outside of here
    // if (depositorATAs.createAccountInstructions.collateral) {
    //   throw new Error("collateral ATA does not exist");
    // }

    return new TransactionEnvelope(this.provider, [
      ...(depositorATAs.createAccountInstructions.cash
        ? [depositorATAs.createAccountInstructions.cash]
        : []),
      this.programs.Brrr.instruction.printCash(collateralAmount.toU64(), {
        accounts: {
          common: await this._getCommonSwapAccounts({
            bank: {
              key: bankKey,
              data: bank,
            },
            swap,
            arrowMint: collateralAmount.token.mintAccount,
          }),
          issueAuthority: BRRR_ISSUE_AUTHORITY,
          depositor,
          depositorSource: depositorATAs.accounts.collateral,
          mintDestination: depositorATAs.accounts.cash,
        },
      }),
    ]);
  }

  /**
   * Burns $CASH.
   * @returns
   */
  async burnCash({
    bankKey,
    cashAmount,
    swap,
    burner = this.provider.wallet.publicKey,
    arrowMint,
  }: {
    bankKey: PublicKey;
    swap: StableSwap;
    cashAmount: TokenAmount;
    burner?: PublicKey;
    arrowMint: PublicKey;
  }): Promise<TransactionEnvelope> {
    const bank: BankData | null = await (
      this.programs.Bankman.account as unknown as {
        bank: AccountClient<BankmanIDL>;
      }
    ).bank.fetchNullable(bankKey);
    if (!bank) {
      throw new Error("No bank found.");
    }

    const bankATAs = await getATAAddresses({
      mints: {
        withdraw: arrowMint,
      },
      owner: bankKey,
    });

    const burnerATAs = await getOrCreateATAs({
      provider: this.provider,
      mints: {
        crate: bank.crateMint,
        withdraw: arrowMint,
      },
      owner: burner,
    });

    const protocolFeeATA = await getATAAddress({
      mint: arrowMint,
      owner: CRATE_FEE_OWNER,
    });

    return new TransactionEnvelope(this.provider, [
      ...burnerATAs.instructions,
      this.programs.Brrr.instruction.burnCash(cashAmount.toU64(), {
        accounts: {
          common: await this._getCommonSwapAccounts({
            bank: {
              key: bankKey,
              data: bank,
            },
            swap,
            arrowMint: arrowMint,
          }),
          withdrawAuthority: BURN_WITHDRAW_AUTHORITY,
          burner,
          burnedCashSource: burnerATAs.accounts.crate,
          withdrawDestination: burnerATAs.accounts.withdraw,
          authorFeeDestination: bankATAs.accounts.withdraw.address,
          protocolFeeDestination: protocolFeeATA,
        },
      }),
    ]);
  }

  /**
   * Unstakes LP tokens from an Arrow.
   */
  async unstake({
    lpAmount,
    arrowMint,
  }: {
    lpAmount: TokenAmount;
    arrowMint: PublicKey;
  }): Promise<TransactionEnvelope> {
    return await this.arrow.unstake({
      amount: lpAmount,
      arrowMint,
    });
  }

  private async _getCommonSwapAccounts({
    bank: { key: bankKey, data: bankData },
    swap,
    arrowMint,
  }: {
    bank: {
      key: PublicKey;
      data: BankData;
    };
    swap: StableSwap;
    arrowMint: PublicKey;
  }) {
    const [collateral] = await generateCollateralAddress(bankKey, arrowMint);
    const crateCollateralTokens = await getATAAddress({
      mint: arrowMint,
      owner: bankData.crateToken,
    });
    const [arrow] = await generateArrowAddress(arrowMint);

    return {
      bank: bankKey,
      collateral,
      crateCollateralTokens,
      crateToken: bankData.crateToken,
      crateMint: bankData.crateMint,
      saberSwap: {
        arrow,
        saberSwap: swap.config.swapAccount,
        poolMint: swap.state.poolTokenMint,
        reserveA: swap.state.tokenA.reserve,
        reserveB: swap.state.tokenB.reserve,
      },
      tokenProgram: TOKEN_PROGRAM_ID,
      crateTokenProgram: CRATE_ADDRESSES.CrateToken,
    };
  }
}
