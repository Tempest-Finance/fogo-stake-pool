//! Fogo Stake Pool Client
//!
//! Generated Rust client for the Fogo Stake Pool program.
//!
//! This crate provides types and utilities for interacting with Fogo Stake Pool
//! accounts from Rust code.

#![deny(missing_docs)]
#![allow(clippy::arithmetic_side_effects)]

#[allow(missing_docs)]
mod generated;

pub use generated::{accounts::*, errors::*, programs::FOGO_STAKE_POOL_ID as ID, types::*};

/// Re-export the program ID module.
pub mod programs {
    pub use super::generated::programs::*;
}
