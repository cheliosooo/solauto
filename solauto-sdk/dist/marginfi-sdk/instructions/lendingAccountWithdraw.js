"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getLendingAccountWithdrawInstructionDataSerializer = getLendingAccountWithdrawInstructionDataSerializer;
exports.lendingAccountWithdraw = lendingAccountWithdraw;
const umi_1 = require("@metaplex-foundation/umi");
const serializers_1 = require("@metaplex-foundation/umi/serializers");
const shared_1 = require("../shared");
function getLendingAccountWithdrawInstructionDataSerializer() {
    return (0, serializers_1.mapSerializer)((0, serializers_1.struct)([
        ['discriminator', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 8 })],
        ['amount', (0, serializers_1.u64)()],
        ['withdrawAll', (0, serializers_1.option)((0, serializers_1.bool)())],
    ], { description: 'LendingAccountWithdrawInstructionData' }), (value) => ({
        ...value,
        discriminator: [36, 72, 74, 19, 210, 210, 192, 192],
    }));
}
// Instruction.
function lendingAccountWithdraw(context, input) {
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
        destinationTokenAccount: {
            index: 4,
            isWritable: true,
            value: input.destinationTokenAccount ?? null,
        },
        bankLiquidityVaultAuthority: {
            index: 5,
            isWritable: true,
            value: input.bankLiquidityVaultAuthority ?? null,
        },
        bankLiquidityVault: {
            index: 6,
            isWritable: true,
            value: input.bankLiquidityVault ?? null,
        },
        tokenProgram: {
            index: 7,
            isWritable: false,
            value: input.tokenProgram ?? null,
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
    const data = getLendingAccountWithdrawInstructionDataSerializer().serialize(resolvedArgs);
    // Bytes Created On Chain.
    const bytesCreatedOnChain = 0;
    return (0, umi_1.transactionBuilder)([
        { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
    ]);
}
