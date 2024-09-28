import { PublicKey } from "@solana/web3.js";
import { Umi } from "@metaplex-foundation/umi";
import { AutomationSettings, DCASettings, DCASettingsInpArgs, PositionState, PositionType, SolautoSettingsParameters, SolautoSettingsParametersInpArgs, TokenType } from "../../generated";
import { RebalanceAction, SolautoPositionDetails } from "../../types/solauto";
export declare function findMintByTicker(ticker: string): PublicKey;
export declare function nextAutomationPeriodTimestamp(automation: AutomationSettings): number;
export declare function eligibleForNextAutomationPeriod(automation: AutomationSettings, currentUnixTime: number): boolean;
export declare function getUpdatedValueFromAutomation(currValue: number, targetValue: number, automation: AutomationSettings, currentUnixTimestamp: number): number;
export declare function getAdjustedSettingsFromAutomation(settings: SolautoSettingsParameters, currentUnixTime: number): SolautoSettingsParameters;
export declare function eligibleForRebalance(positionState: PositionState, positionSettings: SolautoSettingsParameters, positionDca: DCASettings | undefined, currentUnixTime: number, bpsDistanceThreshold?: number): RebalanceAction | undefined;
export declare function eligibleForRefresh(positionState: PositionState, positionSettings: SolautoSettingsParameters, currentUnixTime: number): boolean;
export declare function getSolautoManagedPositions(umi: Umi, authority?: PublicKey, positionTypeFilter?: PositionType): Promise<SolautoPositionDetails[]>;
export declare function getAllReferralStates(umi: Umi): Promise<PublicKey[]>;
export declare function getReferralsByUser(umi: Umi, user: PublicKey): Promise<PublicKey[]>;
export declare function getAllPositionsByAuthority(umi: Umi, user: PublicKey, positionTypeFilter?: PositionType): Promise<SolautoPositionDetails[]>;
export declare function positionStateWithLatestPrices(state: PositionState, supplyPrice?: number, debtPrice?: number): Promise<PositionState>;
interface AssetProps {
    mint: PublicKey;
    price?: number;
    amountUsed?: number;
}
export declare function createFakePositionState(supply: AssetProps, debt: AssetProps, maxLtvBps: number, liqThresholdBps: number): PositionState;
export declare function createSolautoSettings(settings: SolautoSettingsParametersInpArgs): SolautoSettingsParameters;
type PositionAdjustment = {
    type: "supply";
    value: bigint;
} | {
    type: "debt";
    value: bigint;
} | {
    type: "settings";
    value: SolautoSettingsParametersInpArgs;
} | {
    type: "dca";
    value: DCASettingsInpArgs;
} | {
    type: "dcaInBalance";
    value: {
        amount: bigint;
        tokenType: TokenType;
    };
} | {
    type: "cancellingDca";
    value: TokenType;
};
export declare class LivePositionUpdates {
    supplyAdjustment: bigint;
    debtAdjustment: bigint;
    settings: SolautoSettingsParameters | undefined;
    activeDca: DCASettings | undefined;
    dcaInBalance?: {
        amount: bigint;
        tokenType: TokenType;
    };
    cancellingDca: TokenType | undefined;
    new(update: PositionAdjustment): void;
    reset(): void;
    hasUpdates(): boolean;
}
export {};
//# sourceMappingURL=generalUtils.d.ts.map