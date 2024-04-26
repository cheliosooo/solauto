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

// Accounts.
export type UpdateReferralStatesInstructionAccounts = {
  signer: Signer;
  systemProgram?: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  ataProgram?: PublicKey | Pda;
  rent?: PublicKey | Pda;
  destReferralFeesMint: PublicKey | Pda;
  signerReferralState: PublicKey | Pda;
  signerReferralDestTa: PublicKey | Pda;
  referredByState?: PublicKey | Pda;
  referredByAuthority?: PublicKey | Pda;
  referredByDestTa?: PublicKey | Pda;
};

// Data.
export type UpdateReferralStatesInstructionData = { discriminator: number };

export type UpdateReferralStatesInstructionDataArgs = {};

export function getUpdateReferralStatesInstructionDataSerializer(): Serializer<
  UpdateReferralStatesInstructionDataArgs,
  UpdateReferralStatesInstructionData
> {
  return mapSerializer<
    UpdateReferralStatesInstructionDataArgs,
    any,
    UpdateReferralStatesInstructionData
  >(
    struct<UpdateReferralStatesInstructionData>([['discriminator', u8()]], {
      description: 'UpdateReferralStatesInstructionData',
    }),
    (value) => ({ ...value, discriminator: 0 })
  ) as Serializer<
    UpdateReferralStatesInstructionDataArgs,
    UpdateReferralStatesInstructionData
  >;
}

// Instruction.
export function updateReferralStates(
  context: Pick<Context, 'programs'>,
  input: UpdateReferralStatesInstructionAccounts
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
    rent: { index: 4, isWritable: false as boolean, value: input.rent ?? null },
    destReferralFeesMint: {
      index: 5,
      isWritable: false as boolean,
      value: input.destReferralFeesMint ?? null,
    },
    signerReferralState: {
      index: 6,
      isWritable: true as boolean,
      value: input.signerReferralState ?? null,
    },
    signerReferralDestTa: {
      index: 7,
      isWritable: true as boolean,
      value: input.signerReferralDestTa ?? null,
    },
    referredByState: {
      index: 8,
      isWritable: true as boolean,
      value: input.referredByState ?? null,
    },
    referredByAuthority: {
      index: 9,
      isWritable: false as boolean,
      value: input.referredByAuthority ?? null,
    },
    referredByDestTa: {
      index: 10,
      isWritable: true as boolean,
      value: input.referredByDestTa ?? null,
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
  const data = getUpdateReferralStatesInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
