"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.getUpdatePositionDataSerializer = getUpdatePositionDataSerializer;
const serializers_1 = require("@metaplex-foundation/umi/serializers");
const _1 = require(".");
function getUpdatePositionDataSerializer() {
    return (0, serializers_1.struct)([
        ['positionId', (0, serializers_1.u8)()],
        ['settingParams', (0, serializers_1.option)((0, _1.getSolautoSettingsParametersInpSerializer)())],
        ['dca', (0, serializers_1.option)((0, _1.getDCASettingsInpSerializer)())],
    ], { description: 'UpdatePositionData' });
}