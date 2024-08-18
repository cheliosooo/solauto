/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Context, Pda, PublicKey, Signer, TransactionBuilder } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
import { BankConfigCompact, BankConfigCompactArgs } from '../types';
export type LendingPoolAddBankWithSeedInstructionAccounts = {
    marginfiGroup: PublicKey | Pda;
    admin: Signer;
    feePayer?: Signer;
    bankMint: PublicKey | Pda;
    bank: PublicKey | Pda;
    liquidityVaultAuthority: PublicKey | Pda;
    liquidityVault: PublicKey | Pda;
    insuranceVaultAuthority: PublicKey | Pda;
    insuranceVault: PublicKey | Pda;
    feeVaultAuthority: PublicKey | Pda;
    feeVault: PublicKey | Pda;
    rent?: PublicKey | Pda;
    tokenProgram?: PublicKey | Pda;
    systemProgram?: PublicKey | Pda;
};
export type LendingPoolAddBankWithSeedInstructionData = {
    discriminator: Array<number>;
    bankConfig: BankConfigCompact;
    bankSeed: bigint;
};
export type LendingPoolAddBankWithSeedInstructionDataArgs = {
    bankConfig: BankConfigCompactArgs;
    bankSeed: number | bigint;
};
export declare function getLendingPoolAddBankWithSeedInstructionDataSerializer(): Serializer<LendingPoolAddBankWithSeedInstructionDataArgs, LendingPoolAddBankWithSeedInstructionData>;
export type LendingPoolAddBankWithSeedInstructionArgs = LendingPoolAddBankWithSeedInstructionDataArgs;
export declare function lendingPoolAddBankWithSeed(context: Pick<Context, 'payer' | 'programs'>, input: LendingPoolAddBankWithSeedInstructionAccounts & LendingPoolAddBankWithSeedInstructionArgs): TransactionBuilder;
//# sourceMappingURL=lendingPoolAddBankWithSeed.d.ts.map