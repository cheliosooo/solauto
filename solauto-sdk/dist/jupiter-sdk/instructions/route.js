"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getRouteInstructionDataSerializer = getRouteInstructionDataSerializer;
exports.route = route;
const umi_1 = require("@metaplex-foundation/umi");
const serializers_1 = require("@metaplex-foundation/umi/serializers");
const shared_1 = require("../shared");
const types_1 = require("../types");
function getRouteInstructionDataSerializer() {
    return (0, serializers_1.mapSerializer)((0, serializers_1.struct)([
        ['discriminator', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 8 })],
        ['routePlan', (0, serializers_1.array)((0, types_1.getRoutePlanStepSerializer)())],
        ['inAmount', (0, serializers_1.u64)()],
        ['quotedOutAmount', (0, serializers_1.u64)()],
        ['slippageBps', (0, serializers_1.u16)()],
        ['platformFeeBps', (0, serializers_1.u8)()],
    ], { description: 'RouteInstructionData' }), (value) => ({
        ...value,
        discriminator: [229, 23, 203, 151, 122, 227, 173, 42],
    }));
}
// Instruction.
function route(context, input) {
    // Program ID.
    const programId = context.programs.getPublicKey('jupiter', 'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4');
    // Accounts.
    const resolvedAccounts = {
        tokenProgram: {
            index: 0,
            isWritable: false,
            value: input.tokenProgram ?? null,
        },
        userTransferAuthority: {
            index: 1,
            isWritable: false,
            value: input.userTransferAuthority ?? null,
        },
        userSourceTokenAccount: {
            index: 2,
            isWritable: true,
            value: input.userSourceTokenAccount ?? null,
        },
        userDestinationTokenAccount: {
            index: 3,
            isWritable: true,
            value: input.userDestinationTokenAccount ?? null,
        },
        destinationTokenAccount: {
            index: 4,
            isWritable: true,
            value: input.destinationTokenAccount ?? null,
        },
        destinationMint: {
            index: 5,
            isWritable: false,
            value: input.destinationMint ?? null,
        },
        platformFeeAccount: {
            index: 6,
            isWritable: true,
            value: input.platformFeeAccount ?? null,
        },
        eventAuthority: {
            index: 7,
            isWritable: false,
            value: input.eventAuthority ?? null,
        },
        program: {
            index: 8,
            isWritable: false,
            value: input.program ?? null,
        },
    };
    // Arguments.
    const resolvedArgs = { ...input };
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
    const data = getRouteInstructionDataSerializer().serialize(resolvedArgs);
    // Bytes Created On Chain.
    const bytesCreatedOnChain = 0;
    return (0, umi_1.transactionBuilder)([
        { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
    ]);
}
