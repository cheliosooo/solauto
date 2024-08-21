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
  publicKey as publicKeySerializer,
  struct,
  u32,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  PodBool,
  PodBoolArgs,
  PositionData,
  PositionDataArgs,
  PositionState,
  PositionStateArgs,
  RebalanceData,
  RebalanceDataArgs,
  getPodBoolSerializer,
  getPositionDataSerializer,
  getPositionStateSerializer,
  getRebalanceDataSerializer,
} from '../types';

export type SolautoPosition = Account<SolautoPositionAccountData>;

export type SolautoPositionAccountData = {
  bump: Array<number>;
  positionId: Array<number>;
  selfManaged: PodBool;
  padding1: Array<number>;
  authority: PublicKey;
  position: PositionData;
  state: PositionState;
  rebalance: RebalanceData;
  padding: Array<number>;
};

export type SolautoPositionAccountDataArgs = {
  bump: Array<number>;
  positionId: Array<number>;
  selfManaged: PodBoolArgs;
  padding1: Array<number>;
  authority: PublicKey;
  position: PositionDataArgs;
  state: PositionStateArgs;
  rebalance: RebalanceDataArgs;
  padding: Array<number>;
};

export function getSolautoPositionAccountDataSerializer(): Serializer<
  SolautoPositionAccountDataArgs,
  SolautoPositionAccountData
> {
  return struct<SolautoPositionAccountData>(
    [
      ['bump', array(u8(), { size: 1 })],
      ['positionId', array(u8(), { size: 1 })],
      ['selfManaged', getPodBoolSerializer()],
      ['padding1', array(u8(), { size: 5 })],
      ['authority', publicKeySerializer()],
      ['position', getPositionDataSerializer()],
      ['state', getPositionStateSerializer()],
      ['rebalance', getRebalanceDataSerializer()],
      ['padding', array(u32(), { size: 32 })],
    ],
    { description: 'SolautoPositionAccountData' }
  ) as Serializer<SolautoPositionAccountDataArgs, SolautoPositionAccountData>;
}

export function deserializeSolautoPosition(
  rawAccount: RpcAccount
): SolautoPosition {
  return deserializeAccount(
    rawAccount,
    getSolautoPositionAccountDataSerializer()
  );
}

export async function fetchSolautoPosition(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<SolautoPosition> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  assertAccountExists(maybeAccount, 'SolautoPosition');
  return deserializeSolautoPosition(maybeAccount);
}

export async function safeFetchSolautoPosition(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<SolautoPosition | null> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  return maybeAccount.exists ? deserializeSolautoPosition(maybeAccount) : null;
}

export async function fetchAllSolautoPosition(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<SolautoPosition[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, 'SolautoPosition');
    return deserializeSolautoPosition(maybeAccount);
  });
}

export async function safeFetchAllSolautoPosition(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<SolautoPosition[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) =>
      deserializeSolautoPosition(maybeAccount as RpcAccount)
    );
}

export function getSolautoPositionGpaBuilder(
  context: Pick<Context, 'rpc' | 'programs'>
) {
  const programId = context.programs.getPublicKey(
    'solauto',
    'AutoyKBRaHSBHy9RsmXCZMy6nNFAg5FYijrvZyQcNLV'
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      bump: Array<number>;
      positionId: Array<number>;
      selfManaged: PodBoolArgs;
      padding1: Array<number>;
      authority: PublicKey;
      position: PositionDataArgs;
      state: PositionStateArgs;
      rebalance: RebalanceDataArgs;
      padding: Array<number>;
    }>({
      bump: [0, array(u8(), { size: 1 })],
      positionId: [1, array(u8(), { size: 1 })],
      selfManaged: [2, getPodBoolSerializer()],
      padding1: [3, array(u8(), { size: 5 })],
      authority: [8, publicKeySerializer()],
      position: [40, getPositionDataSerializer()],
      state: [360, getPositionStateSerializer()],
      rebalance: [648, getRebalanceDataSerializer()],
      padding: [704, array(u32(), { size: 32 })],
    })
    .deserializeUsing<SolautoPosition>((account) =>
      deserializeSolautoPosition(account)
    );
}

export function getSolautoPositionSize(): number {
  return 832;
}
