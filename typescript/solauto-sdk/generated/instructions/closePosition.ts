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

// Accounts.
export type ClosePositionInstructionAccounts = {
  signer: Signer;
  systemProgram?: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  ataProgram?: PublicKey | Pda;
  solautoPosition: PublicKey | Pda;
  signerSupplyLiquidityTa: PublicKey | Pda;
  positionSupplyLiquidityTa: PublicKey | Pda;
  positionSupplyCollateralTa?: PublicKey | Pda;
  positionDebtLiquidityTa?: PublicKey | Pda;
};

// Data.
export type ClosePositionInstructionData = { discriminator: number };

export type ClosePositionInstructionDataArgs = {};

export function getClosePositionInstructionDataSerializer(): Serializer<
  ClosePositionInstructionDataArgs,
  ClosePositionInstructionData
> {
  return mapSerializer<
    ClosePositionInstructionDataArgs,
    any,
    ClosePositionInstructionData
  >(
    struct<ClosePositionInstructionData>([['discriminator', u8()]], {
      description: 'ClosePositionInstructionData',
    }),
    (value) => ({ ...value, discriminator: 6 })
  ) as Serializer<
    ClosePositionInstructionDataArgs,
    ClosePositionInstructionData
  >;
}

// Instruction.
export function closePosition(
  context: Pick<Context, 'programs'>,
  input: ClosePositionInstructionAccounts
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
    systemProgram: {
      index: 1,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    tokenProgram: {
      index: 2,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    ataProgram: {
      index: 3,
      isWritable: false as boolean,
      value: input.ataProgram ?? null,
    },
    solautoPosition: {
      index: 4,
      isWritable: true as boolean,
      value: input.solautoPosition ?? null,
    },
    signerSupplyLiquidityTa: {
      index: 5,
      isWritable: true as boolean,
      value: input.signerSupplyLiquidityTa ?? null,
    },
    positionSupplyLiquidityTa: {
      index: 6,
      isWritable: true as boolean,
      value: input.positionSupplyLiquidityTa ?? null,
    },
    positionSupplyCollateralTa: {
      index: 7,
      isWritable: true as boolean,
      value: input.positionSupplyCollateralTa ?? null,
    },
    positionDebtLiquidityTa: {
      index: 8,
      isWritable: true as boolean,
      value: input.positionDebtLiquidityTa ?? null,
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
  if (!resolvedAccounts.ataProgram.value) {
    resolvedAccounts.ataProgram.value = context.programs.getPublicKey(
      'splAssociatedToken',
      'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
    );
    resolvedAccounts.ataProgram.isWritable = false;
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
  const data = getClosePositionInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
