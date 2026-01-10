use crate::helpers::{
    get_token_balance, program_test, StakePoolAccounts, TEST_STAKE_AMOUNT,
};
use fogo_sessions_sdk::session::{MAJOR, SESSION_MANAGER_ID};
use fogo_sessions_sdk::token::PROGRAM_SIGNER_SEED;
use solana_program::clock::Clock;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::account::Account;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use spl_stake_pool::{id, MINIMUM_RESERVE_LAMPORTS};

pub const TRANSIENT_WSOL_SEED: &[u8] = b"transient_wsol";

const SESSION_DISCRIMINATOR: [u8; 8] = [243, 81, 72, 115, 214, 188, 72, 144];

/// Sets up a test environment with a stake pool and a session account.
/// This version does NOT pre-create the pool token ATA to test on-chain ATA creation.
pub async fn setup_with_session_account_no_ata(
    token_program_id: Pubkey,
) -> (
    ProgramTestContext,
    StakePoolAccounts,
    Keypair,
    Pubkey,
    Keypair,
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

    // Get the user's pool token ATA address (but DON'T create it)
    let pool_token_ata = spl_associated_token_account::get_associated_token_address_with_program_id(
        &user.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &token_program_id,
    );

    // Do an initial deposit to the pool so it has liquidity (using payer, not user)
    let payer_pool_ata = spl_associated_token_account::get_associated_token_address_with_program_id(
        &context.payer.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &token_program_id,
    );

    let create_payer_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &context.payer.pubkey(),
        &context.payer.pubkey(),
        &stake_pool_accounts.pool_mint.pubkey(),
        &token_program_id,
    );

    let create_ata_tx = Transaction::new_signed_with_payer(
        &[create_payer_ata_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(create_ata_tx)
        .await
        .unwrap();

    let deposit_amount = TEST_STAKE_AMOUNT;
    let error = stake_pool_accounts
        .deposit_sol(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            &payer_pool_ata,
            deposit_amount,
            None,
        )
        .await;
    assert!(error.is_none(), "{:?}", error);

    (
        context,
        stake_pool_accounts,
        user,
        pool_token_ata,
        session_keypair,
    )
}

/// Sets up a test environment with a stake pool and a session account.
pub async fn setup_with_session_account(
    token_program_id: Pubkey,
) -> (
    ProgramTestContext,
    StakePoolAccounts,
    Keypair,
    Pubkey,
    Keypair,
    u64,
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

    let deposit_amount = TEST_STAKE_AMOUNT;
    let error = stake_pool_accounts
        .deposit_sol(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            &pool_token_ata,
            deposit_amount,
            None,
        )
        .await;
    assert!(error.is_none(), "{:?}", error);

    let pool_tokens =
        get_token_balance(&mut context.banks_client, &pool_token_ata).await;

    (
        context,
        stake_pool_accounts,
        user,
        pool_token_ata,
        session_keypair,
        pool_tokens,
    )
}

/// Helper function to manually serialize a Session account using borsh 0.10 format
/// This works around the borsh version mismatch between fogo-sessions-sdk (0.10) and spl-stake-pool (1.5.7)
pub fn manually_serialize_session(
    sponsor: &Pubkey,
    user: &Pubkey,
    expiration: i64,
    authorized_program_id: &Pubkey,
    signer_pda: &Pubkey,
) -> Vec<u8> {
    let mut data = Vec::new();

    // Session discriminator (8 bytes)
    data.extend_from_slice(&SESSION_DISCRIMINATOR);

    // Sponsor pubkey (32 bytes)
    data.extend_from_slice(sponsor.as_ref());

    // Major version (1 byte): 0
    data.push(MAJOR);

    // SessionInfo enum variant: V4 = 4 (u8)
    // Note: Invalid=0, V1=1, V2=2, V3=3, V4=4
    data.push(4);

    // V4 enum variant: Active = 1 (u8)
    // Note: Revoked=0, Active=1
    data.push(1);

    // ActiveSessionInfoWithDomainHash
    // - domain_hash: [u8; 32]
    data.extend_from_slice(&[0u8; 32]);

    // ActiveSessionInfo
    // - user: Pubkey (32 bytes)
    data.extend_from_slice(user.as_ref());

    // - expiration: i64 (8 bytes, little endian)
    data.extend_from_slice(&expiration.to_le_bytes());

    // - authorized_programs: AuthorizedPrograms enum
    // Specific variant = 0 (u8), followed by Vec length (u32 LE) and items
    data.push(0); // Specific variant
    data.extend_from_slice(&1u32.to_le_bytes()); // Vec length = 1

    // AuthorizedProgram struct
    // - program_id: Pubkey (32 bytes)
    data.extend_from_slice(authorized_program_id.as_ref());
    // - signer_pda: Pubkey (32 bytes)
    data.extend_from_slice(signer_pda.as_ref());

    // - authorized_tokens: AuthorizedTokensWithMints enum
    // All variant = 1 (u8)
    data.push(1);

    // - extra: Extra (HashMap<String, String>)
    // Empty HashMap: length (u32 LE) = 0
    data.extend_from_slice(&0u32.to_le_bytes());

    data
}
