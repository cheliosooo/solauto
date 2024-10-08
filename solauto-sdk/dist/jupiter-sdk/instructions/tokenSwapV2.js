"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getTokenSwapV2InstructionDataSerializer = getTokenSwapV2InstructionDataSerializer;
exports.tokenSwapV2 = tokenSwapV2;
const umi_1 = require("@metaplex-foundation/umi");
const serializers_1 = require("@metaplex-foundation/umi/serializers");
const shared_1 = require("../shared");
function getTokenSwapV2InstructionDataSerializer() {
    return (0, serializers_1.mapSerializer)((0, serializers_1.struct)([['discriminator', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 8 })]], { description: 'TokenSwapV2InstructionData' }), (value) => ({
        ...value,
        discriminator: [51, 48, 145, 115, 123, 95, 71, 138],
    }));
}
// Instruction.
function tokenSwapV2(context, input) {
    // Program ID.
    const programId = context.programs.getPublicKey('jupiter', 'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4');
    // Accounts.
    const resolvedAccounts = {
        swapProgram: {
            index: 0,
            isWritable: false,
            value: input.swapProgram ?? null,
        },
        swap: { index: 1, isWritable: false, value: input.swap ?? null },
        authority: {
            index: 2,
            isWritable: false,
            value: input.authority ?? null,
        },
        userTransferAuthority: {
            index: 3,
            isWritable: false,
            value: input.userTransferAuthority ?? null,
        },
        source: {
            index: 4,
            isWritable: true,
            value: input.source ?? null,
        },
        swapSource: {
            index: 5,
            isWritable: true,
            value: input.swapSource ?? null,
        },
        swapDestination: {
            index: 6,
            isWritable: true,
            value: input.swapDestination ?? null,
        },
        destination: {
            index: 7,
            isWritable: true,
            value: input.destination ?? null,
        },
        poolMint: {
            index: 8,
            isWritable: true,
            value: input.poolMint ?? null,
        },
        poolFee: {
            index: 9,
            isWritable: true,
            value: input.poolFee ?? null,
        },
        sourceMint: {
            index: 10,
            isWritable: false,
            value: input.sourceMint ?? null,
        },
        destinationMint: {
            index: 11,
            isWritable: false,
            value: input.destinationMint ?? null,
        },
        sourceTokenProgram: {
            index: 12,
            isWritable: false,
            value: input.sourceTokenProgram ?? null,
        },
        destinationTokenProgram: {
            index: 13,
            isWritable: false,
            value: input.destinationTokenProgram ?? null,
        },
        poolTokenProgram: {
            index: 14,
            isWritable: false,
            value: input.poolTokenProgram ?? null,
        },
    };
    // Default values.
    if (!resolvedAccounts.authority.value) {
        resolvedAccounts.authority.value = context.identity.publicKey;
    }
    // Accounts in order.
    const orderedAccounts = Object.values(resolvedAccounts).sort((a, b) => a.index - b.index);
    // Keys and Signers.
    const [keys, signers] = (0, shared_1.getAccountMetasAndSigners)(orderedAccounts, 'programId', programId);
    // Data.
    const data = getTokenSwapV2InstructionDataSerializer().serialize({});
    // Bytes Created On Chain.
    const bytesCreatedOnChain = 0;
    return (0, umi_1.transactionBuilder)([
        { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
    ]);
}
