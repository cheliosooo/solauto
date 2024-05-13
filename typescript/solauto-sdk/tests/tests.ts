import {
  Transaction,
  UmiPlugin,
  createSignerFromKeypair,
  publicKey,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  toWeb3JsKeypair,
  toWeb3JsLegacyTransaction,
} from "@metaplex-foundation/umi-web3js-adapters";
import {
  createSolautoProgram,
  deserializePosition,
  solendOpenPosition,
} from "../generated";
import { generateRandomU8, getSecretKey } from "./testUtils";
import {
  Connection,
  Keypair,
  PublicKey,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  clusterApiUrl,
} from "@solana/web3.js";
import { getSolendAccounts } from "../accounts";
import { getPositionAccount, getSolendObligationAccount } from "../utils";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import { assert, expect } from "chai";

const connection = new Connection(clusterApiUrl("mainnet-beta"), "confirmed");
let umi = createUmi(connection);
const secretKey = getSecretKey();
const signerKeypair = umi.eddsa.createKeypairFromSecretKey(secretKey);
const signer = createSignerFromKeypair(umi, signerKeypair);
const signerPublicKey = Keypair.fromSecretKey(secretKey).publicKey;

async function simulateTransaction(transaction: Transaction) {
  const web3Transaction = toWeb3JsLegacyTransaction(transaction);
  web3Transaction.sign(toWeb3JsKeypair(signerKeypair));

  const simulationResult = await connection.simulateTransaction(
    web3Transaction
  );
  if (simulationResult.value.err) {
    console.log(simulationResult.value.logs);
  }
  assert.equal(simulationResult.value.err, undefined);
}

export const solauto = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createSolautoProgram(), false);
  },
});

describe("Solauto tests", async () => {
  umi = umi.use(solauto()).use(signerIdentity(signer));

  const reuseAccounts = false;
  const payForTransactions = false;

  const solautoManaged = true;
  const positionId = generateRandomU8();

  const solendAccounts = getSolendAccounts("main");
  const solautoPosition = solautoManaged
    ? await getPositionAccount(signerPublicKey, positionId, reuseAccounts)
    : undefined;
  const obligation = await getSolendObligationAccount(
    solautoPosition,
    signerPublicKey,
    solendAccounts.lendingMarket,
    solendAccounts.solendProgram,
    reuseAccounts
  );
  const positionSupplyLiquidityTokenAccount = await getAssociatedTokenAddress(
    solendAccounts.solReserve.liquidityTokenMint,
    solautoPosition,
    solautoManaged
  );
  const positionSupplyCollateralTokenAccount = await getAssociatedTokenAddress(
    solendAccounts.solReserve.collateralTokenMint,
    solautoPosition,
    solautoManaged
  );
  const positionDebtLiquidityTokenAccount = await getAssociatedTokenAddress(
    solendAccounts.usdcReserve.liquidityTokenMint,
    solautoPosition,
    solautoManaged
  );

  it("should open position", async () => {
    const settingParams = {
      repayFromBps: 9500,
      repayToBps: 9000,
      boostFromBps: 4000,
      boostToBps: 5000,
    };
    const builder = solendOpenPosition(umi, {
      signer,
      solendProgram: publicKey(solendAccounts.solendProgram),
      systemProgram: publicKey(SystemProgram.programId),
      tokenProgram: publicKey(TOKEN_PROGRAM_ID),
      ataProgram: publicKey(ASSOCIATED_TOKEN_PROGRAM_ID),
      rent: publicKey(SYSVAR_RENT_PUBKEY),
      solautoPosition: solautoManaged ? publicKey(solautoPosition) : undefined,
      obligation: publicKey(obligation),
      lendingMarket: publicKey(solendAccounts.lendingMarket),
      supplyReserve: publicKey(solendAccounts.solReserve.reserve),
      positionSupplyLiquidityTa: publicKey(positionSupplyLiquidityTokenAccount),
      supplyLiquidityMint: publicKey(
        solendAccounts.solReserve.liquidityTokenMint
      ),
      positionSupplyCollateralTa: publicKey(
        positionSupplyCollateralTokenAccount
      ),
      supplyCollateralMint: publicKey(
        solendAccounts.solReserve.collateralTokenMint
      ),
      positionDebtLiquidityTa: publicKey(positionDebtLiquidityTokenAccount),
      debtLiquidityMint: publicKey(
        solendAccounts.usdcReserve.liquidityTokenMint
      ),
      updatePositionData: {
        positionId,
        settingParams,
        protocolData: {
          protocolPosition: publicKey(obligation),
          supplyMint: publicKey(solendAccounts.solReserve.liquidityTokenMint),
          debtMint: publicKey(solendAccounts.usdcReserve.liquidityTokenMint),
        },
      },
    });

    const transaction = await builder.buildWithLatestBlockhash(umi);
    await simulateTransaction(transaction);

    if (payForTransactions) {
      await builder.sendAndConfirm(umi);
    }

    if (reuseAccounts || payForTransactions) {
      const account = await umi.rpc.getAccount(publicKey(solautoPosition));
      assert(account.exists);
      const position = deserializePosition(account);
      expect(position.position.setting_params).to.deep.equal(settingParams);
    }
  });
});
