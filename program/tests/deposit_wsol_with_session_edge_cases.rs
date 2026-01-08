#![allow(clippy::arithmetic_side_effects)]
#![cfg(feature = "test-sbf")]

mod helpers;

use crate::helpers::wsol::{setup_with_session_account, setup_with_session_account_no_ata, TRANSIENT_WSOL_SEED};
use spl_stake_pool::instruction::deposit_wsol_with_session;
use {
    fogo_sessions_sdk::token::PROGRAM_SIGNER_SEED,
    helpers::*,
    solana_program::{
        borsh1::try_from_slice_unchecked, instruction::InstructionError, program_pack::Pack,
        pubkey::Pubkey,
    },
    solana_program_test::*,
    solana_sdk::{
        signature::{Keypair, Signer},
        transaction::{Transaction, TransactionError},
    },
    spl_stake_pool::{error::StakePoolError, id},
    spl_token::native_mint,
    test_case::test_case,
};

/// Test with wrong wsol_token account (not user's ATA)
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn fail_wrong_wsol_token_ata(token_program_id: Pubkey) {
    let (context, stake_pool_accounts, user, pool_token_account, session_signer, _pool_tokens) =
        setup_with_session_account(token_program_id).await;

    let (transient_wsol_pda, _transient_bump) =
        Pubkey::find_program_address(&[TRANSIENT_WSOL_SEED, user.pubkey().as_ref()], &id());

    // Create a different user's WSOL ATA
    let wrong_user = Keypair::new();
    let wrong_wsol_ata = spl_associated_token_account::get_associated_token_address_with_program_id(
        &wrong_user.pubkey(),
        &native_mint::id(),
        &spl_token::id(),
    );

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &context.payer.pubkey(),
        &wrong_user.pubkey(),
        &native_mint::id(),
        &spl_token::id(),
    );

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[create_ata_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let (program_signer, _program_signer_bump) =
        Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    let instruction = deposit_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &wrong_wsol_ata, // Wrong ATA!
        &transient_wsol_pda,
        &program_signer,
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        TEST_STAKE_AMOUNT,
        0, // minimum_pool_tokens_out - accept any amount
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_signer],
        context.last_blockhash,
    );

    let error = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    match error {
        TransactionError::InstructionError(_, InstructionError::InvalidAccountData) => {}
        _ => panic!("Expected InvalidAccountData error, got: {:?}", error),
    }
}

/// Test with wrong transient PDA
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn fail_wrong_transient_pda(token_program_id: Pubkey) {
    let (context, stake_pool_accounts, user, pool_token_account, session_signer, _pool_tokens) =
        setup_with_session_account(token_program_id).await;

    let wsol_token_account =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &user.pubkey(),
            &native_mint::id(),
            &spl_token::id(),
        );

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &context.payer.pubkey(),
        &user.pubkey(),
        &native_mint::id(),
        &spl_token::id(),
    );

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[create_ata_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let (program_signer, _program_signer_bump) =
        Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    // Use wrong transient PDA (using different seed)
    let (wrong_transient_pda, _) =
        Pubkey::find_program_address(&[b"wrong_seed", user.pubkey().as_ref()], &id());

    let instruction = deposit_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &wsol_token_account,
        &wrong_transient_pda, // Wrong PDA!
        &program_signer,
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        TEST_STAKE_AMOUNT,
        0, // minimum_pool_tokens_out - accept any amount
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_signer],
        context.last_blockhash,
    );

    let error = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    match error {
        TransactionError::InstructionError(_, InstructionError::InvalidSeeds) => {}
        _ => panic!("Expected InvalidSeeds error, got: {:?}", error),
    }
}

