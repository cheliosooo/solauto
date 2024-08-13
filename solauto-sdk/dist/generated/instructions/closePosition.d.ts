/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Context, Pda, PublicKey, Signer, TransactionBuilder } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
export type ClosePositionInstructionAccounts = {
    signer: Signer;
    systemProgram?: PublicKey | Pda;
    tokenProgram?: PublicKey | Pda;
    ataProgram?: PublicKey | Pda;
    solautoPosition: PublicKey | Pda;
    signerSupplyTa: PublicKey | Pda;
    positionSupplyTa: PublicKey | Pda;
    positionSupplyCollateralTa?: PublicKey | Pda;
    positionDebtTa: PublicKey | Pda;
    signerDebtTa: PublicKey | Pda;
};
export type ClosePositionInstructionData = {
    discriminator: number;
};
export type ClosePositionInstructionDataArgs = {};
export declare function getClosePositionInstructionDataSerializer(): Serializer<ClosePositionInstructionDataArgs, ClosePositionInstructionData>;
export declare function closePosition(context: Pick<Context, 'programs'>, input: ClosePositionInstructionAccounts): TransactionBuilder;
//# sourceMappingURL=closePosition.d.ts.map