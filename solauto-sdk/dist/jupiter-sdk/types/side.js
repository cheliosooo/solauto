"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.Side = void 0;
exports.getSideSerializer = getSideSerializer;
const serializers_1 = require("@metaplex-foundation/umi/serializers");
var Side;
(function (Side) {
    Side[Side["Bid"] = 0] = "Bid";
    Side[Side["Ask"] = 1] = "Ask";
})(Side || (exports.Side = Side = {}));
function getSideSerializer() {
    return (0, serializers_1.scalarEnum)(Side, { description: 'Side' });
}
