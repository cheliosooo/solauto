"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.BankVaultType = void 0;
exports.getBankVaultTypeSerializer = getBankVaultTypeSerializer;
const serializers_1 = require("@metaplex-foundation/umi/serializers");
var BankVaultType;
(function (BankVaultType) {
    BankVaultType[BankVaultType["Liquidity"] = 0] = "Liquidity";
    BankVaultType[BankVaultType["Insurance"] = 1] = "Insurance";
    BankVaultType[BankVaultType["Fee"] = 2] = "Fee";
})(BankVaultType || (exports.BankVaultType = BankVaultType = {}));
function getBankVaultTypeSerializer() {
    return (0, serializers_1.scalarEnum)(BankVaultType, {
        description: 'BankVaultType',
    });
}
