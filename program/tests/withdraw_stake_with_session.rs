#![allow(clippy::arithmetic_side_effects)]
#![cfg(feature = "test-sbf")]

mod helpers;

use {
    fogo_sessions_sdk::session::SESSION_MANAGER_ID,
    fogo_sessions_sdk::token::PROGRAM_SIGNER_SEED,
    helpers::{wsol::manually_serialize_session, *},
    solana_program::{
        instruction::InstructionError, program_pack::Pack, stake::state::StakeStateV2,
    },
    solana_program_test::*,
    solana_sdk::{
        account::Account,
        clock::Clock,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::{Transaction, TransactionError},
    },
    spl_stake_pool::{
        error::StakePoolError,
        find_user_stake_program_address, id,
        instruction::{withdraw_from_stake_account_with_session, withdraw_stake_with_session},
        MINIMUM_RESERVE_LAMPORTS,
    },
    test_case::test_case,
};

/// Setup helper for withdraw stake with session tests
async fn setup_withdraw_stake_with_session(
    token_program_id: Pubkey,
) -> (
    ProgramTestContext,
    StakePoolAccounts,
    Keypair,               // user
    Pubkey,                // pool_token_ata
    Keypair,               // session_keypair
    u64,                   // pool_tokens
    ValidatorStakeAccount, // validator
) {
    let mut context = program_test().start_with_context().await;

    let stake_pool_accounts = StakePoolAccounts::new_with_token_program(token_program_id);
    stake_pool_accounts
        .initialize_stake_pool(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            MINIMUM_RESERVE_LAMPORTS,
        )
        .await
        .unwrap();

    // Add a validator to the pool
    let validator = simple_add_validator_to_pool(
        &mut context.banks_client,
        &context.payer,
        &context.last_blockhash,
        &stake_pool_accounts,
        None,
    )
    .await;

    // Get minimum delegation for stake deposits
    let current_minimum_delegation = stake_pool_get_minimum_delegation(
        &mut context.banks_client,
        &context.payer,
        &context.last_blockhash,
    )
    .await;

    // Deposit stake (not SOL) to get pool tokens and fund the validator
    let deposit_info = simple_deposit_stake(
        &mut context.banks_client,
        &context.payer,
        &context.last_blockhash,
        &stake_pool_accounts,
        &validator,
        current_minimum_delegation * 3, // Enough for multiple withdrawals
    )
    .await
    .unwrap();

    let pool_tokens = deposit_info.pool_tokens;

    let user = Keypair::new();
    let session_keypair = Keypair::new();

    let clock = context.banks_client.get_sysvar::<Clock>().await.unwrap();
    let expiration = clock.unix_timestamp + 3600;

    let (signer_pda, _) = Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    let session_data = manually_serialize_session(
        &context.payer.pubkey(),
        &user.pubkey(),
        expiration,
        &id(),
        &signer_pda,
    );

    let session_account = Account {
        lamports: context
            .banks_client
            .get_rent()
            .await
            .unwrap()
            .minimum_balance(session_data.len()),
        data: session_data,
        owner: SESSION_MANAGER_ID,
        executable: false,
        rent_epoch: 0,
    };

    context.set_account(&session_keypair.pubkey(), &session_account.into());

    // Create the user's pool token ATA
    let pool_token_ata = spl_associated_token_account::get_associated_token_address_with_program_id(
        &user.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &token_program_id,
    );

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &context.payer.pubkey(),
        &user.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &token_program_id,
    );

    let create_ata_tx = Transaction::new_signed_with_payer(
        &[create_ata_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(create_ata_tx)
        .await
        .unwrap();

    // Transfer pool tokens from deposit account to user's ATA
    transfer_spl_tokens(
        &mut context.banks_client,
        &context.payer,
        &context.last_blockhash,
        &token_program_id,
        &deposit_info.pool_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &pool_token_ata,
        &deposit_info.authority,
        pool_tokens,
        stake_pool_accounts.pool_decimals,
    )
    .await;

    // Approve session_signer as delegate for pool tokens
    {
        let pool_mint_account = get_account(
            &mut context.banks_client,
            &stake_pool_accounts.pool_mint.pubkey(),
        )
        .await;
        let pool_mint = spl_token::state::Mint::unpack(&pool_mint_account.data).unwrap();

        let approve_ix = spl_token::instruction::approve_checked(
            &token_program_id,
            &pool_token_ata,
            &stake_pool_accounts.pool_mint.pubkey(),
            &session_keypair.pubkey(),
            &user.pubkey(),
            &[],
            pool_tokens,
            pool_mint.decimals,
        )
        .unwrap();

        let approve_tx = Transaction::new_signed_with_payer(
            &[approve_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &user],
            context.last_blockhash,
        );

        context
            .banks_client
            .process_transaction(approve_tx)
            .await
            .unwrap();
    }

    (
        context,
        stake_pool_accounts,
        user,
        pool_token_ata,
        session_keypair,
        pool_tokens,
        validator,
    )
}

#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn success_withdraw_stake_with_session(token_program_id: Pubkey) {
    let (
        mut context,
        stake_pool_accounts,
        user,
        pool_token_ata,
        session_keypair,
        pool_tokens,
        validator,
    ) = setup_withdraw_stake_with_session(token_program_id).await;

    let user_stake_seed: u64 = 0;
    let pool_tokens_to_withdraw = pool_tokens / 2; // Withdraw half

    // Derive PDAs
    let (program_signer, _) = Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());
    let (user_stake_pda, _) =
        find_user_stake_program_address(&id(), &user.pubkey(), user_stake_seed);

    // Create the withdraw stake with session instruction
    let instruction = withdraw_stake_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.validator_list.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &validator.stake_account,
        &user_stake_pda,
        &session_keypair.pubkey(),
        &pool_token_ata,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &token_program_id,
        &program_signer,
        &context.payer.pubkey(), // payer for rent
        pool_tokens_to_withdraw,
        0, // minimum_lamports_out
        user_stake_seed,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_keypair],
        context.last_blockhash,
    );

    let result = context.banks_client.process_transaction(transaction).await;
    assert!(result.is_ok(), "Transaction failed: {:?}", result.err());

    // Verify user stake account was created
    let user_stake_account = context
        .banks_client
        .get_account(user_stake_pda)
        .await
        .unwrap();
    assert!(
        user_stake_account.is_some(),
        "User stake account should exist"
    );

    let user_stake_account = user_stake_account.unwrap();
    assert_eq!(
        user_stake_account.owner,
        solana_stake_interface::program::id(),
        "User stake account should be owned by stake program"
    );

    // Verify stake account is deactivating (deactivation_epoch should be set)
    let stake_state: StakeStateV2 =
        bincode::deserialize(&user_stake_account.data).expect("Failed to deserialize stake state");

    match stake_state {
        StakeStateV2::Stake(meta, stake, _) => {
            // Verify authority is the user stake PDA itself (so it can self-sign the later withdrawal)
            assert_eq!(
                meta.authorized.staker, user_stake_pda,
                "Staker should be user stake PDA"
            );
            assert_eq!(
                meta.authorized.withdrawer, user_stake_pda,
                "Withdrawer should be user stake PDA"
            );
            // Verify stake is deactivating (deactivation_epoch != u64::MAX)
            assert_ne!(
                stake.delegation.deactivation_epoch,
                u64::MAX,
                "Stake should be deactivating"
            );
        }
        _ => panic!("Expected StakeStateV2::Stake, got {:?}", stake_state),
    }

    // Verify pool tokens were burned
    let pool_tokens_after = get_token_balance(&mut context.banks_client, &pool_token_ata).await;
    assert!(
        pool_tokens_after < pool_tokens,
        "Pool tokens should have been burned"
    );
}

