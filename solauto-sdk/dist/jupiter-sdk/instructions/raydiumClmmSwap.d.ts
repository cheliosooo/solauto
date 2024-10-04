/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Context, Pda, PublicKey, TransactionBuilder } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
export type RaydiumClmmSwapInstructionAccounts = {
    swapProgram: PublicKey | Pda;
    payer?: PublicKey | Pda;
    ammConfig: PublicKey | Pda;
    poolState: PublicKey | Pda;
    inputTokenAccount: PublicKey | Pda;
    outputTokenAccount: PublicKey | Pda;
    inputVault: PublicKey | Pda;
    outputVault: PublicKey | Pda;
    observationState: PublicKey | Pda;
    tokenProgram?: PublicKey | Pda;
    tickArray: PublicKey | Pda;
};
export type RaydiumClmmSwapInstructionData = {
    discriminator: Array<number>;
};
export type RaydiumClmmSwapInstructionDataArgs = {};
export declare function getRaydiumClmmSwapInstructionDataSerializer(): Serializer<RaydiumClmmSwapInstructionDataArgs, RaydiumClmmSwapInstructionData>;
export declare function raydiumClmmSwap(context: Pick<Context, 'payer' | 'programs'>, input: RaydiumClmmSwapInstructionAccounts): TransactionBuilder;
//# sourceMappingURL=raydiumClmmSwap.d.ts.map