"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getWrappedI80F48Serializer = getWrappedI80F48Serializer;
const serializers_1 = require("@metaplex-foundation/umi/serializers");
function getWrappedI80F48Serializer() {
    return (0, serializers_1.struct)([["value", (0, serializers_1.array)((0, serializers_1.u8)(), { size: 16 })]], {
        description: "WrappedI80F48",
    });
}
