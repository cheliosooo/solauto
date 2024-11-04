/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Program, ProgramError } from '@metaplex-foundation/umi';

type ProgramErrorConstructor = new (
  program: Program,
  cause?: Error
) => ProgramError;
const codeToErrorMap: Map<number, ProgramErrorConstructor> = new Map();
const nameToErrorMap: Map<string, ProgramErrorConstructor> = new Map();

/** EmptyRoute: Empty route */
export class EmptyRouteError extends ProgramError {
  override readonly name: string = 'EmptyRoute';

  readonly code: number = 0x1770; // 6000

  constructor(program: Program, cause?: Error) {
    super('Empty route', program, cause);
  }
}
codeToErrorMap.set(0x1770, EmptyRouteError);
nameToErrorMap.set('EmptyRoute', EmptyRouteError);

/** SlippageToleranceExceeded: Slippage tolerance exceeded */
export class SlippageToleranceExceededError extends ProgramError {
  override readonly name: string = 'SlippageToleranceExceeded';

  readonly code: number = 0x1771; // 6001

  constructor(program: Program, cause?: Error) {
    super('Slippage tolerance exceeded', program, cause);
  }
}
codeToErrorMap.set(0x1771, SlippageToleranceExceededError);
nameToErrorMap.set('SlippageToleranceExceeded', SlippageToleranceExceededError);

/** InvalidCalculation: Invalid calculation */
export class InvalidCalculationError extends ProgramError {
  override readonly name: string = 'InvalidCalculation';

  readonly code: number = 0x1772; // 6002

  constructor(program: Program, cause?: Error) {
    super('Invalid calculation', program, cause);
  }
}
codeToErrorMap.set(0x1772, InvalidCalculationError);
nameToErrorMap.set('InvalidCalculation', InvalidCalculationError);

/** MissingPlatformFeeAccount: Missing platform fee account */
export class MissingPlatformFeeAccountError extends ProgramError {
  override readonly name: string = 'MissingPlatformFeeAccount';

  readonly code: number = 0x1773; // 6003

  constructor(program: Program, cause?: Error) {
    super('Missing platform fee account', program, cause);
  }
}
codeToErrorMap.set(0x1773, MissingPlatformFeeAccountError);
nameToErrorMap.set('MissingPlatformFeeAccount', MissingPlatformFeeAccountError);

/** InvalidSlippage: Invalid slippage */
export class InvalidSlippageError extends ProgramError {
  override readonly name: string = 'InvalidSlippage';

  readonly code: number = 0x1774; // 6004

  constructor(program: Program, cause?: Error) {
    super('Invalid slippage', program, cause);
  }
}
codeToErrorMap.set(0x1774, InvalidSlippageError);
nameToErrorMap.set('InvalidSlippage', InvalidSlippageError);

/** NotEnoughPercent: Not enough percent to 100 */
export class NotEnoughPercentError extends ProgramError {
  override readonly name: string = 'NotEnoughPercent';

  readonly code: number = 0x1775; // 6005

  constructor(program: Program, cause?: Error) {
    super('Not enough percent to 100', program, cause);
  }
}
codeToErrorMap.set(0x1775, NotEnoughPercentError);
nameToErrorMap.set('NotEnoughPercent', NotEnoughPercentError);

/** InvalidInputIndex: Token input index is invalid */
export class InvalidInputIndexError extends ProgramError {
  override readonly name: string = 'InvalidInputIndex';

  readonly code: number = 0x1776; // 6006

  constructor(program: Program, cause?: Error) {
    super('Token input index is invalid', program, cause);
  }
}
codeToErrorMap.set(0x1776, InvalidInputIndexError);
nameToErrorMap.set('InvalidInputIndex', InvalidInputIndexError);

/** InvalidOutputIndex: Token output index is invalid */
export class InvalidOutputIndexError extends ProgramError {
  override readonly name: string = 'InvalidOutputIndex';

  readonly code: number = 0x1777; // 6007

  constructor(program: Program, cause?: Error) {
    super('Token output index is invalid', program, cause);
  }
}
codeToErrorMap.set(0x1777, InvalidOutputIndexError);
nameToErrorMap.set('InvalidOutputIndex', InvalidOutputIndexError);

/** NotEnoughAccountKeys: Not Enough Account keys */
export class NotEnoughAccountKeysError extends ProgramError {
  override readonly name: string = 'NotEnoughAccountKeys';

  readonly code: number = 0x1778; // 6008

  constructor(program: Program, cause?: Error) {
    super('Not Enough Account keys', program, cause);
  }
}
codeToErrorMap.set(0x1778, NotEnoughAccountKeysError);
nameToErrorMap.set('NotEnoughAccountKeys', NotEnoughAccountKeysError);

/** NonZeroMinimumOutAmountNotSupported: Non zero minimum out amount not supported */
export class NonZeroMinimumOutAmountNotSupportedError extends ProgramError {
  override readonly name: string = 'NonZeroMinimumOutAmountNotSupported';

  readonly code: number = 0x1779; // 6009

