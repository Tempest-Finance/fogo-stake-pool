# Fogo Stake Pool Architecture

This document provides a comprehensive overview of the Fogo Stake Pool system architecture, design decisions, and core concepts.

## System Overview

The Fogo Stake Pool program implements a liquid staking protocol on FOGO blockchain, allowing users to pool their stake and receive liquid tokens in return. 
The system is designed for security, efficiency, and scalability while maintaining decentralization.

```
┌─────────────────────────────────────────────────────────────────┐
│                   Fogo Stake Pool System                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐    ┌──────────────┐    ┌─────────────────┐     │
│  │   Users     │    │ Stake Pool   │    │   Validators    │     │
│  │             │    │   Program    │    │                 │     │
│  │ - Deposit   │───▶│              │◀───│ - Vote Accounts │     │
│  │ - Withdraw  │    │ - Pool State │    │ - Stake Accounts│     │
│  │ - Transfer  │    │ - Validator  │    │ - Performance   │     │
│  │   Tokens    │    │   List       │    │   Tracking      │     │
│  └─────────────┘    │ - Fee Mgmt   │    └─────────────────┘     │
│                     │ - Liquidity  │                            │
│                     └──────────────┘                            │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                        Client Libraries                         │
│  ┌─────────────┐ ┌─────────────┐ ┌───────────┐                  │
│  │ Rust CLI    │ │ TypeScript  │ │  Python   │                  │
│  │             │ │    SDK      │ │  Client   │                  │
│  └─────────────┘ └─────────────┘ └───────────┘                  │
└─────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. On-Chain Program (`program/`)

The core FOGO blockchain program written in Rust that manages all stake pool operations.

#### Key Files:
- **`lib.rs`** (Entry Point): Program constants, PDA derivation, core utilities
- **`instruction.rs`**: Complete instruction set with serialization
- **`processor.rs`**: Main business logic for all operations
- **`state.rs`**: State management and data structures
- **`error.rs`**: Custom error types and handling
- **`big_vec.rs`**: Efficient storage for large validator lists
- **`inline_mpl_token_metadata.rs`**: Token metadata handling

#### Program Architecture:

```
┌─────────────────────────────────────────────────────────┐
│                   FOGO Blockchain Program               │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Instructions                 State Management          │
│  ┌─────────────────┐         ┌─────────────────┐        │
│  │ Pool Management │         │ StakePool       │        │
│  │ - Initialize    │         │ - Authorities   │        │
│  │ - Update        │         │ - Fees          │        │
│  │ - SetFee        │         │ - Mint Info     │        │
│  └─────────────────┘         │ - Validator List│        │
│                              └─────────────────┘        │
│  ┌─────────────────┐         ┌─────────────────┐        │
│  │ Stake Ops       │         │ ValidatorList   │        │
│  │ - Deposit       │         │ - BigVec Storage│        │
│  │ - Withdraw      │         │ - Validator Info│        │
│  │ - AddValidator  │         │ - Performance   │        │
│  │ - RemoveValid.  │         │   Tracking      │        │
│  └─────────────────┘         └─────────────────┘        │
│                                                         │
│  ┌─────────────────┐         ┌─────────────────┐        │
│  │ Token Operations│         │ Transient Accts │        │
│  │ - DepositToken  │         │ - Temporary     │        │
│  │ - WithdrawToken │         │ - Secure Ops    │        │
│  │ - IncreaseValid │         │ - Auto Cleanup  │        │
│  │ - DecreaseValid │         └─────────────────┘        │
│  └─────────────────┘                                    │
└─────────────────────────────────────────────────────────┘
```

### 2. Program Derived Addresses (PDAs)

The system uses PDAs for secure authority management:

```rust
// Authority seeds defined in lib.rs
AUTHORITY_DEPOSIT = b"deposit"
AUTHORITY_WITHDRAW = b"withdraw"
TRANSIENT_STAKE_SEED_PREFIX = b"transient"
EPHEMERAL_STAKE_SEED_PREFIX = b"ephemeral"
```

#### PDA Structure:
```
Stake Pool Address + Seeds → PDA
├── [pool_address, "deposit"] → Deposit Authority
├── [pool_address, "withdraw"] → Withdraw Authority
├── [pool_address, "transient", validator, seed] → Transient Stake Account
└── [pool_address, "ephemeral", seed] → Ephemeral Stake Account
```

### 3. Data Structures

The program maintains several key data structures:

- **StakePool**: Main pool account containing authorities, fees, and financial state
- **ValidatorList**: Efficiently stores validator information using BigVec implementation
- **ValidatorStakeInfo**: Tracks individual validator stake and performance
- **Fee Structures**: Manages various fee types and future epoch configurations

For detailed field-level specifications, refer to the source code in `program/src/state.rs`.

### 4. Security Model

#### Fee Protection Mechanism:
```rust
// Maximum 3/2 ratio increase per epoch prevents malicious fee increases
pub const MAX_WITHDRAWAL_FEE_INCREASE: Fee = Fee {
    numerator: 3,
    denominator: 2,
};
```

#### Key Security Features:

1. **Authority Separation**: Different authorities for different operations
2. **Fee Rate Limiting**: Prevents sudden fee increases that could harm users
3. **PDA Security**: All critical authorities use program-derived addresses
4. **Transient Account Limits**: Maximum 10 transient operations per transaction
5. **Minimum Stake Requirements**: 1,000,000 lamports minimum per validator

#### Compute Budget Management:
```rust
pub const MAX_VALIDATORS_TO_UPDATE: usize = 4;  // Per instruction limit
```

## Transaction Flow Architecture

### Deposit Flow:
```
User tokens/Stake → Validation → Pool Integration → Token Minting
     │              │               │                │
     ▼              ▼               ▼                ▼
