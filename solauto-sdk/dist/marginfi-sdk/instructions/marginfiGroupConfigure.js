"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getMarginfiGroupConfigureInstructionDataSerializer = getMarginfiGroupConfigureInstructionDataSerializer;
exports.marginfiGroupConfigure = marginfiGroupConfigure;
const umi_1 = require("@metaplex-foundation/umi");
const serializers_1 = require("@metaplex-foundation/umi/serializers");
const shared_1 = require("../shared");
function getMarginfiGroupConfigureInstructionDataSerializer() {
    return (0, serializers_1.mapSerializer)((0, serializers_1.struct)([
        ['discriminator', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 8 })],
        ['admin', (0, serializers_1.option)((0, serializers_1.publicKey)())],
    ], { description: 'MarginfiGroupConfigureInstructionData' }), (value) => ({ ...value, discriminator: [62, 199, 81, 78, 33, 13, 236, 61] }));
}
// Instruction.
function marginfiGroupConfigure(context, accounts, args) {
    // Program ID.
    const programId = context.programs.getPublicKey('marginfi', 'MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA');
    // Accounts.
    const resolvedAccounts = {
        marginfiGroup: {
            index: 0,
            isWritable: true,
            value: accounts.marginfiGroup ?? null,
        },
        admin: {
            index: 1,
            isWritable: false,
            value: accounts.admin ?? null,
        },
    };
    // Arguments.
    const resolvedArgs = { ...args };
    // Accounts in order.
    const orderedAccounts = Object.values(resolvedAccounts).sort((a, b) => a.index - b.index);
    // Keys and Signers.
    const [keys, signers] = (0, shared_1.getAccountMetasAndSigners)(orderedAccounts, 'programId', programId);
    // Data.
    const data = getMarginfiGroupConfigureInstructionDataSerializer().serialize(resolvedArgs);
    // Bytes Created On Chain.
    const bytesCreatedOnChain = 0;
    return (0, umi_1.transactionBuilder)([
        { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
    ]);
}
