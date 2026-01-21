# Fogo Stake Pool Rust Client

Generated Rust client for the Fogo Stake Pool program.

This crate provides types and utilities for interacting with Fogo Stake Pool accounts from Rust code.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fogo-stake-pool-client = "0.1"
```

## Usage

```rust
use fogo_stake_pool_client::{ID, StakePool, ValidatorList};
use solana_program::pubkey::Pubkey;

// Program ID
assert_eq!(ID.to_string(), "SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr");
```

## Features

- `serde` - Enable serde serialization/deserialization for all types
