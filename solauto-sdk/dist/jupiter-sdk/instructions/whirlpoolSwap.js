"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getWhirlpoolSwapInstructionDataSerializer = getWhirlpoolSwapInstructionDataSerializer;
exports.whirlpoolSwap = whirlpoolSwap;
const umi_1 = require("@metaplex-foundation/umi");
const serializers_1 = require("@metaplex-foundation/umi/serializers");
const shared_1 = require("../shared");
function getWhirlpoolSwapInstructionDataSerializer() {
    return (0, serializers_1.mapSerializer)((0, serializers_1.struct)([['discriminator', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 8 })]], { description: 'WhirlpoolSwapInstructionData' }), (value) => ({
        ...value,
        discriminator: [123, 229, 184, 63, 12, 0, 92, 145],
    }));
}
// Instruction.
function whirlpoolSwap(context, input) {
    // Program ID.
    const programId = context.programs.getPublicKey('jupiter', 'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4');
    // Accounts.
    const resolvedAccounts = {
        swapProgram: {
            index: 0,
            isWritable: false,
            value: input.swapProgram ?? null,
        },
        tokenProgram: {
            index: 1,
            isWritable: false,
            value: input.tokenProgram ?? null,
        },
        tokenAuthority: {
            index: 2,
            isWritable: false,
            value: input.tokenAuthority ?? null,
        },
        whirlpool: {
            index: 3,
            isWritable: true,
            value: input.whirlpool ?? null,
        },
        tokenOwnerAccountA: {
            index: 4,
            isWritable: true,
            value: input.tokenOwnerAccountA ?? null,
        },
        tokenVaultA: {
            index: 5,
            isWritable: true,
            value: input.tokenVaultA ?? null,
        },
        tokenOwnerAccountB: {
            index: 6,
            isWritable: true,
            value: input.tokenOwnerAccountB ?? null,
        },
        tokenVaultB: {
            index: 7,
            isWritable: true,
            value: input.tokenVaultB ?? null,
        },
        tickArray0: {
            index: 8,
            isWritable: true,
            value: input.tickArray0 ?? null,
        },
        tickArray1: {
            index: 9,
            isWritable: true,
            value: input.tickArray1 ?? null,
        },
        tickArray2: {
            index: 10,
            isWritable: true,
            value: input.tickArray2 ?? null,
        },
        oracle: {
            index: 11,
            isWritable: false,
            value: input.oracle ?? null,
        },
    };
    // Default values.
    if (!resolvedAccounts.tokenProgram.value) {
        resolvedAccounts.tokenProgram.value = context.programs.getPublicKey('splToken', 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
        resolvedAccounts.tokenProgram.isWritable = false;
    }
    // Accounts in order.
    const orderedAccounts = Object.values(resolvedAccounts).sort((a, b) => a.index - b.index);
    // Keys and Signers.
    const [keys, signers] = (0, shared_1.getAccountMetasAndSigners)(orderedAccounts, 'programId', programId);
    // Data.
    const data = getWhirlpoolSwapInstructionDataSerializer().serialize({});
    // Bytes Created On Chain.
    const bytesCreatedOnChain = 0;
    return (0, umi_1.transactionBuilder)([
        { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
    ]);
}
