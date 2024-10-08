/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Context, Pda, PublicKey, TransactionBuilder } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
export type WhirlpoolSwapInstructionAccounts = {
    swapProgram: PublicKey | Pda;
    tokenProgram?: PublicKey | Pda;
    tokenAuthority: PublicKey | Pda;
    whirlpool: PublicKey | Pda;
    tokenOwnerAccountA: PublicKey | Pda;
    tokenVaultA: PublicKey | Pda;
    tokenOwnerAccountB: PublicKey | Pda;
    tokenVaultB: PublicKey | Pda;
    tickArray0: PublicKey | Pda;
    tickArray1: PublicKey | Pda;
    tickArray2: PublicKey | Pda;
    /** Oracle is currently unused and will be enabled on subsequent updates */
    oracle: PublicKey | Pda;
};
export type WhirlpoolSwapInstructionData = {
    discriminator: Array<number>;
};
export type WhirlpoolSwapInstructionDataArgs = {};
export declare function getWhirlpoolSwapInstructionDataSerializer(): Serializer<WhirlpoolSwapInstructionDataArgs, WhirlpoolSwapInstructionData>;
export declare function whirlpoolSwap(context: Pick<Context, 'programs'>, input: WhirlpoolSwapInstructionAccounts): TransactionBuilder;
//# sourceMappingURL=whirlpoolSwap.d.ts.map