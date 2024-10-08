import { PublicKey } from "@solana/web3.js";
import { isOption, isSome, publicKey, Umi } from "@metaplex-foundation/umi";
import {
  AutomationSettings,
  DCASettings,
  DCASettingsInpArgs,
  LendingPlatform,
  PositionState,
  PositionType,
  SOLAUTO_PROGRAM_ID,
  SolautoSettingsParameters,
  SolautoSettingsParametersInpArgs,
  TokenType,
  getReferralStateSize,
  getSolautoPositionAccountDataSerializer,
  getSolautoPositionSize,
} from "../../generated";
import { currentUnixSeconds, fetchTokenPrices } from "../generalUtils";
import {
  fromBaseUnit,
  getLiqUtilzationRateBps,
  toBaseUnit,
} from "../numberUtils";
import { getReferralState } from "../accountUtils";
import { toWeb3JsPublicKey } from "@metaplex-foundation/umi-web3js-adapters";
import {
  ALL_SUPPORTED_TOKENS,
  TOKEN_INFO,
  USD_DECIMALS,
} from "../../constants";
import { getAllMarginfiAccountsByAuthority } from "../marginfiUtils";
import { RebalanceAction, SolautoPositionDetails } from "../../types/solauto";

function newPeriodsPassed(
  automation: AutomationSettings,
  currentUnixTimestamp: number
): number {
  return Math.min(
    automation.targetPeriods,
    automation.periodsPassed +
      Math.floor(
        (currentUnixTimestamp - Number(automation.unixStartDate)) /
          Number(automation.intervalSeconds)
      )
  );
}

export function nextAutomationPeriodTimestamp(
  automation: AutomationSettings
): number {
  return automation.periodsPassed === 0
    ? Number(automation.unixStartDate)
    : Number(automation.unixStartDate) +
        automation.periodsPassed * Number(automation.intervalSeconds);
}

export function eligibleForNextAutomationPeriod(
  automation: AutomationSettings,
  currentUnixTime: number
): boolean {
  return currentUnixTime >= nextAutomationPeriodTimestamp(automation);
}

export function getUpdatedValueFromAutomation(
  currValue: number,
  targetValue: number,
  automation: AutomationSettings,
  currentUnixTimestamp: number
) {
  const currRateDiff = currValue - targetValue;
  const progressPct =
    1 /
    Math.max(
      1,
      automation.targetPeriods -
        newPeriodsPassed(automation, currentUnixTimestamp)
    );
  const newValue = currValue - currRateDiff * progressPct;
  return newValue;
}

export function getAdjustedSettingsFromAutomation(
  settings: SolautoSettingsParameters,
  currentUnixTime: number
): SolautoSettingsParameters {
  const boostToBps =
    settings.automation.targetPeriods > 0 &&
    eligibleForNextAutomationPeriod(settings.automation, currentUnixTime)
      ? getUpdatedValueFromAutomation(
          settings.boostToBps,
          settings.targetBoostToBps,
          settings.automation,
          currentUnixTime
        )
      : settings.boostToBps;

  return {
    ...settings,
    boostToBps,
  };
}

export function eligibleForRebalance(
  positionState: PositionState,
  positionSettings: SolautoSettingsParameters,
  positionDca: DCASettings | undefined,
  currentUnixTime: number,
  bpsDistanceThreshold = 0
): RebalanceAction | undefined {
  if (
    positionDca &&
    positionDca.automation.targetPeriods > 0 &&
    eligibleForNextAutomationPeriod(positionDca.automation, currentUnixTime)
  ) {
    return "dca";
  }

  if (positionState.supply.amountUsed.baseUnit === BigInt(0)) {
    return undefined;
  }

  const boostToBps =
    eligibleForRefresh(positionState, positionSettings, currentUnixTime) &&
    positionSettings.automation.targetPeriods > 0
      ? getUpdatedValueFromAutomation(
          positionSettings.boostToBps,
          positionSettings.targetBoostToBps,
          positionSettings.automation,
          currentUnixTime
        )
      : positionSettings.boostToBps;
  const repayFrom = positionSettings.repayToBps + positionSettings.repayGap;
  const boostFrom = boostToBps - positionSettings.boostGap;

  if (Math.max(0, positionState.liqUtilizationRateBps - boostFrom) <= bpsDistanceThreshold) {
    return "boost";
  } else if (Math.max(0, repayFrom - positionState.liqUtilizationRateBps) <= bpsDistanceThreshold) {
    return "repay";
  }

  return undefined;
}

