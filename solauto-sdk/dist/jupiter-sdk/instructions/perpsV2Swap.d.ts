/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Context, Pda, PublicKey, TransactionBuilder } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
export type PerpsV2SwapInstructionAccounts = {
    swapProgram: PublicKey | Pda;
    owner: PublicKey | Pda;
    fundingAccount: PublicKey | Pda;
    receivingAccount: PublicKey | Pda;
    transferAuthority: PublicKey | Pda;
    perpetuals: PublicKey | Pda;
    pool: PublicKey | Pda;
    receivingCustody: PublicKey | Pda;
    receivingCustodyDovesPriceAccount: PublicKey | Pda;
    receivingCustodyPythnetPriceAccount: PublicKey | Pda;
    receivingCustodyTokenAccount: PublicKey | Pda;
    dispensingCustody: PublicKey | Pda;
    dispensingCustodyDovesPriceAccount: PublicKey | Pda;
    dispensingCustodyPythnetPriceAccount: PublicKey | Pda;
    dispensingCustodyTokenAccount: PublicKey | Pda;
    tokenProgram?: PublicKey | Pda;
    eventAuthority: PublicKey | Pda;
    program: PublicKey | Pda;
};
export type PerpsV2SwapInstructionData = {
    discriminator: Array<number>;
};
export type PerpsV2SwapInstructionDataArgs = {};
export declare function getPerpsV2SwapInstructionDataSerializer(): Serializer<PerpsV2SwapInstructionDataArgs, PerpsV2SwapInstructionData>;
export declare function perpsV2Swap(context: Pick<Context, 'programs'>, input: PerpsV2SwapInstructionAccounts): TransactionBuilder;
//# sourceMappingURL=perpsV2Swap.d.ts.map