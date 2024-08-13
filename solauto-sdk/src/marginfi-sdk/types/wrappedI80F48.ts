/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, array, struct, u8 } from '@metaplex-foundation/umi/serializers';

export type WrappedI80F48 = { value: number[] };

export type WrappedI80F48Args = { value: number[] };

export function getWrappedI80F48Serializer(): Serializer<
  WrappedI80F48Args,
  WrappedI80F48
> {
  return struct<WrappedI80F48>([['value', array(u8(), { size: 16 })]], {
    description: 'WrappedI80F48',
  }) as Serializer<WrappedI80F48Args, WrappedI80F48>;
}
