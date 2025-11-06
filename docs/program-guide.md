# Ignition Stake Pool Program Guide

This guide provides comprehensive documentation for the on-chain Ignition Stake Pool program, including all instructions, account structures, and program-derived addresses.

## Program Overview

**Program ID:** `SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr`

The Ignition Stake Pool program is a FOGO blockchain program that enables the creation and management of stake pools - collections of delegated stake accounts that are managed collectively and represented by fungible pool tokens.

## Core Concepts

### Stake Pool Mechanics

A stake pool consists of:
- **Pool Account**: The main account containing pool configuration and state
- **Validator List**: A list of validators and their associated stake accounts
- **Reserve Account**: Holds deactivated stake for immediate withdrawals
- **Pool Token Mint**: Issues tokens representing proportional ownership of the pool
- **Manager Fee Account**: Receives management fees from the pool

### Token Economics

```
Pool Token Value = Total Pool Lamports / Pool Token Supply
```

When users deposit stake:
1. Their stake is added to the pool's total lamports
2. Pool tokens are minted proportionally to their contribution
3. Fees are deducted and distributed to managers and referrers

### Security Model

- **Authority Separation**: Different keys control different operations
- **PDA Security**: Critical authorities use program-derived addresses
- **Fee Protection**: Gradual fee changes prevent sudden increases
- **Compute Limits**: Batch operations respect FOGO's compute budget

## Program Derived Addresses (PDAs)

The program uses several PDA patterns for secure authority management:

### Authority PDAs

```rust
// Deposit authority (for permissionless deposits)
[stake_pool_address, b"deposit"] → deposit_authority

// Withdraw authority (controls all pool stake accounts)
[stake_pool_address, b"withdraw"] → (withdraw_authority, bump_seed)
```

### Stake Account PDAs

```rust
// Transient stake accounts (temporary during rebalancing)
[stake_pool_address, b"transient", validator_vote_account, seed] → transient_stake

// Ephemeral stake accounts (very temporary during complex operations)
[stake_pool_address, b"ephemeral", seed] → ephemeral_stake
```

### Usage Example

```rust
use solana_program::pubkey::Pubkey;

// Find withdraw authority
let (withdraw_authority, bump) = Pubkey::find_program_address(
    &[stake_pool_address.as_ref(), b"withdraw"],
    &program_id,
);

// Find deposit authority
let (deposit_authority, _) = Pubkey::find_program_address(
    &[stake_pool_address.as_ref(), b"deposit"],
    &program_id,
);
```

## Account Structures

The program uses several key account structures to manage pool state. For detailed field-level specifications and complete struct definitions, refer to the source code in `program/src/state.rs`.

### Key Accounts

- **StakePool Account**: Main pool account containing authorities, fees, financial state, and configuration
- **ValidatorList Account**: Stores validator information using BigVec for efficient large-scale management
- **ValidatorStakeInfo**: Tracks individual validator stake amounts, status, and performance
- **Fee Structures**: Manages various fee types including epoch fees, deposit fees, withdrawal fees, and referral fees

The program uses efficient data structures optimized for on-chain storage and processing.

## Instructions

The program supports comprehensive stake pool operations through various instructions:

### Pool Management Instructions

#### Initialize

Creates a new stake pool with specified configuration.

```rust
Initialize {
    fee: Fee,              // Management fee on rewards
    withdrawal_fee: Fee,   // Fee on withdrawals
    deposit_fee: Fee,      // Fee on deposits
    referral_fee: u8,      // Percentage of deposit fees for referrers
    max_validators: u32,   // Expected maximum validators
}
```

**Accounts:**
1. `[w]` New StakePool account
2. `[s]` Manager
3. `[]` Staker
4. `[]` Stake pool withdraw authority (PDA)
5. `[w]` Validator list storage account
6. `[]` Reserve stake account
7. `[]` Pool token mint
8. `[]` Manager fee account
9. `[]` Token program
10. `[]` (Optional) Deposit authority

#### SetManager

Updates the pool manager (manager only).

```rust
SetManager
```

**Accounts:**
1. `[w]` Stake pool
2. `[s]` Current manager
3. `[s]` New manager
4. `[]` New manager fee account

#### SetFee

Updates pool fees (manager only).

```rust
SetFee {
    fee: FeeType,  // Which fee to update and new value
}

pub enum FeeType {
    SolReferral(u8),
    StakeReferral(u8),
    Epoch(Fee),
    StakeDeposit(Fee),
    SolDeposit(Fee),
    StakeWithdrawal(Fee),
    SolWithdrawal(Fee),
}
```

