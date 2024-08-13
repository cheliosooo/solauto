/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Context, Pda, PublicKey, Signer, TransactionBuilder } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
export type LendingAccountStartFlashloanInstructionAccounts = {
    marginfiAccount: PublicKey | Pda;
    signer: Signer;
    ixsSysvar: PublicKey | Pda;
};
export type LendingAccountStartFlashloanInstructionData = {
    discriminator: Array<number>;
    endIndex: bigint;
};
export type LendingAccountStartFlashloanInstructionDataArgs = {
    endIndex: number | bigint;
};
export declare function getLendingAccountStartFlashloanInstructionDataSerializer(): Serializer<LendingAccountStartFlashloanInstructionDataArgs, LendingAccountStartFlashloanInstructionData>;
export type LendingAccountStartFlashloanInstructionArgs = LendingAccountStartFlashloanInstructionDataArgs;
export declare function lendingAccountStartFlashloan(context: Pick<Context, 'programs'>, input: LendingAccountStartFlashloanInstructionAccounts & LendingAccountStartFlashloanInstructionArgs): TransactionBuilder;
//# sourceMappingURL=lendingAccountStartFlashloan.d.ts.map