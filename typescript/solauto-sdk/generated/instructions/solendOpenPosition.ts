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
  UpdatePositionData,
  UpdatePositionDataArgs,
  getUpdatePositionDataSerializer,
} from '../types';

// Accounts.
export type SolendOpenPositionInstructionAccounts = {
  signer: Signer;
  solendProgram: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  ataProgram?: PublicKey | Pda;
  rent?: PublicKey | Pda;
  solautoFeesReceiver: PublicKey | Pda;
  solautoFeesReceiverTa: PublicKey | Pda;
  signerReferralState: PublicKey | Pda;
  referralFeesMint: PublicKey | Pda;
  signerReferralDestTa: PublicKey | Pda;
  referredByState?: PublicKey | Pda;
  referredByAuthority?: PublicKey | Pda;
  referredByDestTa?: PublicKey | Pda;
  referredBySupplyTa?: PublicKey | Pda;
  solautoPosition: PublicKey | Pda;
  lendingMarket: PublicKey | Pda;
  obligation: PublicKey | Pda;
  supplyReserve: PublicKey | Pda;
  positionSupplyLiquidityTa: PublicKey | Pda;
  supplyLiquidityMint: PublicKey | Pda;
  positionSupplyCollateralTa: PublicKey | Pda;
  supplyCollateralMint: PublicKey | Pda;
  positionDebtLiquidityTa: PublicKey | Pda;
  debtLiquidityMint: PublicKey | Pda;
};

// Data.
export type SolendOpenPositionInstructionData = {
  discriminator: number;
  updatePositionData: UpdatePositionData;
};

export type SolendOpenPositionInstructionDataArgs = {
  updatePositionData: UpdatePositionDataArgs;
};

export function getSolendOpenPositionInstructionDataSerializer(): Serializer<
  SolendOpenPositionInstructionDataArgs,
  SolendOpenPositionInstructionData
> {
  return mapSerializer<
    SolendOpenPositionInstructionDataArgs,
    any,
    SolendOpenPositionInstructionData
  >(
    struct<SolendOpenPositionInstructionData>(
      [
        ['discriminator', u8()],
        ['updatePositionData', getUpdatePositionDataSerializer()],
      ],
      { description: 'SolendOpenPositionInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 2 })
  ) as Serializer<
    SolendOpenPositionInstructionDataArgs,
    SolendOpenPositionInstructionData
  >;
}

// Args.
export type SolendOpenPositionInstructionArgs =
  SolendOpenPositionInstructionDataArgs;

// Instruction.
export function solendOpenPosition(
  context: Pick<Context, 'programs'>,
  input: SolendOpenPositionInstructionAccounts &
    SolendOpenPositionInstructionArgs
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
    solendProgram: {
      index: 1,
      isWritable: false as boolean,
      value: input.solendProgram ?? null,
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
    solautoFeesReceiver: {
      index: 6,
      isWritable: false as boolean,
      value: input.solautoFeesReceiver ?? null,
    },
    solautoFeesReceiverTa: {
      index: 7,
      isWritable: false as boolean,
      value: input.solautoFeesReceiverTa ?? null,
    },
    signerReferralState: {
      index: 8,
      isWritable: true as boolean,
      value: input.signerReferralState ?? null,
    },
    referralFeesMint: {
      index: 9,
      isWritable: true as boolean,
      value: input.referralFeesMint ?? null,
    },
    signerReferralDestTa: {
      index: 10,
      isWritable: true as boolean,
      value: input.signerReferralDestTa ?? null,
    },
    referredByState: {
      index: 11,
      isWritable: true as boolean,
      value: input.referredByState ?? null,
    },
    referredByAuthority: {
      index: 12,
      isWritable: false as boolean,
      value: input.referredByAuthority ?? null,
    },
    referredByDestTa: {
      index: 13,
      isWritable: true as boolean,
      value: input.referredByDestTa ?? null,
    },
    referredBySupplyTa: {
      index: 14,
      isWritable: true as boolean,
      value: input.referredBySupplyTa ?? null,
    },
    solautoPosition: {
      index: 15,
      isWritable: true as boolean,
      value: input.solautoPosition ?? null,
    },
    lendingMarket: {
      index: 16,
      isWritable: false as boolean,
      value: input.lendingMarket ?? null,
    },
    obligation: {
      index: 17,
      isWritable: true as boolean,
      value: input.obligation ?? null,
    },
    supplyReserve: {
      index: 18,
      isWritable: false as boolean,
      value: input.supplyReserve ?? null,
    },
    positionSupplyLiquidityTa: {
      index: 19,
      isWritable: true as boolean,
      value: input.positionSupplyLiquidityTa ?? null,
    },
    supplyLiquidityMint: {
      index: 20,
      isWritable: false as boolean,
      value: input.supplyLiquidityMint ?? null,
    },
    positionSupplyCollateralTa: {
      index: 21,
      isWritable: true as boolean,
      value: input.positionSupplyCollateralTa ?? null,
    },
    supplyCollateralMint: {
      index: 22,
      isWritable: false as boolean,
      value: input.supplyCollateralMint ?? null,
    },
    positionDebtLiquidityTa: {
      index: 23,
      isWritable: true as boolean,
      value: input.positionDebtLiquidityTa ?? null,
    },
    debtLiquidityMint: {
      index: 24,
      isWritable: false as boolean,
      value: input.debtLiquidityMint ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: SolendOpenPositionInstructionArgs = { ...input };

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
  const data = getSolendOpenPositionInstructionDataSerializer().serialize(
    resolvedArgs as SolendOpenPositionInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
