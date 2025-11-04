import { Buffer } from 'node:buffer'
import { PublicKey } from '@solana/web3.js'
import BN from 'bn.js'
import {
  EPHEMERAL_STAKE_SEED_PREFIX,
  METADATA_PROGRAM_ID,
  TRANSIENT_STAKE_SEED_PREFIX,
} from '../constants'

/**
 * Generates the wSOL transient program address for the stake pool
 */
export function findWsolTransientProgramAddress(
  programId: PublicKey,
  userPubkey: PublicKey,
) {
  const [publicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from('transient_wsol'), userPubkey.toBuffer()],
    programId,
  )
  return publicKey
}

/**
 * Generates the withdraw authority program address for the stake pool
 */
export async function findWithdrawAuthorityProgramAddress(
  programId: PublicKey,
  stakePoolAddress: PublicKey,
) {
  const [publicKey] = PublicKey.findProgramAddressSync(
    [stakePoolAddress.toBuffer(), Buffer.from('withdraw')],
    programId,
  )
  return publicKey
}

/**
 * Generates the stake program address for a validator's vote account
 */
export async function findStakeProgramAddress(
  programId: PublicKey,
  voteAccountAddress: PublicKey,
  stakePoolAddress: PublicKey,
  seed?: number,
) {
  const [publicKey] = PublicKey.findProgramAddressSync(
    [
      voteAccountAddress.toBuffer(),
      stakePoolAddress.toBuffer(),
      seed ? new BN(seed).toArrayLike(Buffer, 'le', 4) : Buffer.alloc(0),
    ],
    programId,
  )
  return publicKey
}

/**
 * Generates the stake program address for a validator's vote account
 */
export async function findTransientStakeProgramAddress(
  programId: PublicKey,
  voteAccountAddress: PublicKey,
  stakePoolAddress: PublicKey,
  seed: BN,
) {
  const [publicKey] = PublicKey.findProgramAddressSync(
    [
      TRANSIENT_STAKE_SEED_PREFIX,
      voteAccountAddress.toBuffer(),
      stakePoolAddress.toBuffer(),
      seed.toArrayLike(Buffer, 'le', 8),
    ],
    programId,
  )
  return publicKey
}

/**
 * Generates the ephemeral program address for stake pool redelegation
 */
export async function findEphemeralStakeProgramAddress(
  programId: PublicKey,
  stakePoolAddress: PublicKey,
  seed: BN,
) {
  const [publicKey] = PublicKey.findProgramAddressSync(
    [EPHEMERAL_STAKE_SEED_PREFIX, stakePoolAddress.toBuffer(), seed.toArrayLike(Buffer, 'le', 8)],
    programId,
  )
  return publicKey
}

/**
 * Generates the metadata program address for the stake pool
 */
export function findMetadataAddress(stakePoolMintAddress: PublicKey) {
  const [publicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from('metadata'), METADATA_PROGRAM_ID.toBuffer(), stakePoolMintAddress.toBuffer()],
    METADATA_PROGRAM_ID,
  )
  return publicKey
}