**Accounts:**
1. `[w]` Stake pool
2. `[s]` Manager

#### SetStaker

Updates the staker authority (manager or current staker only).

```rust
SetStaker
```

**Accounts:**
1. `[w]` Stake pool
2. `[s]` Manager or current staker
3. `[]` New staker

#### SetFundingAuthority

Updates deposit/withdrawal authorities (manager only).

```rust
SetFundingAuthority(FundingType)

pub enum FundingType {
    StakeDeposit,   // Sets stake deposit authority
    SolDeposit,     // Sets tokens deposit authority
    SolWithdraw,    // Sets tokens withdrawal authority
}
```

**Accounts:**
1. `[w]` Stake pool
2. `[s]` Manager
3. `[]` New authority (or None)

### Validator Management Instructions

#### AddValidatorToPool

Adds a validator to the pool (staker only).

```rust
AddValidatorToPool(u32)  // Optional seed for validator stake account
```

**Accounts:**
1. `[w]` Stake pool
2. `[s]` Staker
3. `[w]` Reserve stake account
4. `[]` Withdraw authority
5. `[w]` Validator list
6. `[w]` Validator stake account
7. `[]` Validator vote account
8. `[]` Rent sysvar
9. `[]` Clock sysvar
10. `[]` Stake history sysvar
11. `[]` Stake config sysvar
12. `[]` System program
13. `[]` Stake program

#### RemoveValidatorFromPool

Removes a validator from the pool (staker only).

```rust
RemoveValidatorFromPool
```

**Accounts:**
1. `[w]` Stake pool
2. `[s]` Staker
3. `[]` Withdraw authority
4. `[w]` Validator list
5. `[w]` Validator stake account
6. `[w]` Transient stake account
7. `[]` Clock sysvar
8. `[]` Stake program

#### SetPreferredValidator

Sets preferred validator for deposits/withdrawals (staker only).

```rust
SetPreferredValidator {
    validator_type: PreferredValidatorType,
    validator_vote_address: Option<Pubkey>,
}

pub enum PreferredValidatorType {
    Deposit,   // For deposits
    Withdraw,  // For withdrawals
}
```

**Accounts:**
1. `[w]` Stake pool
2. `[s]` Staker
3. `[]` Validator list

### Stake Rebalancing Instructions

#### IncreaseValidatorStake

Increases stake on a validator from the reserve (staker only).

```rust
IncreaseValidatorStake {
    lamports: u64,              // Amount to increase
    transient_stake_seed: u64,  // Seed for transient account
}
```

**Accounts:**
1. `[]` Stake pool
2. `[s]` Staker
3. `[]` Withdraw authority
4. `[w]` Validator list
5. `[w]` Reserve stake
6. `[w]` Transient stake account
7. `[]` Validator stake account
8. `[]` Validator vote account
9. `[]` Clock sysvar
10. `[]` Rent sysvar
11. `[]` Stake history sysvar
12. `[]` Stake config sysvar
13. `[]` System program
14. `[]` Stake program

#### DecreaseValidatorStakeWithReserve

Decreases stake on a validator to the reserve (staker only).

```rust
DecreaseValidatorStakeWithReserve {
    lamports: u64,              // Amount to decrease
    transient_stake_seed: u64,  // Seed for transient account
}
```

#### IncreaseAdditionalValidatorStake

Increases stake on a validator again in the same epoch (staker only).

```rust
IncreaseAdditionalValidatorStake {
    lamports: u64,              // Amount to increase
    transient_stake_seed: u64,  // Seed for transient account
    ephemeral_stake_seed: u64,  // Seed for ephemeral account
}
```

#### DecreaseAdditionalValidatorStake

Decreases stake on a validator again in the same epoch (staker only).

```rust
DecreaseAdditionalValidatorStake {
    lamports: u64,              // Amount to decrease
    transient_stake_seed: u64,  // Seed for transient account
    ephemeral_stake_seed: u64,  // Seed for ephemeral account
}
```

### Update Instructions

#### UpdateValidatorListBalance

Updates validator balances and processes transient stakes.

```rust
UpdateValidatorListBalance {
    start_index: u32,  // Starting validator index
    no_merge: bool,    // Skip merging for testing
}
```

**Accounts:**
1. `[]` Stake pool
2. `[]` Withdraw authority
3. `[w]` Validator list
4. `[w]` Reserve stake
5. `[]` Clock sysvar
6. `[]` Stake history sysvar
7. `[]` Stake program
8. `..` Validator and transient stake account pairs (up to MAX_VALIDATORS_TO_UPDATE * 2)

