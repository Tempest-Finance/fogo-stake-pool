//! Compatibility tests to ensure interface types stay in sync with program types.
//!
//! These tests serialize data using the original program types and then deserialize
//! using the interface types. If the program's state.rs changes in a way that breaks
//! binary compatibility, these tests will fail.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{pubkey::Pubkey, stake::state::Lockup};
use spl_pod::primitives::{PodU32, PodU64};

// Import program types (the source of truth)
use spl_stake_pool::state as program;

// Import interface types (must be compatible)
use fogo_stake_pool_interface::state as interface;

/// Helper function to serialize using borsh
fn serialize<T: BorshSerialize>(value: &T) -> Vec<u8> {
    borsh::to_vec(value).expect("serialization failed")
}

/// Helper function to deserialize using borsh
fn deserialize<T: BorshDeserialize>(data: &[u8]) -> T {
    T::try_from_slice(data).expect("deserialization failed")
}

/// Creates a sample StakePool with all fields populated for testing
fn create_sample_program_stake_pool() -> program::StakePool {
    program::StakePool {
        account_type: program::AccountType::StakePool,
        manager: Pubkey::new_unique(),
        staker: Pubkey::new_unique(),
        stake_deposit_authority: Pubkey::new_unique(),
        stake_withdraw_bump_seed: 255,
        validator_list: Pubkey::new_unique(),
        reserve_stake: Pubkey::new_unique(),
        pool_mint: Pubkey::new_unique(),
        manager_fee_account: Pubkey::new_unique(),
        token_program_id: Pubkey::new_unique(),
        total_lamports: 1_000_000_000,
        pool_token_supply: 500_000_000,
        last_update_epoch: 100,
        lockup: Lockup {
            unix_timestamp: 1234567890,
            epoch: 50,
            custodian: Pubkey::new_unique(),
        },
        epoch_fee: program::Fee {
            denominator: 100,
            numerator: 1,
        },
        next_epoch_fee: program::FutureEpoch::None,
        preferred_deposit_validator_vote_address: Some(Pubkey::new_unique()),
        preferred_withdraw_validator_vote_address: None,
        stake_deposit_fee: program::Fee {
            denominator: 1000,
            numerator: 5,
        },
        stake_withdrawal_fee: program::Fee {
            denominator: 1000,
            numerator: 10,
        },
        next_stake_withdrawal_fee: program::FutureEpoch::None,
        stake_referral_fee: 50,
        sol_deposit_authority: Some(Pubkey::new_unique()),
        sol_deposit_fee: program::Fee {
            denominator: 1000,
            numerator: 3,
        },
        sol_referral_fee: 25,
        sol_withdraw_authority: None,
        sol_withdrawal_fee: program::Fee {
            denominator: 1000,
            numerator: 8,
        },
        next_sol_withdrawal_fee: program::FutureEpoch::None,
        last_epoch_pool_token_supply: 400_000_000,
        last_epoch_total_lamports: 900_000_000,
    }
}

/// Creates a sample ValidatorListHeader for testing
fn create_sample_program_validator_list_header() -> program::ValidatorListHeader {
    program::ValidatorListHeader {
        account_type: program::AccountType::ValidatorList,
        max_validators: 1000,
    }
}

/// Creates a sample ValidatorStakeInfo for testing
fn create_sample_program_validator_stake_info() -> program::ValidatorStakeInfo {
    program::ValidatorStakeInfo {
        active_stake_lamports: PodU64::from(100_000_000),
        transient_stake_lamports: PodU64::from(50_000_000),
        last_update_epoch: PodU64::from(99),
        transient_seed_suffix: PodU64::from(1),
        unused: PodU32::from(0),
        validator_seed_suffix: PodU32::from(42),
        status: program::StakeStatus::Active.into(),
        vote_account_address: Pubkey::new_unique(),
    }
}

/// Creates a sample Fee for testing
fn create_sample_program_fee() -> program::Fee {
    program::Fee {
        denominator: 10000,
        numerator: 250,
    }
}

