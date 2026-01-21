//! PDA (Program Derived Address) derivation functions for the Fogo Stake Pool.
//!
//! These functions derive the addresses for various accounts used by the stake pool program.

use {
    crate::{
        AUTHORITY_DEPOSIT, AUTHORITY_WITHDRAW, EPHEMERAL_STAKE_SEED_PREFIX,
        TRANSIENT_STAKE_SEED_PREFIX, USER_STAKE_SEED_PREFIX,
    },
    solana_program::pubkey::Pubkey,
    std::num::NonZeroU32,
};

/// Generates the deposit authority program address for the stake pool
pub fn find_deposit_authority_program_address(
    program_id: &Pubkey,
    stake_pool_address: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[stake_pool_address.as_ref(), AUTHORITY_DEPOSIT],
        program_id,
    )
}

/// Generates the withdraw authority program address for the stake pool
pub fn find_withdraw_authority_program_address(
    program_id: &Pubkey,
    stake_pool_address: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[stake_pool_address.as_ref(), AUTHORITY_WITHDRAW],
        program_id,
    )
}

/// Generates the stake program address for a validator's vote account.
///
/// The `seed` parameter allows for multiple stake accounts per validator.
/// When `None`, derives the primary stake account. When `Some(n)`, derives
/// an additional stake account with that seed suffix.
pub fn find_stake_program_address(
    program_id: &Pubkey,
    vote_account_address: &Pubkey,
    stake_pool_address: &Pubkey,
    seed: Option<NonZeroU32>,
) -> (Pubkey, u8) {
    let seed = seed.map(|s| s.get().to_le_bytes());
    Pubkey::find_program_address(
        &[
            vote_account_address.as_ref(),
            stake_pool_address.as_ref(),
            seed.as_ref().map(|s| s.as_slice()).unwrap_or(&[]),
        ],
        program_id,
    )
}

/// Generates the transient stake account address for a validator.
///
/// Transient stake accounts are used during stake activation/deactivation
/// transitions.
pub fn find_transient_stake_program_address(
    program_id: &Pubkey,
    vote_account_address: &Pubkey,
    stake_pool_address: &Pubkey,
    seed: u64,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            TRANSIENT_STAKE_SEED_PREFIX,
            vote_account_address.as_ref(),
            stake_pool_address.as_ref(),
            &seed.to_le_bytes(),
        ],
        program_id,
    )
}

/// Generates the ephemeral stake account address.
///
/// Ephemeral stake accounts are used during stake pool redelegation operations.
pub fn find_ephemeral_stake_program_address(
    program_id: &Pubkey,
    stake_pool_address: &Pubkey,
    seed: u64,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            EPHEMERAL_STAKE_SEED_PREFIX,
            stake_pool_address.as_ref(),
            &seed.to_le_bytes(),
        ],
        program_id,
    )
}

/// Generates the user stake account PDA for session-based withdrawals.
///
/// This PDA is derived from the user's wallet and a unique seed, allowing
/// users to create stake accounts during session-based withdrawal operations.
pub fn find_user_stake_program_address(
    program_id: &Pubkey,
    user_wallet: &Pubkey,
    seed: u64,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            USER_STAKE_SEED_PREFIX,
            user_wallet.as_ref(),
            &seed.to_le_bytes(),
        ],
        program_id,
    )
}

/// Checks that the supplied program ID is correct for the Fogo Stake Pool
pub fn check_program_account(
    program_id: &Pubkey,
) -> Result<(), solana_program::program_error::ProgramError> {
    if program_id != &crate::id() {
        return Err(solana_program::program_error::ProgramError::IncorrectProgramId);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id;

    #[test]
    fn test_validator_stake_account_derivation() {
        let vote = Pubkey::new_unique();
        let stake_pool = Pubkey::new_unique();
        let function_derived = find_stake_program_address(&id(), &vote, &stake_pool, None);
        let hand_derived =
            Pubkey::find_program_address(&[vote.as_ref(), stake_pool.as_ref()], &id());
        assert_eq!(function_derived, hand_derived);
    }

    #[test]
    fn test_deposit_authority_derivation() {
        let stake_pool = Pubkey::new_unique();
        let (pda, _bump) = find_deposit_authority_program_address(&id(), &stake_pool);
        // Verify it's a valid PDA (not on the ed25519 curve)
        assert!(!pda.is_on_curve());
    }

    #[test]
    fn test_withdraw_authority_derivation() {
        let stake_pool = Pubkey::new_unique();
        let (pda, _bump) = find_withdraw_authority_program_address(&id(), &stake_pool);
        assert!(!pda.is_on_curve());
    }

    #[test]
    fn test_transient_stake_derivation() {
        let vote = Pubkey::new_unique();
        let stake_pool = Pubkey::new_unique();
        let (pda, _bump) =
            find_transient_stake_program_address(&id(), &vote, &stake_pool, 0);
        assert!(!pda.is_on_curve());
    }

    #[test]
    fn test_ephemeral_stake_derivation() {
        let stake_pool = Pubkey::new_unique();
        let (pda, _bump) = find_ephemeral_stake_program_address(&id(), &stake_pool, 0);
        assert!(!pda.is_on_curve());
    }

    #[test]
    fn test_user_stake_derivation() {
        let user = Pubkey::new_unique();
        let (pda, _bump) = find_user_stake_program_address(&id(), &user, 0);
        assert!(!pda.is_on_curve());
    }
}
