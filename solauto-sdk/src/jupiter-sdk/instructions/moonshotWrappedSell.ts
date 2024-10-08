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
export type MoonshotWrappedSellInstructionAccounts = {
  swapProgram: PublicKey | Pda;
  sender: PublicKey | Pda;
  senderTokenAccount: PublicKey | Pda;
  curveAccount: PublicKey | Pda;
  curveTokenAccount: PublicKey | Pda;
  dexFee: PublicKey | Pda;
  helioFee: PublicKey | Pda;
  mint: PublicKey | Pda;
  configAccount: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  associatedTokenProgram: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
  userWsolTokenAccount: PublicKey | Pda;
};

// Data.
export type MoonshotWrappedSellInstructionData = {
  discriminator: Array<number>;
};

export type MoonshotWrappedSellInstructionDataArgs = {};

export function getMoonshotWrappedSellInstructionDataSerializer(): Serializer<
  MoonshotWrappedSellInstructionDataArgs,
  MoonshotWrappedSellInstructionData
> {
  return mapSerializer<
    MoonshotWrappedSellInstructionDataArgs,
    any,
    MoonshotWrappedSellInstructionData
  >(
    struct<MoonshotWrappedSellInstructionData>(
      [['discriminator', array(u8(), { size: 8 })]],
      { description: 'MoonshotWrappedSellInstructionData' }
    ),
    (value) => ({ ...value, discriminator: [248, 2, 240, 253, 17, 184, 57, 8] })
  ) as Serializer<
    MoonshotWrappedSellInstructionDataArgs,
    MoonshotWrappedSellInstructionData
  >;
}

// Instruction.
export function moonshotWrappedSell(
  context: Pick<Context, 'programs'>,
  input: MoonshotWrappedSellInstructionAccounts
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
    sender: {
      index: 1,
      isWritable: true as boolean,
      value: input.sender ?? null,
    },
    senderTokenAccount: {
      index: 2,
      isWritable: true as boolean,
      value: input.senderTokenAccount ?? null,
    },
    curveAccount: {
      index: 3,
      isWritable: true as boolean,
      value: input.curveAccount ?? null,
    },
    curveTokenAccount: {
      index: 4,
      isWritable: true as boolean,
      value: input.curveTokenAccount ?? null,
    },
    dexFee: {
      index: 5,
      isWritable: true as boolean,
      value: input.dexFee ?? null,
    },
    helioFee: {
      index: 6,
      isWritable: true as boolean,
      value: input.helioFee ?? null,
    },
    mint: { index: 7, isWritable: false as boolean, value: input.mint ?? null },
    configAccount: {
      index: 8,
      isWritable: false as boolean,
      value: input.configAccount ?? null,
    },
    tokenProgram: {
      index: 9,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    associatedTokenProgram: {
      index: 10,
      isWritable: false as boolean,
      value: input.associatedTokenProgram ?? null,
    },
    systemProgram: {
      index: 11,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    userWsolTokenAccount: {
      index: 12,
      isWritable: true as boolean,
      value: input.userWsolTokenAccount ?? null,
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
  const data = getMoonshotWrappedSellInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
