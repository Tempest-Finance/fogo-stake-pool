//! Squads v3 (squads-mpl) integration for multisig transaction proposals.
//!
//! This module provides functionality to wrap stake pool instructions into
//! Squads multisig proposals, allowing multisig members to approve and execute
//! stake pool operations through their Squads vault.

#![allow(dead_code)]

use {
    solana_client::rpc_client::RpcClient,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::signature::Signer,
    solana_sdk_ids::system_program,
};

/// Squads MPL program ID (mainnet)
pub const SQUADS_MPL_PROGRAM_ID: Pubkey =
    solana_program::pubkey!("SMPLecH534NA9acpos4G6x7uf3LWbCAwZQE9e8ZekMu");

/// Seeds for PDA derivation
const SEED_PREFIX: &[u8] = b"squad";
const SEED_MULTISIG: &[u8] = b"multisig";
const SEED_TRANSACTION: &[u8] = b"transaction";
const SEED_INSTRUCTION: &[u8] = b"instruction";
const SEED_AUTHORITY: &[u8] = b"authority";

/// Squads multisig account state (partial, only fields we need)
#[derive(Debug)]
pub struct MultisigAccount {
    /// Number of signers required to execute a transaction
    pub threshold: u16,
    /// Index to seed the authority
    pub authority_index: u32,
    /// Index for the next transaction
    pub transaction_index: u32,
    /// Bump seed for the multisig PDA
    pub bump: u8,
    /// Key used to seed the multisig PDA
    pub create_key: Pubkey,
    /// Members of the multisig
    pub keys: Vec<Pubkey>,
}

impl MultisigAccount {
    /// Deserialize a multisig account from account data
    pub fn deserialize(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        // Skip 8-byte anchor discriminator
        if data.len() < 8 {
            return Err("Account data too short".into());
        }
        let data = &data[8..];

        // Layout (after discriminator) - verified against squads-mpl source:
        // threshold: u16 (2 bytes) - offset 0
        // authority_index: u16 (2 bytes) - offset 2
        // transaction_index: u32 (4 bytes) - offset 4
        // ms_change_index: u32 (4 bytes) - offset 8
        // bump: u8 (1 byte) - offset 12
        // create_key: Pubkey (32 bytes) - offset 13
        // allow_external_execute: bool (1 byte) - offset 45
        // keys: Vec<Pubkey> (4 bytes length + 32 * n bytes) - offset 46

        if data.len() < 50 {
            return Err("Account data too short for basic fields".into());
        }

        let threshold = u16::from_le_bytes([data[0], data[1]]);
        let authority_index = u16::from_le_bytes([data[2], data[3]]) as u32;
        let transaction_index = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        // ms_change_index at 8..12 (skip)
        let bump = data[12];
        let create_key = Pubkey::try_from(&data[13..45]).map_err(|_| "Invalid create_key")?;
        // allow_external_execute at 45 (skip)

        // Parse keys vector (starts at offset 46)
        let keys_len = u32::from_le_bytes([data[46], data[47], data[48], data[49]]) as usize;
        let keys_start = 50;
        let keys_end = keys_start + keys_len * 32;

        if data.len() < keys_end {
            return Err("Account data too short for keys".into());
        }

        let mut keys = Vec::with_capacity(keys_len);
        for i in 0..keys_len {
            let start = keys_start + i * 32;
            let end = start + 32;
            let key = Pubkey::try_from(&data[start..end]).map_err(|_| "Invalid key")?;
            keys.push(key);
        }

        Ok(Self {
            threshold,
            authority_index,
            transaction_index,
            bump,
            create_key,
            keys,
        })
    }

    /// Check if a pubkey is a member of this multisig
    pub fn is_member(&self, key: &Pubkey) -> bool {
        self.keys.contains(key)
    }
}

/// Account meta for Squads instructions (serializable)
#[derive(Clone, Debug)]
pub struct MsAccountMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl MsAccountMeta {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(34);
        buf.extend_from_slice(self.pubkey.as_ref());
        buf.push(if self.is_signer { 1 } else { 0 });
        buf.push(if self.is_writable { 1 } else { 0 });
        buf
    }
}

impl From<&AccountMeta> for MsAccountMeta {
    fn from(meta: &AccountMeta) -> Self {
        Self {
            pubkey: meta.pubkey,
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        }
    }
}

/// Incoming instruction data for add_instruction
#[derive(Clone, Debug)]
pub struct IncomingInstruction {
    pub program_id: Pubkey,
    pub keys: Vec<MsAccountMeta>,
    pub data: Vec<u8>,
}

impl IncomingInstruction {
    /// Create from a native Solana instruction
    pub fn from_instruction(ix: &Instruction) -> Self {
        Self {
            program_id: ix.program_id,
            keys: ix.accounts.iter().map(MsAccountMeta::from).collect(),
            data: ix.data.clone(),
        }
    }

