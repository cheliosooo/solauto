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
export type ClaimInstructionAccounts = {
  wallet: PublicKey | Pda;
  programAuthority: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
};

// Data.
export type ClaimInstructionData = { discriminator: Array<number>; id: number };

export type ClaimInstructionDataArgs = { id: number };

export function getClaimInstructionDataSerializer(): Serializer<
  ClaimInstructionDataArgs,
  ClaimInstructionData
> {
  return mapSerializer<ClaimInstructionDataArgs, any, ClaimInstructionData>(
    struct<ClaimInstructionData>(
      [
        ['discriminator', array(u8(), { size: 8 })],
        ['id', u8()],
      ],
      { description: 'ClaimInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [62, 198, 214, 193, 213, 159, 108, 210],
    })
  ) as Serializer<ClaimInstructionDataArgs, ClaimInstructionData>;
}

// Args.
export type ClaimInstructionArgs = ClaimInstructionDataArgs;

// Instruction.
export function claim(
  context: Pick<Context, 'programs'>,
  input: ClaimInstructionAccounts & ClaimInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'jupiter',
    'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4'
  );

  // Accounts.
  const resolvedAccounts = {
    wallet: {
      index: 0,
      isWritable: true as boolean,
      value: input.wallet ?? null,
    },
    programAuthority: {
      index: 1,
      isWritable: true as boolean,
      value: input.programAuthority ?? null,
    },
    systemProgram: {
      index: 2,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: ClaimInstructionArgs = { ...input };

  // Default values.
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
  const data = getClaimInstructionDataSerializer().serialize(
    resolvedArgs as ClaimInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