#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn success_withdraw_from_stake_account_with_session(token_program_id: Pubkey) {
    // Note: This test verifies the full flow of WithdrawStakeWithSession creating
    // a deactivating stake account. The actual WithdrawFromStakeAccountWithSession
    // will fail with UserStakeNotActive until the cooldown period completes,
    // which we can't easily simulate in solana-program-test.
    //
    // The test validates that:
    // 1. WithdrawStakeWithSession creates a user stake account
    // 2. The stake account is in deactivating state
    // 3. WithdrawFromStakeAccountWithSession correctly rejects early withdrawals
    let (
        mut context,
        stake_pool_accounts,
        user,
        pool_token_ata,
        session_keypair,
        pool_tokens,
        validator,
    ) = setup_withdraw_stake_with_session(token_program_id).await;

    let user_stake_seed: u64 = 0;
    let pool_tokens_to_withdraw = pool_tokens / 2;

    // Derive PDAs
    let (program_signer, _) = Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());
    let (user_stake_pda, _) =
        find_user_stake_program_address(&id(), &user.pubkey(), user_stake_seed);

    // First, do WithdrawStakeWithSession
    let withdraw_stake_ix = withdraw_stake_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.validator_list.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &validator.stake_account,
        &user_stake_pda,
        &session_keypair.pubkey(),
        &pool_token_ata,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &token_program_id,
        &program_signer,
        &context.payer.pubkey(), // payer for rent
        pool_tokens_to_withdraw,
        0,
        user_stake_seed,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[withdraw_stake_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_keypair],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Verify the user stake account exists and has correct state
    let stake_account = get_account(&mut context.banks_client, &user_stake_pda).await;
    assert!(
        stake_account.lamports > 0,
        "User stake account should have lamports"
    );
    assert_eq!(
        stake_account.owner,
        solana_stake_interface::program::id(),
        "User stake account should be owned by stake program"
    );

    // Verify the stake is in deactivating state
    let stake_state: StakeStateV2 =
        bincode::deserialize(&stake_account.data).expect("Failed to deserialize stake state");

    match stake_state {
        StakeStateV2::Stake(meta, stake, _) => {
            // Verify authority is the user stake PDA itself (so it can self-sign the later withdrawal)
            assert_eq!(
                meta.authorized.staker, user_stake_pda,
                "Staker should be user stake PDA"
            );
            assert_eq!(
                meta.authorized.withdrawer, user_stake_pda,
                "Withdrawer should be user stake PDA"
            );
            // Verify stake is deactivating (deactivation_epoch != u64::MAX)
            assert_ne!(
                stake.delegation.deactivation_epoch,
                u64::MAX,
                "Stake should be deactivating"
            );
        }
        _ => panic!("Expected StakeStateV2::Stake, got {:?}", stake_state),
    }

    // Try to call WithdrawFromStakeAccountWithSession
    // It should fail because stake hasn't completed cooldown (same epoch)
    context.last_blockhash = context.banks_client.get_latest_blockhash().await.unwrap();

    let stake_lamports = stake_account.lamports;

    let withdraw_from_stake_ix = withdraw_from_stake_account_with_session(
        &id(),
        &user_stake_pda,
        &user.pubkey(),
        &session_keypair.pubkey(),
        u64::MAX, // Full withdrawal (lamports)
        user_stake_seed,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[withdraw_from_stake_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_keypair],
        context.last_blockhash,
    );

    // This will fail because stake hasn't completed cooldown (still in same epoch)
    let result = context.banks_client.process_transaction(transaction).await;
    assert!(
        result.is_err(),
        "Should fail - stake not yet fully deactivated"
    );

    // Verify the error is UserStakeNotActive
    let err = result.unwrap_err().unwrap();
    match err {
        TransactionError::InstructionError(_, InstructionError::Custom(code)) => {
            assert_eq!(
                code,
                StakePoolError::UserStakeNotActive as u32,
                "Expected UserStakeNotActive error"
            );
        }
        _ => panic!("Expected Custom error, got: {:?}", err),
    }

    // Stake account should still exist with full lamports
    let stake_account_after = get_account(&mut context.banks_client, &user_stake_pda).await;
    assert_eq!(
        stake_account_after.lamports, stake_lamports,
        "Stake account should still have all lamports"
    );
}

