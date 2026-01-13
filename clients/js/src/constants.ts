import { Buffer } from 'node:buffer'
import { LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'

// Public key that identifies the metadata program.
export const METADATA_PROGRAM_ID = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s')
export const METADATA_MAX_NAME_LENGTH = 32
export const METADATA_MAX_SYMBOL_LENGTH = 10
export const METADATA_MAX_URI_LENGTH = 200

// Public key that identifies the SPL Stake Pool program.
export const STAKE_POOL_PROGRAM_ID = new PublicKey('SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr')

// Public key that identifies the SPL Stake Pool program deployed to devnet.
export const DEVNET_STAKE_POOL_PROGRAM_ID = STAKE_POOL_PROGRAM_ID

// Maximum number of validators to update during UpdateValidatorListBalance.
export const MAX_VALIDATORS_TO_UPDATE = 4

// Seed for ephemeral stake account
export const EPHEMERAL_STAKE_SEED_PREFIX = Buffer.from('ephemeral')

// Seed used to derive transient stake accounts.
export const TRANSIENT_STAKE_SEED_PREFIX = Buffer.from('transient')

// Seed for user stake account created during session withdrawal
export const USER_STAKE_SEED_PREFIX = Buffer.from('user_stake')

// Minimum amount of staked SOL required in a validator stake account to allow
// for merges without a mismatch on credits observed
export const MINIMUM_ACTIVE_STAKE = LAMPORTS_PER_SOL
