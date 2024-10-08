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
export type WhirlpoolSwapInstructionAccounts = {
  swapProgram: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  tokenAuthority: PublicKey | Pda;
  whirlpool: PublicKey | Pda;
  tokenOwnerAccountA: PublicKey | Pda;
  tokenVaultA: PublicKey | Pda;
  tokenOwnerAccountB: PublicKey | Pda;
  tokenVaultB: PublicKey | Pda;
  tickArray0: PublicKey | Pda;
  tickArray1: PublicKey | Pda;
  tickArray2: PublicKey | Pda;
  /** Oracle is currently unused and will be enabled on subsequent updates */
  oracle: PublicKey | Pda;
};

// Data.
export type WhirlpoolSwapInstructionData = { discriminator: Array<number> };

export type WhirlpoolSwapInstructionDataArgs = {};

export function getWhirlpoolSwapInstructionDataSerializer(): Serializer<
  WhirlpoolSwapInstructionDataArgs,
  WhirlpoolSwapInstructionData
> {
  return mapSerializer<
    WhirlpoolSwapInstructionDataArgs,
    any,
    WhirlpoolSwapInstructionData
  >(
    struct<WhirlpoolSwapInstructionData>(
      [['discriminator', array(u8(), { size: 8 })]],
      { description: 'WhirlpoolSwapInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [123, 229, 184, 63, 12, 0, 92, 145],
    })
  ) as Serializer<
    WhirlpoolSwapInstructionDataArgs,
    WhirlpoolSwapInstructionData
  >;
}

// Instruction.
export function whirlpoolSwap(
  context: Pick<Context, 'programs'>,
  input: WhirlpoolSwapInstructionAccounts
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
    tokenProgram: {
      index: 1,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    tokenAuthority: {
      index: 2,
      isWritable: false as boolean,
      value: input.tokenAuthority ?? null,
    },
    whirlpool: {
      index: 3,
      isWritable: true as boolean,
      value: input.whirlpool ?? null,
    },
    tokenOwnerAccountA: {
      index: 4,
      isWritable: true as boolean,
      value: input.tokenOwnerAccountA ?? null,
    },
    tokenVaultA: {
      index: 5,
      isWritable: true as boolean,
      value: input.tokenVaultA ?? null,
    },
    tokenOwnerAccountB: {
      index: 6,
      isWritable: true as boolean,
      value: input.tokenOwnerAccountB ?? null,
    },
    tokenVaultB: {
      index: 7,
      isWritable: true as boolean,
      value: input.tokenVaultB ?? null,
    },
    tickArray0: {
      index: 8,
      isWritable: true as boolean,
      value: input.tickArray0 ?? null,
    },
    tickArray1: {
      index: 9,
      isWritable: true as boolean,
      value: input.tickArray1 ?? null,
    },
    tickArray2: {
      index: 10,
      isWritable: true as boolean,
      value: input.tickArray2 ?? null,
    },
    oracle: {
      index: 11,
      isWritable: false as boolean,
      value: input.oracle ?? null,
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
  const data = getWhirlpoolSwapInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