export function eligibleForRefresh(
  positionState: PositionState,
  positionSettings: SolautoSettingsParameters,
  currentUnixTime: number
): boolean {
  if (positionSettings.automation.targetPeriods > 0) {
    return eligibleForNextAutomationPeriod(
      positionSettings.automation,
      currentUnixTime
    );
  } else {
    return (
      currentUnixSeconds() - Number(positionState.lastUpdated) >
      60 * 60 * 24 * 7
    );
  }
}

export async function getSolautoManagedPositions(
  umi: Umi,
  authority?: PublicKey,
  positionTypeFilter?: PositionType
): Promise<SolautoPositionDetails[]> {
  // bump: [u8; 1]
  // position_id: [u8; 1]
  // self_managed: u8 - (1 for true, 0 for false)
  // position_type: PositionType
  // padding: [u8; 4]
  // authority: pubkey
  // lending_platform: u8
  // padding: [u8; 7]
  // protocol account: pubkey
  // supply mint: pubkey
  // debt mint: pubkey

  const accounts = await umi.rpc.getProgramAccounts(SOLAUTO_PROGRAM_ID, {
    commitment: "confirmed",
    dataSlice: {
      offset: 0,
      length: 1 + 1 + 1 + 1 + 4 + 32 + 1 + 7 + 32 + 32 + 32, // bump + position_id + self_managed + position_type + padding (4) + authority (pubkey) + lending_platform + padding (7) + protocol account (pubkey) + supply mint (pubkey) + debt mint (pubkey)
    },
    filters: [
      {
        dataSize: getSolautoPositionSize(),
      },
      {
        memcmp: {
          bytes: new Uint8Array([0]),
          offset: 2,
        },
      },
      ...(authority
        ? [
            {
              memcmp: {
                bytes: new Uint8Array(authority.toBuffer()),
                offset: 8,
              },
            },
          ]
        : []),
      ...(positionTypeFilter !== undefined ? [
        {
          memcmp: {
            bytes: new Uint8Array(positionTypeFilter),
            offset: 3
          }
        }
      ] : [])
    ],
  });

  return accounts.map((x) => {
    const [position, _] = getSolautoPositionAccountDataSerializer().deserialize(
      new Uint8Array([
        ...x.data,
        ...Array(getSolautoPositionSize() - x.data.length).fill(0),
      ])
    );
    return {
      publicKey: toWeb3JsPublicKey(x.publicKey),
      authority: toWeb3JsPublicKey(position.authority),
      positionId: position.positionId[0],
      lendingPlatform: position.position.lendingPlatform,
      positionType: position.positionType,
      protocolAccount: toWeb3JsPublicKey(position.position.protocolAccount),
      supplyMint: toWeb3JsPublicKey(position.position.supplyMint),
      debtMint: toWeb3JsPublicKey(position.position.debtMint),
    };
  });
}

export async function getAllReferralStates(umi: Umi): Promise<PublicKey[]> {
  const accounts = await umi.rpc.getProgramAccounts(SOLAUTO_PROGRAM_ID, {
    commitment: "confirmed",
    dataSlice: {
      offset: 0,
      length: 0,
    },
    filters: [
      {
        dataSize: getReferralStateSize(),
      },
    ],
  });

  return accounts.map((x) => toWeb3JsPublicKey(x.publicKey));
}

export async function getReferralsByUser(
  umi: Umi,
  user: PublicKey
): Promise<PublicKey[]> {
  // bump: [u8; 1],
  // padding: [u8; 7],
  // authority: Pubkey,
  // referred_by_state: Pubkey,

  const userReferralState = getReferralState(user);
  const accounts = await umi.rpc.getProgramAccounts(SOLAUTO_PROGRAM_ID, {
    commitment: "confirmed",
    dataSlice: {
      offset: 0,
      length: 0,
    },
    filters: [
      {
        dataSize: getReferralStateSize(),
      },
      {
        memcmp: {
          bytes: userReferralState.toBytes(),
          offset: 1 + 7 + 32, // bump + padding + authority - target the referred_by_state field
        },
      },
    ],
  });

  return accounts.map((x) => toWeb3JsPublicKey(x.publicKey));
}

