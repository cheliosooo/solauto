/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Serializer,
  array,
  bytes,
  struct,
  u16,
  u64,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  SolautoRebalanceType,
  SolautoRebalanceTypeArgs,
  getSolautoRebalanceTypeSerializer,
} from '.';

export type RebalanceData = {
  rebalanceType: SolautoRebalanceType;
  padding1: Array<number>;
  priceSlippageBps: number;
  targetLiqUtilizationRateBps: number;
  padding2: Array<number>;
  flashLoanAmount: bigint;
  padding: Uint8Array;
};

export type RebalanceDataArgs = {
  rebalanceType: SolautoRebalanceTypeArgs;
  padding1: Array<number>;
  priceSlippageBps: number;
  targetLiqUtilizationRateBps: number;
  padding2: Array<number>;
  flashLoanAmount: number | bigint;
  padding: Uint8Array;
};

export function getRebalanceDataSerializer(): Serializer<
  RebalanceDataArgs,
  RebalanceData
> {
  return struct<RebalanceData>(
    [
      ['rebalanceType', getSolautoRebalanceTypeSerializer()],
      ['padding1', array(u8(), { size: 7 })],
      ['priceSlippageBps', u16()],
      ['targetLiqUtilizationRateBps', u16()],
      ['padding2', array(u8(), { size: 4 })],
      ['flashLoanAmount', u64()],
      ['padding', bytes({ size: 32 })],
    ],
    { description: 'RebalanceData' }
  ) as Serializer<RebalanceDataArgs, RebalanceData>;
}
