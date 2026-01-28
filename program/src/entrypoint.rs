//! Program entrypoint

#![cfg(all(target_os = "solana", not(feature = "no-entrypoint")))]

use {
    crate::{error::StakePoolError, processor::Processor},
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program_error::PrintProgramError,
        pubkey::Pubkey,
    },
    solana_security_txt::security_txt,
};

solana_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<StakePoolError>();
        Err(error)
    } else {
        Ok(())
    }
}

security_txt! {
    // Required fields
    name: "Fogo Stake Pool",
    project_url: "https://github.com/Tempest-Finance/fogo-stake-pool",
    contacts: "link:https://github.com/Tempest-Finance/fogo-stake-pool/security/advisories/new",
    policy: "https://github.com/Tempest-Finance/fogo-stake-pool/blob/main/SECURITY.md",

    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/Tempest-Finance/fogo-stake-pool/tree/main/program",
    source_revision: "fabdb9796802bf67263ef0ac369e884f8c4c88fd",
    source_release: "stake-pool-v2.0.3",
    auditors: "https://github.com/Tempest-Finance/fogo-stake-pool/tree/main/audits,https://github.com/solana-labs/security-audits#stake-pool"
}
