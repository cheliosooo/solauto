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
  Option,
  OptionOrNullable,
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
  option,
  publicKey as publicKeySerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  GeneralPositionData,
  GeneralPositionDataArgs,
  KaminoPositionData,
  KaminoPositionDataArgs,
  LendingPlatform,
  LendingPlatformArgs,
  MarginfiPositionData,
  MarginfiPositionDataArgs,
  SolautoSettingsParameters,
  SolautoSettingsParametersArgs,
  SolendPositionData,
  SolendPositionDataArgs,
  getGeneralPositionDataSerializer,
  getKaminoPositionDataSerializer,
  getLendingPlatformSerializer,
  getMarginfiPositionDataSerializer,
  getSolautoSettingsParametersSerializer,
  getSolendPositionDataSerializer,
} from '../types';

export type Position = Account<PositionAccountData>;

export type PositionAccountData = {
  positionId: number;
  authority: PublicKey;
  lendingPlatform: LendingPlatform;
  settingParams: SolautoSettingsParameters;
  generalData: GeneralPositionData;
  marginfiData: Option<MarginfiPositionData>;
  solendData: Option<SolendPositionData>;
  kaminoData: Option<KaminoPositionData>;
};

export type PositionAccountDataArgs = {
  positionId: number;
  authority: PublicKey;
  lendingPlatform: LendingPlatformArgs;
  settingParams: SolautoSettingsParametersArgs;
  generalData: GeneralPositionDataArgs;
  marginfiData: OptionOrNullable<MarginfiPositionDataArgs>;
  solendData: OptionOrNullable<SolendPositionDataArgs>;
  kaminoData: OptionOrNullable<KaminoPositionDataArgs>;
};

export function getPositionAccountDataSerializer(): Serializer<
  PositionAccountDataArgs,
  PositionAccountData
> {
  return struct<PositionAccountData>(
    [
      ['positionId', u8()],
      ['authority', publicKeySerializer()],
      ['lendingPlatform', getLendingPlatformSerializer()],
      ['settingParams', getSolautoSettingsParametersSerializer()],
      ['generalData', getGeneralPositionDataSerializer()],
      ['marginfiData', option(getMarginfiPositionDataSerializer())],
      ['solendData', option(getSolendPositionDataSerializer())],
      ['kaminoData', option(getKaminoPositionDataSerializer())],
    ],
    { description: 'PositionAccountData' }
  ) as Serializer<PositionAccountDataArgs, PositionAccountData>;
}

export function deserializePosition(rawAccount: RpcAccount): Position {
  return deserializeAccount(rawAccount, getPositionAccountDataSerializer());
}

export async function fetchPosition(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<Position> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  assertAccountExists(maybeAccount, 'Position');
  return deserializePosition(maybeAccount);
}

export async function safeFetchPosition(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<Position | null> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  return maybeAccount.exists ? deserializePosition(maybeAccount) : null;
}

export async function fetchAllPosition(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<Position[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, 'Position');
    return deserializePosition(maybeAccount);
  });
}

export async function safeFetchAllPosition(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<Position[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializePosition(maybeAccount as RpcAccount));
}

export function getPositionGpaBuilder(
  context: Pick<Context, 'rpc' | 'programs'>
) {
  const programId = context.programs.getPublicKey(
    'solauto',
    'AutoyKBRaHSBHy9RsmXCZMy6nNFAg5FYijrvZyQcNLV'
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      positionId: number;
      authority: PublicKey;
      lendingPlatform: LendingPlatformArgs;
      settingParams: SolautoSettingsParametersArgs;
      generalData: GeneralPositionDataArgs;
      marginfiData: OptionOrNullable<MarginfiPositionDataArgs>;
      solendData: OptionOrNullable<SolendPositionDataArgs>;
      kaminoData: OptionOrNullable<KaminoPositionDataArgs>;
    }>({
      positionId: [0, u8()],
      authority: [1, publicKeySerializer()],
      lendingPlatform: [33, getLendingPlatformSerializer()],
      settingParams: [34, getSolautoSettingsParametersSerializer()],
      generalData: [42, getGeneralPositionDataSerializer()],
      marginfiData: [76, option(getMarginfiPositionDataSerializer())],
      solendData: [null, option(getSolendPositionDataSerializer())],
      kaminoData: [null, option(getKaminoPositionDataSerializer())],
    })
    .deserializeUsing<Position>((account) => deserializePosition(account));
}
