/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Context, Pda, PublicKey, TransactionBuilder } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
export type PhoenixSwapInstructionAccounts = {
    swapProgram: PublicKey | Pda;
    logAuthority: PublicKey | Pda;
    market: PublicKey | Pda;
    trader: PublicKey | Pda;
    baseAccount: PublicKey | Pda;
    quoteAccount: PublicKey | Pda;
    baseVault: PublicKey | Pda;
    quoteVault: PublicKey | Pda;
    tokenProgram?: PublicKey | Pda;
};
export type PhoenixSwapInstructionData = {
    discriminator: Array<number>;
};
export type PhoenixSwapInstructionDataArgs = {};
export declare function getPhoenixSwapInstructionDataSerializer(): Serializer<PhoenixSwapInstructionDataArgs, PhoenixSwapInstructionData>;
export declare function phoenixSwap(context: Pick<Context, 'programs'>, input: PhoenixSwapInstructionAccounts): TransactionBuilder;
//# sourceMappingURL=phoenixSwap.d.ts.map