/// Test with wrong program_signer PDA
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn fail_wrong_program_signer(token_program_id: Pubkey) {
    let (context, stake_pool_accounts, user, pool_token_account, session_signer, _pool_tokens) =
        setup_with_session_account(token_program_id).await;

    let (transient_wsol_pda, _transient_bump) =
        Pubkey::find_program_address(&[TRANSIENT_WSOL_SEED, user.pubkey().as_ref()], &id());

    let wsol_token_account =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &user.pubkey(),
            &native_mint::id(),
            &spl_token::id(),
        );

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &context.payer.pubkey(),
        &user.pubkey(),
        &native_mint::id(),
        &spl_token::id(),
    );

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[create_ata_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    // Use wrong program signer (arbitrary key)
    let wrong_program_signer = Keypair::new().pubkey();

    let instruction = deposit_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &wsol_token_account,
        &transient_wsol_pda,
        &wrong_program_signer, // Wrong signer!
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        TEST_STAKE_AMOUNT,
        0, // minimum_pool_tokens_out - accept any amount
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_signer],
        context.last_blockhash,
    );

    let error = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    match error {
        TransactionError::InstructionError(_, InstructionError::InvalidSeeds) => {}
        _ => panic!("Expected InvalidSeeds error, got: {:?}", error),
    }
}

/// Test dust deposit (should fail with DepositTooSmall)
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn fail_dust_deposit(token_program_id: Pubkey) {
    let (mut context, stake_pool_accounts, user, pool_token_account, session_signer, _pool_tokens) =
        setup_with_session_account(token_program_id).await;

    let (transient_wsol_pda, _transient_bump) =
        Pubkey::find_program_address(&[TRANSIENT_WSOL_SEED, user.pubkey().as_ref()], &id());

    let wsol_token_account =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &user.pubkey(),
            &native_mint::id(),
            &spl_token::id(),
        );

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &context.payer.pubkey(),
        &user.pubkey(),
        &native_mint::id(),
        &spl_token::id(),
    );

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[create_ata_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let dust_amount = 1; // 1 lamport

    let approve_ix = spl_token::instruction::approve_checked(
        &spl_token::id(),
        &wsol_token_account,
        &native_mint::id(),
        &session_signer.pubkey(),
        &user.pubkey(),
        &[],
        dust_amount,
        native_mint::DECIMALS,
    )
    .unwrap();

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[approve_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &user],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    transfer(
        &mut context.banks_client,
        &context.payer,
        &context.last_blockhash,
        &wsol_token_account,
        dust_amount,
    )
    .await;

    let sync_native_ix =
        spl_token::instruction::sync_native(&spl_token::id(), &wsol_token_account).unwrap();

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[sync_native_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let (program_signer, _program_signer_bump) =
        Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    let instruction = deposit_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &wsol_token_account,
        &transient_wsol_pda,
        &program_signer,
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        dust_amount,
        0, // minimum_pool_tokens_out - accept any amount
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_signer],
        context.last_blockhash,
    );

    let error = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    match error {
        TransactionError::InstructionError(_, InstructionError::Custom(error_index)) => {
            assert_eq!(
                error_index,
                StakePoolError::DepositTooSmall as u32,
                "Expected DepositTooSmall error"
            );
        }
        _ => panic!("Expected DepositTooSmall error, got: {:?}", error),
    }
}

