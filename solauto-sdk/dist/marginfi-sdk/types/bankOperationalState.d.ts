/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Serializer } from '@metaplex-foundation/umi/serializers';
export declare enum BankOperationalState {
    Paused = 0,
    Operational = 1,
    ReduceOnly = 2
}
export type BankOperationalStateArgs = BankOperationalState;
export declare function getBankOperationalStateSerializer(): Serializer<BankOperationalStateArgs, BankOperationalState>;
//# sourceMappingURL=bankOperationalState.d.ts.map