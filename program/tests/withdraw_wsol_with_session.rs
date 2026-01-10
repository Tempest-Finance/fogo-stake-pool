#![allow(clippy::arithmetic_side_effects)]
#![cfg(feature = "test-sbf")]

mod helpers;

use crate::helpers::wsol::setup_with_session_account;
use spl_stake_pool::instruction::withdraw_wsol_with_session;
use {
    fogo_sessions_sdk::token::PROGRAM_SIGNER_SEED,
    helpers::*,
    solana_program::{borsh1::try_from_slice_unchecked, program_pack::Pack},
    solana_program_test::*,
    solana_sdk::{pubkey::Pubkey, signature::Signer, transaction::Transaction},
    spl_stake_pool::id,
    spl_token::native_mint,
    test_case::test_case,
};

#[test_case(spl_token::id(); "token")]
#[tokio::test]
async fn success(token_program_id: Pubkey) {
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

    let (program_signer, _program_signer_bump) =
        Pubkey::find_program_address(&[PROGRAM_SIGNER_SEED], &id());

    let initial_reserve_lamports = {
        let reserve_stake = get_account(
            &mut context.banks_client,
            &stake_pool_accounts.reserve_stake.pubkey(),
        )
        .await;
        reserve_stake.lamports
    };

    let initial_stake_pool_state = {
        let stake_pool = get_account(
            &mut context.banks_client,
            &stake_pool_accounts.stake_pool.pubkey(),
        )
        .await;
        try_from_slice_unchecked::<spl_stake_pool::state::StakePool>(&stake_pool.data).unwrap()
    };

    let initial_pool_mint_supply = {
        let pool_mint_account = get_account(
            &mut context.banks_client,
            &stake_pool_accounts.pool_mint.pubkey(),
        )
        .await;
        let pool_mint = spl_token::state::Mint::unpack(&pool_mint_account.data).unwrap();
        pool_mint.supply
    };

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

    let result = context.banks_client.process_transaction(transaction).await;

    if let Err(ref err) = result {
        panic!("Transaction failed: {:?}", err);
    }

    let wsol_balance = get_token_balance(&mut context.banks_client, &wsol_token_account).await;
    let expected_wsol_min = (pool_tokens_to_withdraw as f64 * 0.95) as u64;
    assert!(
        wsol_balance >= expected_wsol_min,
        "User should have received WSOL tokens: got {}, expected at least {}",
        wsol_balance,
        expected_wsol_min
    );

    let remaining_pool_tokens =
        get_token_balance(&mut context.banks_client, &pool_token_account).await;
    let expected_remaining = pool_tokens - pool_tokens_to_withdraw;
    assert!(
        remaining_pool_tokens <= expected_remaining,
        "Pool tokens should have been burned: remaining {}, expected ~{}",
        remaining_pool_tokens,
        expected_remaining
    );

    let manager_fee_balance = get_token_balance(
        &mut context.banks_client,
        &stake_pool_accounts.pool_fee_account.pubkey(),
    )
    .await;

    let final_pool_mint_supply = {
        let pool_mint_account = get_account(
            &mut context.banks_client,
            &stake_pool_accounts.pool_mint.pubkey(),
        )
        .await;
        let pool_mint = spl_token::state::Mint::unpack(&pool_mint_account.data).unwrap();
        pool_mint.supply
    };

    assert!(
        final_pool_mint_supply < initial_pool_mint_supply,
        "Pool mint supply should have decreased: initial {}, final {}",
        initial_pool_mint_supply,
        final_pool_mint_supply
    );

    let total_remaining_pool_tokens = remaining_pool_tokens + manager_fee_balance;
    assert_eq!(
        final_pool_mint_supply, total_remaining_pool_tokens,
        "Pool token supply should equal total remaining tokens (user + manager fee)"
    );

    let final_stake_pool_state = {
        let stake_pool = get_account(
            &mut context.banks_client,
            &stake_pool_accounts.stake_pool.pubkey(),
        )
        .await;
        try_from_slice_unchecked::<spl_stake_pool::state::StakePool>(&stake_pool.data).unwrap()
    };

    assert!(
        final_stake_pool_state.total_lamports < initial_stake_pool_state.total_lamports,
        "Stake pool total_lamports should have decreased: initial {}, final {}",
        initial_stake_pool_state.total_lamports,
        final_stake_pool_state.total_lamports
    );

    let final_reserve_lamports = {
        let reserve_stake = get_account(
            &mut context.banks_client,
            &stake_pool_accounts.reserve_stake.pubkey(),
        )
        .await;
        reserve_stake.lamports
    };

    assert!(
        final_reserve_lamports < initial_reserve_lamports,
        "Reserve stake lamports should have decreased: initial {}, final {}",
        initial_reserve_lamports,
        final_reserve_lamports
    );

    let expected_reserve_max = initial_reserve_lamports - wsol_balance + 100_000_000;
    assert!(
        final_reserve_lamports <= expected_reserve_max,
        "Reserve stake should have decreased by approximately WSOL amount: {} lamports, max expected {}",
        final_reserve_lamports,
        expected_reserve_max
    );

    let wsol_account_result = context
        .banks_client
        .get_account(wsol_token_account)
        .await
        .unwrap();

    if let Some(wsol_account) = wsol_account_result {
        let wsol_token_state = spl_token::state::Account::unpack(&wsol_account.data).unwrap();
        assert_eq!(
            wsol_token_state.amount, wsol_balance,
            "WSOL token account balance should match expected amount"
        );
        assert_eq!(
            wsol_token_state.owner,
            user.pubkey(),
            "WSOL token account should be owned by user"
        );
        assert_eq!(
            wsol_token_state.mint,
            native_mint::id(),
            "WSOL token account should be for native mint"
        );
    }
}
