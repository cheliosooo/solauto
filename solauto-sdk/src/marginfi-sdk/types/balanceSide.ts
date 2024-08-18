/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

export enum BalanceSide {
  Assets,
  Liabilities,
}

export type BalanceSideArgs = BalanceSide;

export function getBalanceSideSerializer(): Serializer<
  BalanceSideArgs,
  BalanceSide
> {
  return scalarEnum<BalanceSide>(BalanceSide, {
    description: 'BalanceSide',
  }) as Serializer<BalanceSideArgs, BalanceSide>;
}