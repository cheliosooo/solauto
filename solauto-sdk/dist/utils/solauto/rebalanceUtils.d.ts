import { PublicKey } from "@solana/web3.js";
import { SolautoClient } from "../../clients/solautoClient";
import { DCASettings, PositionState, SolautoSettingsParameters, TokenType } from "../../generated";
import { QuoteResponse } from "@jup-ag/api";
import { JupSwapDetails } from "../jupiterUtils";
export interface RebalanceValues {
    increasingLeverage: boolean;
    debtAdjustmentUsd: number;
    amountToDcaIn: number;
    amountUsdToDcaIn: number;
    dcaTokenType?: TokenType;
}
export declare function getRebalanceValues(state: PositionState, settings: SolautoSettingsParameters | undefined, dca: DCASettings | undefined, currentUnixTime: number, supplyPrice: number, debtPrice: number, targetLiqUtilizationRateBps?: number, limitGapBps?: number): RebalanceValues;
export interface FlashLoanDetails {
    baseUnitAmount: bigint;
    mint: PublicKey;
}
export declare function getFlashLoanDetails(client: SolautoClient, values: RebalanceValues, jupQuote: QuoteResponse, priceImpactBps: number): FlashLoanDetails | undefined;
export declare function getJupSwapRebalanceDetails(client: SolautoClient, values: RebalanceValues, targetLiqUtilizationRateBps?: number, attemptNum?: number): JupSwapDetails;
//# sourceMappingURL=rebalanceUtils.d.ts.map