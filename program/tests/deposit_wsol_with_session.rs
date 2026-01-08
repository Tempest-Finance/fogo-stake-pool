#![allow(clippy::arithmetic_side_effects)]
#![cfg(feature = "test-sbf")]

mod helpers;

use crate::helpers::wsol::{setup_with_session_account, TRANSIENT_WSOL_SEED};
use spl_stake_pool::instruction::deposit_wsol_with_session;
use {
    fogo_sessions_sdk::{session::SESSION_MANAGER_ID, token::PROGRAM_SIGNER_SEED},
    helpers::*,
    solana_program::{borsh1::try_from_slice_unchecked, program_pack::Pack},
    solana_program_test::*,
    solana_sdk::{pubkey::Pubkey, signature::Signer, transaction::Transaction},
    spl_stake_pool::{id, MINIMUM_RESERVE_LAMPORTS},
    spl_token::native_mint,
    test_case::test_case,
};

/// Test with proper FOGO Session account owned by SESSION_MANAGER_ID
#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn success(token_program_id: Pubkey) {
    let (mut context, stake_pool_accounts, user, pool_token_account, session_signer, pool_tokens) =
        setup_with_session_account(token_program_id).await;

    let (transient_wsol_pda, _transient_bump) =
        Pubkey::find_program_address(&[TRANSIENT_WSOL_SEED, user.pubkey().as_ref()], &id());

    let wsol_token_account =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &user.pubkey(),
            &native_mint::id(),
            &spl_token::id(),
        );

    // Create the WSOL ATA
    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &context.payer.pubkey(),
        &user.pubkey(),
        &native_mint::id(),
        &spl_token::id(),
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

    // Approve session_signer as delegate for WSOL tokens
    {
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

        let approve_tx = Transaction::new_signed_with_payer(
            &[approve_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &user], // User signs to approve delegation
            context.last_blockhash,
        );

        context
            .banks_client
            .process_transaction(approve_tx)
            .await
            .unwrap();
    }

    // Fund the WSOL account by transferring SOL to it
    transfer(
        &mut context.banks_client,
        &context.payer,
        &context.last_blockhash,
        &wsol_token_account,
        TEST_STAKE_AMOUNT,
    )
    .await;

    // Sync native - synchronize the lamports balance with the token balance
    let sync_native_ix =
        spl_token::instruction::sync_native(&spl_token::id(), &wsol_token_account).unwrap();

    let sync_native_tx = Transaction::new_signed_with_payer(
        &[sync_native_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(sync_native_tx)
        .await
        .unwrap();

    // Create program signer PDA (from fogo_sessions_sdk)
    let (program_signer, _program_signer_bump) =
        Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    // Create instruction
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

    let result = context.banks_client.process_transaction(transaction).await;

    if let Err(ref err) = result {
        panic!("Transaction failed: {:?}", err);
    }

    // Pool token balance assertions
    let user_pool_token_balance =
        get_token_balance(&mut context.banks_client, &pool_token_account).await;

    assert!(
        user_pool_token_balance > 0,
        "User should have received pool tokens"
    );

    let expected_min = TEST_STAKE_AMOUNT - 100_000_000; // Allow 0.1 SOL tolerance for fees
    assert!(
        user_pool_token_balance >= expected_min,
        "User received {} pool tokens, expected at least {}",
        user_pool_token_balance,
        expected_min
    );

    // Manager fee account
    let manager_fee_balance = get_token_balance(
        &mut context.banks_client,
        &stake_pool_accounts.pool_fee_account.pubkey(),
    )
    .await;

    let total_pool_tokens = user_pool_token_balance + manager_fee_balance;
    assert!(
        total_pool_tokens >= TEST_STAKE_AMOUNT - 50_000_000,
        "Total pool tokens ({}) should approximately equal deposit amount ({})",
        total_pool_tokens,
        TEST_STAKE_AMOUNT
    );

    // WSOL transient account was closed
    let transient_account_result = context
        .banks_client
        .get_account(transient_wsol_pda)
        .await
        .unwrap();
    assert!(
        transient_account_result.is_none(),
        "Transient WSOL account should be closed after deposit"
    );

    // Stake pool state updated correctly
    let stake_pool = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.stake_pool.pubkey(),
    )
    .await;
    let stake_pool_state =
        try_from_slice_unchecked::<spl_stake_pool::state::StakePool>(&stake_pool.data).unwrap();

    assert!(
        stake_pool_state.total_lamports >= TEST_STAKE_AMOUNT,
        "Stake pool total_lamports should have increased by at least deposit amount"
    );

    // Pool token supply matches total minted
    let pool_mint_account = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.pool_mint.pubkey(),
    )
    .await;
    let pool_mint = spl_token::state::Mint::unpack(&pool_mint_account.data).unwrap();

    assert_eq!(
        pool_mint.supply, total_pool_tokens,
        "Pool token supply should equal total minted tokens (user + manager fee)"
    );

    // Reserve stake account received the deposit
    let reserve_stake = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.reserve_stake.pubkey(),
    )
    .await;

    let expected_reserve_min = MINIMUM_RESERVE_LAMPORTS + TEST_STAKE_AMOUNT - 50_000_000;
    assert!(
        reserve_stake.lamports >= expected_reserve_min,
        "Reserve stake should have received deposit: {} lamports, expected at least {}",
        reserve_stake.lamports,
        expected_reserve_min
    );

    // Session account still exists and is valid
    let session_account = get_account(&mut context.banks_client, &session_signer.pubkey()).await;
    assert_eq!(
        session_account.owner, SESSION_MANAGER_ID,
        "Session account should still be owned by SESSION_MANAGER_ID"
    );
    assert!(
        !session_account.data.is_empty(),
        "Session account should still have data"
    );

    // WSOL token account state
    let wsol_account_result = context
        .banks_client
        .get_account(wsol_token_account)
        .await
        .unwrap();

    if let Some(wsol_account) = wsol_account_result {
        let wsol_token_state = spl_token::state::Account::unpack(&wsol_account.data).unwrap();
        assert_eq!(
            wsol_token_state.amount, 0,
            "WSOL token account should have 0 token balance after deposit"
        );
    }
}