    /// Serialize for the add_instruction instruction data
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // program_id
        buf.extend_from_slice(self.program_id.as_ref());

        // keys (Vec<MsAccountMeta>)
        buf.extend_from_slice(&(self.keys.len() as u32).to_le_bytes());
        for key in &self.keys {
            buf.extend_from_slice(&key.serialize());
        }

        // data (Vec<u8>)
        buf.extend_from_slice(&(self.data.len() as u32).to_le_bytes());
        buf.extend_from_slice(&self.data);

        buf
    }

    /// Calculate the max size needed for the instruction account
    pub fn get_max_size(&self) -> usize {
        // program_id (32) + keys vec (4 + 34 * n) + data vec (4 + len) + index (4) + bump (1) + executed (1)
        32 + 4 + (34 * self.keys.len()) + 4 + self.data.len() + 4 + 1 + 1
    }
}

/// Derive the multisig PDA
pub fn find_multisig_pda(create_key: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[SEED_PREFIX, create_key.as_ref(), SEED_MULTISIG],
        &SQUADS_MPL_PROGRAM_ID,
    )
}

/// Derive a transaction PDA
pub fn find_transaction_pda(multisig: &Pubkey, transaction_index: u32) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            SEED_PREFIX,
            multisig.as_ref(),
            &transaction_index.to_le_bytes(),
            SEED_TRANSACTION,
        ],
        &SQUADS_MPL_PROGRAM_ID,
    )
}

/// Derive an instruction PDA
pub fn find_instruction_pda(transaction: &Pubkey, instruction_index: u8) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            SEED_PREFIX,
            transaction.as_ref(),
            &[instruction_index],
            SEED_INSTRUCTION,
        ],
        &SQUADS_MPL_PROGRAM_ID,
    )
}

/// Derive the vault (authority) PDA for a multisig
/// Default vault uses authority_index = 1
pub fn find_vault_pda(multisig: &Pubkey, authority_index: u32) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            SEED_PREFIX,
            multisig.as_ref(),
            &authority_index.to_le_bytes(),
            SEED_AUTHORITY,
        ],
        &SQUADS_MPL_PROGRAM_ID,
    )
}

/// Get the vault pubkey for a multisig (default vault uses authority_index = 1)
pub fn get_vault_pubkey(
    rpc_client: &RpcClient,
    multisig_address: &Pubkey,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    // Verify the multisig account exists
    let _multisig = get_multisig_account(rpc_client, multisig_address)?;
    // Default vault is always authority_index = 1
    let (vault_pda, _) = find_vault_pda(multisig_address, 1);
    Ok(vault_pda)
}

/// Anchor instruction discriminators (first 8 bytes of sha256("global:<method_name>"))
mod discriminator {
    pub const CREATE_TRANSACTION: [u8; 8] = [227, 193, 53, 239, 55, 126, 112, 105];
    pub const ADD_INSTRUCTION: [u8; 8] = [11, 70, 136, 166, 202, 55, 246, 74];
    pub const ACTIVATE_TRANSACTION: [u8; 8] = [56, 17, 0, 163, 135, 11, 135, 32];
    pub const APPROVE_TRANSACTION: [u8; 8] = [224, 39, 88, 181, 36, 59, 155, 122];
}

/// Build a create_transaction instruction
pub fn create_transaction_instruction(
    multisig: &Pubkey,
    transaction_pda: &Pubkey,
    creator: &Pubkey,
    authority_index: u32,
) -> Instruction {
    let mut data = Vec::with_capacity(12);
    data.extend_from_slice(&discriminator::CREATE_TRANSACTION);
    data.extend_from_slice(&authority_index.to_le_bytes());

    Instruction {
        program_id: SQUADS_MPL_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*multisig, false),          // mut, not signer
            AccountMeta::new(*transaction_pda, false),   // mut (init), not signer
            AccountMeta::new(*creator, true),            // mut, signer
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data,
    }
}

/// Build an add_instruction instruction
pub fn add_instruction_instruction(
    multisig: &Pubkey,
    transaction_pda: &Pubkey,
    instruction_pda: &Pubkey,
    creator: &Pubkey,
    incoming_instruction: &IncomingInstruction,
) -> Instruction {
    let mut data = Vec::new();
    data.extend_from_slice(&discriminator::ADD_INSTRUCTION);
    data.extend_from_slice(&incoming_instruction.serialize());

    Instruction {
        program_id: SQUADS_MPL_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(*multisig, false), // not mut, not signer
            AccountMeta::new(*transaction_pda, false),   // mut, not signer
            AccountMeta::new(*instruction_pda, false),   // mut (init), not signer
            AccountMeta::new(*creator, true),            // mut, signer
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data,
    }
}