export async function getAllPositionsByAuthority(
  umi: Umi,
  user: PublicKey,
  positionTypeFilter?: PositionType
): Promise<SolautoPositionDetails[]> {
  const solautoCompatiblePositions: SolautoPositionDetails[][] =
    await Promise.all([
      (async () => {
        const solautoManagedPositions = await getSolautoManagedPositions(
          umi,
          user,
          positionTypeFilter
        );
        return solautoManagedPositions.map((x) => ({
          ...x,
          authority: user,
        }));
      })(),
      (async () => {
        if (positionTypeFilter === PositionType.SafeLoan) {
          return [];
        }

        let marginfiPositions = await getAllMarginfiAccountsByAuthority(
          umi,
          user,
          true
        );
        marginfiPositions = marginfiPositions.filter(
          (x) =>
            x.supplyMint &&
            (x.debtMint!.equals(PublicKey.default) ||
              ALL_SUPPORTED_TOKENS.includes(x.debtMint!.toString()))
        );
        return marginfiPositions.map((x) => ({
          publicKey: x.marginfiAccount,
          authority: user,
          positionId: 0,
          positionType: PositionType.Leverage,
          lendingPlatform: LendingPlatform.Marginfi,
          protocolAccount: x.marginfiAccount,
          supplyMint: x.supplyMint,
          debtMint: x.debtMint,
        }));
      })(),
    ]);

  return solautoCompatiblePositions.flat();
}

export async function positionStateWithLatestPrices(
  state: PositionState,
  supplyPrice?: number,
  debtPrice?: number
): Promise<PositionState> {
  if (!supplyPrice || !debtPrice) {
    [supplyPrice, debtPrice] = await fetchTokenPrices([
      toWeb3JsPublicKey(state.supply.mint),
      toWeb3JsPublicKey(state.debt.mint),
    ]);
  }

  const supplyUsd =
    fromBaseUnit(state.supply.amountUsed.baseUnit, state.supply.decimals) *
    supplyPrice;
  const debtUsd =
    fromBaseUnit(state.debt.amountUsed.baseUnit, state.debt.decimals) *
    debtPrice;
  return {
    ...state,
    liqUtilizationRateBps: getLiqUtilzationRateBps(
      supplyUsd,
      debtUsd,
      state.liqThresholdBps
    ),
    netWorth: {
      baseUnit: toBaseUnit(
        (supplyUsd - debtUsd) / supplyPrice,
        state.supply.decimals
      ),
      baseAmountUsdValue: toBaseUnit(supplyUsd - debtUsd, USD_DECIMALS),
    },
    supply: {
      ...state.supply,
      amountUsed: {
        ...state.supply.amountUsed,
        baseAmountUsdValue: toBaseUnit(supplyUsd, USD_DECIMALS),
      },
    },
    debt: {
      ...state.debt,
      amountUsed: {
        ...state.debt.amountUsed,
        baseAmountUsdValue: toBaseUnit(debtUsd, USD_DECIMALS),
      },
    },
  };
}

interface AssetProps {
  mint: PublicKey;
  price?: number;
  amountUsed?: number;
}

export function createFakePositionState(
  supply: AssetProps,
  debt: AssetProps,
  maxLtvBps: number,
  liqThresholdBps: number
): PositionState {
  const supplyDecimals = TOKEN_INFO[supply.mint.toString()].decimals;
  const debtDecimals = TOKEN_INFO[debt.mint.toString()].decimals;

  const supplyUsd = (supply.amountUsed ?? 0) * (supply.price ?? 0);
  const debtUsd = (debt.amountUsed ?? 0) * (debt.price ?? 0);

  return {
    liqUtilizationRateBps: getLiqUtilzationRateBps(
      supplyUsd,
      debtUsd,
      liqThresholdBps
    ),
    supply: {
      amountUsed: {
        baseUnit: toBaseUnit(supply.amountUsed ?? 0, supplyDecimals),
        baseAmountUsdValue: toBaseUnit(supplyUsd, USD_DECIMALS),
      },
      amountCanBeUsed: {
        baseUnit: toBaseUnit(1000000, supplyDecimals),
        baseAmountUsdValue: BigInt(Math.round(1000000 * (supply.price ?? 0))),
      },
      baseAmountMarketPriceUsd: toBaseUnit(supply.price ?? 0, USD_DECIMALS),
      borrowFeeBps: 0,
      decimals: supplyDecimals,
      flashLoanFeeBps: 0,
      mint: publicKey(supply.mint),
      padding1: [],
      padding2: [],
      padding: new Uint8Array([]),
    },
    debt: {
      amountUsed: {
        baseUnit: toBaseUnit(debt.amountUsed ?? 0, debtDecimals),
        baseAmountUsdValue: toBaseUnit(debtUsd, USD_DECIMALS),
      },
      amountCanBeUsed: {
        baseUnit: toBaseUnit(1000000, debtDecimals),
        baseAmountUsdValue: BigInt(Math.round(1000000 * (debt.price ?? 0))),
      },
      baseAmountMarketPriceUsd: toBaseUnit(debt.price ?? 0, USD_DECIMALS),
      borrowFeeBps: 0,
      decimals: debtDecimals,
      flashLoanFeeBps: 0,
      mint: publicKey(debt.mint),
      padding1: [],
      padding2: [],
      padding: new Uint8Array([]),
    },
    netWorth: {
      baseUnit: supply.price
        ? toBaseUnit((supplyUsd - debtUsd) / supply.price, supplyDecimals)
        : BigInt(0),
      baseAmountUsdValue: toBaseUnit(supplyUsd - debtUsd, USD_DECIMALS),
    },
    maxLtvBps,
    liqThresholdBps,
    lastUpdated: BigInt(currentUnixSeconds()),
    padding1: [],
    padding2: [],
    padding: [],
  };
}