/// Test multiple consecutive deposits by same user (verify no collision)
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn success_multiple_deposits(token_program_id: Pubkey) {
    let (mut context, stake_pool_accounts, user, pool_token_account, session_signer, _pool_tokens) =
        setup_with_session_account(token_program_id).await;

    let (transient_wsol_pda, _transient_bump) =
        Pubkey::find_program_address(&[TRANSIENT_WSOL_SEED, user.pubkey().as_ref()], &id());

    let wsol_token_account =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &user.pubkey(),
            &native_mint::id(),
            &spl_token::id(),
        );

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &context.payer.pubkey(),
        &user.pubkey(),
        &native_mint::id(),
        &spl_token::id(),
    );

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[create_ata_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let (program_signer, _program_signer_bump) =
        Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    let deposit_amount = TEST_STAKE_AMOUNT / 2;

    // Perform 2 deposits
    for i in 0..2 {
        let approve_ix = spl_token::instruction::approve_checked(
            &spl_token::id(),
            &wsol_token_account,
            &native_mint::id(),
            &session_signer.pubkey(),
            &user.pubkey(),
            &[],
            deposit_amount,
            native_mint::DECIMALS,
        )
        .unwrap();

        context
            .banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[approve_ix],
                Some(&context.payer.pubkey()),
                &[&context.payer, &user],
                context.last_blockhash,
            ))
            .await
            .unwrap();

        transfer(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            &wsol_token_account,
            deposit_amount,
        )
        .await;

        let sync_native_ix =
            spl_token::instruction::sync_native(&spl_token::id(), &wsol_token_account).unwrap();

        context
            .banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[sync_native_ix],
                Some(&context.payer.pubkey()),
                &[&context.payer],
                context.last_blockhash,
            ))
            .await
            .unwrap();

        let instruction = deposit_wsol_with_session(
            &id(),
            &stake_pool_accounts.stake_pool.pubkey(),
            &stake_pool_accounts.withdraw_authority,
            &stake_pool_accounts.reserve_stake.pubkey(),
            &session_signer.pubkey(),
            &pool_token_account,
            &stake_pool_accounts.pool_fee_account.pubkey(),
            &stake_pool_accounts.pool_fee_account.pubkey(),
            &stake_pool_accounts.pool_mint.pubkey(),
            &stake_pool_accounts.token_program_id,
            &wsol_token_account,
            &transient_wsol_pda,
            &program_signer,
            &context.payer.pubkey(),
            &user.pubkey(),
            None,
            deposit_amount,
            0, // minimum_pool_tokens_out - accept any amount
        );

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &session_signer],
            context.last_blockhash,
        );

        context
            .banks_client
            .process_transaction(transaction)
            .await
            .unwrap_or_else(|e| panic!("Deposit {} failed: {:?}", i + 1, e));

        // Verify transient account is closed after each deposit
        let transient_account_result = context
            .banks_client
            .get_account(transient_wsol_pda)
            .await
            .unwrap();
        assert!(
            transient_account_result.is_none(),
            "Transient account should be closed after deposit {}",
            i + 1
        );
    }

    // Verify total pool tokens received
    let final_pool_balance =
        get_token_balance(&mut context.banks_client, &pool_token_account).await;

    assert!(
        final_pool_balance > deposit_amount,
        "Should have received pool tokens from both deposits"
    );
}

