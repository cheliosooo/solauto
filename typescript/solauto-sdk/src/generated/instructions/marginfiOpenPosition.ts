/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Option,
  OptionOrNullable,
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
  option,
  struct,
  u64,
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
export type MarginfiOpenPositionInstructionAccounts = {
  signer: Signer;
  marginfiProgram: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  ataProgram?: PublicKey | Pda;
  rent?: PublicKey | Pda;
  solautoFeesWallet: PublicKey | Pda;
  solautoFeesSupplyTa: PublicKey | Pda;
  signerReferralState: PublicKey | Pda;
  referredByState?: PublicKey | Pda;
  referredBySupplyTa?: PublicKey | Pda;
  solautoPosition: PublicKey | Pda;
  marginfiGroup: PublicKey | Pda;
  marginfiAccount: PublicKey | Pda | Signer;
  supplyMint: PublicKey | Pda;
  positionSupplyTa: PublicKey | Pda;
  debtMint: PublicKey | Pda;
  positionDebtTa: PublicKey | Pda;
  signerDebtTa?: PublicKey | Pda;
};

// Data.
export type MarginfiOpenPositionInstructionData = {
  discriminator: number;
  positionData: UpdatePositionData;
  marginfiAccountSeedIdx: Option<bigint>;
};

export type MarginfiOpenPositionInstructionDataArgs = {
  positionData: UpdatePositionDataArgs;
  marginfiAccountSeedIdx: OptionOrNullable<number | bigint>;
};

export function getMarginfiOpenPositionInstructionDataSerializer(): Serializer<
  MarginfiOpenPositionInstructionDataArgs,
  MarginfiOpenPositionInstructionData
> {
  return mapSerializer<
    MarginfiOpenPositionInstructionDataArgs,
    any,
    MarginfiOpenPositionInstructionData
  >(
    struct<MarginfiOpenPositionInstructionData>(
      [
        ['discriminator', u8()],
        ['positionData', getUpdatePositionDataSerializer()],
        ['marginfiAccountSeedIdx', option(u64())],
      ],
      { description: 'MarginfiOpenPositionInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 3 })
  ) as Serializer<
    MarginfiOpenPositionInstructionDataArgs,
    MarginfiOpenPositionInstructionData
  >;
}

// Args.
export type MarginfiOpenPositionInstructionArgs =
  MarginfiOpenPositionInstructionDataArgs;

// Instruction.
export function marginfiOpenPosition(
  context: Pick<Context, 'programs'>,
  input: MarginfiOpenPositionInstructionAccounts &
    MarginfiOpenPositionInstructionArgs
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
    solautoFeesWallet: {
      index: 6,
      isWritable: false as boolean,
      value: input.solautoFeesWallet ?? null,
    },
    solautoFeesSupplyTa: {
      index: 7,
      isWritable: true as boolean,
      value: input.solautoFeesSupplyTa ?? null,
    },
    signerReferralState: {
      index: 8,
      isWritable: false as boolean,
      value: input.signerReferralState ?? null,
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
    marginfiGroup: {
      index: 12,
      isWritable: false as boolean,
      value: input.marginfiGroup ?? null,
    },
    marginfiAccount: {
      index: 13,
      isWritable: true as boolean,
      value: input.marginfiAccount ?? null,
    },
    supplyMint: {
      index: 14,
      isWritable: false as boolean,
      value: input.supplyMint ?? null,
    },
    positionSupplyTa: {
      index: 15,
      isWritable: true as boolean,
      value: input.positionSupplyTa ?? null,
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
    signerDebtTa: {
      index: 18,
      isWritable: true as boolean,
      value: input.signerDebtTa ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: MarginfiOpenPositionInstructionArgs = { ...input };

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
  const data = getMarginfiOpenPositionInstructionDataSerializer().serialize(
    resolvedArgs as MarginfiOpenPositionInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