┌─────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│ Prepare │  │   Verify    │  │  Update     │  │   Issue     │
│ Account │  │ Authorities │  │ Pool State  │  │ Pool Tokens │
│ & Funds │  │ & Amounts   │  │ & Balances  │  │ to User     │
└─────────┘  └─────────────┘  └─────────────┘  └─────────────┘
```

### Withdrawal Flow:
```
Pool Tokens → Validation → Stake Preparation → tokens Transfer
     │            │              │                 │
     ▼            ▼              ▼                 ▼
┌─────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│  Burn   │  │   Check     │  │ Deactivate  │  │  Transfer   │
│ Tokens  │  │ Balances &  │  │ Stake or    │  │ tokens from │
│ & Fees  │  │ Permissions │  │ Use Reserve │  │ Reserve     │
└─────────┘  └─────────────┘  └─────────────┘  └─────────────┘
```

## Client Architecture

### 1. CLI Tool (`clients/cli/`)
- **Purpose**: Administrative operations and power-user functionality
- **Architecture**: Single binary with command-based interface
- **Key Features**: Pool creation, validator management, fee adjustment

### 2. TypeScript SDK (`packages/sdk/`)

## Performance Considerations

### Compute Budget Management
- **Validator Updates**: Limited to 4 per instruction due to compute constraints
- **Batch Processing**: Large operations split across multiple transactions
- **Optimize Account Access**: Minimize account reads in hot paths

### Storage Optimization
- **BigVec**: Efficient growth strategy for validator lists
- **Account Size**: Minimized through careful struct packing
- **Rent Optimization**: Accounts sized for rent exemption

### Network Efficiency
- **Transaction Batching**: Related operations grouped when possible
- **Account Prefetching**: SDK pre-loads related accounts
- **Connection Pooling**: Efficient RPC usage patterns

## Upgrade Strategy

### Program Upgrades
- **Immutable Core**: Critical logic paths use immutable deployment
- **State Migrations**: Versioned account structures with migration support
- **Backward Compatibility**: New features maintain compatibility with existing pools

### Client Library Versioning
- **Semantic Versioning**: Standard semver across all client libraries
- **API Stability**: Stable public APIs with deprecation notices
- **Cross-Version Support**: SDKs support multiple program versions

## Monitoring & Observability

### On-Chain Metrics
- **Pool Performance**: APY, fees collected, validator performance
- **Health Indicators**: Reserve levels, validator activation status
- **Usage Statistics**: Deposits, withdrawals, token transfers

### Off-Chain Monitoring
- **RPC Performance**: Connection reliability and response times
- **Transaction Success Rates**: Success/failure ratios by operation
- **Client Library Usage**: SDK adoption and error patterns

## Design Principles

1. **Security First**: All operations validate inputs and maintain invariants
2. **Efficiency**: Compute and storage optimized for blockchain constraints
3. **Composability**: Designed to integrate with broader DeFi ecosystem
4. **Transparency**: All operations are auditable on-chain
5. **Decentralization**: No single points of failure or control
6. **User Experience**: Simple interfaces hiding complex operations

This architecture enables secure, efficient, and scalable liquid staking on FOGO blockchain while maintaining the decentralized principles of the network.
