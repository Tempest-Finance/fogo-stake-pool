#![allow(clippy::arithmetic_side_effects)]
#![cfg(feature = "test-sbf")]

mod helpers;

use crate::helpers::wsol::setup_with_session_account;
use spl_stake_pool::instruction::withdraw_wsol_with_session;
use {
    fogo_sessions_sdk::token::PROGRAM_SIGNER_SEED,
    helpers::*,
    solana_program::{instruction::InstructionError, pubkey::Pubkey},
    solana_program_test::*,
    solana_sdk::{
        signature::{Keypair, Signer},
        transaction::{Transaction, TransactionError},
    },
    spl_stake_pool::{error::StakePoolError, id},
    spl_token::native_mint,
    test_case::test_case,
};

/// Test with wrong destination account (not user's ATA)
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn fail_wrong_destination_ata(token_program_id: Pubkey) {
    let (context, stake_pool_accounts, user, pool_token_account, session_signer, pool_tokens) =
        setup_with_session_account(token_program_id).await;

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

    let pool_tokens_to_withdraw = pool_tokens / 2;

    let approve_ix = spl_token::instruction::approve_checked(
        &spl_token::id(),
        &pool_token_account,
        &stake_pool_accounts.pool_mint.pubkey(),
        &session_signer.pubkey(),
        &user.pubkey(),
        &[],
        pool_tokens_to_withdraw,
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

    let withdraw_instruction = withdraw_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &wrong_wsol_ata, // Wrong ATA!
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &program_signer,
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        pool_tokens_to_withdraw,
        0, // minimum_lamports_out - accept any amount
    );

    let transaction = Transaction::new_signed_with_payer(
        &[withdraw_instruction],
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

/// Test dust withdrawal (should fail with WithdrawalTooSmall)
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn fail_dust_withdrawal(token_program_id: Pubkey) {
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

    let dust_amount = 1; // 1 lamport worth of pool tokens

    let approve_ix = spl_token::instruction::approve_checked(
        &spl_token::id(),
        &pool_token_account,
        &stake_pool_accounts.pool_mint.pubkey(),
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

    let withdraw_instruction = withdraw_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &wsol_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &program_signer,
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        dust_amount,
        0, // minimum_lamports_out - accept any amount
    );

    let transaction = Transaction::new_signed_with_payer(
        &[withdraw_instruction],
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
                StakePoolError::WithdrawalTooSmall as u32,
                "Expected WithdrawalTooSmall error"
            );
        }
        _ => panic!("Expected WithdrawalTooSmall error, got: {:?}", error),
    }
}

