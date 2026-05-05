import { LAMPORTS_PER_SOL } from '@solana/web3.js'
import BN from 'bn.js'

export function solToLamports(amount: number): number {
  if (Number.isNaN(amount)) {
    return Number(0)
  }
  return Number(amount * LAMPORTS_PER_SOL)
}

/**
 * Precision-safe SOL→lamports conversion. Returns `bigint` so callers can pass
 * amounts above ~9.007M FOGO (the point where lamports cross
 * Number.MAX_SAFE_INTEGER and JS `number` arithmetic silently rounds).
 *
 * Conversion is done in string-domain to avoid the float multiplication
 * `amount * 1e9` which itself loses precision for large `amount`.
 */
export function solToLamportsBigInt(amount: number): bigint {
  // Non-finite input (NaN, ±Infinity) is a caller bug, not a float-precision
  // artifact — silently returning 0n would build a no-op withdrawal and hide
  // the upstream defect.
  if (!Number.isFinite(amount)) {
    throw new TypeError(`solToLamportsBigInt: amount must be a finite number, got ${amount}`)
  }
  // Reject negative input here so a sub-lamport negative (e.g. `-1e-10`)
  // can't floor to `-0n === 0n` and slip past the wrappers' downstream
  // non-negative check as a no-op withdrawal.
  if (amount < 0) {
    throw new TypeError(`solToLamportsBigInt: amount must be non-negative, got ${amount}`)
  }

  const abs = Math.abs(amount)
  const str = abs.toString()

  // Scientific notation appears for very small or very large `number`s.
  // Use floor so this branch matches the "never more than requested" policy
  // of the main string-domain path below.
  if (str.includes('e') || str.includes('E')) {
    return BigInt(Math.floor(abs * LAMPORTS_PER_SOL))
  }

  const [whole, frac = ''] = str.split('.')
  // Sub-lamport precision (>9 fractional digits) is silently floored. This is
  // the friendly choice for UI callers, where float arithmetic routinely
  // produces trailing precision noise (e.g. `1.2 + 0.1 = 1.3000000000000003`).
  // Flooring is safe for withdrawals: at most one lamport less than requested
  // (~1e-9 FOGO, sub-cent), and never more than requested.
  const fracPadded = (`${frac}000000000`).slice(0, 9)
  return BigInt(whole) * BigInt(LAMPORTS_PER_SOL) + BigInt(fracPadded)
}

export function lamportsToSol(lamports: number | BN | bigint): number {
  if (typeof lamports === 'number') {
    return Math.abs(lamports) / LAMPORTS_PER_SOL
  }
  if (typeof lamports === 'bigint') {
    return Math.abs(Number(lamports)) / LAMPORTS_PER_SOL
  }

  let signMultiplier = 1
  if (lamports.isNeg()) {
    signMultiplier = -1
  }

  const absLamports = lamports.abs()
  const lamportsString = absLamports.toString(10).padStart(10, '0')
  const splitIndex = lamportsString.length - 9
  const solString = `${lamportsString.slice(0, splitIndex)}.${lamportsString.slice(splitIndex)}`
  return signMultiplier * Number.parseFloat(solString)
}
