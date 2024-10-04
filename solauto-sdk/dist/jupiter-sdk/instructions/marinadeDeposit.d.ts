/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Context, Pda, PublicKey, TransactionBuilder } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
export type MarinadeDepositInstructionAccounts = {
    marinadeFinanceProgram: PublicKey | Pda;
    state: PublicKey | Pda;
    msolMint: PublicKey | Pda;
    liqPoolSolLegPda: PublicKey | Pda;
    liqPoolMsolLeg: PublicKey | Pda;
    liqPoolMsolLegAuthority: PublicKey | Pda;
    reservePda: PublicKey | Pda;
    transferFrom: PublicKey | Pda;
    mintTo: PublicKey | Pda;
    msolMintAuthority: PublicKey | Pda;
    systemProgram?: PublicKey | Pda;
    tokenProgram?: PublicKey | Pda;
    userWsolTokenAccount: PublicKey | Pda;
    tempWsolTokenAccount: PublicKey | Pda;
    userTransferAuthority: PublicKey | Pda;
    payer?: PublicKey | Pda;
    wsolMint: PublicKey | Pda;
    rent?: PublicKey | Pda;
};
export type MarinadeDepositInstructionData = {
    discriminator: Array<number>;
};
export type MarinadeDepositInstructionDataArgs = {};
export declare function getMarinadeDepositInstructionDataSerializer(): Serializer<MarinadeDepositInstructionDataArgs, MarinadeDepositInstructionData>;
export declare function marinadeDeposit(context: Pick<Context, 'payer' | 'programs'>, input: MarinadeDepositInstructionAccounts): TransactionBuilder;
//# sourceMappingURL=marinadeDeposit.d.ts.map