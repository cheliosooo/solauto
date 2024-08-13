/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Account,
  Context,
  Pda,
  PublicKey,
  RpcAccount,
  RpcGetAccountOptions,
  RpcGetAccountsOptions,
  assertAccountExists,
  deserializeAccount,
  gpaBuilder,
  publicKey as toPublicKey,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  i64,
  mapSerializer,
  publicKey as publicKeySerializer,
  struct,
  u128,
  u64,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  BankConfig,
  BankConfigArgs,
  WrappedI80F48,
  WrappedI80F48Args,
  getBankConfigSerializer,
  getWrappedI80F48Serializer,
} from '../types';

export type Bank = Account<BankAccountData>;

export type BankAccountData = {
  discriminator: Array<number>;
  mint: PublicKey;
  mintDecimals: number;
  group: PublicKey;
  autoPadding0: Array<number>;
  assetShareValue: WrappedI80F48;
  liabilityShareValue: WrappedI80F48;
  liquidityVault: PublicKey;
  liquidityVaultBump: number;
  liquidityVaultAuthorityBump: number;
  insuranceVault: PublicKey;
  insuranceVaultBump: number;
  insuranceVaultAuthorityBump: number;
  autoPadding1: Array<number>;
  collectedInsuranceFeesOutstanding: WrappedI80F48;
  feeVault: PublicKey;
  feeVaultBump: number;
  feeVaultAuthorityBump: number;
  autoPadding2: Array<number>;
  collectedGroupFeesOutstanding: WrappedI80F48;
  totalLiabilityShares: WrappedI80F48;
  totalAssetShares: WrappedI80F48;
  lastUpdate: bigint;
  config: BankConfig;
  /**
   * Emissions Config Flags
   *
   * - EMISSIONS_FLAG_BORROW_ACTIVE: 1
   * - EMISSIONS_FLAG_LENDING_ACTIVE: 2
   *
   */
  emissionsFlags: bigint;
  /**
   * Emissions APR.
   * Number of emitted tokens (emissions_mint) per 1e(bank.mint_decimal) tokens (bank mint) (native amount) per 1 YEAR.
   */
  emissionsRate: bigint;
  emissionsRemaining: WrappedI80F48;
  emissionsMint: PublicKey;
  padding0: Array<bigint>;
  padding1: Array<bigint>;
};

export type BankAccountDataArgs = {
  mint: PublicKey;
  mintDecimals: number;
  group: PublicKey;
  autoPadding0: Array<number>;
  assetShareValue: WrappedI80F48Args;
  liabilityShareValue: WrappedI80F48Args;
  liquidityVault: PublicKey;
  liquidityVaultBump: number;
  liquidityVaultAuthorityBump: number;
  insuranceVault: PublicKey;
  insuranceVaultBump: number;
  insuranceVaultAuthorityBump: number;
  autoPadding1: Array<number>;
  collectedInsuranceFeesOutstanding: WrappedI80F48Args;
  feeVault: PublicKey;
  feeVaultBump: number;
  feeVaultAuthorityBump: number;
  autoPadding2: Array<number>;
  collectedGroupFeesOutstanding: WrappedI80F48Args;
  totalLiabilityShares: WrappedI80F48Args;
  totalAssetShares: WrappedI80F48Args;
  lastUpdate: number | bigint;
  config: BankConfigArgs;
  /**
   * Emissions Config Flags
   *
   * - EMISSIONS_FLAG_BORROW_ACTIVE: 1
   * - EMISSIONS_FLAG_LENDING_ACTIVE: 2
   *
   */
  emissionsFlags: number | bigint;
  /**
   * Emissions APR.
   * Number of emitted tokens (emissions_mint) per 1e(bank.mint_decimal) tokens (bank mint) (native amount) per 1 YEAR.
   */
  emissionsRate: number | bigint;
  emissionsRemaining: WrappedI80F48Args;
  emissionsMint: PublicKey;
  padding0: Array<number | bigint>;
  padding1: Array<number | bigint>;
};

export function getBankAccountDataSerializer(): Serializer<
  BankAccountDataArgs,
  BankAccountData
