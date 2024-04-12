/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Option,
  OptionOrNullable,
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
  option,
  struct,
  u16,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type SolendRebalanceInstructionAccounts = {
  signer: Signer;
  solendProgram: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  ataProgram?: PublicKey | Pda;
  clock: PublicKey | Pda;
  rent?: PublicKey | Pda;
  ixSysvar: PublicKey | Pda;
  solautoAdminSettings: PublicKey | Pda;
  solautoFeesReceiver: PublicKey | Pda;
  solautoPosition?: PublicKey | Pda;
  lendingMarket: PublicKey | Pda;
  obligation: PublicKey | Pda;
  supplyReserve?: PublicKey | Pda;
  supplyReservePythPriceOracle?: PublicKey | Pda;
  supplyReserveSwitchboardOracle?: PublicKey | Pda;
  supplyLiquidityTokenMint?: PublicKey | Pda;
  sourceSupplyLiquidity?: PublicKey | Pda;
  reserveSupplyLiquidity?: PublicKey | Pda;
  supplyCollateralTokenMint?: PublicKey | Pda;
  sourceSupplyCollateral?: PublicKey | Pda;
  reserveSupplyCollateral?: PublicKey | Pda;
  debtReserve?: PublicKey | Pda;
  debtReserveFeeReceiver?: PublicKey | Pda;
  debtLiquidityTokenMint?: PublicKey | Pda;
  sourceDebtLiquidity?: PublicKey | Pda;
  reserveDebtLiquidity?: PublicKey | Pda;
};

// Data.
export type SolendRebalanceInstructionData = {
  discriminator: number;
  args: Option<number>;
};

export type SolendRebalanceInstructionDataArgs = {
  args: OptionOrNullable<number>;
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
        ['args', option(u16())],
      ],
      { description: 'SolendRebalanceInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 9 })
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
    ixSysvar: {
      index: 7,
      isWritable: false as boolean,
      value: input.ixSysvar ?? null,
    },
    solautoAdminSettings: {
      index: 8,
      isWritable: false as boolean,
      value: input.solautoAdminSettings ?? null,
    },
    solautoFeesReceiver: {
      index: 9,
      isWritable: true as boolean,
      value: input.solautoFeesReceiver ?? null,
    },
    solautoPosition: {
      index: 10,
      isWritable: true as boolean,
      value: input.solautoPosition ?? null,
    },
    lendingMarket: {
      index: 11,
      isWritable: false as boolean,
      value: input.lendingMarket ?? null,
    },
    obligation: {
      index: 12,
      isWritable: true as boolean,
      value: input.obligation ?? null,
    },
    supplyReserve: {
      index: 13,
      isWritable: true as boolean,
      value: input.supplyReserve ?? null,
    },
    supplyReservePythPriceOracle: {
      index: 14,
      isWritable: false as boolean,
      value: input.supplyReservePythPriceOracle ?? null,
    },
    supplyReserveSwitchboardOracle: {
      index: 15,
      isWritable: false as boolean,
      value: input.supplyReserveSwitchboardOracle ?? null,
    },
    supplyLiquidityTokenMint: {
      index: 16,
      isWritable: false as boolean,
      value: input.supplyLiquidityTokenMint ?? null,
    },
    sourceSupplyLiquidity: {
      index: 17,
      isWritable: true as boolean,
      value: input.sourceSupplyLiquidity ?? null,
    },
    reserveSupplyLiquidity: {
      index: 18,
      isWritable: true as boolean,
      value: input.reserveSupplyLiquidity ?? null,
    },
    supplyCollateralTokenMint: {
      index: 19,
      isWritable: false as boolean,
      value: input.supplyCollateralTokenMint ?? null,
    },
    sourceSupplyCollateral: {
      index: 20,
      isWritable: true as boolean,
      value: input.sourceSupplyCollateral ?? null,
    },
    reserveSupplyCollateral: {
      index: 21,
      isWritable: true as boolean,
      value: input.reserveSupplyCollateral ?? null,
    },
    debtReserve: {
      index: 22,
      isWritable: true as boolean,
      value: input.debtReserve ?? null,
    },
    debtReserveFeeReceiver: {
      index: 23,
      isWritable: true as boolean,
      value: input.debtReserveFeeReceiver ?? null,
    },
    debtLiquidityTokenMint: {
      index: 24,
      isWritable: false as boolean,
      value: input.debtLiquidityTokenMint ?? null,
    },
    sourceDebtLiquidity: {
      index: 25,
      isWritable: true as boolean,
      value: input.sourceDebtLiquidity ?? null,
    },
    reserveDebtLiquidity: {
      index: 26,
      isWritable: true as boolean,
      value: input.reserveDebtLiquidity ?? null,
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