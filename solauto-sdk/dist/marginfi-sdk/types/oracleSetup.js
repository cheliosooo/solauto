"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.OracleSetup = void 0;
exports.getOracleSetupSerializer = getOracleSetupSerializer;
const serializers_1 = require("@metaplex-foundation/umi/serializers");
var OracleSetup;
(function (OracleSetup) {
    OracleSetup[OracleSetup["None"] = 0] = "None";
    OracleSetup[OracleSetup["PythLegacy"] = 1] = "PythLegacy";
    OracleSetup[OracleSetup["SwitchboardV2"] = 2] = "SwitchboardV2";
    OracleSetup[OracleSetup["PythPushOracle"] = 3] = "PythPushOracle";
})(OracleSetup || (exports.OracleSetup = OracleSetup = {}));
function getOracleSetupSerializer() {
    return (0, serializers_1.scalarEnum)(OracleSetup, {
        description: 'OracleSetup',
    });
}
