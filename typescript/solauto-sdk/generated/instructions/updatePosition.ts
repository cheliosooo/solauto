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
  UpdatePositionData,
  UpdatePositionDataArgs,
  getUpdatePositionDataSerializer,
} from '../types';

// Accounts.
export type UpdatePositionInstructionAccounts = {
  signer: Signer;
  tokenProgram?: PublicKey | Pda;
  solautoPosition: PublicKey | Pda;
  positionDebtTa?: PublicKey | Pda;
  signerDebtTa?: PublicKey | Pda;
  debtMint?: PublicKey | Pda;
};

// Data.
export type UpdatePositionInstructionData = {
  discriminator: number;
  updatePositionData: UpdatePositionData;
};

export type UpdatePositionInstructionDataArgs = {
  updatePositionData: UpdatePositionDataArgs;
};

export function getUpdatePositionInstructionDataSerializer(): Serializer<
  UpdatePositionInstructionDataArgs,
  UpdatePositionInstructionData
> {
  return mapSerializer<
    UpdatePositionInstructionDataArgs,
    any,
    UpdatePositionInstructionData
  >(
    struct<UpdatePositionInstructionData>(
      [
        ['discriminator', u8()],
        ['updatePositionData', getUpdatePositionDataSerializer()],
      ],
      { description: 'UpdatePositionInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 5 })
  ) as Serializer<
    UpdatePositionInstructionDataArgs,
    UpdatePositionInstructionData
  >;
}

// Args.
export type UpdatePositionInstructionArgs = UpdatePositionInstructionDataArgs;

// Instruction.
export function updatePosition(
  context: Pick<Context, 'programs'>,
  input: UpdatePositionInstructionAccounts & UpdatePositionInstructionArgs
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
      isWritable: false as boolean,
      value: input.signer ?? null,
    },
    tokenProgram: {
      index: 1,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    solautoPosition: {
      index: 2,
      isWritable: true as boolean,
      value: input.solautoPosition ?? null,
    },
    positionDebtTa: {
      index: 3,
      isWritable: true as boolean,
      value: input.positionDebtTa ?? null,
    },
    signerDebtTa: {
      index: 4,
      isWritable: true as boolean,
      value: input.signerDebtTa ?? null,
    },
    debtMint: {
      index: 5,
      isWritable: false as boolean,
      value: input.debtMint ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: UpdatePositionInstructionArgs = { ...input };

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
  const data = getUpdatePositionInstructionDataSerializer().serialize(
    resolvedArgs as UpdatePositionInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
