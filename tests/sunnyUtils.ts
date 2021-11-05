import {
  generateSunnyPoolAddress,
  SUNNY_CREATOR_KEY,
  SUNNY_PROGRAM,
  SunnyPoolQuarryJSON,
} from "@arrowprotocol/arrow";
import { Program } from "@project-serum/anchor";
import { expectTX } from "@saberhq/chai-solana";
import type { Provider } from "@saberhq/solana-contrib";
import { TransactionEnvelope } from "@saberhq/solana-contrib";
import { createInitMintInstructions } from "@saberhq/token-utils";
import type { PublicKey } from "@solana/web3.js";
import { Keypair, SystemProgram } from "@solana/web3.js";

export const createSunnyPool = async ({
  provider,
  rewarder,
  quarry,
}: {
  provider: Provider;
  rewarder: PublicKey;
  quarry: PublicKey;
}): Promise<{ sunnyPool: PublicKey; internalMint: PublicKey }> => {
  const sunny = new Program(SunnyPoolQuarryJSON, SUNNY_PROGRAM);
  // set up the sunny pool
  const [pool, bump] = await generateSunnyPoolAddress({
    quarry,
  });
  const sunnyMintKP = Keypair.generate();
  const createInternalMint = await createInitMintInstructions({
    provider,
    mintKP: sunnyMintKP,
    decimals: 6,
    mintAuthority: pool,
    freezeAuthority: pool,
  });
  const newPoolIx = sunny.instruction.newPool(bump, {
    accounts: {
      creator: SUNNY_CREATOR_KEY,
      rewarder,
      quarry,
      pool,
      payer: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
      internalMint: sunnyMintKP.publicKey,
    },
  });
  await expectTX(
    createInternalMint.combine(new TransactionEnvelope(provider, [newPoolIx])),
    "create sunny pool"
  ).to.be.fulfilled;
  return { sunnyPool: pool, internalMint: sunnyMintKP.publicKey };
};
