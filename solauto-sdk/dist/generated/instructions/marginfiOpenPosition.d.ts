/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Context, Option, OptionOrNullable, Pda, PublicKey, Signer, TransactionBuilder } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
import { PositionType, PositionTypeArgs, UpdatePositionData, UpdatePositionDataArgs } from '../types';
export type MarginfiOpenPositionInstructionAccounts = {
    signer: Signer;
    marginfiProgram: PublicKey | Pda;
    systemProgram?: PublicKey | Pda;
    tokenProgram?: PublicKey | Pda;
    ataProgram?: PublicKey | Pda;
    rent?: PublicKey | Pda;
    signerReferralState: PublicKey | Pda;
    referredByState?: PublicKey | Pda;
    referredBySupplyTa?: PublicKey | Pda;
    solautoPosition: PublicKey | Pda;
    marginfiGroup: PublicKey | Pda;
    marginfiAccount: PublicKey | Pda | Signer;
    supplyMint: PublicKey | Pda;
    supplyBank: PublicKey | Pda;
    positionSupplyTa: PublicKey | Pda;
    debtMint: PublicKey | Pda;
    debtBank: PublicKey | Pda;
    positionDebtTa: PublicKey | Pda;
    signerDebtTa?: PublicKey | Pda;
};
export type MarginfiOpenPositionInstructionData = {
    discriminator: number;
    positionType: PositionType;
    positionData: UpdatePositionData;
    marginfiAccountSeedIdx: Option<bigint>;
};
export type MarginfiOpenPositionInstructionDataArgs = {
    positionType: PositionTypeArgs;
    positionData: UpdatePositionDataArgs;
    marginfiAccountSeedIdx: OptionOrNullable<number | bigint>;
};
export declare function getMarginfiOpenPositionInstructionDataSerializer(): Serializer<MarginfiOpenPositionInstructionDataArgs, MarginfiOpenPositionInstructionData>;
export type MarginfiOpenPositionInstructionArgs = MarginfiOpenPositionInstructionDataArgs;
export declare function marginfiOpenPosition(context: Pick<Context, 'programs'>, input: MarginfiOpenPositionInstructionAccounts & MarginfiOpenPositionInstructionArgs): TransactionBuilder;
//# sourceMappingURL=marginfiOpenPosition.d.ts.map