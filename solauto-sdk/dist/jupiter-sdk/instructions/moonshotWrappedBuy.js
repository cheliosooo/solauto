"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getMoonshotWrappedBuyInstructionDataSerializer = getMoonshotWrappedBuyInstructionDataSerializer;
exports.moonshotWrappedBuy = moonshotWrappedBuy;
const umi_1 = require("@metaplex-foundation/umi");
const serializers_1 = require("@metaplex-foundation/umi/serializers");
const shared_1 = require("../shared");
function getMoonshotWrappedBuyInstructionDataSerializer() {
    return (0, serializers_1.mapSerializer)((0, serializers_1.struct)([['discriminator', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 8 })]], { description: 'MoonshotWrappedBuyInstructionData' }), (value) => ({
        ...value,
        discriminator: [207, 150, 213, 156, 138, 104, 238, 142],
    }));
}
// Instruction.
function moonshotWrappedBuy(context, input) {
    // Program ID.
    const programId = context.programs.getPublicKey('jupiter', 'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4');
    // Accounts.
    const resolvedAccounts = {
        swapProgram: {
            index: 0,
            isWritable: false,
            value: input.swapProgram ?? null,
        },
        sender: {
            index: 1,
            isWritable: true,
            value: input.sender ?? null,
        },
        senderTokenAccount: {
            index: 2,
            isWritable: true,
            value: input.senderTokenAccount ?? null,
        },
        curveAccount: {
            index: 3,
            isWritable: true,
            value: input.curveAccount ?? null,
        },
        curveTokenAccount: {
            index: 4,
            isWritable: true,
            value: input.curveTokenAccount ?? null,
        },
        dexFee: {
            index: 5,
            isWritable: true,
            value: input.dexFee ?? null,
        },
        helioFee: {
            index: 6,
            isWritable: true,
            value: input.helioFee ?? null,
        },
        mint: { index: 7, isWritable: false, value: input.mint ?? null },
        configAccount: {
            index: 8,
            isWritable: false,
            value: input.configAccount ?? null,
        },
        tokenProgram: {
            index: 9,
            isWritable: false,
            value: input.tokenProgram ?? null,
        },
        associatedTokenProgram: {
            index: 10,
            isWritable: false,
            value: input.associatedTokenProgram ?? null,
        },
        systemProgram: {
            index: 11,
            isWritable: false,
            value: input.systemProgram ?? null,
        },
        userWsolTokenAccount: {
            index: 12,
            isWritable: true,
            value: input.userWsolTokenAccount ?? null,
        },
        tempWsolTokenAccount: {
            index: 13,
            isWritable: true,
            value: input.tempWsolTokenAccount ?? null,
        },
        wsolMint: {
            index: 14,
            isWritable: false,
            value: input.wsolMint ?? null,
        },
    };
    // Default values.
    if (!resolvedAccounts.tokenProgram.value) {
        resolvedAccounts.tokenProgram.value = context.programs.getPublicKey('splToken', 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
        resolvedAccounts.tokenProgram.isWritable = false;
    }
    if (!resolvedAccounts.systemProgram.value) {
        resolvedAccounts.systemProgram.value = context.programs.getPublicKey('splSystem', '11111111111111111111111111111111');
        resolvedAccounts.systemProgram.isWritable = false;
    }
    // Accounts in order.
    const orderedAccounts = Object.values(resolvedAccounts).sort((a, b) => a.index - b.index);
    // Keys and Signers.
    const [keys, signers] = (0, shared_1.getAccountMetasAndSigners)(orderedAccounts, 'programId', programId);
    // Data.
    const data = getMoonshotWrappedBuyInstructionDataSerializer().serialize({});
    // Bytes Created On Chain.
    const bytesCreatedOnChain = 0;
    return (0, umi_1.transactionBuilder)([
        { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
    ]);
}
