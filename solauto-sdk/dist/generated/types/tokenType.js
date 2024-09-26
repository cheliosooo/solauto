"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.TokenType = void 0;
exports.getTokenTypeSerializer = getTokenTypeSerializer;
const serializers_1 = require("@metaplex-foundation/umi/serializers");
var TokenType;
(function (TokenType) {
    TokenType[TokenType["Supply"] = 0] = "Supply";
    TokenType[TokenType["Debt"] = 1] = "Debt";
})(TokenType || (exports.TokenType = TokenType = {}));
function getTokenTypeSerializer() {
    return (0, serializers_1.scalarEnum)(TokenType, {
        description: 'TokenType',
    });
}
