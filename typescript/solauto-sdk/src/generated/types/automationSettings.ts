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

export type AutomationSettings = {
  unixStartDate: bigint;
  intervalSeconds: bigint;
  periodsPassed: number;
  targetPeriods: number;
};

export type AutomationSettingsArgs = {
  unixStartDate: number | bigint;
  intervalSeconds: number | bigint;
  periodsPassed: number;
  targetPeriods: number;
};

export function getAutomationSettingsSerializer(): Serializer<
  AutomationSettingsArgs,
  AutomationSettings
> {
  return struct<AutomationSettings>(
    [
      ['unixStartDate', u64()],
      ['intervalSeconds', u64()],
      ['periodsPassed', u16()],
      ['targetPeriods', u16()],
    ],
    { description: 'AutomationSettings' }
  ) as Serializer<AutomationSettingsArgs, AutomationSettings>;
}