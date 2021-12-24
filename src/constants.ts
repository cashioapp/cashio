import { ENV, Token } from "@saberhq/token-utils";
import { PublicKey } from "@solana/web3.js";

export const CASHIO_ADDRESSES = {
  Brrr: new PublicKey("BRRRot6ig147TBU6EGp7TMesmQrwu729CbG6qu2ZUHWm"),
  Bankman: new PublicKey("BANKhiCgEYd7QmcWwPLkqvTuuLN6qEwXDZgTe6HEbwv1"),
};

export const BRRR_ISSUE_AUTHORITY = new PublicKey(
  "BJ9L3jNu6tvrUxPHTMfwyA8Lgw2X6ky5bVNyDqiXSxgA"
);

export const BURN_WITHDRAW_AUTHORITY = new PublicKey(
  "7Twx9JYz3gB4rF3h2cyUMnQWj9QEtmwviTvVD7xjAGEw"
);

/**
 * Number of decimals in $CASH.
 */
export const CASH_DECIMALS = 6;

export const BANK_KEY = new PublicKey(
  "Em1PdaWY1NSpyGgKUstvZu3HzJNe9d15c3dePzBr9QwM"
);

export const CRATE_TOKEN = new PublicKey(
  "J77Nq48nbq4Etf1voss38R3dTdR3yD7y5F6W6TaVHvmb"
);

export const CRATE_MINT = new PublicKey(
  "CASHVDm2wsJXfhj6VWxb7GiMdoLc17Du7paH4bNr5woT"
);

export const CASH_TOKEN_INFO = {
  name: "CASH",
  decimals: 6,
  address: CRATE_MINT.toString(),
  symbol: "CASH",
  logoURI: "/images/icon.png",
  extensions: {
    website: "https://cashio.app",
  },
};

export const CASH_TOKEN = new Token({
  ...CASH_TOKEN_INFO,
  chainId: ENV.MainnetBeta,
});
