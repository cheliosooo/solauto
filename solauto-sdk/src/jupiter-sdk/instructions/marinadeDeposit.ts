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
export type MarinadeDepositInstructionAccounts = {
  marinadeFinanceProgram: PublicKey | Pda;
  state: PublicKey | Pda;
  msolMint: PublicKey | Pda;
  liqPoolSolLegPda: PublicKey | Pda;
  liqPoolMsolLeg: PublicKey | Pda;
  liqPoolMsolLegAuthority: PublicKey | Pda;
  reservePda: PublicKey | Pda;
  transferFrom: PublicKey | Pda;
  mintTo: PublicKey | Pda;
  msolMintAuthority: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  userWsolTokenAccount: PublicKey | Pda;
  tempWsolTokenAccount: PublicKey | Pda;
  userTransferAuthority: PublicKey | Pda;
  payer?: PublicKey | Pda;
  wsolMint: PublicKey | Pda;
  rent?: PublicKey | Pda;
};

// Data.
export type MarinadeDepositInstructionData = { discriminator: Array<number> };

export type MarinadeDepositInstructionDataArgs = {};

export function getMarinadeDepositInstructionDataSerializer(): Serializer<
  MarinadeDepositInstructionDataArgs,
  MarinadeDepositInstructionData
> {
  return mapSerializer<
    MarinadeDepositInstructionDataArgs,
    any,
    MarinadeDepositInstructionData
  >(
    struct<MarinadeDepositInstructionData>(
      [['discriminator', array(u8(), { size: 8 })]],
      { description: 'MarinadeDepositInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [62, 236, 248, 28, 222, 232, 182, 73],
    })
  ) as Serializer<
    MarinadeDepositInstructionDataArgs,
    MarinadeDepositInstructionData
  >;
}

// Instruction.
export function marinadeDeposit(
  context: Pick<Context, 'payer' | 'programs'>,
  input: MarinadeDepositInstructionAccounts
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'jupiter',
    'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4'
  );

  // Accounts.
  const resolvedAccounts = {
    marinadeFinanceProgram: {
      index: 0,
      isWritable: false as boolean,
      value: input.marinadeFinanceProgram ?? null,
    },
    state: {
      index: 1,
      isWritable: true as boolean,
      value: input.state ?? null,
    },
    msolMint: {
      index: 2,
      isWritable: true as boolean,
      value: input.msolMint ?? null,
    },
    liqPoolSolLegPda: {
      index: 3,
      isWritable: true as boolean,
      value: input.liqPoolSolLegPda ?? null,
    },
    liqPoolMsolLeg: {
      index: 4,
      isWritable: true as boolean,
      value: input.liqPoolMsolLeg ?? null,
    },
    liqPoolMsolLegAuthority: {
      index: 5,
      isWritable: false as boolean,
      value: input.liqPoolMsolLegAuthority ?? null,
    },
    reservePda: {
      index: 6,
      isWritable: true as boolean,
      value: input.reservePda ?? null,
    },
    transferFrom: {
      index: 7,
      isWritable: true as boolean,
      value: input.transferFrom ?? null,
    },
    mintTo: {
      index: 8,
      isWritable: true as boolean,
      value: input.mintTo ?? null,
    },
    msolMintAuthority: {
      index: 9,
      isWritable: false as boolean,
      value: input.msolMintAuthority ?? null,
    },
    systemProgram: {
      index: 10,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    tokenProgram: {
      index: 11,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    userWsolTokenAccount: {
      index: 12,
      isWritable: true as boolean,
      value: input.userWsolTokenAccount ?? null,
    },
    tempWsolTokenAccount: {
      index: 13,
      isWritable: true as boolean,
      value: input.tempWsolTokenAccount ?? null,
    },
    userTransferAuthority: {
      index: 14,
      isWritable: false as boolean,
      value: input.userTransferAuthority ?? null,
    },
    payer: {
      index: 15,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    wsolMint: {
      index: 16,
      isWritable: false as boolean,
      value: input.wsolMint ?? null,
    },
    rent: {
      index: 17,
      isWritable: false as boolean,
      value: input.rent ?? null,
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
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer.publicKey;
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
  const data = getMarinadeDepositInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