/// Test full withdrawal (withdraw all pool tokens)
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn success_full_withdrawal(token_program_id: Pubkey) {
    let (mut context, stake_pool_accounts, user, pool_token_account, session_signer, pool_tokens) =
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

    // Withdraw all pool tokens
    let approve_ix = spl_token::instruction::approve_checked(
        &spl_token::id(),
        &pool_token_account,
        &stake_pool_accounts.pool_mint.pubkey(),
        &session_signer.pubkey(),
        &user.pubkey(),
        &[],
        pool_tokens,
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

    let withdraw_instruction = withdraw_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &wsol_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &program_signer,
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        pool_tokens,
        0, // minimum_lamports_out - accept any amount
    );

    let transaction = Transaction::new_signed_with_payer(
        &[withdraw_instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_signer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Verify user received WSOL
    let wsol_balance = get_token_balance(&mut context.banks_client, &wsol_token_account).await;
    assert!(wsol_balance > 0, "User should have received WSOL");

    // Verify user has 0 pool tokens left
    let remaining_pool_tokens =
        get_token_balance(&mut context.banks_client, &pool_token_account).await;
    assert_eq!(
        remaining_pool_tokens, 0,
        "User should have 0 pool tokens after full withdrawal"
    );
}

/// Test multiple withdrawals
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn success_multiple_withdrawals(token_program_id: Pubkey) {
    let (mut context, stake_pool_accounts, user, pool_token_account, session_signer, pool_tokens) =
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

    let withdraw_amount = pool_tokens / 3;

    // Perform 2 withdrawals
    for i in 0..2 {
        let approve_ix = spl_token::instruction::approve_checked(
            &spl_token::id(),
            &pool_token_account,
            &stake_pool_accounts.pool_mint.pubkey(),
            &session_signer.pubkey(),
            &user.pubkey(),
            &[],
            withdraw_amount,
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

        let withdraw_instruction = withdraw_wsol_with_session(
            &id(),
            &stake_pool_accounts.stake_pool.pubkey(),
            &stake_pool_accounts.withdraw_authority,
            &session_signer.pubkey(),
            &pool_token_account,
            &stake_pool_accounts.reserve_stake.pubkey(),
            &wsol_token_account,
            &stake_pool_accounts.pool_fee_account.pubkey(),
            &stake_pool_accounts.pool_mint.pubkey(),
            &stake_pool_accounts.token_program_id,
            &program_signer,
            &context.payer.pubkey(),
            &user.pubkey(),
            None,
            withdraw_amount,
            0, // minimum_lamports_out - accept any amount
        );

        let transaction = Transaction::new_signed_with_payer(
            &[withdraw_instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &session_signer],
            context.last_blockhash,
        );

        context
            .banks_client
            .process_transaction(transaction)
            .await
            .unwrap_or_else(|e| panic!("Withdrawal {} failed: {:?}", i + 1, e));
    }

    // Verify accumulated WSOL balance
    let wsol_balance = get_token_balance(&mut context.banks_client, &wsol_token_account).await;
    assert!(
        wsol_balance > 0,
        "User should have accumulated WSOL from multiple withdrawals"
    );

    // Verify pool tokens decreased
    let remaining_pool_tokens =
        get_token_balance(&mut context.banks_client, &pool_token_account).await;
    assert!(
        remaining_pool_tokens < pool_tokens,
        "Pool tokens should have decreased"
    );
}

/// Test wrong program signer PDA
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn fail_wrong_program_signer(token_program_id: Pubkey) {
    let (context, stake_pool_accounts, user, pool_token_account, session_signer, pool_tokens) =
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

    let pool_tokens_to_withdraw = pool_tokens / 2;

    let approve_ix = spl_token::instruction::approve_checked(
        &spl_token::id(),
        &pool_token_account,
        &stake_pool_accounts.pool_mint.pubkey(),
        &session_signer.pubkey(),
        &user.pubkey(),
        &[],
        pool_tokens_to_withdraw,
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

    // Use wrong program signer
    let wrong_program_signer = Keypair::new().pubkey();

    let withdraw_instruction = withdraw_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &wsol_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &wrong_program_signer, // Wrong signer!
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        pool_tokens_to_withdraw,
        0, // minimum_lamports_out - accept any amount
    );

    let transaction = Transaction::new_signed_with_payer(
        &[withdraw_instruction],
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

/// Test on-chain wSOL ATA creation
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn success_onchain_wsol_ata_creation(token_program_id: Pubkey) {
    let (mut context, stake_pool_accounts, user, pool_token_account, session_signer, pool_tokens) =
        setup_with_session_account(token_program_id).await;

    let wsol_token_account =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &user.pubkey(),
            &native_mint::id(),
            &spl_token::id(),
        );

    let (program_signer, _program_signer_bump) =
        Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    let pool_tokens_to_withdraw = pool_tokens / 2;

    let approve_ix = spl_token::instruction::approve_checked(
        &spl_token::id(),
        &pool_token_account,
        &stake_pool_accounts.pool_mint.pubkey(),
        &session_signer.pubkey(),
        &user.pubkey(),
        &[],
        pool_tokens_to_withdraw,
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

    let withdraw_instruction = withdraw_wsol_with_session(
        &id(),
        &stake_pool_accounts.stake_pool.pubkey(),
        &stake_pool_accounts.withdraw_authority,
        &session_signer.pubkey(),
        &pool_token_account,
        &stake_pool_accounts.reserve_stake.pubkey(),
        &wsol_token_account,
        &stake_pool_accounts.pool_fee_account.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &stake_pool_accounts.token_program_id,
        &program_signer,
        &context.payer.pubkey(),
        &user.pubkey(),
        None,
        pool_tokens_to_withdraw,
        0,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[withdraw_instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &session_signer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Verify wSOL ATA was created and user received wSOL
    let wsol_balance = get_token_balance(&mut context.banks_client, &wsol_token_account).await;
    assert!(wsol_balance > 0, "User should have received wSOL");

    // Verify pool tokens were burned
    let remaining_pool_tokens =
        get_token_balance(&mut context.banks_client, &pool_token_account).await;
    assert!(
        remaining_pool_tokens < pool_tokens,
        "Pool tokens should have decreased"
    );
}
