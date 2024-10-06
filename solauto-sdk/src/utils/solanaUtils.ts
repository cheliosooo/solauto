import bs58 from "bs58";
import {
  AddressLookupTableInput,
  Signer,
  TransactionBuilder,
  Umi,
  WrappedInstruction,
  publicKey,
  transactionBuilder,
} from "@metaplex-foundation/umi";
import {
  fromWeb3JsInstruction,
  toWeb3JsPublicKey,
  toWeb3JsTransaction,
} from "@metaplex-foundation/umi-web3js-adapters";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  AddressLookupTableAccount,
  ComputeBudgetProgram,
  Connection,
  PublicKey,
  RpcResponseAndContext,
  SimulatedTransactionResponse,
  SystemProgram,
  TransactionInstruction,
  VersionedTransaction,
} from "@solana/web3.js";
import {
  createAssociatedTokenAccountIdempotentInstruction,
  createCloseAccountInstruction,
  createTransferInstruction,
} from "@solana/spl-token";
import { getTokenAccount } from "./accountUtils";
import { arraysAreEqual, consoleLog, retryWithExponentialBackoff } from "./generalUtils";
import {
  getLendingAccountEndFlashloanInstructionDataSerializer,
  getLendingAccountStartFlashloanInstructionDataSerializer,
} from "../marginfi-sdk";
import { PriorityFeeSetting, TransactionRunType } from "../types";

export function buildHeliusApiUrl(heliusApiKey: string) {
  return `https://mainnet.helius-rpc.com/?api-key=${heliusApiKey}`;
}

export function buildIronforgeApiUrl(ironforgeApiKey: string) {
  return `https://rpc.ironforge.network/mainnet?apiKey=${ironforgeApiKey}`;
}

export function getSolanaRpcConnection(
  rpcUrl: string
): [Connection, Umi] {
  const connection = new Connection(rpcUrl, "confirmed");
  const umi = createUmi(connection);
  return [connection, umi];
}

export function getWrappedInstruction(
  signer: Signer,
  ix: TransactionInstruction
): WrappedInstruction {
  return {
    instruction: fromWeb3JsInstruction(ix),
    signers: [signer],
    bytesCreatedOnChain: 0,
  };
}

export function setComputeUnitLimitUmiIx(
  signer: Signer,
  maxComputeUnits: number
): WrappedInstruction {
  return getWrappedInstruction(
    signer,
    ComputeBudgetProgram.setComputeUnitLimit({
      units: maxComputeUnits,
    })
  );
}

export function setComputeUnitPriceUmiIx(
  signer: Signer,
  lamports: number
): WrappedInstruction {
  return getWrappedInstruction(
    signer,
    ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: lamports,
    })
  );
}

export function createAssociatedTokenAccountUmiIx(
  signer: Signer,
  wallet: PublicKey,
  mint: PublicKey
): WrappedInstruction {
  return getWrappedInstruction(
    signer,
    createAssociatedTokenAccountIdempotentInstruction(
      toWeb3JsPublicKey(signer.publicKey),
      getTokenAccount(wallet, mint),
      wallet,
      mint
    )
  );
}

export function systemTransferUmiIx(
  signer: Signer,
  destination: PublicKey,
  lamports: bigint
): WrappedInstruction {
  return getWrappedInstruction(
    signer,
    SystemProgram.transfer({
      fromPubkey: toWeb3JsPublicKey(signer.publicKey),
      toPubkey: destination,
      lamports,
    })
  );
}

export function closeTokenAccountUmiIx(
  signer: Signer,
  tokenAccount: PublicKey,
  authority: PublicKey
): WrappedInstruction {
  return getWrappedInstruction(
    signer,
    createCloseAccountInstruction(tokenAccount, authority, authority)
  );
}

export function splTokenTransferUmiIx(
  signer: Signer,
  fromTa: PublicKey,
  toTa: PublicKey,
  authority: PublicKey,
  amount: bigint
): WrappedInstruction {
  return getWrappedInstruction(
    signer,
    createTransferInstruction(fromTa, toTa, authority, amount)
  );
}

export async function getAdressLookupInputs(
  umi: Umi,
  lookupTableAddresses: string[]
): Promise<AddressLookupTableInput[]> {
  const addressLookupTableAccountInfos = await umi.rpc.getAccounts(
    lookupTableAddresses.map((key) => publicKey(key))
  );

  return addressLookupTableAccountInfos.reduce((acc, accountInfo, index) => {
    const addressLookupTableAddress = lookupTableAddresses[index];
    if (accountInfo.exists) {
      acc.push({
        publicKey: publicKey(addressLookupTableAddress),
        addresses: AddressLookupTableAccount.deserialize(
          accountInfo.data
        ).addresses.map((x) => publicKey(x)),
      } as AddressLookupTableInput);
    }

    return acc;
  }, new Array<AddressLookupTableInput>());
}