#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn fail_withdraw_from_wrong_user_stake_account(token_program_id: Pubkey) {
    let (
        mut context,
        stake_pool_accounts,
        user,
        pool_token_ata,
        session_keypair,
        pool_tokens,
        validator,
    ) = setup_withdraw_stake_with_session(token_program_id).await;

    let user_stake_seed: u64 = 0;
    let pool_tokens_to_withdraw = pool_tokens / 2;

    // Derive PDAs
    let (program_signer, _) = Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());
    let (user_stake_pda, _) =
        find_user_stake_program_address(&id(), &user.pubkey(), user_stake_seed);

    // First, do WithdrawStakeWithSession to create the stake account
    let withdraw_stake_ix = withdraw_stake_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.validator_list.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &validator.stake_account,
        &user_stake_pda,
        &session_keypair.pubkey(),
        &pool_token_ata,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &token_program_id,
        &program_signer,
        &context.payer.pubkey(), // payer for rent
        pool_tokens_to_withdraw,
        0,
        user_stake_seed,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[withdraw_stake_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_keypair],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Try to withdraw with wrong seed (should fail)
    let wrong_seed: u64 = 999;
    let (wrong_stake_pda, _) = find_user_stake_program_address(&id(), &user.pubkey(), wrong_seed);

    context.last_blockhash = context.banks_client.get_latest_blockhash().await.unwrap();

    let withdraw_from_stake_ix = withdraw_from_stake_account_with_session(
        &id(),
        &wrong_stake_pda, // Wrong PDA
        &user.pubkey(),
        &session_keypair.pubkey(),
        u64::MAX, // Full withdrawal (lamports)
        wrong_seed,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[withdraw_from_stake_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_keypair],
        context.last_blockhash,
    );

    let result = context.banks_client.process_transaction(transaction).await;
    assert!(result.is_err(), "Should fail with wrong stake account");
}

