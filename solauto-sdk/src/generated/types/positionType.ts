/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

export enum PositionType {
  Leverage,
  SafeLoan,
}

export type PositionTypeArgs = PositionType;

export function getPositionTypeSerializer(): Serializer<
  PositionTypeArgs,
  PositionType
> {
  return scalarEnum<PositionType>(PositionType, {
    description: 'PositionType',
  }) as Serializer<PositionTypeArgs, PositionType>;
}
