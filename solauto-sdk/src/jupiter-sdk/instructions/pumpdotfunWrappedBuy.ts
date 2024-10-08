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
  publicKey,
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
export type PumpdotfunWrappedBuyInstructionAccounts = {
  swapProgram: PublicKey | Pda;
  global: PublicKey | Pda;
  feeRecipient: PublicKey | Pda;
  mint: PublicKey | Pda;
  bondingCurve: PublicKey | Pda;
  associatedBondingCurve: PublicKey | Pda;
  associatedUser: PublicKey | Pda;
  user: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  rent?: PublicKey | Pda;
  eventAuthority: PublicKey | Pda;
  program: PublicKey | Pda;
  userWsolTokenAccount: PublicKey | Pda;
  tempWsolTokenAccount: PublicKey | Pda;
  wsolMint: PublicKey | Pda;
};

// Data.
export type PumpdotfunWrappedBuyInstructionData = {
  discriminator: Array<number>;
};

export type PumpdotfunWrappedBuyInstructionDataArgs = {};

export function getPumpdotfunWrappedBuyInstructionDataSerializer(): Serializer<
  PumpdotfunWrappedBuyInstructionDataArgs,
  PumpdotfunWrappedBuyInstructionData
> {
  return mapSerializer<
    PumpdotfunWrappedBuyInstructionDataArgs,
    any,
    PumpdotfunWrappedBuyInstructionData
  >(
    struct<PumpdotfunWrappedBuyInstructionData>(
      [['discriminator', array(u8(), { size: 8 })]],
      { description: 'PumpdotfunWrappedBuyInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [138, 139, 167, 134, 208, 91, 138, 158],
    })
  ) as Serializer<
    PumpdotfunWrappedBuyInstructionDataArgs,
    PumpdotfunWrappedBuyInstructionData
  >;
}

// Instruction.
export function pumpdotfunWrappedBuy(
  context: Pick<Context, 'programs'>,
  input: PumpdotfunWrappedBuyInstructionAccounts
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
    global: {
      index: 1,
      isWritable: false as boolean,
      value: input.global ?? null,
    },
    feeRecipient: {
      index: 2,
      isWritable: true as boolean,
      value: input.feeRecipient ?? null,
    },
    mint: { index: 3, isWritable: false as boolean, value: input.mint ?? null },
    bondingCurve: {
      index: 4,
      isWritable: true as boolean,
      value: input.bondingCurve ?? null,
    },
    associatedBondingCurve: {
      index: 5,
      isWritable: true as boolean,
      value: input.associatedBondingCurve ?? null,
    },
    associatedUser: {
      index: 6,
      isWritable: true as boolean,
      value: input.associatedUser ?? null,
    },
    user: { index: 7, isWritable: true as boolean, value: input.user ?? null },
    systemProgram: {
      index: 8,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    tokenProgram: {
      index: 9,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    rent: {
      index: 10,
      isWritable: false as boolean,
      value: input.rent ?? null,
    },
    eventAuthority: {
      index: 11,
      isWritable: false as boolean,
      value: input.eventAuthority ?? null,
    },
    program: {
      index: 12,
      isWritable: false as boolean,
      value: input.program ?? null,
    },
    userWsolTokenAccount: {
      index: 13,
      isWritable: true as boolean,
      value: input.userWsolTokenAccount ?? null,
    },
    tempWsolTokenAccount: {
      index: 14,
      isWritable: true as boolean,
      value: input.tempWsolTokenAccount ?? null,
    },
    wsolMint: {
      index: 15,
      isWritable: false as boolean,
      value: input.wsolMint ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

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
  const data = getPumpdotfunWrappedBuyInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
