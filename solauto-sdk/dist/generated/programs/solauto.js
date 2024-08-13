"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.SOLAUTO_PROGRAM_ID = void 0;
exports.createSolautoProgram = createSolautoProgram;
exports.getSolautoProgram = getSolautoProgram;
exports.getSolautoProgramId = getSolautoProgramId;
const errors_1 = require("../errors");
exports.SOLAUTO_PROGRAM_ID = 'AutoyKBRaHSBHy9RsmXCZMy6nNFAg5FYijrvZyQcNLV';
function createSolautoProgram() {
    return {
        name: 'solauto',
        publicKey: exports.SOLAUTO_PROGRAM_ID,
        getErrorFromCode(code, cause) {
            return (0, errors_1.getSolautoErrorFromCode)(code, this, cause);
        },
        getErrorFromName(name, cause) {
            return (0, errors_1.getSolautoErrorFromName)(name, this, cause);
        },
        isOnCluster() {
            return true;
        },
    };
}
function getSolautoProgram(context, clusterFilter) {
    return context.programs.get('solauto', clusterFilter);
}
function getSolautoProgramId(context, clusterFilter) {
    return context.programs.getPublicKey('solauto', exports.SOLAUTO_PROGRAM_ID, clusterFilter);
}
