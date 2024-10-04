"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getDradexSwapInstructionDataSerializer = getDradexSwapInstructionDataSerializer;
exports.dradexSwap = dradexSwap;
const umi_1 = require("@metaplex-foundation/umi");
const serializers_1 = require("@metaplex-foundation/umi/serializers");
const shared_1 = require("../shared");
function getDradexSwapInstructionDataSerializer() {
    return (0, serializers_1.mapSerializer)((0, serializers_1.struct)([['discriminator', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 8 })]], { description: 'DradexSwapInstructionData' }), (value) => ({
        ...value,
        discriminator: [34, 146, 160, 38, 51, 85, 58, 151],
    }));
}
// Instruction.
function dradexSwap(context, input) {
    // Program ID.
    const programId = context.programs.getPublicKey('jupiter', 'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4');
    // Accounts.
    const resolvedAccounts = {
        swapProgram: {
            index: 0,
            isWritable: false,
            value: input.swapProgram ?? null,
        },
        pair: { index: 1, isWritable: true, value: input.pair ?? null },
        market: {
            index: 2,
            isWritable: true,
            value: input.market ?? null,
        },
        eventQueue: {
            index: 3,
            isWritable: true,
            value: input.eventQueue ?? null,
        },
        dexUser: {
            index: 4,
            isWritable: false,
            value: input.dexUser ?? null,
        },
        marketUser: {
            index: 5,
            isWritable: true,
            value: input.marketUser ?? null,
        },
        bids: { index: 6, isWritable: true, value: input.bids ?? null },
        asks: { index: 7, isWritable: true, value: input.asks ?? null },
        t0Vault: {
            index: 8,
            isWritable: true,
            value: input.t0Vault ?? null,
        },
        t1Vault: {
            index: 9,
            isWritable: true,
            value: input.t1Vault ?? null,
        },
        t0User: {
            index: 10,
            isWritable: true,
            value: input.t0User ?? null,
        },
        t1User: {
            index: 11,
            isWritable: true,
            value: input.t1User ?? null,
        },
        master: {
            index: 12,
            isWritable: false,
            value: input.master ?? null,
        },
        signer: {
            index: 13,
            isWritable: true,
            value: input.signer ?? null,
        },
        systemProgram: {
            index: 14,
            isWritable: false,
            value: input.systemProgram ?? null,
        },
        tokenProgram: {
            index: 15,
            isWritable: false,
            value: input.tokenProgram ?? null,
        },
        logger: {
            index: 16,
            isWritable: false,
            value: input.logger ?? null,
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
    // Accounts in order.
    const orderedAccounts = Object.values(resolvedAccounts).sort((a, b) => a.index - b.index);
    // Keys and Signers.
    const [keys, signers] = (0, shared_1.getAccountMetasAndSigners)(orderedAccounts, 'programId', programId);
    // Data.
    const data = getDradexSwapInstructionDataSerializer().serialize({});
    // Bytes Created On Chain.
    const bytesCreatedOnChain = 0;
    return (0, umi_1.transactionBuilder)([
        { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
    ]);
}
