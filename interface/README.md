# Fogo Stake Pool Interface

A stake pool program interface on the Fogo blockchain, usable for liquid staking operations.

This crate provides state types and utilities that integrators can use to deserialize and interact with stake pool accounts.

Full documentation is available at https://github.com/Tempest-Finance/fogo-stake-pool/docs

## Usage

```rust
use fogo_stake_pool_interface::{
    StakePool, ValidatorList, Fee, id,
    find_withdraw_authority_program_address,
};
use solana_program::borsh1::try_from_slice_unchecked;

// Deserialize a stake pool account
let stake_pool: StakePool = try_from_slice_unchecked(&account_data)?;

// Check the program ID
assert_eq!(stake_pool_account.owner, &id());

// Derive PDAs
let (withdraw_authority, _bump) = find_withdraw_authority_program_address(&id(), &stake_pool_pubkey);
```

## Modules

- **`state`** - Account state types (`StakePool`, `ValidatorList`, `Fee`, etc.)
- **`pda`** - PDA derivation functions
- **`error`** - Error types

## Types Included

- `StakePool` - Main stake pool state
- `ValidatorList` - List of validators in the pool
- `ValidatorListHeader` - Header portion of validator list
- `ValidatorStakeInfo` - Per-validator stake information
- `Fee` - Fee structure (numerator/denominator)
- `FeeType` - Enum of different fee types
- `FutureEpoch<T>` - Epoch-delayed value changes (generic)
- `FutureEpochFee` - Epoch-delayed fee changes (concrete type for IDL)
- `AccountType` - Account discriminator enum
- `StakeStatus` - Validator stake status enum

## PDA Functions

- `find_deposit_authority_program_address` - Derive deposit authority PDA
- `find_withdraw_authority_program_address` - Derive withdraw authority PDA
- `find_stake_program_address` - Derive validator stake account PDA
- `find_transient_stake_program_address` - Derive transient stake account PDA
- `find_ephemeral_stake_program_address` - Derive ephemeral stake account PDA
- `find_user_stake_program_address` - Derive user stake account PDA (for session withdrawals)
- `check_program_account` - Verify program ID matches

## Constants

- `MINIMUM_ACTIVE_STAKE` - Minimum lamports in a validator stake account (1,000,000)
- `MINIMUM_RESERVE_LAMPORTS` - Minimum lamports in reserve (0)
- `MAX_VALIDATORS_TO_UPDATE` - Max validators per update instruction (4)
- `MAX_TRANSIENT_STAKE_ACCOUNTS` - Max transient accounts per transaction (10)
- `MAX_VALIDATORS_IN_POOL` - Maximum validators supported (20,000)
- `MAX_WITHDRAWAL_FEE_INCREASE` - Max fee increase factor per epoch (3/2)
- `WITHDRAWAL_BASELINE_FEE` - Baseline fee for increase calculations (1/1000)

## Features

- `borsh` (default) - Enables Borsh serialization/deserialization
- `serde` - Enables Serde serialization
- `codama` - Enables IDL generation via Codama

## IDL Generation

To generate the Codama IDL:

```bash
cargo run --bin generate-idl --features codama
```

This outputs `idl.json` in the crate root.

## Program ID

- Mainnet/Testnet: `SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr`