/// Build an activate_transaction instruction
pub fn activate_transaction_instruction(
    multisig: &Pubkey,
    transaction_pda: &Pubkey,
    creator: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: SQUADS_MPL_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(*multisig, false), // not mut, not signer
            AccountMeta::new(*transaction_pda, false),   // mut, not signer
            AccountMeta::new(*creator, true),            // mut, signer
        ],
        data: discriminator::ACTIVATE_TRANSACTION.to_vec(),
    }
}

/// Build an approve_transaction instruction
pub fn approve_transaction_instruction(
    multisig: &Pubkey,
    transaction_pda: &Pubkey,
    member: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: SQUADS_MPL_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(*multisig, false), // not mut, not signer
            AccountMeta::new(*transaction_pda, false),   // mut, not signer
            AccountMeta::new(*member, true),             // mut, signer
        ],
        data: discriminator::APPROVE_TRANSACTION.to_vec(),
    }
}

/// Fetch and deserialize a multisig account
pub fn get_multisig_account(
    rpc_client: &RpcClient,
    multisig_address: &Pubkey,
) -> Result<MultisigAccount, Box<dyn std::error::Error>> {
    let account_data = rpc_client.get_account_data(multisig_address)?;
    MultisigAccount::deserialize(&account_data)
}

/// Configuration for proposing to a Squads multisig
pub struct SquadsProposalConfig<'a> {
    pub rpc_client: &'a RpcClient,
    pub multisig_address: Pubkey,
    pub proposer: &'a dyn Signer,
    pub fee_payer: &'a dyn Signer,
}

/// Result of creating a Squads proposal
#[derive(Debug)]
pub struct SquadsProposalResult {
    pub transaction_pda: Pubkey,
    pub transaction_index: u32,
    pub instructions: Vec<Instruction>,
    /// External signers that must sign the execute_transaction call
    /// These are accounts that need is_signer=true but are not the vault PDA
    pub external_signers: Vec<Pubkey>,
}

/// Build instructions to propose stake pool operation(s) to a Squads multisig
///
/// This creates a new Squads transaction, adds all the provided instructions to it,
/// activates it, and optionally approves it (if the proposer is a member).
///
/// Returns the instructions to submit and the transaction PDA for tracking.
pub fn build_proposal_instructions(
    config: &SquadsProposalConfig,
    stake_pool_instructions: &[Instruction],
    auto_approve: bool,
) -> Result<SquadsProposalResult, Box<dyn std::error::Error>> {
    build_proposal_instructions_with_external_signers(config, stake_pool_instructions, auto_approve, &[])
}

/// Build instructions to propose stake pool operation(s) to a Squads multisig
/// with support for external signers.
///
/// `external_signers` are pubkeys that must sign the execute_transaction call
/// (in addition to the vault PDA which signs via invoke_signed).
pub fn build_proposal_instructions_with_external_signers(
    config: &SquadsProposalConfig,
    stake_pool_instructions: &[Instruction],
    auto_approve: bool,
    external_signers: &[Pubkey],
) -> Result<SquadsProposalResult, Box<dyn std::error::Error>> {
    // Fetch the multisig account to get current state
    let multisig = get_multisig_account(config.rpc_client, &config.multisig_address)?;

    // Calculate the new transaction index
    let new_transaction_index = multisig.transaction_index.checked_add(1).ok_or("Transaction index overflow")?;

    // Derive PDAs
    let (transaction_pda, _) = find_transaction_pda(&config.multisig_address, new_transaction_index);

    let proposer_pubkey = config.proposer.pubkey();

    // Verify proposer is a member
    if !multisig.is_member(&proposer_pubkey) {
        return Err(format!(
            "Proposer {} is not a member of the multisig {}",
            proposer_pubkey, config.multisig_address
        )
        .into());
    }

    let mut instructions = Vec::new();

    // 1. Create the transaction
    instructions.push(create_transaction_instruction(
        &config.multisig_address,
        &transaction_pda,
        &proposer_pubkey,
        multisig.authority_index,
    ));

    // 2. Add each stake pool instruction
    for (i, ix) in stake_pool_instructions.iter().enumerate() {
        let instruction_index = (i + 1) as u8;
        let (instruction_pda, _) = find_instruction_pda(&transaction_pda, instruction_index);
        let incoming = IncomingInstruction::from_instruction(ix);

        instructions.push(add_instruction_instruction(
            &config.multisig_address,
            &transaction_pda,
            &instruction_pda,
            &proposer_pubkey,
            &incoming,
        ));
    }

    // 3. Activate the transaction
    instructions.push(activate_transaction_instruction(
        &config.multisig_address,
        &transaction_pda,
        &proposer_pubkey,
    ));

    // 4. Optionally auto-approve
    if auto_approve {
        instructions.push(approve_transaction_instruction(
            &config.multisig_address,
            &transaction_pda,
            &proposer_pubkey,
        ));
    }

    Ok(SquadsProposalResult {
        transaction_pda,
        transaction_index: new_transaction_index,
        instructions,
        external_signers: external_signers.to_vec(),
    })
}
