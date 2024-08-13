"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getLendingAccountWithdrawEmissionsInstructionDataSerializer = getLendingAccountWithdrawEmissionsInstructionDataSerializer;
exports.lendingAccountWithdrawEmissions = lendingAccountWithdrawEmissions;
const umi_1 = require("@metaplex-foundation/umi");
const serializers_1 = require("@metaplex-foundation/umi/serializers");
const shared_1 = require("../shared");
function getLendingAccountWithdrawEmissionsInstructionDataSerializer() {
    return (0, serializers_1.mapSerializer)((0, serializers_1.struct)([['discriminator', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 8 })]], { description: 'LendingAccountWithdrawEmissionsInstructionData' }), (value) => ({
        ...value,
        discriminator: [234, 22, 84, 214, 118, 176, 140, 170],
    }));
}
// Instruction.
function lendingAccountWithdrawEmissions(context, input) {
    // Program ID.
    const programId = context.programs.getPublicKey('marginfi', 'MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA');
    // Accounts.
    const resolvedAccounts = {
        marginfiGroup: {
            index: 0,
            isWritable: false,
            value: input.marginfiGroup ?? null,
        },
        marginfiAccount: {
            index: 1,
            isWritable: true,
            value: input.marginfiAccount ?? null,
        },
        signer: {
            index: 2,
            isWritable: false,
            value: input.signer ?? null,
        },
        bank: { index: 3, isWritable: true, value: input.bank ?? null },
        emissionsMint: {
            index: 4,
            isWritable: false,
            value: input.emissionsMint ?? null,
        },
        emissionsAuth: {
            index: 5,
            isWritable: false,
            value: input.emissionsAuth ?? null,
        },
        emissionsVault: {
            index: 6,
            isWritable: true,
            value: input.emissionsVault ?? null,
        },
        destinationAccount: {
            index: 7,
            isWritable: true,
            value: input.destinationAccount ?? null,
        },
        tokenProgram: {
            index: 8,
            isWritable: false,
            value: input.tokenProgram ?? null,
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
    const data = getLendingAccountWithdrawEmissionsInstructionDataSerializer().serialize({});
    // Bytes Created On Chain.
    const bytesCreatedOnChain = 0;
    return (0, umi_1.transactionBuilder)([
        { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
    ]);
}
