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
export type CreateTokenAccountInstructionAccounts = {
  tokenAccount: PublicKey | Pda;
  user: Signer;
  mint: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
};

// Data.
export type CreateTokenAccountInstructionData = {
  discriminator: Array<number>;
  bump: number;
};

export type CreateTokenAccountInstructionDataArgs = { bump: number };

export function getCreateTokenAccountInstructionDataSerializer(): Serializer<
  CreateTokenAccountInstructionDataArgs,
  CreateTokenAccountInstructionData
> {
  return mapSerializer<
    CreateTokenAccountInstructionDataArgs,
    any,
    CreateTokenAccountInstructionData
  >(
    struct<CreateTokenAccountInstructionData>(
      [
        ['discriminator', array(u8(), { size: 8 })],
        ['bump', u8()],
      ],
      { description: 'CreateTokenAccountInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [147, 241, 123, 100, 244, 132, 174, 118],
    })
  ) as Serializer<
    CreateTokenAccountInstructionDataArgs,
    CreateTokenAccountInstructionData
  >;
}

// Args.
export type CreateTokenAccountInstructionArgs =
  CreateTokenAccountInstructionDataArgs;

// Instruction.
export function createTokenAccount(
  context: Pick<Context, 'programs'>,
  input: CreateTokenAccountInstructionAccounts &
    CreateTokenAccountInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'jupiter',
    'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4'
  );

  // Accounts.
  const resolvedAccounts = {
    tokenAccount: {
      index: 0,
      isWritable: true as boolean,
      value: input.tokenAccount ?? null,
    },
    user: { index: 1, isWritable: true as boolean, value: input.user ?? null },
    mint: { index: 2, isWritable: false as boolean, value: input.mint ?? null },
    tokenProgram: {
      index: 3,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    systemProgram: {
      index: 4,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: CreateTokenAccountInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.tokenProgram.value) {
    resolvedAccounts.tokenProgram.value = context.programs.getPublicKey(
      'splToken',
      'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'
    );
    resolvedAccounts.tokenProgram.isWritable = false;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
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
  const data = getCreateTokenAccountInstructionDataSerializer().serialize(
    resolvedArgs as CreateTokenAccountInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
