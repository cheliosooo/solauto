/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  ClusterFilter,
  Context,
  Program,
  PublicKey,
} from '@metaplex-foundation/umi';
import { getJupiterErrorFromCode, getJupiterErrorFromName } from '../errors';

export const JUPITER_PROGRAM_ID =
  'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4' as PublicKey<'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4'>;

export function createJupiterProgram(): Program {
  return {
    name: 'jupiter',
    publicKey: JUPITER_PROGRAM_ID,
    getErrorFromCode(code: number, cause?: Error) {
      return getJupiterErrorFromCode(code, this, cause);
    },
    getErrorFromName(name: string, cause?: Error) {
      return getJupiterErrorFromName(name, this, cause);
    },
    isOnCluster() {
      return true;
    },
  };
}

export function getJupiterProgram<T extends Program = Program>(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): T {
  return context.programs.get<T>('jupiter', clusterFilter);
}

export function getJupiterProgramId(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): PublicKey {
  return context.programs.getPublicKey(
    'jupiter',
    JUPITER_PROGRAM_ID,
    clusterFilter
  );
}
