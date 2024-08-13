/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Option, OptionOrNullable, PublicKey } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
export type GroupEventHeader = {
    signer: Option<PublicKey>;
    marginfiGroup: PublicKey;
};
export type GroupEventHeaderArgs = {
    signer: OptionOrNullable<PublicKey>;
    marginfiGroup: PublicKey;
};
export declare function getGroupEventHeaderSerializer(): Serializer<GroupEventHeaderArgs, GroupEventHeader>;
//# sourceMappingURL=groupEventHeader.d.ts.map