export function assembleFinalTransaction(
  signer: Signer,
  tx: TransactionBuilder,
  computeUnitPrice: number,
  computeUnitLimit?: number
) {
  tx = tx
    .prepend(setComputeUnitPriceUmiIx(signer, computeUnitPrice))
    .prepend(
      computeUnitLimit
        ? setComputeUnitLimitUmiIx(signer, computeUnitLimit)
        : transactionBuilder()
    );

  const marginfiStartFlSerializer =
    getLendingAccountStartFlashloanInstructionDataSerializer();
  const marginfiStartFlDiscriminator = marginfiStartFlSerializer
    .serialize({
      endIndex: 0,
    })
    .slice(0, 8);

  const marginfiEndFlSerializer =
    getLendingAccountEndFlashloanInstructionDataSerializer();
  const marginfiEndFlDiscriminator = marginfiEndFlSerializer
    .serialize({
      endIndex: 0,
    })
    .slice(0, 8);

  let endFlIndex = 0;
  const instructions = tx.getInstructions();

  for (let i = instructions.length - 1; i >= 0; i--) {
    const ix = instructions[i];

    try {
      const [data, _] = marginfiStartFlSerializer.deserialize(ix.data);
      if (
        arraysAreEqual(
          data.discriminator,
          Array.from(marginfiStartFlDiscriminator)
        )
      ) {
        ix.data = marginfiStartFlSerializer.serialize({
          endIndex: endFlIndex,
        });
      }
    } catch {}

    try {
      const [data, _] = marginfiEndFlSerializer.deserialize(ix.data);
      if (
        arraysAreEqual(
          data.discriminator,
          Array.from(marginfiEndFlDiscriminator)
        )
      ) {
        endFlIndex = i;
      }
    } catch {}
  }

  return tx;
}

async function simulateTransaction(
  connection: Connection,
  transaction: VersionedTransaction
): Promise<RpcResponseAndContext<SimulatedTransactionResponse>> {
  const simulationResult = await connection.simulateTransaction(transaction, {
    sigVerify: false,
    commitment: "processed",
  });
  if (simulationResult.value.err) {
    simulationResult.value.logs?.forEach((x: any) => {
      consoleLog(x);
    });
    throw simulationResult.value.err;
  }
  return simulationResult;
}

export async function getComputeUnitPriceEstimate(
  umi: Umi,
  tx: TransactionBuilder,
  prioritySetting: PriorityFeeSetting
): Promise<number | undefined> {
  const web3Transaction = toWeb3JsTransaction(
    (await tx.setLatestBlockhash(umi, { commitment: "finalized" })).build(umi)
  );
  const serializedTransaction = bs58.encode(web3Transaction.serialize());

  let feeEstimate: number | undefined;
  try {
    const resp = await umi.rpc.call("getPriorityFeeEstimate", [
      {
        transaction: serializedTransaction,
        options: {
          priorityLevel: prioritySetting.toString(),
        },
      },
    ]);
    feeEstimate = Math.round((resp as any).priorityFeeEstimate as number);
  } catch (e) {
    console.error(e);
  }

  return feeEstimate;
}

export async function sendSingleOptimizedTransaction(
  umi: Umi,
  connection: Connection,
  tx: TransactionBuilder,
  txType?: TransactionRunType,
  attemptNum?: number,
  prioritySetting: PriorityFeeSetting = PriorityFeeSetting.Default,
  onAwaitingSign?: () => void
): Promise<Uint8Array | undefined> {
  consoleLog("Sending single optimized transaction...");
  consoleLog("Instructions: ", tx.getInstructions().length);
  consoleLog("Serialized transaction size: ", tx.getTransactionSize(umi));

  let cuPrice = await getComputeUnitPriceEstimate(
    umi,
    tx,
    prioritySetting
  );
  if (!cuPrice) {
    cuPrice = 1000000;
  }
  consoleLog("Compute unit price: ", cuPrice);

  let computeUnitLimit = undefined;
  if (txType !== "skip-simulation") {
    // TODO: we should only retry simulation if it's not a solauto error
    const simulationResult = await retryWithExponentialBackoff(
      async () =>
        await simulateTransaction(
          connection,
          toWeb3JsTransaction(
            await (
              await assembleFinalTransaction(
                umi.identity,
                tx,
                cuPrice,
                1_400_000
              ).setLatestBlockhash(umi)
            ).build(umi)
          )
        ),
      3
    );

    computeUnitLimit = Math.round(
      simulationResult.value.unitsConsumed! * 1.1
    );
    consoleLog("Compute unit limit: ", computeUnitLimit);
  }

  if (txType !== "only-simulate") {
    onAwaitingSign?.();
    const result = await assembleFinalTransaction(
      umi.identity,
      tx,
      cuPrice,
      computeUnitLimit
    ).sendAndConfirm(umi, {
      send: {
        skipPreflight: true,
        commitment: "confirmed",
      },
      confirm: { commitment: "confirmed" },
    });
    const txSig = bs58.encode(result.signature);
    consoleLog(`Transaction signature: ${txSig}`);
    consoleLog(`https://solscan.io/tx/${txSig}`);
    if (result.result.value.err !== null) {
      throw new Error(result.result.value.err.toString());
    }
    return result.signature;
  }

  return undefined;
}
