/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Serializer } from '@metaplex-foundation/umi/serializers';
import { AutomationSettings, AutomationSettingsArgs } from '.';
export type DCASettings = {
    automation: AutomationSettings;
    debtToAddBaseUnit: bigint;
    padding: Uint8Array;
};
export type DCASettingsArgs = {
    automation: AutomationSettingsArgs;
    debtToAddBaseUnit: number | bigint;
    padding: Uint8Array;
};
export declare function getDCASettingsSerializer(): Serializer<DCASettingsArgs, DCASettings>;
//# sourceMappingURL=dCASettings.d.ts.map