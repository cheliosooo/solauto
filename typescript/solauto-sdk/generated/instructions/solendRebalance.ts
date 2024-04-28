/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  publicKey,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';
import {
  RebalanceArgs,
  RebalanceArgsArgs,
  getRebalanceArgsSerializer,
} from '../types';

// Accounts.
export type SolendRebalanceInstructionAccounts = {
  signer: Signer;
  solendProgram: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  ataProgram?: PublicKey | Pda;
  clock: PublicKey | Pda;
  rent?: PublicKey | Pda;
  ixsSysvar: PublicKey | Pda;
  solautoFeesSupplyTa: PublicKey | Pda;
  authorityReferralState: PublicKey | Pda;
  referredBySupplyTa?: PublicKey | Pda;
  solautoPosition: PublicKey | Pda;
  lendingMarket: PublicKey | Pda;
  obligation: PublicKey | Pda;
  intermediaryTa: PublicKey | Pda;
  supplyReserve: PublicKey | Pda;
  supplyReservePythPriceOracle: PublicKey | Pda;
  supplyReserveSwitchboardOracle: PublicKey | Pda;
  positionSupplyLiquidityTa: PublicKey | Pda;
  reserveSupplyLiquidityTa: PublicKey | Pda;
  supplyCollateralMint: PublicKey | Pda;
  positionSupplyCollateralTa: PublicKey | Pda;
  reserveSupplyCollateralTa: PublicKey | Pda;
  debtReserve: PublicKey | Pda;
  debtReserveFeeReceiverTa: PublicKey | Pda;
  positionDebtLiquidityTa: PublicKey | Pda;
  reserveDebtLiquidityTa: PublicKey | Pda;
};

// Data.
export type SolendRebalanceInstructionData = {
  discriminator: number;
  rebalanceArgs: RebalanceArgs;
};

export type SolendRebalanceInstructionDataArgs = {
  rebalanceArgs: RebalanceArgsArgs;
};

export function getSolendRebalanceInstructionDataSerializer(): Serializer<
  SolendRebalanceInstructionDataArgs,
  SolendRebalanceInstructionData
> {
  return mapSerializer<
    SolendRebalanceInstructionDataArgs,
    any,
    SolendRebalanceInstructionData
  >(
    struct<SolendRebalanceInstructionData>(
      [
        ['discriminator', u8()],
        ['rebalanceArgs', getRebalanceArgsSerializer()],
      ],
      { description: 'SolendRebalanceInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 12 })
  ) as Serializer<
    SolendRebalanceInstructionDataArgs,
    SolendRebalanceInstructionData
  >;
}

// Args.
export type SolendRebalanceInstructionArgs = SolendRebalanceInstructionDataArgs;

// Instruction.
export function solendRebalance(
  context: Pick<Context, 'programs'>,
  input: SolendRebalanceInstructionAccounts & SolendRebalanceInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'solauto',
    'AutoyKBRaHSBHy9RsmXCZMy6nNFAg5FYijrvZyQcNLV'
  );

  // Accounts.
  const resolvedAccounts = {
    signer: {
      index: 0,
      isWritable: true as boolean,
      value: input.signer ?? null,
    },
    solendProgram: {
      index: 1,
      isWritable: false as boolean,
      value: input.solendProgram ?? null,
    },
    systemProgram: {
      index: 2,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    tokenProgram: {
      index: 3,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    ataProgram: {
      index: 4,
      isWritable: false as boolean,
      value: input.ataProgram ?? null,
    },
    clock: {
      index: 5,
      isWritable: false as boolean,
      value: input.clock ?? null,
    },
    rent: { index: 6, isWritable: false as boolean, value: input.rent ?? null },
    ixsSysvar: {
      index: 7,
      isWritable: false as boolean,
      value: input.ixsSysvar ?? null,
    },
    solautoFeesSupplyTa: {
      index: 8,
      isWritable: true as boolean,
      value: input.solautoFeesSupplyTa ?? null,
    },
    authorityReferralState: {
      index: 9,
      isWritable: false as boolean,
      value: input.authorityReferralState ?? null,
    },
    referredBySupplyTa: {
      index: 10,
      isWritable: true as boolean,
      value: input.referredBySupplyTa ?? null,
    },
    solautoPosition: {
      index: 11,
      isWritable: true as boolean,
      value: input.solautoPosition ?? null,
    },
    lendingMarket: {
      index: 12,
      isWritable: false as boolean,
      value: input.lendingMarket ?? null,
    },
    obligation: {
      index: 13,
      isWritable: true as boolean,
      value: input.obligation ?? null,
    },
    intermediaryTa: {
      index: 14,
      isWritable: true as boolean,
      value: input.intermediaryTa ?? null,
    },
    supplyReserve: {
      index: 15,
      isWritable: true as boolean,
      value: input.supplyReserve ?? null,
    },
    supplyReservePythPriceOracle: {
      index: 16,
      isWritable: false as boolean,
      value: input.supplyReservePythPriceOracle ?? null,
    },
    supplyReserveSwitchboardOracle: {
      index: 17,
      isWritable: false as boolean,
      value: input.supplyReserveSwitchboardOracle ?? null,
    },
    positionSupplyLiquidityTa: {
      index: 18,
      isWritable: true as boolean,
      value: input.positionSupplyLiquidityTa ?? null,
    },
    reserveSupplyLiquidityTa: {
      index: 19,
      isWritable: true as boolean,
      value: input.reserveSupplyLiquidityTa ?? null,
    },
    supplyCollateralMint: {
      index: 20,
      isWritable: false as boolean,
      value: input.supplyCollateralMint ?? null,
    },
    positionSupplyCollateralTa: {
      index: 21,
      isWritable: true as boolean,
      value: input.positionSupplyCollateralTa ?? null,
    },
    reserveSupplyCollateralTa: {
      index: 22,
      isWritable: true as boolean,
      value: input.reserveSupplyCollateralTa ?? null,
    },
    debtReserve: {
      index: 23,
      isWritable: true as boolean,
      value: input.debtReserve ?? null,
    },
    debtReserveFeeReceiverTa: {
      index: 24,
      isWritable: true as boolean,
      value: input.debtReserveFeeReceiverTa ?? null,
    },
    positionDebtLiquidityTa: {
      index: 25,
      isWritable: true as boolean,
      value: input.positionDebtLiquidityTa ?? null,
    },
    reserveDebtLiquidityTa: {
      index: 26,
      isWritable: true as boolean,
      value: input.reserveDebtLiquidityTa ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: SolendRebalanceInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }
  if (!resolvedAccounts.tokenProgram.value) {
    resolvedAccounts.tokenProgram.value = context.programs.getPublicKey(
      'splToken',
      'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'
    );
    resolvedAccounts.tokenProgram.isWritable = false;
  }
  if (!resolvedAccounts.ataProgram.value) {
    resolvedAccounts.ataProgram.value = context.programs.getPublicKey(
      'splAssociatedToken',
      'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
    );
    resolvedAccounts.ataProgram.isWritable = false;
  }
  if (!resolvedAccounts.rent.value) {
    resolvedAccounts.rent.value = publicKey(
      'SysvarRent111111111111111111111111111111111'
    );
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getSolendRebalanceInstructionDataSerializer().serialize(
    resolvedArgs as SolendRebalanceInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
