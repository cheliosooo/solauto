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
export type SenchaSwapInstructionAccounts = {
  swapProgram: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  swap: PublicKey | Pda;
  userAuthority: PublicKey | Pda;
  inputUserAccount: PublicKey | Pda;
  inputTokenAccount: PublicKey | Pda;
  inputFeesAccount: PublicKey | Pda;
  outputUserAccount: PublicKey | Pda;
  outputTokenAccount: PublicKey | Pda;
  outputFeesAccount: PublicKey | Pda;
};

// Data.
export type SenchaSwapInstructionData = { discriminator: Array<number> };

export type SenchaSwapInstructionDataArgs = {};

export function getSenchaSwapInstructionDataSerializer(): Serializer<
  SenchaSwapInstructionDataArgs,
  SenchaSwapInstructionData
> {
  return mapSerializer<
    SenchaSwapInstructionDataArgs,
    any,
    SenchaSwapInstructionData
  >(
    struct<SenchaSwapInstructionData>(
      [['discriminator', array(u8(), { size: 8 })]],
      { description: 'SenchaSwapInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [25, 50, 7, 21, 207, 248, 230, 194],
    })
  ) as Serializer<SenchaSwapInstructionDataArgs, SenchaSwapInstructionData>;
}

// Instruction.
export function senchaSwap(
  context: Pick<Context, 'programs'>,
  input: SenchaSwapInstructionAccounts
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
    swap: { index: 2, isWritable: true as boolean, value: input.swap ?? null },
    userAuthority: {
      index: 3,
      isWritable: false as boolean,
      value: input.userAuthority ?? null,
    },
    inputUserAccount: {
      index: 4,
      isWritable: true as boolean,
      value: input.inputUserAccount ?? null,
    },
    inputTokenAccount: {
      index: 5,
      isWritable: true as boolean,
      value: input.inputTokenAccount ?? null,
    },
    inputFeesAccount: {
      index: 6,
      isWritable: true as boolean,
      value: input.inputFeesAccount ?? null,
    },
    outputUserAccount: {
      index: 7,
      isWritable: true as boolean,
      value: input.outputUserAccount ?? null,
    },
    outputTokenAccount: {
      index: 8,
      isWritable: true as boolean,
      value: input.outputTokenAccount ?? null,
    },
    outputFeesAccount: {
      index: 9,
      isWritable: true as boolean,
      value: input.outputFeesAccount ?? null,
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
  const data = getSenchaSwapInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
