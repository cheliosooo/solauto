"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getSenchaSwapInstructionDataSerializer = getSenchaSwapInstructionDataSerializer;
exports.senchaSwap = senchaSwap;
const umi_1 = require("@metaplex-foundation/umi");
const serializers_1 = require("@metaplex-foundation/umi/serializers");
const shared_1 = require("../shared");
function getSenchaSwapInstructionDataSerializer() {
    return (0, serializers_1.mapSerializer)((0, serializers_1.struct)([['discriminator', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 8 })]], { description: 'SenchaSwapInstructionData' }), (value) => ({
        ...value,
        discriminator: [25, 50, 7, 21, 207, 248, 230, 194],
    }));
}
// Instruction.
function senchaSwap(context, input) {
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
        swap: { index: 2, isWritable: true, value: input.swap ?? null },
        userAuthority: {
            index: 3,
            isWritable: false,
            value: input.userAuthority ?? null,
        },
        inputUserAccount: {
            index: 4,
            isWritable: true,
            value: input.inputUserAccount ?? null,
        },
        inputTokenAccount: {
            index: 5,
            isWritable: true,
            value: input.inputTokenAccount ?? null,
        },
        inputFeesAccount: {
            index: 6,
            isWritable: true,
            value: input.inputFeesAccount ?? null,
        },
        outputUserAccount: {
            index: 7,
            isWritable: true,
            value: input.outputUserAccount ?? null,
        },
        outputTokenAccount: {
            index: 8,
            isWritable: true,
            value: input.outputTokenAccount ?? null,
        },
        outputFeesAccount: {
            index: 9,
            isWritable: true,
            value: input.outputFeesAccount ?? null,
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
    const data = getSenchaSwapInstructionDataSerializer().serialize({});
    // Bytes Created On Chain.
    const bytesCreatedOnChain = 0;
    return (0, umi_1.transactionBuilder)([
        { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
    ]);
}