export function createSolautoSettings(
  settings: SolautoSettingsParametersInpArgs
): SolautoSettingsParameters {
  return {
    automation:
      isOption(settings.automation) && isSome(settings.automation)
        ? {
            ...settings.automation.value,
            intervalSeconds: BigInt(settings.automation.value.intervalSeconds),
            unixStartDate: BigInt(settings.automation.value.unixStartDate),
            padding: new Uint8Array([]),
            padding1: [],
          }
        : {
            targetPeriods: 0,
            periodsPassed: 0,
            intervalSeconds: BigInt(0),
            unixStartDate: BigInt(0),
            padding: new Uint8Array([]),
            padding1: [],
          },
    targetBoostToBps:
      isOption(settings.targetBoostToBps) && isSome(settings.targetBoostToBps)
        ? settings.targetBoostToBps.value
        : 0,
    boostGap: settings.boostGap,
    boostToBps: settings.boostToBps,
    repayGap: settings.repayGap,
    repayToBps: settings.repayToBps,
    padding: new Uint8Array([]),
    padding1: [],
  };
}

type PositionAdjustment =
  | { type: "supply"; value: bigint }
  | { type: "debt"; value: bigint }
  | { type: "settings"; value: SolautoSettingsParametersInpArgs }
  | { type: "dca"; value: DCASettingsInpArgs }
  | { type: "dcaInBalance"; value: { amount: bigint; tokenType: TokenType; } }
  | { type: "cancellingDca"; value: TokenType; };

export class LivePositionUpdates {
  public supplyAdjustment = BigInt(0);
  public debtAdjustment = BigInt(0);
  public settings: SolautoSettingsParameters | undefined = undefined;
  public activeDca: DCASettings | undefined = undefined;
  public dcaInBalance?: { amount: bigint; tokenType: TokenType; } = undefined;
  public cancellingDca: TokenType | undefined = undefined;
  
  new(update: PositionAdjustment) {
    if (update.type === "supply") {
      this.supplyAdjustment += update.value;
    } else if (update.type === "debt") {
      this.debtAdjustment += update.value;
    } else if (update.type === "settings") {
      const settings = update.value;
      this.settings = createSolautoSettings(settings);
    } else if (update.type === "dca") {
      const dca = update.value;
      this.activeDca = {
        automation: {
          ...dca.automation,
          intervalSeconds: BigInt(dca.automation.intervalSeconds),
          unixStartDate: BigInt(dca.automation.unixStartDate),
          padding: new Uint8Array([]),
          padding1: [],
        },
        dcaInBaseUnit: BigInt(dca.dcaInBaseUnit),
        tokenType: dca.tokenType,
        padding: [],
      };
    } else if (update.type === "cancellingDca") {
      this.cancellingDca = update.value;
    } else if (update.type === "dcaInBalance") {
      this.dcaInBalance = update.value;
    }
  }

  reset() {
    this.supplyAdjustment = BigInt(0);
    this.debtAdjustment = BigInt(0);
    this.settings = undefined;
    this.activeDca = undefined;
    this.dcaInBalance = undefined;
    this.cancellingDca = undefined;
  }

  hasUpdates(): boolean {
    return (
      this.supplyAdjustment !== BigInt(0) ||
      this.debtAdjustment !== BigInt(0) ||
      this.dcaInBalance !== undefined ||
      this.settings !== undefined ||
      this.cancellingDca !== undefined
    );
  }
}
