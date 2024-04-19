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
  publicKey,
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
  RebalanceArgs,
  RebalanceArgsArgs,
  getRebalanceArgsSerializer,
} from '../types';

// Accounts.
export type MarginfiRebalanceInstructionAccounts = {
  signer: Signer;
  marginfiProgram: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  ataProgram?: PublicKey | Pda;
  rent?: PublicKey | Pda;
  ixsSysvar: PublicKey | Pda;
  solautoFeesSupplyTa: PublicKey | Pda;
  authorityReferralState: PublicKey | Pda;
  referredByState?: PublicKey | Pda;
  referredBySupplyTa?: PublicKey | Pda;
  solautoPosition: PublicKey | Pda;
  intermediaryTa: PublicKey | Pda;
  supplyMint: PublicKey | Pda;
  positionSupplyTa: PublicKey | Pda;
  bankSupplyTa: PublicKey | Pda;
  debtMint: PublicKey | Pda;
  positionDebtTa: PublicKey | Pda;
  bankDebtTa: PublicKey | Pda;
};

// Data.
export type MarginfiRebalanceInstructionData = {
  discriminator: number;
  rebalanceArgs: RebalanceArgs;
};

export type MarginfiRebalanceInstructionDataArgs = {
  rebalanceArgs: RebalanceArgsArgs;
};

export function getMarginfiRebalanceInstructionDataSerializer(): Serializer<
  MarginfiRebalanceInstructionDataArgs,
  MarginfiRebalanceInstructionData
> {
  return mapSerializer<
    MarginfiRebalanceInstructionDataArgs,
    any,
    MarginfiRebalanceInstructionData
  >(
    struct<MarginfiRebalanceInstructionData>(
      [
        ['discriminator', u8()],
        ['rebalanceArgs', getRebalanceArgsSerializer()],
      ],
      { description: 'MarginfiRebalanceInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 10 })
  ) as Serializer<
    MarginfiRebalanceInstructionDataArgs,
    MarginfiRebalanceInstructionData
  >;
}

// Args.
export type MarginfiRebalanceInstructionArgs =
  MarginfiRebalanceInstructionDataArgs;

// Instruction.
export function marginfiRebalance(
  context: Pick<Context, 'programs'>,
  input: MarginfiRebalanceInstructionAccounts & MarginfiRebalanceInstructionArgs
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
      isWritable: true as boolean,
      value: input.signer ?? null,
    },
    marginfiProgram: {
      index: 1,
      isWritable: false as boolean,
      value: input.marginfiProgram ?? null,
    },
    systemProgram: {
      index: 2,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    tokenProgram: {
      index: 3,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    ataProgram: {
      index: 4,
      isWritable: false as boolean,
      value: input.ataProgram ?? null,
    },
    rent: { index: 5, isWritable: false as boolean, value: input.rent ?? null },
    ixsSysvar: {
      index: 6,
      isWritable: false as boolean,
      value: input.ixsSysvar ?? null,
    },
    solautoFeesSupplyTa: {
      index: 7,
      isWritable: true as boolean,
      value: input.solautoFeesSupplyTa ?? null,
    },
    authorityReferralState: {
      index: 8,
      isWritable: false as boolean,
      value: input.authorityReferralState ?? null,
    },
    referredByState: {
      index: 9,
      isWritable: false as boolean,
      value: input.referredByState ?? null,
    },
    referredBySupplyTa: {
      index: 10,
      isWritable: true as boolean,
      value: input.referredBySupplyTa ?? null,
    },
    solautoPosition: {
      index: 11,
      isWritable: true as boolean,
      value: input.solautoPosition ?? null,
    },
    intermediaryTa: {
      index: 12,
      isWritable: true as boolean,
      value: input.intermediaryTa ?? null,
    },
    supplyMint: {
      index: 13,
      isWritable: false as boolean,
      value: input.supplyMint ?? null,
    },
    positionSupplyTa: {
      index: 14,
      isWritable: true as boolean,
      value: input.positionSupplyTa ?? null,
    },
    bankSupplyTa: {
      index: 15,
      isWritable: true as boolean,
      value: input.bankSupplyTa ?? null,
    },
    debtMint: {
      index: 16,
      isWritable: false as boolean,
      value: input.debtMint ?? null,
    },
    positionDebtTa: {
      index: 17,
      isWritable: true as boolean,
      value: input.positionDebtTa ?? null,
    },
    bankDebtTa: {
      index: 18,
      isWritable: true as boolean,
      value: input.bankDebtTa ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: MarginfiRebalanceInstructionArgs = { ...input };

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
  const data = getMarginfiRebalanceInstructionDataSerializer().serialize(
    resolvedArgs as MarginfiRebalanceInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