#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn fail_withdraw_to_different_user_wallet(token_program_id: Pubkey) {
    let (
        mut context,
        stake_pool_accounts,
        user,
        pool_token_ata,
        session_keypair,
        pool_tokens,
        validator,
    ) = setup_withdraw_stake_with_session(token_program_id).await;

    let user_stake_seed: u64 = 0;
    let pool_tokens_to_withdraw = pool_tokens / 2;

    // Derive PDAs
    let (program_signer, _) = Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());
    let (user_stake_pda, _) =
        find_user_stake_program_address(&id(), &user.pubkey(), user_stake_seed);

    // First, do WithdrawStakeWithSession
    let withdraw_stake_ix = withdraw_stake_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.validator_list.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &validator.stake_account,
        &user_stake_pda,
        &session_keypair.pubkey(),
        &pool_token_ata,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &token_program_id,
        &program_signer,
        &context.payer.pubkey(), // payer for rent
        pool_tokens_to_withdraw,
        0,
        user_stake_seed,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[withdraw_stake_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_keypair],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    context.last_blockhash = context.banks_client.get_latest_blockhash().await.unwrap();

    // Try to withdraw to a different wallet (attacker)
    let attacker = Keypair::new();

    let withdraw_from_stake_ix = withdraw_from_stake_account_with_session(
        &id(),
        &user_stake_pda,
        &attacker.pubkey(), // Different wallet than session user
        &session_keypair.pubkey(),
        u64::MAX, // Full withdrawal (lamports)
        user_stake_seed,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[withdraw_from_stake_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_keypair],
        context.last_blockhash,
    );

    let result = context.banks_client.process_transaction(transaction).await;
    assert!(
        result.is_err(),
        "Should fail when wallet doesn't match session user"
    );
}
