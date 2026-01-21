#![deny(missing_docs)]

//! Fogo Stake Pool Interface
//!
//! This crate provides the state types for deserializing Fogo Stake Pool accounts
//! without pulling in the full program dependencies.
//!
//! # Example
//!
//! ```ignore
//! use fogo_stake_pool_interface::{state::StakePool, id};
//! use solana_program::borsh1::try_from_slice_unchecked;
//!
//! let stake_pool: StakePool = try_from_slice_unchecked(&account_data)?;
//! ```
//!
//! # Features
//!
//! - `borsh` (default) - Enables Borsh serialization/deserialization
//! - `serde` - Enables Serde serialization
//! - `codama` - Enables IDL generation via Codama

#[cfg(feature = "codama")]
use codama_macros::codama;

pub mod error;
pub mod pda;
pub mod state;

// Re-export commonly used types at the crate root
pub use state::{
    AccountType, Fee, FeeType, FutureEpoch, FutureEpochFee, StakePool, StakeStatus, ValidatorList,
    ValidatorListHeader, ValidatorStakeInfo,
};

// Re-export PDA functions at the crate root for convenience
pub use pda::{
    check_program_account, find_deposit_authority_program_address,
    find_ephemeral_stake_program_address, find_stake_program_address,
    find_transient_stake_program_address, find_user_stake_program_address,
    find_withdraw_authority_program_address,
};

/// Program module with the program ID
#[cfg_attr(feature = "codama", codama(name = "fogoStakePool"))]
pub mod program {
    solana_program::declare_id!("SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr");
}

pub use program::id;

/// Seed for deposit authority
pub const AUTHORITY_DEPOSIT: &[u8] = b"deposit";

/// Seed for withdraw authority
pub const AUTHORITY_WITHDRAW: &[u8] = b"withdraw";

/// Seed for transient stake account
pub const TRANSIENT_STAKE_SEED_PREFIX: &[u8] = b"transient";

/// Seed for ephemeral stake account
pub const EPHEMERAL_STAKE_SEED_PREFIX: &[u8] = b"ephemeral";

/// Seed for user stake account created during session withdrawal
pub const USER_STAKE_SEED_PREFIX: &[u8] = b"user_stake";

/// Minimum amount of staked lamports required in a validator stake account to
/// allow for merges without a mismatch on credits observed
pub const MINIMUM_ACTIVE_STAKE: u64 = 1_000_000;

/// Minimum amount of lamports in the reserve
pub const MINIMUM_RESERVE_LAMPORTS: u64 = 0;

/// Maximum amount of validator stake accounts to update per
/// `UpdateValidatorListBalance` instruction, based on compute limits
pub const MAX_VALIDATORS_TO_UPDATE: usize = 4;

/// The maximum number of transient stake accounts respecting
/// transaction account limits.
pub const MAX_TRANSIENT_STAKE_ACCOUNTS: usize = 10;

/// The maximum number of validators that can be supported in a pool in order
/// for stake withdrawals to still work
pub const MAX_VALIDATORS_IN_POOL: u32 = 20_000;

/// Maximum factor by which a withdrawal fee can be increased per epoch,
/// protecting stakers from malicious fee increases.
/// If current fee is 0, `WITHDRAWAL_BASELINE_FEE` is used as the baseline.
pub const MAX_WITHDRAWAL_FEE_INCREASE: Fee = Fee {
    numerator: 3,
    denominator: 2,
};

/// Drop-in baseline fee when evaluating withdrawal fee increases when fee is 0
pub const WITHDRAWAL_BASELINE_FEE: Fee = Fee {
    numerator: 1,
    denominator: 1000,
};
