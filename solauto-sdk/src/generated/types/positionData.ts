/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { PublicKey } from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  publicKey as publicKeySerializer,
  struct,
  u32,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  DCASettings,
  DCASettingsArgs,
  LendingPlatform,
  LendingPlatformArgs,
  SolautoSettingsParameters,
  SolautoSettingsParametersArgs,
  getDCASettingsSerializer,
  getLendingPlatformSerializer,
  getSolautoSettingsParametersSerializer,
} from '.';

export type PositionData = {
  lendingPlatform: LendingPlatform;
  padding1: Array<number>;
  protocolAccount: PublicKey;
  supplyMint: PublicKey;
  debtMint: PublicKey;
  settingParams: SolautoSettingsParameters;
  dca: DCASettings;
  padding: Array<number>;
};

export type PositionDataArgs = {
  lendingPlatform: LendingPlatformArgs;
  padding1: Array<number>;
  protocolAccount: PublicKey;
  supplyMint: PublicKey;
  debtMint: PublicKey;
  settingParams: SolautoSettingsParametersArgs;
  dca: DCASettingsArgs;
  padding: Array<number>;
};

export function getPositionDataSerializer(): Serializer<
  PositionDataArgs,
  PositionData
> {
  return struct<PositionData>(
    [
      ['lendingPlatform', getLendingPlatformSerializer()],
      ['padding1', array(u8(), { size: 7 })],
      ['protocolAccount', publicKeySerializer()],
      ['supplyMint', publicKeySerializer()],
      ['debtMint', publicKeySerializer()],
      ['settingParams', getSolautoSettingsParametersSerializer()],
      ['dca', getDCASettingsSerializer()],
      ['padding', array(u32(), { size: 4 })],
    ],
    { description: 'PositionData' }
  ) as Serializer<PositionDataArgs, PositionData>;
}
