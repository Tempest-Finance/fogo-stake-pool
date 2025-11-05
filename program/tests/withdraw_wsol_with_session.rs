#![allow(clippy::arithmetic_side_effects)]
#![cfg(feature = "test-sbf")]

mod helpers;

use crate::helpers::wsol::setup_with_session_account;
use spl_stake_pool::instruction::withdraw_wsol_with_session;
use {
    fogo_sessions_sdk::{session::SESSION_MANAGER_ID, token::PROGRAM_SIGNER_SEED},
    helpers::*,
    solana_program::program_pack::Pack,
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
        None,
        pool_tokens_to_withdraw,
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
    assert!(wsol_balance > 0, "User should have received WSOL tokens");

    let remaining_pool_tokens =
        get_token_balance(&mut context.banks_client, &pool_token_account).await;
    assert!(
        remaining_pool_tokens < pool_tokens,
        "Pool tokens should have been burned"
    );

    let session_account = get_account(&mut context.banks_client, &session_signer.pubkey()).await;
    assert_eq!(
        session_account.owner, SESSION_MANAGER_ID,
        "Session account should still be owned by SESSION_MANAGER_ID"
    );

    let wsol_account_result = context
        .banks_client
        .get_account(wsol_token_account)
        .await
        .unwrap();

    if let Some(wsol_account) = wsol_account_result {
        let wsol_token_state = spl_token::state::Account::unpack(&wsol_account.data).unwrap();
        assert!(
            wsol_token_state.amount > 0,
            "WSOL token account should have balance after withdrawal"
        );
    }
}
