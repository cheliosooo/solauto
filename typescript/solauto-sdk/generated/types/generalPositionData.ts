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

export type GeneralPositionData = {
  liqUtilizationRateBps: number;
  netWorthUsdBaseAmount: bigint;
  baseAmountLiquidityNetWorth: bigint;
  baseAmountSupplied: bigint;
  baseAmountBorrowed: bigint;
};

export type GeneralPositionDataArgs = {
  liqUtilizationRateBps: number;
  netWorthUsdBaseAmount: number | bigint;
  baseAmountLiquidityNetWorth: number | bigint;
  baseAmountSupplied: number | bigint;
  baseAmountBorrowed: number | bigint;
};

export function getGeneralPositionDataSerializer(): Serializer<
  GeneralPositionDataArgs,
  GeneralPositionData
> {
  return struct<GeneralPositionData>(
    [
      ['liqUtilizationRateBps', u16()],
      ['netWorthUsdBaseAmount', u64()],
      ['baseAmountLiquidityNetWorth', u64()],
      ['baseAmountSupplied', u64()],
      ['baseAmountBorrowed', u64()],
    ],
    { description: 'GeneralPositionData' }
  ) as Serializer<GeneralPositionDataArgs, GeneralPositionData>;
}