#### UpdateStakePoolBalance

Updates the stake pool's total balance from validator list.

```rust
UpdateStakePoolBalance
```

**Accounts:**
1. `[w]` Stake pool
2. `[]` Withdraw authority
3. `[w]` Validator list
4. `[]` Reserve stake
5. `[w]` Manager fee account
6. `[w]` Pool token mint
7. `[]` Token program

#### CleanupRemovedValidatorEntries

Removes validators marked as ReadyForRemoval.

```rust
CleanupRemovedValidatorEntries
```

**Accounts:**
1. `[]` Stake pool
2. `[w]` Validator list

### User Operations

#### DepositStake

Deposits a stake account into the pool in exchange for pool tokens.

```rust
DepositStake
```

**Accounts:**
1. `[w]` Stake pool
2. `[w]` Validator list
3. `[s]/[]` Deposit authority (if required)
4. `[]` Withdraw authority
5. `[w]` Stake account to deposit
6. `[w]` Validator stake account to merge with
7. `[w]` Reserve stake account
8. `[w]` User pool token account
9. `[w]` Manager fee account
10. `[w]` Referrer pool token account
11. `[w]` Pool token mint
12. `[]` Clock sysvar
13. `[]` Stake history sysvar
14. `[]` Token program
15. `[]` Stake program

#### WithdrawStake

Withdraws stake from the pool by burning pool tokens.

```rust
WithdrawStake(u64)  // Amount of pool tokens to burn
```

**Accounts:**
1. `[w]` Stake pool
2. `[w]` Validator list
3. `[]` Withdraw authority
4. `[w]` Validator or reserve stake account to split from
5. `[w]` New stake account to receive withdrawal
6. `[]` User account (new withdraw authority)
7. `[s]` User transfer authority
8. `[w]` User pool token account
9. `[w]` Manager fee account
10. `[w]` Pool token mint
11. `[]` Clock sysvar
12. `[]` Token program
13. `[]` Stake program

#### DepositSol

Deposits tokens directly into the pool's reserve.

```rust
DepositSol(u64)  // Amount of tokens to deposit
```

**Accounts:**
1. `[w]` Stake pool
2. `[]` Withdraw authority
3. `[w]` Reserve stake account
4. `[s]` Funding account
5. `[w]` User pool token account
6. `[w]` Manager fee account
7. `[w]` Referrer pool token account
8. `[w]` Pool token mint
9. `[]` System program
10. `[]` Token program
11. `[s]` (Optional) tokens deposit authority

#### WithdrawSol

Withdraws tokens directly from the pool's reserve.

```rust
WithdrawSol(u64)  // Amount of tokens to withdraw
```

**Accounts:**
1. `[w]` Stake pool
2. `[]` Withdraw authority
3. `[s]` User transfer authority
4. `[w]` User pool token account
5. `[w]` Reserve stake account
6. `[w]` Receiving system account
7. `[w]` Manager fee account
8. `[w]` Pool token mint
9. `[]` Clock sysvar
10. `[]` Stake history sysvar
11. `[]` Stake program
12. `[]` Token program
13. `[s]` (Optional) tokens withdraw authority

#### DepositWsolWithSession

Deposits wrapped SOL (WSOL) into the pool using a session token for authentication (FOGO blockchain specific).

```rust
DepositWsolWithSession {
    amount: u64,  // Amount of WSOL to deposit
}
```

**Accounts:**
1. `[w]` Stake pool
2. `[]` Withdraw authority
3. `[w]` Reserve stake account
4. `[s]` Funding account (session authority)
5. `[w]` User pool token account
6. `[w]` Manager fee account
7. `[w]` Referrer pool token account
8. `[w]` Pool token mint
9. `[]` System program
10. `[]` Token program
11. `[]` Session token account
12. `[s]` Session authority

**Description:**
This instruction enables gasless transactions on FOGO blockchain by using session tokens. The session authority signs the transaction on behalf of the user, allowing for improved UX without requiring users to sign each transaction individually.

#### WithdrawWsolWithSession

Withdraws wrapped SOL (WSOL) from the pool using a session token for authentication (FOGO blockchain specific).

```rust
WithdrawWsolWithSession {
    amount: u64,  // Amount of pool tokens to burn
}
```

**Accounts:**
1. `[w]` Stake pool
2. `[]` Withdraw authority
3. `[s]` Session authority
4. `[w]` User pool token account
5. `[w]` Reserve stake account
6. `[w]` Receiving WSOL account
7. `[w]` Manager fee account
8. `[w]` Pool token mint
9. `[]` Clock sysvar
10. `[]` Stake history sysvar
11. `[]` Stake program
12. `[]` Token program
13. `[]` Session token account

