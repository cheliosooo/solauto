"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getReferralStateAccountDataSerializer = getReferralStateAccountDataSerializer;
exports.deserializeReferralState = deserializeReferralState;
exports.fetchReferralState = fetchReferralState;
exports.safeFetchReferralState = safeFetchReferralState;
exports.fetchAllReferralState = fetchAllReferralState;
exports.safeFetchAllReferralState = safeFetchAllReferralState;
exports.getReferralStateGpaBuilder = getReferralStateGpaBuilder;
exports.getReferralStateSize = getReferralStateSize;
const umi_1 = require("@metaplex-foundation/umi");
const serializers_1 = require("@metaplex-foundation/umi/serializers");
function getReferralStateAccountDataSerializer() {
    return (0, serializers_1.struct)([
        ['bump', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 1 })],
        ['padding1', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 7 })],
        ['authority', (0, serializers_1.publicKey)()],
        ['referredByState', (0, serializers_1.publicKey)()],
        ['destFeesMint', (0, serializers_1.publicKey)()],
        ['lookupTable', (0, serializers_1.publicKey)()],
        ['padding', (0, serializers_1.array)((0, serializers_1.u8)(), { size: 96 })],
    ], { description: 'ReferralStateAccountData' });
}
function deserializeReferralState(rawAccount) {
    return (0, umi_1.deserializeAccount)(rawAccount, getReferralStateAccountDataSerializer());
}
async function fetchReferralState(context, publicKey, options) {
    const maybeAccount = await context.rpc.getAccount((0, umi_1.publicKey)(publicKey, false), options);
    (0, umi_1.assertAccountExists)(maybeAccount, 'ReferralState');
    return deserializeReferralState(maybeAccount);
}
async function safeFetchReferralState(context, publicKey, options) {
    const maybeAccount = await context.rpc.getAccount((0, umi_1.publicKey)(publicKey, false), options);
    return maybeAccount.exists ? deserializeReferralState(maybeAccount) : null;
}
async function fetchAllReferralState(context, publicKeys, options) {
    const maybeAccounts = await context.rpc.getAccounts(publicKeys.map((key) => (0, umi_1.publicKey)(key, false)), options);
    return maybeAccounts.map((maybeAccount) => {
        (0, umi_1.assertAccountExists)(maybeAccount, 'ReferralState');
        return deserializeReferralState(maybeAccount);
    });
}
async function safeFetchAllReferralState(context, publicKeys, options) {
    const maybeAccounts = await context.rpc.getAccounts(publicKeys.map((key) => (0, umi_1.publicKey)(key, false)), options);
    return maybeAccounts
        .filter((maybeAccount) => maybeAccount.exists)
        .map((maybeAccount) => deserializeReferralState(maybeAccount));
}
function getReferralStateGpaBuilder(context) {
    const programId = context.programs.getPublicKey('solauto', 'AutoyKBRaHSBHy9RsmXCZMy6nNFAg5FYijrvZyQcNLV');
    return (0, umi_1.gpaBuilder)(context, programId)
        .registerFields({
        bump: [0, (0, serializers_1.array)((0, serializers_1.u8)(), { size: 1 })],
        padding1: [1, (0, serializers_1.array)((0, serializers_1.u8)(), { size: 7 })],
        authority: [8, (0, serializers_1.publicKey)()],
        referredByState: [40, (0, serializers_1.publicKey)()],
        destFeesMint: [72, (0, serializers_1.publicKey)()],
        lookupTable: [104, (0, serializers_1.publicKey)()],
        padding: [136, (0, serializers_1.array)((0, serializers_1.u8)(), { size: 96 })],
    })
        .deserializeUsing((account) => deserializeReferralState(account));
}
function getReferralStateSize() {
    return 232;
}