/// Test exact deposit fee calculation
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn success_exact_fee_calculation(token_program_id: Pubkey) {
    let (mut context, stake_pool_accounts, user, pool_token_account, session_signer, _pool_tokens) =
        setup_with_session_account(token_program_id).await;

    let (transient_wsol_pda, _transient_bump) =
        Pubkey::find_program_address(&[TRANSIENT_WSOL_SEED, user.pubkey().as_ref()], &id());

    let wsol_token_account =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &user.pubkey(),
            &native_mint::id(),
            &spl_token::id(),
        );

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &context.payer.pubkey(),
        &user.pubkey(),
        &native_mint::id(),
        &spl_token::id(),
    );

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[create_ata_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let approve_ix = spl_token::instruction::approve_checked(
        &spl_token::id(),
        &wsol_token_account,
        &native_mint::id(),
        &session_signer.pubkey(),
        &user.pubkey(),
        &[],
        TEST_STAKE_AMOUNT,
        native_mint::DECIMALS,
    )
    .unwrap();

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[approve_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &user],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    transfer(
        &mut context.banks_client,
        &context.payer,
        &context.last_blockhash,
        &wsol_token_account,
        TEST_STAKE_AMOUNT,
    )
    .await;

    let sync_native_ix =
        spl_token::instruction::sync_native(&spl_token::id(), &wsol_token_account).unwrap();

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[sync_native_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let (program_signer, _program_signer_bump) =
        Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    // Get initial user balance before this deposit
    let initial_user_balance =
        get_token_balance(&mut context.banks_client, &pool_token_account).await;

    // Get stake pool state to calculate expected fees
    let stake_pool = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.stake_pool.pubkey(),
    )
    .await;
    let stake_pool_state =
        try_from_slice_unchecked::<spl_stake_pool::state::StakePool>(&stake_pool.data).unwrap();

    // Calculate expected pool tokens and fees
    let expected_new_pool_tokens = stake_pool_state
        .calc_pool_tokens_for_deposit(TEST_STAKE_AMOUNT)
        .unwrap();
    let expected_fee = stake_pool_accounts.calculate_sol_deposit_fee(expected_new_pool_tokens);
    let expected_user_tokens_from_deposit = expected_new_pool_tokens - expected_fee;

    let instruction = deposit_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &wsol_token_account,
        &transient_wsol_pda,
        &program_signer,
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        TEST_STAKE_AMOUNT,
        0, // minimum_pool_tokens_out - accept any amount
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_signer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Verify exact fee amounts
    let final_user_balance =
        get_token_balance(&mut context.banks_client, &pool_token_account).await;
    let user_tokens_received = final_user_balance - initial_user_balance;

    let manager_balance = get_token_balance(
        &mut context.banks_client,
        &stake_pool_accounts.pool_fee_account.pubkey(),
    )
    .await;

    assert_eq!(
        user_tokens_received, expected_user_tokens_from_deposit,
        "User should receive exact amount after fees"
    );
    assert!(
        manager_balance >= expected_fee,
        "Manager should receive at least the deposit fee amount"
    );

    // Verify total tokens minted in this deposit
    let total_minted_this_deposit = user_tokens_received + expected_fee;
    assert_eq!(
        total_minted_this_deposit, expected_new_pool_tokens,
        "Total minted tokens should equal user tokens + manager fee"
    );

    // Verify transient account was closed (rent refunded)
    let transient_account_result = context
        .banks_client
        .get_account(transient_wsol_pda)
        .await
        .unwrap();
    assert!(
        transient_account_result.is_none(),
        "Transient account should be closed and rent refunded"
    );
}

