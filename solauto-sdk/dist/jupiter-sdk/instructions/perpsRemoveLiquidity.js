"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getPerpsRemoveLiquidityInstructionDataSerializer = getPerpsRemoveLiquidityInstructionDataSerializer;
exports.perpsRemoveLiquidity = perpsRemoveLiquidity;
const umi_1 = require("@metaplex-foundation/umi");
const serializers_1 = require("@metaplex-foundation/umi/serializers");
const shared_1 = require("../shared");
function getPerpsRemoveLiquidityInstructionDataSerializer() {
    return (0, serializers_1.mapSerializer)((0, serializers_1.struct)([['discriminator', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 8 })]], { description: 'PerpsRemoveLiquidityInstructionData' }), (value) => ({
        ...value,
        discriminator: [79, 211, 232, 140, 8, 78, 220, 34],
    }));
}
// Instruction.
function perpsRemoveLiquidity(context, input) {
    // Program ID.
    const programId = context.programs.getPublicKey('jupiter', 'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4');
    // Accounts.
    const resolvedAccounts = {
        swapProgram: {
            index: 0,
            isWritable: false,
            value: input.swapProgram ?? null,
        },
        owner: {
            index: 1,
            isWritable: true,
            value: input.owner ?? null,
        },
        fundingOrReceivingAccount: {
            index: 2,
            isWritable: true,
            value: input.fundingOrReceivingAccount ?? null,
        },
        lpTokenAccount: {
            index: 3,
            isWritable: true,
            value: input.lpTokenAccount ?? null,
        },
        transferAuthority: {
            index: 4,
            isWritable: false,
            value: input.transferAuthority ?? null,
        },
        perpetuals: {
            index: 5,
            isWritable: false,
            value: input.perpetuals ?? null,
        },
        pool: { index: 6, isWritable: true, value: input.pool ?? null },
        custody: {
            index: 7,
            isWritable: true,
            value: input.custody ?? null,
        },
        custodyOracleAccount: {
            index: 8,
            isWritable: false,
            value: input.custodyOracleAccount ?? null,
        },
        custodyTokenAccount: {
            index: 9,
            isWritable: true,
            value: input.custodyTokenAccount ?? null,
        },
        lpTokenMint: {
            index: 10,
            isWritable: true,
            value: input.lpTokenMint ?? null,
        },
        tokenProgram: {
            index: 11,
            isWritable: false,
            value: input.tokenProgram ?? null,
        },
        eventAuthority: {
            index: 12,
            isWritable: false,
            value: input.eventAuthority ?? null,
        },
        program: {
            index: 13,
            isWritable: false,
            value: input.program ?? null,
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
    const data = getPerpsRemoveLiquidityInstructionDataSerializer().serialize({});
    // Bytes Created On Chain.
    const bytesCreatedOnChain = 0;
    return (0, umi_1.transactionBuilder)([
        { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
    ]);
}