**Description:**
This instruction enables gasless withdrawals on FOGO blockchain by using session tokens. The session authority validates and processes the withdrawal, providing a seamless user experience for withdrawing stake pool tokens back to WSOL.

### Token Metadata Instructions

#### CreateTokenMetadata

Creates metadata for the pool token.

```rust
CreateTokenMetadata {
    name: String,    // Token name
    symbol: String,  // Token symbol (e.g., "stktokens")
    uri: String,     // Metadata URI
}
```

#### UpdateTokenMetadata

Updates existing token metadata.

```rust
UpdateTokenMetadata {
    name: String,    // Updated token name
    symbol: String,  // Updated token symbol
    uri: String,     // Updated metadata URI
}
```

## Constants and Limits

### Core Constants

```rust
pub const MINIMUM_ACTIVE_STAKE: u64 = 1_000_000;  // Minimum stake per validator
pub const MINIMUM_RESERVE_LAMPORTS: u64 = 0;       // Minimum reserve balance
pub const MAX_VALIDATORS_TO_UPDATE: usize = 4;     // Per instruction limit
```

### Fee Protection

```rust
pub const MAX_WITHDRAWAL_FEE_INCREASE: Fee = Fee {
    numerator: 3,
    denominator: 2,
};  // Maximum 3/2 ratio increase per epoch

pub const WITHDRAWAL_BASELINE_FEE: Fee = Fee {
    numerator: 3,
    denominator: 1000,
};  // 0.3% baseline for fee increase calculations
```

### PDA Seeds

```rust
pub const AUTHORITY_DEPOSIT: &[u8] = b"deposit";
pub const AUTHORITY_WITHDRAW: &[u8] = b"withdraw";
pub const TRANSIENT_STAKE_SEED_PREFIX: &[u8] = b"transient";
pub const EPHEMERAL_STAKE_SEED_PREFIX: &[u8] = b"ephemeral";
```

## Error Codes

The program defines comprehensive error codes for different failure conditions:

```rust
pub enum StakePoolError {
    // Setup errors
    AlreadyInUse,
    InvalidProgramAddress,
    InvalidState,
    IncorrectProgramId,
    IncorrectOwner,

    // Authority errors
    WrongAccountMint,
    NonzeroPoolTokenSupply,
    StakeListAndPoolLamportsMismatch,
    UnknownValidatorStakeAccount,

    // Operation errors
    StakeListOutOfDate,
    StakeNotActive,
    ValidatorAlreadyAdded,
    ValidatorNotFound,
    InvalidValidatorStakeList,

    // Fee errors
    CalculationFailure,
    FeeTooHigh,
    WithdrawTooLarge,
    WithdrawTooSmall,
    InsufficientPoolTokens,
    DepositTooSmall,

    // And many more...
}
```

## Best Practices

### For Pool Creators

1. **Set Reasonable Fees**: Start with low fees and increase gradually if needed
2. **Choose Reliable Validators**: Research validator performance and commission rates
3. **Regular Updates**: Keep the pool updated each epoch for accurate balances
4. **Monitor Health**: Track validator performance and remove underperforming ones

### For Integrators

1. **Check Pool Health**: Verify the pool is regularly updated and has reasonable fees
2. **Handle Errors Gracefully**: Account for temporary failures and retry logic
3. **Respect Compute Limits**: Batch operations appropriately
4. **Validate Inputs**: Always validate amounts and accounts before submitting

### For Stakers

1. **Understand Fees**: Review all applicable fees before depositing
2. **Check Pool Performance**: Compare APY with direct staking
3. **Monitor Validator Set**: Ensure the pool uses reputable validators
4. **Consider Liquidity**: Pool tokens provide liquidity not available with direct staking

## Security Considerations

### Authority Management
- Pool authorities should use multisig or hardware wallets
- Separate hot/cold wallet strategies for different operations
- Regular authority rotation following security best practices

### Fee Protection
- Withdrawal fees can only increase by 3/2 ratio per epoch
- Monitor fee changes and validator additions/removals
- Understand referral fee implications

### Validator Risk
- Pool performance depends on validator selection
- Diversify across multiple high-performing validators
- Monitor validator health and commission rates

This comprehensive guide covers all aspects of the Ignition Stake Pool program. 
For implementation examples and integration patterns, see the [API Reference](./api-reference.md#typescript-sdk-api) and [API Reference](./api-reference.md).