/// Test transient account rent is properly refunded
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn success_transient_rent_refunded(token_program_id: Pubkey) {
    let (mut context, stake_pool_accounts, user, pool_token_account, session_signer, _pool_tokens) =
        setup_with_session_account(token_program_id).await;

    let (transient_wsol_pda, _transient_bump) =
        Pubkey::find_program_address(&[TRANSIENT_WSOL_SEED, user.pubkey().as_ref()], &id());

    let wsol_token_account =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &user.pubkey(),
            &native_mint::id(),
            &spl_token::id(),
        );

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &context.payer.pubkey(),
        &user.pubkey(),
        &native_mint::id(),
        &spl_token::id(),
    );

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[create_ata_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let approve_ix = spl_token::instruction::approve_checked(
        &spl_token::id(),
        &wsol_token_account,
        &native_mint::id(),
        &session_signer.pubkey(),
        &user.pubkey(),
        &[],
        TEST_STAKE_AMOUNT,
        native_mint::DECIMALS,
    )
    .unwrap();

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[approve_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &user],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    // Track payer balance before deposit to verify rent refund
    let payer_balance_before = {
        let payer_account = context
            .banks_client
            .get_account(context.payer.pubkey())
            .await
            .unwrap()
            .unwrap();
        payer_account.lamports
    };

    transfer(
        &mut context.banks_client,
        &context.payer,
        &context.last_blockhash,
        &wsol_token_account,
        TEST_STAKE_AMOUNT,
    )
    .await;

    let sync_native_ix =
        spl_token::instruction::sync_native(&spl_token::id(), &wsol_token_account).unwrap();

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[sync_native_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let (program_signer, _program_signer_bump) =
        Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    let instruction = deposit_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &wsol_token_account,
        &transient_wsol_pda,
        &program_signer,
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        TEST_STAKE_AMOUNT,
        0, // minimum_pool_tokens_out - accept any amount
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_signer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Verify transient account doesn't exist (was closed)
    let transient_account_result = context
        .banks_client
        .get_account(transient_wsol_pda)
        .await
        .unwrap();
    assert!(
        transient_account_result.is_none(),
        "Transient account should be closed"
    );

    // Verify payer got rent back
    // The payer should have lost: TEST_STAKE_AMOUNT + transaction fees
    // But should have received back: rent for transient account
    let payer_balance_after = {
        let payer_account = context
            .banks_client
            .get_account(context.payer.pubkey())
            .await
            .unwrap()
            .unwrap();
        payer_account.lamports
    };

    // Payer should have lost approximately TEST_STAKE_AMOUNT (plus some tx fees, minus rent refund)
    // Allow generous tolerance for transaction fees
    let expected_loss_max = TEST_STAKE_AMOUNT + 10_000_000; // 0.01 SOL tolerance for fees
    let expected_loss_min = TEST_STAKE_AMOUNT - 3_000_000; // Account for rent refund (~2M lamports)
    let actual_loss = payer_balance_before - payer_balance_after;

    assert!(
        actual_loss <= expected_loss_max && actual_loss >= expected_loss_min,
        "Payer balance change should account for deposit amount and rent refund: lost {} lamports, expected between {} and {}",
        actual_loss,
        expected_loss_min,
        expected_loss_max
    );
}

#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn success_different_payer_from_fee_payer(token_program_id: Pubkey) {
    let (mut context, stake_pool_accounts, user, pool_token_account, session_signer, _pool_tokens) =
        setup_with_session_account(token_program_id).await;

    let (transient_wsol_pda, _) =
        Pubkey::find_program_address(&[TRANSIENT_WSOL_SEED, user.pubkey().as_ref()], &id());

    let wsol_token_account =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &user.pubkey(),
            &native_mint::id(),
            &spl_token::id(),
        );

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[spl_associated_token_account::instruction::create_associated_token_account(
                &context.payer.pubkey(),
                &user.pubkey(),
                &native_mint::id(),
                &spl_token::id(),
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let different_payer = Keypair::new();
    transfer(
        &mut context.banks_client,
        &context.payer,
        &context.last_blockhash,
        &different_payer.pubkey(),
        10_000_000_000,
    )
    .await;

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[spl_token::instruction::approve_checked(
                &spl_token::id(),
                &wsol_token_account,
                &native_mint::id(),
                &session_signer.pubkey(),
                &user.pubkey(),
                &[],
                TEST_STAKE_AMOUNT,
                native_mint::DECIMALS,
            )
            .unwrap()],
            Some(&context.payer.pubkey()),
            &[&context.payer, &user],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    transfer(
        &mut context.banks_client,
        &context.payer,
        &context.last_blockhash,
        &wsol_token_account,
        TEST_STAKE_AMOUNT,
    )
    .await;

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[spl_token::instruction::sync_native(&spl_token::id(), &wsol_token_account).unwrap()],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let (program_signer, _) = Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    let instruction = deposit_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &wsol_token_account,
        &transient_wsol_pda,
        &program_signer,
        &different_payer.pubkey(),
        &user.pubkey(),
        None,
        TEST_STAKE_AMOUNT,
        0,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_signer, &different_payer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .expect("Deposit with different payer should succeed");

    assert!(
        get_token_balance(&mut context.banks_client, &pool_token_account).await > 0,
        "User should have received pool tokens"
    );
}

/// Test on-chain ATA creation when pool token ATA doesn't exist
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn success_onchain_ata_creation(token_program_id: Pubkey) {
    let (mut context, stake_pool_accounts, user, pool_token_account, session_signer) =
        setup_with_session_account_no_ata(token_program_id).await;

    let (transient_wsol_pda, _) =
        Pubkey::find_program_address(&[TRANSIENT_WSOL_SEED, user.pubkey().as_ref()], &id());

    let wsol_token_account =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &user.pubkey(),
            &native_mint::id(),
            &spl_token::id(),
        );

    // Create WSOL ATA for user
    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[spl_associated_token_account::instruction::create_associated_token_account(
                &context.payer.pubkey(),
                &user.pubkey(),
                &native_mint::id(),
                &spl_token::id(),
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    // Approve session signer as delegate
    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[spl_token::instruction::approve_checked(
                &spl_token::id(),
                &wsol_token_account,
                &native_mint::id(),
                &session_signer.pubkey(),
                &user.pubkey(),
                &[],
                TEST_STAKE_AMOUNT,
                native_mint::DECIMALS,
            )
            .unwrap()],
            Some(&context.payer.pubkey()),
            &[&context.payer, &user],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    // Fund WSOL account
    transfer(
        &mut context.banks_client,
        &context.payer,
        &context.last_blockhash,
        &wsol_token_account,
        TEST_STAKE_AMOUNT,
    )
    .await;

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[spl_token::instruction::sync_native(&spl_token::id(), &wsol_token_account).unwrap()],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    // Verify pool token ATA does NOT exist before deposit
    let ata_before = context
        .banks_client
        .get_account(pool_token_account)
        .await
        .unwrap();
    assert!(
        ata_before.is_none(),
        "Pool token ATA should NOT exist before deposit"
    );

    let (program_signer, _) = Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    let instruction = deposit_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &wsol_token_account,
        &transient_wsol_pda,
        &program_signer,
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        TEST_STAKE_AMOUNT,
        0,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_signer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .expect("Deposit with on-chain ATA creation should succeed");

    // Verify pool token ATA was created and has tokens
    let ata_after = context
        .banks_client
        .get_account(pool_token_account)
        .await
        .unwrap();
    assert!(
        ata_after.is_some(),
        "Pool token ATA should exist after deposit"
    );

    let pool_token_balance =
        get_token_balance(&mut context.banks_client, &pool_token_account).await;
    assert!(
        pool_token_balance > 0,
        "User should have received pool tokens"
    );

    // ATA rent should have been deducted from deposit
    // With ATA rent ~0.002 SOL, user should receive slightly fewer pool tokens
    let ata_rent = context
        .banks_client
        .get_rent()
        .await
        .unwrap()
        .minimum_balance(spl_token::state::Account::LEN);

    // Pool tokens should be less than if no ATA creation was needed
    // (deposit_amount - ata_rent) worth of pool tokens
    let expected_max_tokens = TEST_STAKE_AMOUNT - ata_rent;
    assert!(
        pool_token_balance <= expected_max_tokens,
        "Pool tokens ({}) should be reduced by ATA rent cost",
        pool_token_balance
    );
}

/// Test deposit fails when amount is less than ATA rent (for first deposit)
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn fail_deposit_less_than_ata_rent(token_program_id: Pubkey) {
    let (mut context, stake_pool_accounts, user, pool_token_account, session_signer) =
        setup_with_session_account_no_ata(token_program_id).await;

    let (transient_wsol_pda, _) =
        Pubkey::find_program_address(&[TRANSIENT_WSOL_SEED, user.pubkey().as_ref()], &id());

    let wsol_token_account =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &user.pubkey(),
            &native_mint::id(),
            &spl_token::id(),
        );

    // Create WSOL ATA for user
    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[spl_associated_token_account::instruction::create_associated_token_account(
                &context.payer.pubkey(),
                &user.pubkey(),
                &native_mint::id(),
                &spl_token::id(),
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    // Get ATA rent to create a deposit that's too small
    let ata_rent = context
        .banks_client
        .get_rent()
        .await
        .unwrap()
        .minimum_balance(spl_token::state::Account::LEN);

    // Deposit amount less than ATA rent
    let small_deposit = ata_rent - 1;

    // Approve session signer as delegate
    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[spl_token::instruction::approve_checked(
                &spl_token::id(),
                &wsol_token_account,
                &native_mint::id(),
                &session_signer.pubkey(),
                &user.pubkey(),
                &[],
                small_deposit,
                native_mint::DECIMALS,
            )
            .unwrap()],
            Some(&context.payer.pubkey()),
            &[&context.payer, &user],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    // Fund WSOL account with small amount
    transfer(
        &mut context.banks_client,
        &context.payer,
        &context.last_blockhash,
        &wsol_token_account,
        small_deposit,
    )
    .await;

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[spl_token::instruction::sync_native(&spl_token::id(), &wsol_token_account).unwrap()],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let (program_signer, _) = Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    let instruction = deposit_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &wsol_token_account,
        &transient_wsol_pda,
        &program_signer,
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        small_deposit,
        0,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_signer],
        context.last_blockhash,
    );

    let error = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    match error {
        TransactionError::InstructionError(_, InstructionError::Custom(error_index)) => {
            assert_eq!(
                error_index,
                StakePoolError::DepositTooSmall as u32,
                "Expected DepositTooSmall error"
            );
        }
        _ => panic!("Expected DepositTooSmall error, got: {:?}", error),
    }
}

/// Test on-chain ATA creation with deposit slightly more than ATA rent
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn success_deposit_slightly_more_than_ata_rent(token_program_id: Pubkey) {
    let (mut context, stake_pool_accounts, user, pool_token_account, session_signer) =
        setup_with_session_account_no_ata(token_program_id).await;

    let (transient_wsol_pda, _) =
        Pubkey::find_program_address(&[TRANSIENT_WSOL_SEED, user.pubkey().as_ref()], &id());

    let wsol_token_account =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &user.pubkey(),
            &native_mint::id(),
            &spl_token::id(),
        );

    // Create WSOL ATA for user
    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[spl_associated_token_account::instruction::create_associated_token_account(
                &context.payer.pubkey(),
                &user.pubkey(),
                &native_mint::id(),
                &spl_token::id(),
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    // Get ATA rent - deposit slightly more to get at least 1 pool token
    let ata_rent = context
        .banks_client
        .get_rent()
        .await
        .unwrap()
        .minimum_balance(spl_token::state::Account::LEN);

    // Deposit ATA rent + enough to get 1 pool token (at least 1 lamport effectively deposited)
    // Use 1 SOL extra to ensure we get pool tokens
    let deposit_amount = ata_rent + 1_000_000_000;

    // Approve session signer as delegate
    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[spl_token::instruction::approve_checked(
                &spl_token::id(),
                &wsol_token_account,
                &native_mint::id(),
                &session_signer.pubkey(),
                &user.pubkey(),
                &[],
                deposit_amount,
                native_mint::DECIMALS,
            )
            .unwrap()],
            Some(&context.payer.pubkey()),
            &[&context.payer, &user],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    // Fund WSOL account
    transfer(
        &mut context.banks_client,
        &context.payer,
        &context.last_blockhash,
        &wsol_token_account,
        deposit_amount,
    )
    .await;

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[spl_token::instruction::sync_native(&spl_token::id(), &wsol_token_account).unwrap()],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let (program_signer, _) = Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    let instruction = deposit_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &wsol_token_account,
        &transient_wsol_pda,
        &program_signer,
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        deposit_amount,
        0,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_signer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .expect("Deposit slightly more than ATA rent should succeed");

    // Verify ATA was created
    let ata_after = context
        .banks_client
        .get_account(pool_token_account)
        .await
        .unwrap();
    assert!(
        ata_after.is_some(),
        "Pool token ATA should exist after deposit"
    );

    // User should have pool tokens from the remaining amount after ATA rent
    let pool_token_balance =
        get_token_balance(&mut context.banks_client, &pool_token_account).await;
    assert!(
        pool_token_balance > 0,
        "User should have pool tokens after deposit"
    );
}
