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
  array,
  mapSerializer,
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
  BankConfigCompact,
  BankConfigCompactArgs,
  getBankConfigCompactSerializer,
} from '../types';

// Accounts.
export type LendingPoolAddBankWithSeedInstructionAccounts = {
  marginfiGroup: PublicKey | Pda;
  admin: Signer;
  feePayer?: Signer;
  bankMint: PublicKey | Pda;
  bank: PublicKey | Pda;
  liquidityVaultAuthority: PublicKey | Pda;
  liquidityVault: PublicKey | Pda;
  insuranceVaultAuthority: PublicKey | Pda;
  insuranceVault: PublicKey | Pda;
  feeVaultAuthority: PublicKey | Pda;
  feeVault: PublicKey | Pda;
  rent?: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
};

// Data.
export type LendingPoolAddBankWithSeedInstructionData = {
  discriminator: Array<number>;
  bankConfig: BankConfigCompact;
  bankSeed: bigint;
};

export type LendingPoolAddBankWithSeedInstructionDataArgs = {
  bankConfig: BankConfigCompactArgs;
  bankSeed: number | bigint;
};

export function getLendingPoolAddBankWithSeedInstructionDataSerializer(): Serializer<
  LendingPoolAddBankWithSeedInstructionDataArgs,
  LendingPoolAddBankWithSeedInstructionData
> {
  return mapSerializer<
    LendingPoolAddBankWithSeedInstructionDataArgs,
    any,
    LendingPoolAddBankWithSeedInstructionData
  >(
    struct<LendingPoolAddBankWithSeedInstructionData>(
      [
        ['discriminator', array(u8(), { size: 8 })],
        ['bankConfig', getBankConfigCompactSerializer()],
        ['bankSeed', u64()],
      ],
      { description: 'LendingPoolAddBankWithSeedInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [76, 211, 213, 171, 117, 78, 158, 76],
    })
  ) as Serializer<
    LendingPoolAddBankWithSeedInstructionDataArgs,
    LendingPoolAddBankWithSeedInstructionData
  >;
}

// Args.
export type LendingPoolAddBankWithSeedInstructionArgs =
  LendingPoolAddBankWithSeedInstructionDataArgs;

// Instruction.
export function lendingPoolAddBankWithSeed(
  context: Pick<Context, 'payer' | 'programs'>,
  input: LendingPoolAddBankWithSeedInstructionAccounts &
    LendingPoolAddBankWithSeedInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'marginfi',
    'MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA'
  );

  // Accounts.
  const resolvedAccounts = {
    marginfiGroup: {
      index: 0,
      isWritable: false as boolean,
      value: input.marginfiGroup ?? null,
    },
    admin: {
      index: 1,
      isWritable: true as boolean,
      value: input.admin ?? null,
    },
    feePayer: {
      index: 2,
      isWritable: true as boolean,
      value: input.feePayer ?? null,
    },
    bankMint: {
      index: 3,
      isWritable: false as boolean,
      value: input.bankMint ?? null,
    },
    bank: { index: 4, isWritable: true as boolean, value: input.bank ?? null },
    liquidityVaultAuthority: {
      index: 5,
      isWritable: false as boolean,
      value: input.liquidityVaultAuthority ?? null,
    },
    liquidityVault: {
      index: 6,
      isWritable: true as boolean,
      value: input.liquidityVault ?? null,
    },
    insuranceVaultAuthority: {
      index: 7,
      isWritable: false as boolean,
      value: input.insuranceVaultAuthority ?? null,
    },
    insuranceVault: {
      index: 8,
      isWritable: true as boolean,
      value: input.insuranceVault ?? null,
    },
    feeVaultAuthority: {
      index: 9,
      isWritable: false as boolean,
      value: input.feeVaultAuthority ?? null,
    },
    feeVault: {
      index: 10,
      isWritable: true as boolean,
      value: input.feeVault ?? null,
    },
    rent: {
      index: 11,
      isWritable: false as boolean,
      value: input.rent ?? null,
    },
    tokenProgram: {
      index: 12,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    systemProgram: {
      index: 13,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: LendingPoolAddBankWithSeedInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.feePayer.value) {
    resolvedAccounts.feePayer.value = context.payer;
  }
  if (!resolvedAccounts.rent.value) {
    resolvedAccounts.rent.value = publicKey(
      'SysvarRent111111111111111111111111111111111'
    );
  }
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
  const data =
    getLendingPoolAddBankWithSeedInstructionDataSerializer().serialize(
      resolvedArgs as LendingPoolAddBankWithSeedInstructionDataArgs
    );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
