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
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  option,
  struct,
  u16,
  u64,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';
import {
  SolautoRebalanceType,
  SolautoRebalanceTypeArgs,
  getSolautoRebalanceTypeSerializer,
} from '../types';

// Accounts.
export type MarginfiRebalanceInstructionAccounts = {
  signer: Signer;
  marginfiProgram: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  ixsSysvar: PublicKey | Pda;
  solautoFeesSupplyTa?: PublicKey | Pda;
  authorityReferralState: PublicKey | Pda;
  referredBySupplyTa?: PublicKey | Pda;
  positionAuthority: PublicKey | Pda;
  solautoPosition: PublicKey | Pda;
  marginfiGroup: PublicKey | Pda;
  marginfiAccount: PublicKey | Pda;
  intermediaryTa?: PublicKey | Pda;
  supplyBank: PublicKey | Pda;
  supplyPriceOracle?: PublicKey | Pda;
  positionSupplyTa: PublicKey | Pda;
  authoritySupplyTa?: PublicKey | Pda;
  vaultSupplyTa?: PublicKey | Pda;
  supplyVaultAuthority?: PublicKey | Pda;
  debtBank: PublicKey | Pda;
  debtPriceOracle?: PublicKey | Pda;
  positionDebtTa: PublicKey | Pda;
  authorityDebtTa?: PublicKey | Pda;
  vaultDebtTa?: PublicKey | Pda;
  debtVaultAuthority?: PublicKey | Pda;
};

// Data.
export type MarginfiRebalanceInstructionData = {
  discriminator: number;
  slippageBps: number;
  rebalanceType: SolautoRebalanceType;
  targetLiqUtilizationRateBps: Option<number>;
  targetInAmountBaseUnit: Option<bigint>;
  limitGapBps: Option<number>;
};

export type MarginfiRebalanceInstructionDataArgs = {
  slippageBps: number;
  rebalanceType: SolautoRebalanceTypeArgs;
  targetLiqUtilizationRateBps: OptionOrNullable<number>;
  targetInAmountBaseUnit: OptionOrNullable<number | bigint>;
  limitGapBps: OptionOrNullable<number>;
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
        ['slippageBps', u16()],
        ['rebalanceType', getSolautoRebalanceTypeSerializer()],
        ['targetLiqUtilizationRateBps', option(u16())],
        ['targetInAmountBaseUnit', option(u64())],
        ['limitGapBps', option(u16())],
      ],
      { description: 'MarginfiRebalanceInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 9 })
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
    ixsSysvar: {
      index: 4,
      isWritable: false as boolean,
      value: input.ixsSysvar ?? null,
    },
    solautoFeesSupplyTa: {
      index: 5,
      isWritable: true as boolean,
      value: input.solautoFeesSupplyTa ?? null,
    },
    authorityReferralState: {
      index: 6,
      isWritable: false as boolean,
      value: input.authorityReferralState ?? null,
    },
    referredBySupplyTa: {
      index: 7,
      isWritable: true as boolean,
      value: input.referredBySupplyTa ?? null,
    },
    positionAuthority: {
      index: 8,
      isWritable: true as boolean,
      value: input.positionAuthority ?? null,
    },
    solautoPosition: {
      index: 9,
      isWritable: true as boolean,
      value: input.solautoPosition ?? null,
    },
    marginfiGroup: {
      index: 10,
      isWritable: false as boolean,
      value: input.marginfiGroup ?? null,
    },
    marginfiAccount: {
      index: 11,
      isWritable: true as boolean,
      value: input.marginfiAccount ?? null,
    },
    intermediaryTa: {
      index: 12,
      isWritable: true as boolean,
      value: input.intermediaryTa ?? null,
    },
    supplyBank: {
      index: 13,
      isWritable: true as boolean,
      value: input.supplyBank ?? null,
    },
    supplyPriceOracle: {
      index: 14,
      isWritable: false as boolean,
      value: input.supplyPriceOracle ?? null,
    },
    positionSupplyTa: {
      index: 15,
      isWritable: true as boolean,
      value: input.positionSupplyTa ?? null,
    },
    authoritySupplyTa: {
      index: 16,
      isWritable: true as boolean,
      value: input.authoritySupplyTa ?? null,
    },
    vaultSupplyTa: {
      index: 17,
      isWritable: true as boolean,
      value: input.vaultSupplyTa ?? null,
    },
    supplyVaultAuthority: {
      index: 18,
      isWritable: true as boolean,
      value: input.supplyVaultAuthority ?? null,
    },
    debtBank: {
      index: 19,
      isWritable: true as boolean,
      value: input.debtBank ?? null,
    },
    debtPriceOracle: {
      index: 20,
      isWritable: false as boolean,
      value: input.debtPriceOracle ?? null,
    },
    positionDebtTa: {
      index: 21,
      isWritable: true as boolean,
      value: input.positionDebtTa ?? null,
    },
    authorityDebtTa: {
      index: 22,
      isWritable: true as boolean,
      value: input.authorityDebtTa ?? null,
    },
    vaultDebtTa: {
      index: 23,
      isWritable: true as boolean,
      value: input.vaultDebtTa ?? null,
    },
    debtVaultAuthority: {
      index: 24,
      isWritable: true as boolean,
      value: input.debtVaultAuthority ?? null,
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
