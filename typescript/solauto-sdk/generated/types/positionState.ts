/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Serializer,
  struct,
  u16,
  u64,
} from '@metaplex-foundation/umi/serializers';

export type PositionState = {
  liqUtilizationRateBps: number;
  netWorthUsdBaseAmount: bigint;
  baseAmountLiquidityNetWorth: bigint;
  baseAmountSupplied: bigint;
  baseAmountBorrowed: bigint;
  maxLtvBps: bigint;
  liqThreshold: bigint;
  lastUpdated: bigint;
};

export type PositionStateArgs = {
  liqUtilizationRateBps: number;
  netWorthUsdBaseAmount: number | bigint;
  baseAmountLiquidityNetWorth: number | bigint;
  baseAmountSupplied: number | bigint;
  baseAmountBorrowed: number | bigint;
  maxLtvBps: number | bigint;
  liqThreshold: number | bigint;
  lastUpdated: number | bigint;
};

export function getPositionStateSerializer(): Serializer<
  PositionStateArgs,
  PositionState
> {
  return struct<PositionState>(
    [
      ['liqUtilizationRateBps', u16()],
      ['netWorthUsdBaseAmount', u64()],
      ['baseAmountLiquidityNetWorth', u64()],
      ['baseAmountSupplied', u64()],
      ['baseAmountBorrowed', u64()],
      ['maxLtvBps', u64()],
      ['liqThreshold', u64()],
      ['lastUpdated', u64()],
    ],
    { description: 'PositionState' }
  ) as Serializer<PositionStateArgs, PositionState>;
}