  constructor(program: Program, cause?: Error) {
    super('Non zero minimum out amount not supported', program, cause);
  }
}
codeToErrorMap.set(0x1779, NonZeroMinimumOutAmountNotSupportedError);
nameToErrorMap.set(
  'NonZeroMinimumOutAmountNotSupported',
  NonZeroMinimumOutAmountNotSupportedError
);

/** InvalidRoutePlan: Invalid route plan */
export class InvalidRoutePlanError extends ProgramError {
  override readonly name: string = 'InvalidRoutePlan';

  readonly code: number = 0x177a; // 6010

  constructor(program: Program, cause?: Error) {
    super('Invalid route plan', program, cause);
  }
}
codeToErrorMap.set(0x177a, InvalidRoutePlanError);
nameToErrorMap.set('InvalidRoutePlan', InvalidRoutePlanError);

/** InvalidReferralAuthority: Invalid referral authority */
export class InvalidReferralAuthorityError extends ProgramError {
  override readonly name: string = 'InvalidReferralAuthority';

  readonly code: number = 0x177b; // 6011

  constructor(program: Program, cause?: Error) {
    super('Invalid referral authority', program, cause);
  }
}
codeToErrorMap.set(0x177b, InvalidReferralAuthorityError);
nameToErrorMap.set('InvalidReferralAuthority', InvalidReferralAuthorityError);

/** LedgerTokenAccountDoesNotMatch: Token account doesn't match the ledger */
export class LedgerTokenAccountDoesNotMatchError extends ProgramError {
  override readonly name: string = 'LedgerTokenAccountDoesNotMatch';

  readonly code: number = 0x177c; // 6012

  constructor(program: Program, cause?: Error) {
    super("Token account doesn't match the ledger", program, cause);
  }
}
codeToErrorMap.set(0x177c, LedgerTokenAccountDoesNotMatchError);
nameToErrorMap.set(
  'LedgerTokenAccountDoesNotMatch',
  LedgerTokenAccountDoesNotMatchError
);

/** InvalidTokenLedger: Invalid token ledger */
export class InvalidTokenLedgerError extends ProgramError {
  override readonly name: string = 'InvalidTokenLedger';

  readonly code: number = 0x177d; // 6013

  constructor(program: Program, cause?: Error) {
    super('Invalid token ledger', program, cause);
  }
}
codeToErrorMap.set(0x177d, InvalidTokenLedgerError);
nameToErrorMap.set('InvalidTokenLedger', InvalidTokenLedgerError);

/** IncorrectTokenProgramID: Token program ID is invalid */
export class IncorrectTokenProgramIDError extends ProgramError {
  override readonly name: string = 'IncorrectTokenProgramID';

  readonly code: number = 0x177e; // 6014

  constructor(program: Program, cause?: Error) {
    super('Token program ID is invalid', program, cause);
  }
}
codeToErrorMap.set(0x177e, IncorrectTokenProgramIDError);
nameToErrorMap.set('IncorrectTokenProgramID', IncorrectTokenProgramIDError);

/** TokenProgramNotProvided: Token program not provided */
export class TokenProgramNotProvidedError extends ProgramError {
  override readonly name: string = 'TokenProgramNotProvided';

  readonly code: number = 0x177f; // 6015

  constructor(program: Program, cause?: Error) {
    super('Token program not provided', program, cause);
  }
}
codeToErrorMap.set(0x177f, TokenProgramNotProvidedError);
nameToErrorMap.set('TokenProgramNotProvided', TokenProgramNotProvidedError);

/** SwapNotSupported: Swap not supported */
export class SwapNotSupportedError extends ProgramError {
  override readonly name: string = 'SwapNotSupported';

  readonly code: number = 0x1780; // 6016

  constructor(program: Program, cause?: Error) {
    super('Swap not supported', program, cause);
  }
}
codeToErrorMap.set(0x1780, SwapNotSupportedError);
nameToErrorMap.set('SwapNotSupported', SwapNotSupportedError);

/** ExactOutAmountNotMatched: Exact out amount doesn't match */
export class ExactOutAmountNotMatchedError extends ProgramError {
  override readonly name: string = 'ExactOutAmountNotMatched';

  readonly code: number = 0x1781; // 6017

  constructor(program: Program, cause?: Error) {
    super("Exact out amount doesn't match", program, cause);
  }
}
codeToErrorMap.set(0x1781, ExactOutAmountNotMatchedError);
nameToErrorMap.set('ExactOutAmountNotMatched', ExactOutAmountNotMatchedError);

/**
 * Attempts to resolve a custom program error from the provided error code.
 * @category Errors
 */
export function getJupiterErrorFromCode(
  code: number,
  program: Program,
  cause?: Error
): ProgramError | null {
  const constructor = codeToErrorMap.get(code);
  return constructor ? new constructor(program, cause) : null;
}

/**
 * Attempts to resolve a custom program error from the provided error name, i.e. 'Unauthorized'.
 * @category Errors
 */
export function getJupiterErrorFromName(
  name: string,
  program: Program,
  cause?: Error
): ProgramError | null {
  const constructor = nameToErrorMap.get(name);
  return constructor ? new constructor(program, cause) : null;
}
