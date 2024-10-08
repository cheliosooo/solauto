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
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  mapSerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type PerpsV2AddLiquidityInstructionAccounts = {
  swapProgram: PublicKey | Pda;
  owner: PublicKey | Pda;
  fundingOrReceivingAccount: PublicKey | Pda;
  lpTokenAccount: PublicKey | Pda;
  transferAuthority: PublicKey | Pda;
  perpetuals: PublicKey | Pda;
  pool: PublicKey | Pda;
  custody: PublicKey | Pda;
  custodyDovesPriceAccount: PublicKey | Pda;
  custodyPythnetPriceAccount: PublicKey | Pda;
  custodyTokenAccount: PublicKey | Pda;
  lpTokenMint: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  eventAuthority: PublicKey | Pda;
  program: PublicKey | Pda;
};

// Data.
export type PerpsV2AddLiquidityInstructionData = {
  discriminator: Array<number>;
};

export type PerpsV2AddLiquidityInstructionDataArgs = {};

export function getPerpsV2AddLiquidityInstructionDataSerializer(): Serializer<
  PerpsV2AddLiquidityInstructionDataArgs,
  PerpsV2AddLiquidityInstructionData
> {
  return mapSerializer<
    PerpsV2AddLiquidityInstructionDataArgs,
    any,
    PerpsV2AddLiquidityInstructionData
  >(
    struct<PerpsV2AddLiquidityInstructionData>(
      [['discriminator', array(u8(), { size: 8 })]],
      { description: 'PerpsV2AddLiquidityInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [18, 66, 88, 194, 197, 52, 116, 212],
    })
  ) as Serializer<
    PerpsV2AddLiquidityInstructionDataArgs,
    PerpsV2AddLiquidityInstructionData
  >;
}

// Instruction.
export function perpsV2AddLiquidity(
  context: Pick<Context, 'programs'>,
  input: PerpsV2AddLiquidityInstructionAccounts
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'jupiter',
    'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4'
  );

  // Accounts.
  const resolvedAccounts = {
    swapProgram: {
      index: 0,
      isWritable: false as boolean,
      value: input.swapProgram ?? null,
    },
    owner: {
      index: 1,
      isWritable: false as boolean,
      value: input.owner ?? null,
    },
    fundingOrReceivingAccount: {
      index: 2,
      isWritable: true as boolean,
      value: input.fundingOrReceivingAccount ?? null,
    },
    lpTokenAccount: {
      index: 3,
      isWritable: true as boolean,
      value: input.lpTokenAccount ?? null,
    },
    transferAuthority: {
      index: 4,
      isWritable: false as boolean,
      value: input.transferAuthority ?? null,
    },
    perpetuals: {
      index: 5,
      isWritable: false as boolean,
      value: input.perpetuals ?? null,
    },
    pool: { index: 6, isWritable: true as boolean, value: input.pool ?? null },
    custody: {
      index: 7,
      isWritable: true as boolean,
      value: input.custody ?? null,
    },
    custodyDovesPriceAccount: {
      index: 8,
      isWritable: false as boolean,
      value: input.custodyDovesPriceAccount ?? null,
    },
    custodyPythnetPriceAccount: {
      index: 9,
      isWritable: false as boolean,
      value: input.custodyPythnetPriceAccount ?? null,
    },
    custodyTokenAccount: {
      index: 10,
      isWritable: true as boolean,
      value: input.custodyTokenAccount ?? null,
    },
    lpTokenMint: {
      index: 11,
      isWritable: true as boolean,
      value: input.lpTokenMint ?? null,
    },
    tokenProgram: {
      index: 12,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    eventAuthority: {
      index: 13,
      isWritable: false as boolean,
      value: input.eventAuthority ?? null,
    },
    program: {
      index: 14,
      isWritable: false as boolean,
      value: input.program ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Default values.
  if (!resolvedAccounts.tokenProgram.value) {
    resolvedAccounts.tokenProgram.value = context.programs.getPublicKey(
      'splToken',
      'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'
    );
    resolvedAccounts.tokenProgram.isWritable = false;
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
  const data = getPerpsV2AddLiquidityInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
