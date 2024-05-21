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
export type MarginfiRefreshDataInstructionAccounts = {
  signer: Signer;
  marginfiProgram: PublicKey | Pda;
  marginfiGroup: PublicKey | Pda;
  marginfiAccount?: PublicKey | Pda;
  supplyBank: PublicKey | Pda;
  supplyPriceOracle: PublicKey | Pda;
  debtBank: PublicKey | Pda;
  debtPriceOracle: PublicKey | Pda;
  solautoPosition?: PublicKey | Pda;
};

// Data.
export type MarginfiRefreshDataInstructionData = { discriminator: number };

export type MarginfiRefreshDataInstructionDataArgs = {};

export function getMarginfiRefreshDataInstructionDataSerializer(): Serializer<
  MarginfiRefreshDataInstructionDataArgs,
  MarginfiRefreshDataInstructionData
> {
  return mapSerializer<
    MarginfiRefreshDataInstructionDataArgs,
    any,
    MarginfiRefreshDataInstructionData
  >(
    struct<MarginfiRefreshDataInstructionData>([['discriminator', u8()]], {
      description: 'MarginfiRefreshDataInstructionData',
    }),
    (value) => ({ ...value, discriminator: 8 })
  ) as Serializer<
    MarginfiRefreshDataInstructionDataArgs,
    MarginfiRefreshDataInstructionData
  >;
}

// Instruction.
export function marginfiRefreshData(
  context: Pick<Context, 'programs'>,
  input: MarginfiRefreshDataInstructionAccounts
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
    marginfiGroup: {
      index: 2,
      isWritable: false as boolean,
      value: input.marginfiGroup ?? null,
    },
    marginfiAccount: {
      index: 3,
      isWritable: false as boolean,
      value: input.marginfiAccount ?? null,
    },
    supplyBank: {
      index: 4,
      isWritable: true as boolean,
      value: input.supplyBank ?? null,
    },
    supplyPriceOracle: {
      index: 5,
      isWritable: false as boolean,
      value: input.supplyPriceOracle ?? null,
    },
    debtBank: {
      index: 6,
      isWritable: true as boolean,
      value: input.debtBank ?? null,
    },
    debtPriceOracle: {
      index: 7,
      isWritable: false as boolean,
      value: input.debtPriceOracle ?? null,
    },
    solautoPosition: {
      index: 8,
      isWritable: true as boolean,
      value: input.solautoPosition ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

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
  const data = getMarginfiRefreshDataInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
