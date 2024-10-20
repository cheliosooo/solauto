"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getClaimReferralFeesInstructionDataSerializer = getClaimReferralFeesInstructionDataSerializer;
exports.claimReferralFees = claimReferralFees;
const umi_1 = require("@metaplex-foundation/umi");
const serializers_1 = require("@metaplex-foundation/umi/serializers");
const shared_1 = require("../shared");
function getClaimReferralFeesInstructionDataSerializer() {
    return (0, serializers_1.mapSerializer)((0, serializers_1.struct)([['discriminator', (0, serializers_1.u8)()]], {
        description: 'ClaimReferralFeesInstructionData',
    }), (value) => ({ ...value, discriminator: 2 }));
}
// Instruction.
function claimReferralFees(context, input) {
    // Program ID.
    const programId = context.programs.getPublicKey('solauto', 'AutoyKBRaHSBHy9RsmXCZMy6nNFAg5FYijrvZyQcNLV');
    // Accounts.
    const resolvedAccounts = {
        signer: {
            index: 0,
            isWritable: false,
            value: input.signer ?? null,
        },
        signerWsolTa: {
            index: 1,
            isWritable: true,
            value: input.signerWsolTa ?? null,
        },
        systemProgram: {
            index: 2,
            isWritable: false,
            value: input.systemProgram ?? null,
        },
        tokenProgram: {
            index: 3,
            isWritable: false,
            value: input.tokenProgram ?? null,
        },
        ataProgram: {
            index: 4,
            isWritable: false,
            value: input.ataProgram ?? null,
        },
        rent: { index: 5, isWritable: false, value: input.rent ?? null },
        referralState: {
            index: 6,
            isWritable: false,
            value: input.referralState ?? null,
        },
        referralFeesDestTa: {
            index: 7,
            isWritable: true,
            value: input.referralFeesDestTa ?? null,
        },
        referralFeesDestMint: {
            index: 8,
            isWritable: false,
            value: input.referralFeesDestMint ?? null,
        },
        referralAuthority: {
            index: 9,
            isWritable: true,
            value: input.referralAuthority ?? null,
        },
        feesDestinationTa: {
            index: 10,
            isWritable: true,
            value: input.feesDestinationTa ?? null,
        },
    };
    // Default values.
    if (!resolvedAccounts.systemProgram.value) {
        resolvedAccounts.systemProgram.value = context.programs.getPublicKey('splSystem', '11111111111111111111111111111111');
        resolvedAccounts.systemProgram.isWritable = false;
    }
    if (!resolvedAccounts.tokenProgram.value) {
        resolvedAccounts.tokenProgram.value = context.programs.getPublicKey('splToken', 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
        resolvedAccounts.tokenProgram.isWritable = false;
    }
    if (!resolvedAccounts.ataProgram.value) {
        resolvedAccounts.ataProgram.value = context.programs.getPublicKey('splAssociatedToken', 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');
        resolvedAccounts.ataProgram.isWritable = false;
    }
    if (!resolvedAccounts.rent.value) {
        resolvedAccounts.rent.value = (0, umi_1.publicKey)('SysvarRent111111111111111111111111111111111');
    }
    // Accounts in order.
    const orderedAccounts = Object.values(resolvedAccounts).sort((a, b) => a.index - b.index);
    // Keys and Signers.
    const [keys, signers] = (0, shared_1.getAccountMetasAndSigners)(orderedAccounts, 'programId', programId);
    // Data.
    const data = getClaimReferralFeesInstructionDataSerializer().serialize({});
    // Bytes Created On Chain.
    const bytesCreatedOnChain = 0;
    return (0, umi_1.transactionBuilder)([
        { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
    ]);
}
