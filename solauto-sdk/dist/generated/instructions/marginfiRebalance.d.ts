/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Context, Option, OptionOrNullable, Pda, PublicKey, Signer, TransactionBuilder } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
import { SolautoRebalanceType, SolautoRebalanceTypeArgs } from '../types';
export type MarginfiRebalanceInstructionAccounts = {
    signer: Signer;
    marginfiProgram: PublicKey | Pda;
    systemProgram?: PublicKey | Pda;
    tokenProgram?: PublicKey | Pda;
    ixsSysvar: PublicKey | Pda;
    solautoFeesTa?: PublicKey | Pda;
    authorityReferralState: PublicKey | Pda;
    referredByTa?: PublicKey | Pda;
    positionAuthority?: PublicKey | Pda;
    solautoPosition: PublicKey | Pda;
    marginfiGroup: PublicKey | Pda;
    marginfiAccount: PublicKey | Pda;
    intermediaryTa?: PublicKey | Pda;
    supplyBank: PublicKey | Pda;
    supplyPriceOracle?: PublicKey | Pda;
    positionSupplyTa: PublicKey | Pda;
    authoritySupplyTa?: PublicKey | Pda;
    vaultSupplyTa?: PublicKey | Pda;
    supplyVaultAuthority?: PublicKey | Pda;
    debtBank: PublicKey | Pda;
    debtPriceOracle?: PublicKey | Pda;
    positionDebtTa: PublicKey | Pda;
    authorityDebtTa?: PublicKey | Pda;
    vaultDebtTa?: PublicKey | Pda;
    debtVaultAuthority?: PublicKey | Pda;
};
export type MarginfiRebalanceInstructionData = {
    discriminator: number;
    rebalanceType: SolautoRebalanceType;
    targetLiqUtilizationRateBps: Option<number>;
    targetInAmountBaseUnit: Option<bigint>;
};
export type MarginfiRebalanceInstructionDataArgs = {
    rebalanceType: SolautoRebalanceTypeArgs;
    targetLiqUtilizationRateBps: OptionOrNullable<number>;
    targetInAmountBaseUnit: OptionOrNullable<number | bigint>;
};
export declare function getMarginfiRebalanceInstructionDataSerializer(): Serializer<MarginfiRebalanceInstructionDataArgs, MarginfiRebalanceInstructionData>;
export type MarginfiRebalanceInstructionArgs = MarginfiRebalanceInstructionDataArgs;
export declare function marginfiRebalance(context: Pick<Context, 'programs'>, input: MarginfiRebalanceInstructionAccounts & MarginfiRebalanceInstructionArgs): TransactionBuilder;
//# sourceMappingURL=marginfiRebalance.d.ts.map