> {
  return mapSerializer<BankAccountDataArgs, any, BankAccountData>(
    struct<BankAccountData>(
      [
        ['discriminator', array(u8(), { size: 8 })],
        ['mint', publicKeySerializer()],
        ['mintDecimals', u8()],
        ['group', publicKeySerializer()],
        ['autoPadding0', array(u8(), { size: 7 })],
        ['assetShareValue', getWrappedI80F48Serializer()],
        ['liabilityShareValue', getWrappedI80F48Serializer()],
        ['liquidityVault', publicKeySerializer()],
        ['liquidityVaultBump', u8()],
        ['liquidityVaultAuthorityBump', u8()],
        ['insuranceVault', publicKeySerializer()],
        ['insuranceVaultBump', u8()],
        ['insuranceVaultAuthorityBump', u8()],
        ['autoPadding1', array(u8(), { size: 4 })],
        ['collectedInsuranceFeesOutstanding', getWrappedI80F48Serializer()],
        ['feeVault', publicKeySerializer()],
        ['feeVaultBump', u8()],
        ['feeVaultAuthorityBump', u8()],
        ['autoPadding2', array(u8(), { size: 6 })],
        ['collectedGroupFeesOutstanding', getWrappedI80F48Serializer()],
        ['totalLiabilityShares', getWrappedI80F48Serializer()],
        ['totalAssetShares', getWrappedI80F48Serializer()],
        ['lastUpdate', i64()],
        ['config', getBankConfigSerializer()],
        ['emissionsFlags', u64()],
        ['emissionsRate', u64()],
        ['emissionsRemaining', getWrappedI80F48Serializer()],
        ['emissionsMint', publicKeySerializer()],
        ['padding0', array(u128(), { size: 28 })],
        ['padding1', array(u128(), { size: 32 })],
      ],
      { description: 'BankAccountData' }
    ),
    (value) => ({
      ...value,
      discriminator: [142, 49, 166, 242, 50, 66, 97, 188],
    })
  ) as Serializer<BankAccountDataArgs, BankAccountData>;
}

export function deserializeBank(rawAccount: RpcAccount): Bank {
  return deserializeAccount(rawAccount, getBankAccountDataSerializer());
}

export async function fetchBank(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<Bank> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  assertAccountExists(maybeAccount, 'Bank');
  return deserializeBank(maybeAccount);
}

export async function safeFetchBank(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<Bank | null> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  return maybeAccount.exists ? deserializeBank(maybeAccount) : null;
}

export async function fetchAllBank(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<Bank[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, 'Bank');
    return deserializeBank(maybeAccount);
  });
}

export async function safeFetchAllBank(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<Bank[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializeBank(maybeAccount as RpcAccount));
}

export function getBankGpaBuilder(context: Pick<Context, 'rpc' | 'programs'>) {
  const programId = context.programs.getPublicKey(
    'marginfi',
    'MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA'
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      discriminator: Array<number>;
      mint: PublicKey;
      mintDecimals: number;
      group: PublicKey;
      autoPadding0: Array<number>;
      assetShareValue: WrappedI80F48Args;
      liabilityShareValue: WrappedI80F48Args;
      liquidityVault: PublicKey;
      liquidityVaultBump: number;
      liquidityVaultAuthorityBump: number;
      insuranceVault: PublicKey;
      insuranceVaultBump: number;
      insuranceVaultAuthorityBump: number;
      autoPadding1: Array<number>;
      collectedInsuranceFeesOutstanding: WrappedI80F48Args;
      feeVault: PublicKey;
      feeVaultBump: number;
      feeVaultAuthorityBump: number;
      autoPadding2: Array<number>;
      collectedGroupFeesOutstanding: WrappedI80F48Args;
      totalLiabilityShares: WrappedI80F48Args;
      totalAssetShares: WrappedI80F48Args;
      lastUpdate: number | bigint;
      config: BankConfigArgs;
      emissionsFlags: number | bigint;
      emissionsRate: number | bigint;
      emissionsRemaining: WrappedI80F48Args;
      emissionsMint: PublicKey;
      padding0: Array<number | bigint>;
      padding1: Array<number | bigint>;
    }>({
      discriminator: [0, array(u8(), { size: 8 })],
      mint: [8, publicKeySerializer()],
      mintDecimals: [40, u8()],
      group: [41, publicKeySerializer()],
      autoPadding0: [73, array(u8(), { size: 7 })],
      assetShareValue: [80, getWrappedI80F48Serializer()],
      liabilityShareValue: [96, getWrappedI80F48Serializer()],
      liquidityVault: [112, publicKeySerializer()],
      liquidityVaultBump: [144, u8()],
      liquidityVaultAuthorityBump: [145, u8()],
      insuranceVault: [146, publicKeySerializer()],
      insuranceVaultBump: [178, u8()],
      insuranceVaultAuthorityBump: [179, u8()],
      autoPadding1: [180, array(u8(), { size: 4 })],
      collectedInsuranceFeesOutstanding: [184, getWrappedI80F48Serializer()],
      feeVault: [200, publicKeySerializer()],
      feeVaultBump: [232, u8()],
      feeVaultAuthorityBump: [233, u8()],
      autoPadding2: [234, array(u8(), { size: 6 })],
      collectedGroupFeesOutstanding: [240, getWrappedI80F48Serializer()],
      totalLiabilityShares: [256, getWrappedI80F48Serializer()],
      totalAssetShares: [272, getWrappedI80F48Serializer()],
      lastUpdate: [288, i64()],
      config: [296, getBankConfigSerializer()],
      emissionsFlags: [840, u64()],
      emissionsRate: [848, u64()],
      emissionsRemaining: [856, getWrappedI80F48Serializer()],
      emissionsMint: [872, publicKeySerializer()],
      padding0: [904, array(u128(), { size: 28 })],
      padding1: [1352, array(u128(), { size: 32 })],
    })
    .deserializeUsing<Bank>((account) => deserializeBank(account))
    .whereField('discriminator', [142, 49, 166, 242, 50, 66, 97, 188]);
}

export function getBankSize(): number {
  return 1864;
}
