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
export type StepSwapInstructionAccounts = {
  tokenSwapProgram: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  swap: PublicKey | Pda;
  authority?: PublicKey | Pda;
  userTransferAuthority: PublicKey | Pda;
  source: PublicKey | Pda;
  swapSource: PublicKey | Pda;
  swapDestination: PublicKey | Pda;
  destination: PublicKey | Pda;
  poolMint: PublicKey | Pda;
  poolFee: PublicKey | Pda;
};

// Data.
export type StepSwapInstructionData = { discriminator: Array<number> };

export type StepSwapInstructionDataArgs = {};

export function getStepSwapInstructionDataSerializer(): Serializer<
  StepSwapInstructionDataArgs,
  StepSwapInstructionData
> {
  return mapSerializer<
    StepSwapInstructionDataArgs,
    any,
    StepSwapInstructionData
  >(
    struct<StepSwapInstructionData>(
      [['discriminator', array(u8(), { size: 8 })]],
      { description: 'StepSwapInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [155, 56, 208, 198, 27, 61, 149, 233],
    })
  ) as Serializer<StepSwapInstructionDataArgs, StepSwapInstructionData>;
}

// Instruction.
export function stepSwap(
  context: Pick<Context, 'identity' | 'programs'>,
  input: StepSwapInstructionAccounts
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'jupiter',
    'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4'
  );

  // Accounts.
  const resolvedAccounts = {
    tokenSwapProgram: {
      index: 0,
      isWritable: false as boolean,
      value: input.tokenSwapProgram ?? null,
    },
    tokenProgram: {
      index: 1,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    swap: { index: 2, isWritable: false as boolean, value: input.swap ?? null },
    authority: {
      index: 3,
      isWritable: false as boolean,
      value: input.authority ?? null,
    },
    userTransferAuthority: {
      index: 4,
      isWritable: false as boolean,
      value: input.userTransferAuthority ?? null,
    },
    source: {
      index: 5,
      isWritable: true as boolean,
      value: input.source ?? null,
    },
    swapSource: {
      index: 6,
      isWritable: true as boolean,
      value: input.swapSource ?? null,
    },
    swapDestination: {
      index: 7,
      isWritable: true as boolean,
      value: input.swapDestination ?? null,
    },
    destination: {
      index: 8,
      isWritable: true as boolean,
      value: input.destination ?? null,
    },
    poolMint: {
      index: 9,
      isWritable: true as boolean,
      value: input.poolMint ?? null,
    },
    poolFee: {
      index: 10,
      isWritable: true as boolean,
      value: input.poolFee ?? null,
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
  if (!resolvedAccounts.authority.value) {
    resolvedAccounts.authority.value = context.identity.publicKey;
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
  const data = getStepSwapInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
