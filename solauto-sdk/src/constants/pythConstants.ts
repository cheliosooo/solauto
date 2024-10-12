import { NATIVE_MINT } from "@solana/spl-token";
import * as tokens from "./tokenConstants";

// https://pyth.network/developers/price-feed-ids#solana-stable
export const PYTH_PRICE_FEED_IDS = {
  [NATIVE_MINT.toString()]:
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d",
  [tokens.B_SOL]:
    "0x89875379e70f8fbadc17aef315adf3a8d5d160b811435537e03c97e8aac97d9c",
  [tokens.JITO_SOL]:
    "0x67be9f519b95cf24338801051f9a808eff0a578ccb388db73b7f6fe1de019ffb",
  [tokens.LST]:
    "0x12fb674ee496045b1d9cf7d5e65379acb026133c2ad69f3ed996fb9fe68e3a37",
  [tokens.M_SOL]:
    "0xc2289a6a43d2ce91c6f55caec370f4acc38a2ed477f58813334c6d03749ff2a4",
  [tokens.INF]:
    "0xf51570985c642c49c2d6e50156390fdba80bb6d5f7fa389d2f012ced4f7d208f",
  [tokens.JUP]:
    "0x0a0408d619e9380abad35060f9192039ed5042fa6f82301d0e48bb52be830996",
  [tokens.BONK]:
    "0x72b021217ca3fe68922a19aaf990109cb9d84e9ad004b4d2025ad6f529314419",
  [tokens.JTO]:
    "0xb43660a5f790c69354b0729a5ef9d50d68f1df92107540210b9cccba1f947cc2",
  [tokens.JLP]:
    "0xc811abc82b4bad1f9bd711a2773ccaa935b03ecef974236942cec5e0eb845a3a",
  [tokens.USDC]:
    "0xeaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a",
  [tokens.USDT]:
    "0x2b89b9dc8fdf9f34709a5b106b472f0f39bb6ca9ce04b0fd7f2e971688e2e53b",
};