#[test]
fn stake_pool_serialization_compatibility() {
    let program_pool = create_sample_program_stake_pool();

    // Serialize using program types
    let serialized = serialize(&program_pool);

    // Deserialize using interface types
    let interface_pool: interface::StakePool = deserialize(&serialized);

    // Verify key fields match
    assert_eq!(
        interface_pool.account_type,
        interface::AccountType::StakePool
    );
    assert_eq!(interface_pool.manager, program_pool.manager);
    assert_eq!(interface_pool.staker, program_pool.staker);
    assert_eq!(
        interface_pool.stake_deposit_authority,
        program_pool.stake_deposit_authority
    );
    assert_eq!(
        interface_pool.stake_withdraw_bump_seed,
        program_pool.stake_withdraw_bump_seed
    );
    assert_eq!(interface_pool.validator_list, program_pool.validator_list);
    assert_eq!(interface_pool.reserve_stake, program_pool.reserve_stake);
    assert_eq!(interface_pool.pool_mint, program_pool.pool_mint);
    assert_eq!(
        interface_pool.manager_fee_account,
        program_pool.manager_fee_account
    );
    assert_eq!(
        interface_pool.token_program_id,
        program_pool.token_program_id
    );
    assert_eq!(interface_pool.total_lamports, program_pool.total_lamports);
    assert_eq!(
        interface_pool.pool_token_supply,
        program_pool.pool_token_supply
    );
    assert_eq!(
        interface_pool.last_update_epoch,
        program_pool.last_update_epoch
    );
    assert_eq!(interface_pool.lockup, program_pool.lockup);
    assert_eq!(
        interface_pool.epoch_fee.numerator,
        program_pool.epoch_fee.numerator
    );
    assert_eq!(
        interface_pool.epoch_fee.denominator,
        program_pool.epoch_fee.denominator
    );
    assert_eq!(
        interface_pool.preferred_deposit_validator_vote_address,
        program_pool.preferred_deposit_validator_vote_address
    );
    assert_eq!(
        interface_pool.preferred_withdraw_validator_vote_address,
        program_pool.preferred_withdraw_validator_vote_address
    );
    assert_eq!(
        interface_pool.stake_deposit_fee.numerator,
        program_pool.stake_deposit_fee.numerator
    );
    assert_eq!(
        interface_pool.stake_withdrawal_fee.numerator,
        program_pool.stake_withdrawal_fee.numerator
    );
    assert_eq!(
        interface_pool.stake_referral_fee,
        program_pool.stake_referral_fee
    );
    assert_eq!(
        interface_pool.sol_deposit_authority,
        program_pool.sol_deposit_authority
    );
    assert_eq!(
        interface_pool.sol_deposit_fee.numerator,
        program_pool.sol_deposit_fee.numerator
    );
    assert_eq!(
        interface_pool.sol_referral_fee,
        program_pool.sol_referral_fee
    );
    assert_eq!(
        interface_pool.sol_withdraw_authority,
        program_pool.sol_withdraw_authority
    );
    assert_eq!(
        interface_pool.sol_withdrawal_fee.numerator,
        program_pool.sol_withdrawal_fee.numerator
    );
    assert_eq!(
        interface_pool.last_epoch_pool_token_supply,
        program_pool.last_epoch_pool_token_supply
    );
    assert_eq!(
        interface_pool.last_epoch_total_lamports,
        program_pool.last_epoch_total_lamports
    );
}

#[test]
fn stake_pool_roundtrip_compatibility() {
    let program_pool = create_sample_program_stake_pool();

    // Serialize with program, deserialize with interface, serialize with interface
    let program_serialized = serialize(&program_pool);
    let interface_pool: interface::StakePool = deserialize(&program_serialized);
    let interface_serialized = serialize(&interface_pool);

    // The serialized bytes should be identical
    assert_eq!(
        program_serialized, interface_serialized,
        "Roundtrip serialization mismatch - program and interface produce different bytes"
    );
}

#[test]
fn validator_list_header_serialization_compatibility() {
    let program_header = create_sample_program_validator_list_header();

    // Serialize using program types
    let serialized = serialize(&program_header);

    // Deserialize using interface types
    let interface_header: interface::ValidatorListHeader = deserialize(&serialized);

    // Verify fields match
    assert_eq!(
        interface_header.account_type,
        interface::AccountType::ValidatorList
    );
    assert_eq!(
        interface_header.max_validators,
        program_header.max_validators
    );
}

#[test]
fn validator_list_header_roundtrip_compatibility() {
    let program_header = create_sample_program_validator_list_header();

    let program_serialized = serialize(&program_header);
    let interface_header: interface::ValidatorListHeader = deserialize(&program_serialized);
    let interface_serialized = serialize(&interface_header);

    assert_eq!(
        program_serialized, interface_serialized,
        "ValidatorListHeader roundtrip serialization mismatch"
    );
}

#[test]
fn validator_stake_info_serialization_compatibility() {
    let program_info = create_sample_program_validator_stake_info();

    // ValidatorStakeInfo uses bytemuck (Pod), so we use bytes_of instead of borsh
    let serialized = bytemuck::bytes_of(&program_info);

    // Deserialize using interface types via bytemuck
    let interface_info: &interface::ValidatorStakeInfo = bytemuck::from_bytes(serialized);

    // Verify fields match
    assert_eq!(
        interface_info.active_stake_lamports,
        program_info.active_stake_lamports
    );
    assert_eq!(
        interface_info.transient_stake_lamports,
        program_info.transient_stake_lamports
    );
    assert_eq!(
        interface_info.last_update_epoch,
        program_info.last_update_epoch
    );
    assert_eq!(
        interface_info.transient_seed_suffix,
        program_info.transient_seed_suffix
    );
    assert_eq!(interface_info.unused, program_info.unused);
    assert_eq!(
        interface_info.validator_seed_suffix,
        program_info.validator_seed_suffix
    );
    assert_eq!(
        interface_info.vote_account_address,
        program_info.vote_account_address
    );
}

