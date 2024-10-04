"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getSetTokenLedgerInstructionDataSerializer = getSetTokenLedgerInstructionDataSerializer;
exports.setTokenLedger = setTokenLedger;
const umi_1 = require("@metaplex-foundation/umi");
const serializers_1 = require("@metaplex-foundation/umi/serializers");
const shared_1 = require("../shared");
function getSetTokenLedgerInstructionDataSerializer() {
    return (0, serializers_1.mapSerializer)((0, serializers_1.struct)([['discriminator', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 8 })]], { description: 'SetTokenLedgerInstructionData' }), (value) => ({ ...value, discriminator: [228, 85, 185, 112, 78, 79, 77, 2] }));
}
// Instruction.
function setTokenLedger(context, input) {
    // Program ID.
    const programId = context.programs.getPublicKey('jupiter', 'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4');
    // Accounts.
    const resolvedAccounts = {
        tokenLedger: {
            index: 0,
            isWritable: true,
            value: input.tokenLedger ?? null,
        },
        tokenAccount: {
            index: 1,
            isWritable: false,
            value: input.tokenAccount ?? null,
        },
    };
    // Accounts in order.
    const orderedAccounts = Object.values(resolvedAccounts).sort((a, b) => a.index - b.index);
    // Keys and Signers.
    const [keys, signers] = (0, shared_1.getAccountMetasAndSigners)(orderedAccounts, 'programId', programId);
    // Data.
    const data = getSetTokenLedgerInstructionDataSerializer().serialize({});
    // Bytes Created On Chain.
    const bytesCreatedOnChain = 0;
    return (0, umi_1.transactionBuilder)([
        { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
    ]);
}
