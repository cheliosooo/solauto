/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Option, OptionOrNullable } from '@metaplex-foundation/umi';
import {
  Serializer,
  option,
  struct,
  u16,
  u64,
} from '@metaplex-foundation/umi/serializers';

export type DebtToAddToPosition = {
  baseUnitDebtAmount: bigint;
  riskAversionBps: Option<number>;
};

export type DebtToAddToPositionArgs = {
  baseUnitDebtAmount: number | bigint;
  riskAversionBps: OptionOrNullable<number>;
};

export function getDebtToAddToPositionSerializer(): Serializer<
  DebtToAddToPositionArgs,
  DebtToAddToPosition
> {
  return struct<DebtToAddToPosition>(
    [
      ['baseUnitDebtAmount', u64()],
      ['riskAversionBps', option(u16())],
    ],
    { description: 'DebtToAddToPosition' }
  ) as Serializer<DebtToAddToPositionArgs, DebtToAddToPosition>;
}