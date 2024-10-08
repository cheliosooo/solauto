/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Option, OptionOrNullable } from '@metaplex-foundation/umi';
import { GetDataEnumKind, GetDataEnumKindContent, Serializer } from '@metaplex-foundation/umi/serializers';
import { RemainingAccountsInfo, RemainingAccountsInfoArgs, Side, SideArgs } from '.';
export type Swap = {
    __kind: 'Saber';
} | {
    __kind: 'SaberAddDecimalsDeposit';
} | {
    __kind: 'SaberAddDecimalsWithdraw';
} | {
    __kind: 'TokenSwap';
} | {
    __kind: 'Sencha';
} | {
    __kind: 'Step';
} | {
    __kind: 'Cropper';
} | {
    __kind: 'Raydium';
} | {
    __kind: 'Crema';
    aToB: boolean;
} | {
    __kind: 'Lifinity';
} | {
    __kind: 'Mercurial';
} | {
    __kind: 'Cykura';
} | {
    __kind: 'Serum';
    side: Side;
} | {
    __kind: 'MarinadeDeposit';
} | {
    __kind: 'MarinadeUnstake';
} | {
    __kind: 'Aldrin';
    side: Side;
} | {
    __kind: 'AldrinV2';
    side: Side;
} | {
    __kind: 'Whirlpool';
    aToB: boolean;
} | {
    __kind: 'Invariant';
    xToY: boolean;
} | {
    __kind: 'Meteora';
} | {
    __kind: 'GooseFX';
} | {
    __kind: 'DeltaFi';
    stable: boolean;
} | {
    __kind: 'Balansol';
} | {
    __kind: 'MarcoPolo';
    xToY: boolean;
} | {
    __kind: 'Dradex';
    side: Side;
} | {
    __kind: 'LifinityV2';
} | {
    __kind: 'RaydiumClmm';
} | {
    __kind: 'Openbook';
    side: Side;
} | {
    __kind: 'Phoenix';
    side: Side;
} | {
    __kind: 'Symmetry';
    fromTokenId: bigint;
    toTokenId: bigint;
} | {
    __kind: 'TokenSwapV2';
} | {
    __kind: 'HeliumTreasuryManagementRedeemV0';
} | {
    __kind: 'StakeDexStakeWrappedSol';
} | {
    __kind: 'StakeDexSwapViaStake';
    bridgeStakeSeed: number;
} | {
    __kind: 'GooseFXV2';
} | {
    __kind: 'Perps';
} | {
    __kind: 'PerpsAddLiquidity';
} | {
    __kind: 'PerpsRemoveLiquidity';
} | {
    __kind: 'MeteoraDlmm';
} | {
    __kind: 'OpenBookV2';
    side: Side;
} | {
    __kind: 'RaydiumClmmV2';
} | {
    __kind: 'StakeDexPrefundWithdrawStakeAndDepositStake';
    bridgeStakeSeed: number;
} | {
    __kind: 'Clone';
    poolIndex: number;
    quantityIsInput: boolean;
    quantityIsCollateral: boolean;
} | {
    __kind: 'SanctumS';
    srcLstValueCalcAccs: number;
    dstLstValueCalcAccs: number;
    srcLstIndex: number;
    dstLstIndex: number;
} | {
    __kind: 'SanctumSAddLiquidity';
    lstValueCalcAccs: number;
    lstIndex: number;
} | {
    __kind: 'SanctumSRemoveLiquidity';
    lstValueCalcAccs: number;
    lstIndex: number;
} | {
    __kind: 'RaydiumCP';
} | {
    __kind: 'WhirlpoolSwapV2';
    aToB: boolean;
    remainingAccountsInfo: Option<RemainingAccountsInfo>;
} | {
    __kind: 'OneIntro';
} | {
    __kind: 'PumpdotfunWrappedBuy';
} | {
    __kind: 'PumpdotfunWrappedSell';
} | {
    __kind: 'PerpsV2';
} | {
    __kind: 'PerpsV2AddLiquidity';
} | {
    __kind: 'PerpsV2RemoveLiquidity';
} | {
    __kind: 'MoonshotWrappedBuy';
} | {
    __kind: 'MoonshotWrappedSell';
} | {
    __kind: 'StabbleStableSwap';
} | {
    __kind: 'StabbleWeightedSwap';
} | {
    __kind: 'Obric';
    xToY: boolean;
};
export type SwapArgs = {
    __kind: 'Saber';
} | {
    __kind: 'SaberAddDecimalsDeposit';
} | {
    __kind: 'SaberAddDecimalsWithdraw';
} | {
    __kind: 'TokenSwap';
} | {
    __kind: 'Sencha';
} | {
    __kind: 'Step';
} | {
    __kind: 'Cropper';
} | {
    __kind: 'Raydium';
} | {
    __kind: 'Crema';
    aToB: boolean;
} | {
    __kind: 'Lifinity';
} | {
    __kind: 'Mercurial';
} | {
    __kind: 'Cykura';
} | {
    __kind: 'Serum';
    side: SideArgs;
} | {
    __kind: 'MarinadeDeposit';
} | {
    __kind: 'MarinadeUnstake';
} | {
    __kind: 'Aldrin';
    side: SideArgs;
} | {
    __kind: 'AldrinV2';
    side: SideArgs;
} | {
    __kind: 'Whirlpool';
    aToB: boolean;
} | {
    __kind: 'Invariant';
    xToY: boolean;
} | {
    __kind: 'Meteora';
} | {
    __kind: 'GooseFX';
} | {
    __kind: 'DeltaFi';
    stable: boolean;
} | {
    __kind: 'Balansol';
} | {
    __kind: 'MarcoPolo';
    xToY: boolean;
} | {
    __kind: 'Dradex';
    side: SideArgs;
} | {
    __kind: 'LifinityV2';
} | {
    __kind: 'RaydiumClmm';
} | {
    __kind: 'Openbook';
    side: SideArgs;
} | {
    __kind: 'Phoenix';
    side: SideArgs;
} | {
    __kind: 'Symmetry';
    fromTokenId: number | bigint;
    toTokenId: number | bigint;
} | {
    __kind: 'TokenSwapV2';
} | {
    __kind: 'HeliumTreasuryManagementRedeemV0';
} | {
    __kind: 'StakeDexStakeWrappedSol';
} | {
    __kind: 'StakeDexSwapViaStake';
    bridgeStakeSeed: number;
} | {
    __kind: 'GooseFXV2';
} | {
    __kind: 'Perps';
} | {
    __kind: 'PerpsAddLiquidity';
} | {
    __kind: 'PerpsRemoveLiquidity';
} | {
    __kind: 'MeteoraDlmm';
} | {
    __kind: 'OpenBookV2';
    side: SideArgs;
} | {
    __kind: 'RaydiumClmmV2';
} | {
    __kind: 'StakeDexPrefundWithdrawStakeAndDepositStake';
    bridgeStakeSeed: number;
} | {
    __kind: 'Clone';
    poolIndex: number;
    quantityIsInput: boolean;
    quantityIsCollateral: boolean;
} | {
    __kind: 'SanctumS';
    srcLstValueCalcAccs: number;
    dstLstValueCalcAccs: number;
    srcLstIndex: number;
    dstLstIndex: number;
} | {
    __kind: 'SanctumSAddLiquidity';
    lstValueCalcAccs: number;
    lstIndex: number;
} | {
    __kind: 'SanctumSRemoveLiquidity';
    lstValueCalcAccs: number;
    lstIndex: number;
} | {
    __kind: 'RaydiumCP';
} | {
    __kind: 'WhirlpoolSwapV2';
    aToB: boolean;
    remainingAccountsInfo: OptionOrNullable<RemainingAccountsInfoArgs>;
} | {
    __kind: 'OneIntro';
} | {
    __kind: 'PumpdotfunWrappedBuy';
} | {
    __kind: 'PumpdotfunWrappedSell';
} | {
    __kind: 'PerpsV2';
} | {
    __kind: 'PerpsV2AddLiquidity';
} | {
    __kind: 'PerpsV2RemoveLiquidity';
} | {
    __kind: 'MoonshotWrappedBuy';
} | {
    __kind: 'MoonshotWrappedSell';
} | {
    __kind: 'StabbleStableSwap';
} | {
    __kind: 'StabbleWeightedSwap';
} | {
    __kind: 'Obric';
    xToY: boolean;
};
export declare function getSwapSerializer(): Serializer<SwapArgs, Swap>;
export declare function swap(kind: 'Saber'): GetDataEnumKind<SwapArgs, 'Saber'>;
export declare function swap(kind: 'SaberAddDecimalsDeposit'): GetDataEnumKind<SwapArgs, 'SaberAddDecimalsDeposit'>;
export declare function swap(kind: 'SaberAddDecimalsWithdraw'): GetDataEnumKind<SwapArgs, 'SaberAddDecimalsWithdraw'>;
export declare function swap(kind: 'TokenSwap'): GetDataEnumKind<SwapArgs, 'TokenSwap'>;
export declare function swap(kind: 'Sencha'): GetDataEnumKind<SwapArgs, 'Sencha'>;
export declare function swap(kind: 'Step'): GetDataEnumKind<SwapArgs, 'Step'>;
export declare function swap(kind: 'Cropper'): GetDataEnumKind<SwapArgs, 'Cropper'>;
export declare function swap(kind: 'Raydium'): GetDataEnumKind<SwapArgs, 'Raydium'>;
export declare function swap(kind: 'Crema', data: GetDataEnumKindContent<SwapArgs, 'Crema'>): GetDataEnumKind<SwapArgs, 'Crema'>;
export declare function swap(kind: 'Lifinity'): GetDataEnumKind<SwapArgs, 'Lifinity'>;
export declare function swap(kind: 'Mercurial'): GetDataEnumKind<SwapArgs, 'Mercurial'>;
export declare function swap(kind: 'Cykura'): GetDataEnumKind<SwapArgs, 'Cykura'>;
export declare function swap(kind: 'Serum', data: GetDataEnumKindContent<SwapArgs, 'Serum'>): GetDataEnumKind<SwapArgs, 'Serum'>;
export declare function swap(kind: 'MarinadeDeposit'): GetDataEnumKind<SwapArgs, 'MarinadeDeposit'>;
export declare function swap(kind: 'MarinadeUnstake'): GetDataEnumKind<SwapArgs, 'MarinadeUnstake'>;
export declare function swap(kind: 'Aldrin', data: GetDataEnumKindContent<SwapArgs, 'Aldrin'>): GetDataEnumKind<SwapArgs, 'Aldrin'>;
export declare function swap(kind: 'AldrinV2', data: GetDataEnumKindContent<SwapArgs, 'AldrinV2'>): GetDataEnumKind<SwapArgs, 'AldrinV2'>;
export declare function swap(kind: 'Whirlpool', data: GetDataEnumKindContent<SwapArgs, 'Whirlpool'>): GetDataEnumKind<SwapArgs, 'Whirlpool'>;
export declare function swap(kind: 'Invariant', data: GetDataEnumKindContent<SwapArgs, 'Invariant'>): GetDataEnumKind<SwapArgs, 'Invariant'>;
export declare function swap(kind: 'Meteora'): GetDataEnumKind<SwapArgs, 'Meteora'>;
export declare function swap(kind: 'GooseFX'): GetDataEnumKind<SwapArgs, 'GooseFX'>;
export declare function swap(kind: 'DeltaFi', data: GetDataEnumKindContent<SwapArgs, 'DeltaFi'>): GetDataEnumKind<SwapArgs, 'DeltaFi'>;
export declare function swap(kind: 'Balansol'): GetDataEnumKind<SwapArgs, 'Balansol'>;
export declare function swap(kind: 'MarcoPolo', data: GetDataEnumKindContent<SwapArgs, 'MarcoPolo'>): GetDataEnumKind<SwapArgs, 'MarcoPolo'>;
export declare function swap(kind: 'Dradex', data: GetDataEnumKindContent<SwapArgs, 'Dradex'>): GetDataEnumKind<SwapArgs, 'Dradex'>;
export declare function swap(kind: 'LifinityV2'): GetDataEnumKind<SwapArgs, 'LifinityV2'>;
export declare function swap(kind: 'RaydiumClmm'): GetDataEnumKind<SwapArgs, 'RaydiumClmm'>;
export declare function swap(kind: 'Openbook', data: GetDataEnumKindContent<SwapArgs, 'Openbook'>): GetDataEnumKind<SwapArgs, 'Openbook'>;
export declare function swap(kind: 'Phoenix', data: GetDataEnumKindContent<SwapArgs, 'Phoenix'>): GetDataEnumKind<SwapArgs, 'Phoenix'>;
export declare function swap(kind: 'Symmetry', data: GetDataEnumKindContent<SwapArgs, 'Symmetry'>): GetDataEnumKind<SwapArgs, 'Symmetry'>;
export declare function swap(kind: 'TokenSwapV2'): GetDataEnumKind<SwapArgs, 'TokenSwapV2'>;
export declare function swap(kind: 'HeliumTreasuryManagementRedeemV0'): GetDataEnumKind<SwapArgs, 'HeliumTreasuryManagementRedeemV0'>;
export declare function swap(kind: 'StakeDexStakeWrappedSol'): GetDataEnumKind<SwapArgs, 'StakeDexStakeWrappedSol'>;
export declare function swap(kind: 'StakeDexSwapViaStake', data: GetDataEnumKindContent<SwapArgs, 'StakeDexSwapViaStake'>): GetDataEnumKind<SwapArgs, 'StakeDexSwapViaStake'>;
export declare function swap(kind: 'GooseFXV2'): GetDataEnumKind<SwapArgs, 'GooseFXV2'>;
export declare function swap(kind: 'Perps'): GetDataEnumKind<SwapArgs, 'Perps'>;
export declare function swap(kind: 'PerpsAddLiquidity'): GetDataEnumKind<SwapArgs, 'PerpsAddLiquidity'>;
export declare function swap(kind: 'PerpsRemoveLiquidity'): GetDataEnumKind<SwapArgs, 'PerpsRemoveLiquidity'>;
export declare function swap(kind: 'MeteoraDlmm'): GetDataEnumKind<SwapArgs, 'MeteoraDlmm'>;
export declare function swap(kind: 'OpenBookV2', data: GetDataEnumKindContent<SwapArgs, 'OpenBookV2'>): GetDataEnumKind<SwapArgs, 'OpenBookV2'>;
export declare function swap(kind: 'RaydiumClmmV2'): GetDataEnumKind<SwapArgs, 'RaydiumClmmV2'>;
export declare function swap(kind: 'StakeDexPrefundWithdrawStakeAndDepositStake', data: GetDataEnumKindContent<SwapArgs, 'StakeDexPrefundWithdrawStakeAndDepositStake'>): GetDataEnumKind<SwapArgs, 'StakeDexPrefundWithdrawStakeAndDepositStake'>;
export declare function swap(kind: 'Clone', data: GetDataEnumKindContent<SwapArgs, 'Clone'>): GetDataEnumKind<SwapArgs, 'Clone'>;
export declare function swap(kind: 'SanctumS', data: GetDataEnumKindContent<SwapArgs, 'SanctumS'>): GetDataEnumKind<SwapArgs, 'SanctumS'>;
export declare function swap(kind: 'SanctumSAddLiquidity', data: GetDataEnumKindContent<SwapArgs, 'SanctumSAddLiquidity'>): GetDataEnumKind<SwapArgs, 'SanctumSAddLiquidity'>;
export declare function swap(kind: 'SanctumSRemoveLiquidity', data: GetDataEnumKindContent<SwapArgs, 'SanctumSRemoveLiquidity'>): GetDataEnumKind<SwapArgs, 'SanctumSRemoveLiquidity'>;
export declare function swap(kind: 'RaydiumCP'): GetDataEnumKind<SwapArgs, 'RaydiumCP'>;
export declare function swap(kind: 'WhirlpoolSwapV2', data: GetDataEnumKindContent<SwapArgs, 'WhirlpoolSwapV2'>): GetDataEnumKind<SwapArgs, 'WhirlpoolSwapV2'>;
export declare function swap(kind: 'OneIntro'): GetDataEnumKind<SwapArgs, 'OneIntro'>;
export declare function swap(kind: 'PumpdotfunWrappedBuy'): GetDataEnumKind<SwapArgs, 'PumpdotfunWrappedBuy'>;
export declare function swap(kind: 'PumpdotfunWrappedSell'): GetDataEnumKind<SwapArgs, 'PumpdotfunWrappedSell'>;
export declare function swap(kind: 'PerpsV2'): GetDataEnumKind<SwapArgs, 'PerpsV2'>;
export declare function swap(kind: 'PerpsV2AddLiquidity'): GetDataEnumKind<SwapArgs, 'PerpsV2AddLiquidity'>;
export declare function swap(kind: 'PerpsV2RemoveLiquidity'): GetDataEnumKind<SwapArgs, 'PerpsV2RemoveLiquidity'>;
export declare function swap(kind: 'MoonshotWrappedBuy'): GetDataEnumKind<SwapArgs, 'MoonshotWrappedBuy'>;
export declare function swap(kind: 'MoonshotWrappedSell'): GetDataEnumKind<SwapArgs, 'MoonshotWrappedSell'>;
export declare function swap(kind: 'StabbleStableSwap'): GetDataEnumKind<SwapArgs, 'StabbleStableSwap'>;
export declare function swap(kind: 'StabbleWeightedSwap'): GetDataEnumKind<SwapArgs, 'StabbleWeightedSwap'>;
export declare function swap(kind: 'Obric', data: GetDataEnumKindContent<SwapArgs, 'Obric'>): GetDataEnumKind<SwapArgs, 'Obric'>;
export declare function isSwap<K extends Swap['__kind']>(kind: K, value: Swap): value is Swap & {
    __kind: K;
};
//# sourceMappingURL=swap.d.ts.map