#[test]
fn fee_serialization_compatibility() {
    let program_fee = create_sample_program_fee();

    // Serialize using program types
    let serialized = serialize(&program_fee);

    // Deserialize using interface types
    let interface_fee: interface::Fee = deserialize(&serialized);

    // Verify fields match
    assert_eq!(interface_fee.numerator, program_fee.numerator);
    assert_eq!(interface_fee.denominator, program_fee.denominator);
}

#[test]
fn fee_roundtrip_compatibility() {
    let program_fee = create_sample_program_fee();

    let program_serialized = serialize(&program_fee);
    let interface_fee: interface::Fee = deserialize(&program_serialized);
    let interface_serialized = serialize(&interface_fee);

    assert_eq!(
        program_serialized, interface_serialized,
        "Fee roundtrip serialization mismatch"
    );
}

#[test]
fn account_type_serialization_compatibility() {
    // Test all AccountType variants
    let variants = [
        (
            program::AccountType::Uninitialized,
            interface::AccountType::Uninitialized,
        ),
        (
            program::AccountType::StakePool,
            interface::AccountType::StakePool,
        ),
        (
            program::AccountType::ValidatorList,
            interface::AccountType::ValidatorList,
        ),
    ];

    for (program_variant, expected_interface_variant) in variants {
        let serialized = serialize(&program_variant);
        let interface_variant: interface::AccountType = deserialize(&serialized);
        assert_eq!(
            interface_variant, expected_interface_variant,
            "AccountType variant mismatch"
        );
    }
}

#[test]
fn stake_status_serialization_compatibility() {
    // Test all StakeStatus variants
    let variants = [
        (program::StakeStatus::Active, interface::StakeStatus::Active),
        (
            program::StakeStatus::DeactivatingTransient,
            interface::StakeStatus::DeactivatingTransient,
        ),
        (
            program::StakeStatus::ReadyForRemoval,
            interface::StakeStatus::ReadyForRemoval,
        ),
        (
            program::StakeStatus::DeactivatingValidator,
            interface::StakeStatus::DeactivatingValidator,
        ),
        (
            program::StakeStatus::DeactivatingAll,
            interface::StakeStatus::DeactivatingAll,
        ),
    ];

    for (program_variant, expected_interface_variant) in variants {
        let serialized = serialize(&program_variant);
        let interface_variant: interface::StakeStatus = deserialize(&serialized);
        assert_eq!(
            interface_variant, expected_interface_variant,
            "StakeStatus variant mismatch"
        );
    }
}

#[test]
fn future_epoch_none_serialization_compatibility() {
    let program_future: program::FutureEpoch<program::Fee> = program::FutureEpoch::None;

    let serialized = serialize(&program_future);
    let interface_future: interface::FutureEpochFee = deserialize(&serialized);

    assert!(
        matches!(interface_future, interface::FutureEpochFee::None),
        "FutureEpoch::None mismatch"
    );
}

#[test]
fn future_epoch_one_serialization_compatibility() {
    let fee = program::Fee {
        denominator: 100,
        numerator: 5,
    };
    let program_future: program::FutureEpoch<program::Fee> = program::FutureEpoch::One(fee);

    let serialized = serialize(&program_future);
    let interface_future: interface::FutureEpochFee = deserialize(&serialized);

    match interface_future {
        interface::FutureEpochFee::One(interface_fee) => {
            assert_eq!(interface_fee.numerator, fee.numerator);
            assert_eq!(interface_fee.denominator, fee.denominator);
        }
        _ => panic!("Expected FutureEpoch::One"),
    }
}

#[test]
fn future_epoch_two_serialization_compatibility() {
    let fee = program::Fee {
        denominator: 200,
        numerator: 10,
    };
    let program_future: program::FutureEpoch<program::Fee> = program::FutureEpoch::Two(fee);

    let serialized = serialize(&program_future);
    let interface_future: interface::FutureEpochFee = deserialize(&serialized);

    match interface_future {
        interface::FutureEpochFee::Two(interface_fee) => {
            assert_eq!(interface_fee.numerator, fee.numerator);
            assert_eq!(interface_fee.denominator, fee.denominator);
        }
        _ => panic!("Expected FutureEpoch::Two"),
    }
}
