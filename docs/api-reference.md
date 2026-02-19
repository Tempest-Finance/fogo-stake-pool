# Fogo Stake Pool API Reference

This document provides a comprehensive API reference for all Fogo Stake Pool components including the FOGO blockchain program instructions,
TypeScript SDK, CLI commands, and Python client.

## Program Instructions API

### Program ID

`SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr`

### Core Data Structures

The program uses several key data structures for managing pool state. For complete struct definitions and field-level details, refer to the source code in `program/src/state.rs`.

**Key Structures:**

- **StakePool**: Main pool account (~555 bytes)
- **ValidatorStakeInfo**: Individual validator information (~88 bytes per validator)
- **Fee**: Numerator/denominator fee structure
- **FutureEpoch**: Time-based configuration changes

Refer to the program source code for detailed field specifications.

## Program Instructions Reference

### Pool Management Instructions

#### Initialize

Creates a new stake pool.

```rust
Initialize {
    fee: Fee,                       // Management fee on rewards
    withdrawal_fee: Fee,            // Fee on withdrawals
    deposit_fee: Fee,               // Fee on deposits
    referral_fee: u8,               // Referral fee percentage (0-100)
    max_validators: u32,            // Maximum expected validators
}
```

**Accounts (10):**

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

**PDA Seeds:**

- Withdraw Authority: `[stake_pool_address, b"withdraw"]`
- Deposit Authority: `[stake_pool_address, b"deposit"]`

#### SetManager

Updates the pool manager.

```rust
SetManager
```

**Accounts (4):**

1. `[w]` Stake pool
2. `[s]` Current manager
3. `[s]` New manager
4. `[]` New manager fee account

#### SetFee

Updates pool fees.

```rust
SetFee {
    fee: FeeType,
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

**Accounts (2):**

1. `[w]` Stake pool
2. `[s]` Manager

### Validator Management Instructions

#### AddValidatorToPool

Adds a validator to the stake pool.

```rust
AddValidatorToPool(u32)  // Optional validator seed
```

**Accounts (13):**

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

Removes a validator from the stake pool.

```rust
RemoveValidatorFromPool
```

**Accounts (7):**

1. `[w]` Stake pool
2. `[s]` Staker
3. `[]` Withdraw authority
4. `[w]` Validator list
5. `[w]` Validator stake account
6. `[w]` Transient stake account
7. `[]` Clock sysvar
8. `[]` Stake program

#### IncreaseValidatorStake

Increases stake on a validator from the reserve.

```rust
IncreaseValidatorStake {
    lamports: u64,
    transient_stake_seed: u64,
}
```

**Accounts (14):**

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

### User Operations Instructions

#### DepositSol

Deposits tokens into the pool's reserve.

```rust
DepositSol(u64)  // Amount in lamports
```

**Accounts (11):**

1. `[w]` Stake pool
2. `[]` Withdraw authority
3. `[w]` Reserve stake account
4. `[s]` Funding account
5. `[w]` Destination pool token account
6. `[w]` Manager fee account
7. `[w]` Referrer pool token account
8. `[w]` Pool token mint
9. `[]` System program
10. `[]` Token program
11. `[s]` (Optional) tokens deposit authority

#### WithdrawSol

Withdraws tokens from the pool's reserve.

```rust
WithdrawSol(u64)  // Pool tokens to burn
```

**Accounts (13):**

1. `[w]` Stake pool
2. `[]` Withdraw authority
3. `[s]` User transfer authority
4. `[w]` Source pool token account
5. `[w]` Reserve stake account
6. `[w]` Destination system account
7. `[w]` Manager fee account
8. `[w]` Pool token mint
9. `[]` Clock sysvar
10. `[]` Stake history sysvar
11. `[]` Stake program
12. `[]` Token program
13. `[s]` (Optional) tokens withdraw authority

#### DepositWsolWithSession

Deposits wrapped SOL (WSOL) into the pool using a session token (FOGO blockchain specific).

```rust
DepositWsolWithSession {
    amount: u64,  // Amount in lamports
}
```

**Accounts (12):**

1. `[w]` Stake pool
2. `[]` Withdraw authority
3. `[w]` Reserve stake account
4. `[s]` Session authority
5. `[w]` User pool token account
6. `[w]` Manager fee account
7. `[w]` Referrer pool token account
8. `[w]` Pool token mint
9. `[]` System program
10. `[]` Token program
11. `[]` Session token account
12. `[s]` User wallet (payer)

**Description:**
Enables gasless deposits on FOGO blockchain using session tokens. Session authority signs on behalf of users for improved UX.

#### WithdrawWsolWithSession

Withdraws wrapped SOL (WSOL) from the pool using a session token (FOGO blockchain specific).

```rust
WithdrawWsolWithSession {
    amount: u64,  // Pool tokens to burn
}
```

**Accounts (13):**

1. `[w]` Stake pool
2. `[]` Withdraw authority
3. `[s]` Session authority
4. `[w]` User pool token account
5. `[w]` Reserve stake account
6. `[w]` Destination WSOL account
7. `[w]` Manager fee account
8. `[w]` Pool token mint
9. `[]` Clock sysvar
10. `[]` Stake history sysvar
11. `[]` Stake program
12. `[]` Token program
13. `[]` Session token account

**Description:**
Enables gasless withdrawals on FOGO blockchain using session tokens. Session authority validates and processes withdrawals without requiring individual wallet signatures.

#### DepositStake

Deposits a stake account into the pool.

```rust
DepositStake
```

**Accounts (15):**

1. `[w]` Stake pool
2. `[w]` Validator list
3. `[s]/[]` Deposit authority
4. `[]` Withdraw authority
5. `[w]` Stake account to deposit
6. `[w]` Validator stake account
7. `[w]` Reserve stake account
8. `[w]` Destination pool token account
9. `[w]` Manager fee account
10. `[w]` Referrer pool token account
11. `[w]` Pool token mint
12. `[]` Clock sysvar
13. `[]` Stake history sysvar
14. `[]` Token program
15. `[]` Stake program

#### WithdrawStake

Withdraws stake from the pool.

```rust
WithdrawStake(u64)  // Pool tokens to burn
```

**Accounts (13):**

1. `[w]` Stake pool
2. `[w]` Validator list
3. `[]` Withdraw authority
4. `[w]` Source validator/reserve stake
5. `[w]` Destination stake account
6. `[]` New stake authority
7. `[s]` User transfer authority
8. `[w]` Source pool token account
9. `[w]` Manager fee account
10. `[w]` Pool token mint
11. `[]` Clock sysvar
12. `[]` Token program
13. `[]` Stake program

### Maintenance Instructions

#### UpdateValidatorListBalance

Updates validator balances and processes transient stakes.

```rust
UpdateValidatorListBalance {
    start_index: u32,
    no_merge: bool,
}
```

**Accounts (7 + 2N):**

1. `[]` Stake pool
2. `[]` Withdraw authority
3. `[w]` Validator list
4. `[w]` Reserve stake
5. `[]` Clock sysvar
6. `[]` Stake history sysvar
7. `[]` Stake program
8. `[]` N pairs of (validator stake, transient stake)

#### UpdateStakePoolBalance

Updates the pool's total balance.

```rust
UpdateStakePoolBalance
```

**Accounts (7):**

1. `[w]` Stake pool
2. `[]` Withdraw authority
3. `[w]` Validator list
4. `[]` Reserve stake
5. `[w]` Manager fee account
6. `[w]` Pool token mint
7. `[]` Token program

## TypeScript SDK API

### Installation

```bash
npm install @ignitionfi/fogo-stake-pool @solana/web3.js @solana/spl-token
```

### Core Functions

#### getStakePoolAccount

```typescript
async function getStakePoolAccount(
  connection: Connection,
  stakePoolAddress: PublicKey
): Promise<StakePoolAccount>
```

Retrieves and deserializes a stake pool account.

**Parameters:**

- `connection`: FOGO RPC connection
- `stakePoolAddress`: Stake pool public key

**Returns:**

- `StakePoolAccount`: Object containing pubkey and decoded account data

**Example:**

```typescript
const pool = await getStakePoolAccount(connection, poolPubkey)
console.log('Total lamports:', pool.account.data.totalLamports)
```

#### getStakePoolAccounts

```typescript
async function getStakePoolAccounts(
  connection: Connection,
  stakePoolProgramAddress: PublicKey
): Promise<(StakePoolAccount | ValidatorListAccount | undefined)[]>
```

Retrieves all stake pool and validator list accounts.

**Parameters:**

- `connection`: FOGO RPC connection
- `stakePoolProgramAddress`: Program ID

**Returns:**

- Array of stake pool and validator list accounts

#### depositSol

```typescript
async function depositSol(
  connection: Connection,
  stakePoolAddress: PublicKey,
  from: PublicKey,
  lamports: number,
  destinationTokenAccount?: PublicKey,
  referrerTokenAccount?: PublicKey,
  depositAuthority?: PublicKey
): Promise<{ instructions: TransactionInstruction[], signers: Signer[] }>
```

Creates instructions for tokens deposit.

**Parameters:**

- `connection`: FOGO RPC connection
- `stakePoolAddress`: Stake pool public key
- `from`: Source tokens account
- `lamports`: Amount to deposit in lamports
- `destinationTokenAccount`: Pool token destination (optional)
- `referrerTokenAccount`: Referrer token account (optional)
- `depositAuthority`: Required deposit authority (optional)

**Returns:**

- Object with transaction instructions and required signers

#### withdrawSol

```typescript
async function withdrawSol(
  connection: Connection,
  stakePoolAddress: PublicKey,
  tokenOwner: PublicKey,
  solReceiver: PublicKey,
  amount: number,
  solWithdrawAuthority?: PublicKey
): Promise<{ instructions: TransactionInstruction[], signers: Signer[] }>
```

Creates instructions for tokens withdrawal.

**Parameters:**

- `connection`: FOGO RPC connection
- `stakePoolAddress`: Stake pool public key
- `tokenOwner`: Pool token owner
- `solReceiver`: tokens destination account
- `amount`: Pool tokens to burn
- `solWithdrawAuthority`: Required withdraw authority (optional)

**Returns:**

- Object with transaction instructions and required signers

#### depositWsolWithSession

```typescript
async function depositWsolWithSession(
  connection: Connection,
  stakePoolAddress: PublicKey,
  sessionAuthority: PublicKey,
  userTokenAccount: PublicKey,
  amount: number,
  referrerTokenAccount?: PublicKey
): Promise<{ instructions: TransactionInstruction[], signers: Signer[] }>
```

Creates instructions for WSOL deposit using FOGO session tokens (gasless transaction).

**Parameters:**

- `connection`: FOGO RPC connection
- `stakePoolAddress`: Stake pool public key
- `sessionAuthority`: Session token authority
- `userTokenAccount`: User's pool token account
- `amount`: Amount of WSOL to deposit in lamports
- `referrerTokenAccount`: Referrer token account (optional)

**Returns:**

- Object with transaction instructions and required signers

**Description:**
This function enables gasless deposits on FOGO blockchain by leveraging session tokens. The session authority signs transactions on behalf of users, providing a seamless UX without requiring wallet signatures for each transaction.

#### withdrawWsolWithSession

```typescript
async function withdrawWsolWithSession(
  connection: Connection,
  stakePoolAddress: PublicKey,
  sessionAuthority: PublicKey,
  userTokenAccount: PublicKey,
  wsolReceiver: PublicKey,
  amount: number
): Promise<{ instructions: TransactionInstruction[], signers: Signer[] }>
```

Creates instructions for WSOL withdrawal using FOGO session tokens (gasless transaction).

**Parameters:**

- `connection`: FOGO RPC connection
- `stakePoolAddress`: Stake pool public key
- `sessionAuthority`: Session token authority
- `userTokenAccount`: User's pool token account
- `wsolReceiver`: Destination WSOL account
- `amount`: Pool tokens to burn

**Returns:**

- Object with transaction instructions and required signers

**Description:**
This function enables gasless withdrawals on FOGO blockchain by leveraging session tokens. The session authority validates and processes withdrawals, allowing users to unstake without signing individual transactions.

#### withdrawStakeWithSession

```typescript
async function withdrawStakeWithSession(
  connection: Connection,
  stakePoolAddress: PublicKey,
  signerOrSession: PublicKey,
  userPubkey: PublicKey,
  payer: PublicKey,
  amount: number,
  userStakeSeedStart?: number,
  useReserve?: boolean,
  voteAccountAddress?: PublicKey,
  minimumLamportsOut?: number,
  validatorComparator?: (a: ValidatorAccount, b: ValidatorAccount) => number
): Promise<{
  instructions: TransactionInstruction[]
  stakeAccountPubkeys: PublicKey[]
  userStakeSeeds: number[]
}>
```

Withdraws stake from the pool using a Fogo session, creating stake account PDAs owned by the user.

**Parameters:**

- `connection`: FOGO RPC connection
- `stakePoolAddress`: Stake pool public key
- `signerOrSession`: Session signer public key
- `userPubkey`: User's wallet (used for PDA derivation and token ownership)
- `payer`: Payer for stake account rent (typically paymaster)
- `amount`: Pool tokens to withdraw
- `userStakeSeedStart`: Starting seed for user stake PDA derivation (default: 0)
- `useReserve`: Whether to withdraw from reserve (default: false)
- `voteAccountAddress`: Optional specific validator to withdraw from
- `minimumLamportsOut`: Minimum lamports to receive (slippage protection)
- `validatorComparator`: Optional comparator for validator selection

**Returns:**

- Object with transaction instructions, stake account pubkeys, and seeds used

**Description:**
This function enables gasless stake withdrawals on FOGO blockchain. The stake accounts are created as PDAs derived from `[b"user_stake", user_wallet, seed]`, allowing users to withdraw stake without signing transactions.

#### findNextUserStakeSeed

```typescript
async function findNextUserStakeSeed(
  connection: Connection,
  programId: PublicKey,
  userPubkey: PublicKey,
  startSeed?: number,
  maxSeed?: number
): Promise<number>
```

Finds the next available seed for creating a user stake PDA.

#### getUserStakeAccounts

```typescript
async function getUserStakeAccounts(
  connection: Connection,
  programId: PublicKey,
  userPubkey: PublicKey,
  maxSeed?: number
): Promise<UserStakeAccount[]>
```

Fetches all user stake accounts created via `WithdrawStakeWithSession`.

**Returns:**

```typescript
interface UserStakeAccount {
  pubkey: PublicKey
  seed: number
  lamports: number
  state: 'inactive' | 'activating' | 'active' | 'deactivating'
  voter?: PublicKey
  activationEpoch?: number
  deactivationEpoch?: number
}
```

#### depositStake

```typescript
async function depositStake(
  connection: Connection,
  stakePoolAddress: PublicKey,
  authorizedPubkey: PublicKey,
  validatorVote: PublicKey,
  depositStake: PublicKey,
  poolTokenReceiverAccount?: PublicKey
): Promise<{ instructions: TransactionInstruction[], signers: Signer[] }>
```

Creates instructions for stake account deposit.

#### withdrawStake

```typescript
async function withdrawStake(
  connection: Connection,
  stakePoolAddress: PublicKey,
  tokenOwner: PublicKey,
  amount: number,
  useReserve?: boolean,
  voteAccountAddress?: PublicKey,
  stakeReceiver?: PublicKey,
  poolTokenAccount?: PublicKey,
  validatorComparator?: (a: ValidatorAccount, b: ValidatorAccount) => number
): Promise<{
  instructions: TransactionInstruction[]
  signers: Signer[]
  stakeReceiver: PublicKey
  totalRentFreeBalances: number
}>
```

Creates instructions for stake withdrawal.

### Validator Management Functions

#### addValidatorToPool

```typescript
async function addValidatorToPool(
  connection: Connection,
  stakePoolAddress: PublicKey,
  validatorVote: PublicKey,
  seed?: number
): Promise<{ instructions: TransactionInstruction[] }>
```

#### removeValidatorFromPool

```typescript
async function removeValidatorFromPool(
  connection: Connection,
  stakePoolAddress: PublicKey,
  validatorVote: PublicKey,
  seed?: number
): Promise<{ instructions: TransactionInstruction[] }>
```

#### increaseValidatorStake

```typescript
async function increaseValidatorStake(
  connection: Connection,
  stakePoolAddress: PublicKey,
  validatorVote: PublicKey,
  lamports: number,
  ephemeralStakeSeed?: number
): Promise<{ instructions: TransactionInstruction[] }>
```

#### decreaseValidatorStake

```typescript
async function decreaseValidatorStake(
  connection: Connection,
  stakePoolAddress: PublicKey,
  validatorVote: PublicKey,
  lamports: number,
  ephemeralStakeSeed?: number
): Promise<{ instructions: TransactionInstruction[] }>
```

### Pool Information Functions

#### stakePoolInfo

```typescript
async function stakePoolInfo(
  connection: Connection,
  stakePoolAddress: PublicKey
): Promise<StakePoolInfo>
```

Retrieves comprehensive stake pool information.

**Returns:**

```typescript
interface StakePoolInfo {
  address: string
  poolWithdrawAuthority: string
  manager: string
  staker: string
  stakeDepositAuthority: string
  maxValidators: number
  validatorList: ValidatorInfo[]
  poolMint: string
  totalLamports: string
  poolTokenSupply: string
  lastUpdateEpoch: string
  fees: {
    epochFee: Fee
    stakeDepositFee: Fee
    stakeWithdrawalFee: Fee
    solDepositFee: Fee
    solWithdrawalFee: Fee
  }
  details: {
    reserveStakeLamports: number
    stakeAccounts: StakeAccountInfo[]
    totalPoolTokens: number
    currentNumberOfValidators: number
    updateRequired: boolean
  }
}
```

#### updateStakePool

```typescript
async function updateStakePool(
  connection: Connection,
  stakePool: StakePoolAccount,
  noMerge?: boolean
): Promise<{
  updateListInstructions: TransactionInstruction[]
  finalInstructions: TransactionInstruction[]
}>
```

Creates all instructions needed to update a stake pool.

### Utility Functions

#### lamportsToSol / solToLamports

```typescript
function lamportsToSol(lamports: number): number
function solToLamports(sol: number): number
```

Convert between tokens and lamports.

#### calcLamportsWithdrawAmount

```typescript
function calcLamportsWithdrawAmount(
  stakePool: StakePool,
  poolTokens: BN
): BN
```

Calculate tokens amount for given pool tokens.

### Constants

```typescript
// Program IDs
export const STAKE_POOL_PROGRAM_ID: PublicKey
export const DEVNET_STAKE_POOL_PROGRAM_ID: PublicKey

// Limits
export const MAX_VALIDATORS_TO_UPDATE = 4
export const MINIMUM_ACTIVE_STAKE = 1_000_000
```

## CLI Commands Reference

### Global Options

```bash
--url <URL>                    # RPC endpoint
--manager <KEYPAIR>            # Manager keypair
--staker <KEYPAIR>             # Staker keypair
--token-owner <KEYPAIR>        # Token owner keypair
--fee-payer <KEYPAIR>          # Fee payer keypair
--verbose                      # Verbose output
--dry-run                      # Simulate only
--output json                  # JSON output format
```

### Pool Management Commands

#### create-pool

```bash
fogo-stake-pool create-pool \
  --epoch-fee-numerator <NUM> \
  --epoch-fee-denominator <NUM> \
  --withdrawal-fee-numerator <NUM> \
  --withdrawal-fee-denominator <NUM> \
  --deposit-fee-numerator <NUM> \
  --deposit-fee-denominator <NUM> \
  --referral-fee <PERCENTAGE> \
  --max-validators <COUNT> \
  [--pool-keypair <KEYPAIR>] \
  [--validator-list-keypair <KEYPAIR>] \
  [--mint-keypair <KEYPAIR>] \
  [--reserve-keypair <KEYPAIR>]
```

#### set-manager

```bash
fogo-stake-pool set-manager <POOL_ADDRESS> \
  --new-manager <PUBKEY> \
  --new-fee-receiver <PUBKEY>
```

#### set-fee

```bash
fogo-stake-pool set-fee <POOL_ADDRESS> <FEE_TYPE> \
  --fee-numerator <NUM> \
  --fee-denominator <NUM>
```

Fee types: `epoch`, `stake-deposit`, `sol-deposit`, `stake-withdrawal`, `sol-withdrawal`

### Validator Management Commands

#### add-validator

```bash
fogo-stake-pool add-validator <POOL_ADDRESS> <VALIDATOR_VOTE_ACCOUNT> \
  [--seed <SEED>]
```

#### remove-validator

```bash
fogo-stake-pool remove-validator <POOL_ADDRESS> <VALIDATOR_VOTE_ACCOUNT>
```

#### increase-validator-stake

```bash
fogo-stake-pool increase-validator-stake <POOL_ADDRESS> <VALIDATOR_VOTE_ACCOUNT> \
  --lamports <AMOUNT>
```

#### decrease-validator-stake

```bash
fogo-stake-pool decrease-validator-stake <POOL_ADDRESS> <VALIDATOR_VOTE_ACCOUNT> \
  --lamports <AMOUNT>
```

### User Operations Commands

#### deposit-sol

```bash
fogo-stake-pool deposit-sol <POOL_ADDRESS> <AMOUNT> \
  [--token-receiver <TOKEN_ACCOUNT>] \
  [--referrer <TOKEN_ACCOUNT>]
```

#### withdraw-sol

```bash
fogo-stake-pool withdraw-sol <POOL_ADDRESS> <POOL_TOKEN_AMOUNT> \
  [--sol-receiver <SOL_ACCOUNT>]
```

#### deposit-stake

```bash
fogo-stake-pool deposit-stake <POOL_ADDRESS> <STAKE_ACCOUNT> \
  [--token-receiver <TOKEN_ACCOUNT>]
```

#### withdraw-stake

```bash
fogo-stake-pool withdraw-stake <POOL_ADDRESS> <POOL_TOKEN_AMOUNT> \
  [--vote-account <VALIDATOR_VOTE_ACCOUNT>] \
  [--stake-receiver <STAKE_ACCOUNT>]
```

### Information Commands

#### list

```bash
fogo-stake-pool list <POOL_ADDRESS>
```

#### list-all

```bash
fogo-stake-pool list-all
```

#### update

```bash
fogo-stake-pool update <POOL_ADDRESS> \
  [--no-merge] \
  [--force]
```

## Error Codes Reference

### Program Errors

```rust
pub enum StakePoolError {
    // Setup errors
    AlreadyInUse = 0,                    // Account already in use
    InvalidProgramAddress = 1,           // Invalid program address
    InvalidState = 2,                    // Invalid account state
    IncorrectProgramId = 3,             // Incorrect program ID
    IncorrectOwner = 4,                 // Incorrect account owner

    // Authority errors
    WrongAccountMint = 5,               // Wrong token mint
    NonzeroPoolTokenSupply = 6,         // Pool tokens exist
    StakeListAndPoolLamportsMismatch = 7, // Balance mismatch
    UnknownValidatorStakeAccount = 8,   // Unknown validator

    // Operation errors
    StakeListOutOfDate = 9,             // Stale validator list
    StakeNotActive = 10,                // Inactive stake account
    ValidatorAlreadyAdded = 11,         // Validator exists
    ValidatorNotFound = 12,             // Validator not found

    // Math/calculation errors
    CalculationFailure = 13,            // Math overflow/underflow
    FeeTooHigh = 14,                    // Fee exceeds 100%
    WithdrawTooLarge = 15,              // Withdrawal too large
    WithdrawTooSmall = 16,              // Withdrawal too small

    // And many more...
}
```

### Common Error Solutions

| Error                   | Cause                        | Solution                            |
| ----------------------- | ---------------------------- | ----------------------------------- |
| `StakeListOutOfDate`    | Validator list needs update  | Run `update-validator-list-balance` |
| `ValidatorAlreadyAdded` | Validator already in pool    | Check validator list before adding  |
| `FeeTooHigh`            | Fee numerator >= denominator | Set reasonable fee values           |
| `WithdrawTooLarge`      | Insufficient pool balance    | Check available withdrawal amount   |
| `CalculationFailure`    | Math overflow                | Use reasonable amounts              |

### Batch Operations

For multiple operations, batch when possible:

```typescript
// TypeScript - batch multiple instructions
const transaction = new Transaction();
transaction.add(...depositInstructions);
transaction.add(...updateInstructions);

// CLI - use update command for multiple validators
fogo-stake-pool update POOL_ADDRESS
```

### Connection Management

```typescript
// Reuse connections
const connection = new Connection(rpcUrl, {
  commitment: 'confirmed',
  disableRetryOnRateLimit: false,
})

// Use connection pooling for high-volume applications
```

## Versioning and Compatibility

### Program Versions

- **v2.0.3**: Current program version
- **API Stability**: All documented APIs are stable
- **Backwards Compatibility**: Maintained for all v2.x releases

### SDK Versions

- **TypeScript SDK**: `@ignitionfi/fogo-stake-pool@1.1.8`
- **Python Client**: `fogo-stake-pool@2.0.x`
- **CLI Tool**: `fogo-stake-pool@2.0.1`

### Breaking Changes

Breaking changes are announced with:

1. Deprecation warnings (minimum 1 release)
2. Migration guides
3. Updated documentation

## Support and Resources

### Documentation

- [Program Guide](./program-guide.md) - Detailed program documentation
- [API Reference](./api-reference.md#typescript-sdk) - TypeScript SDK examples
- [CLI Reference](./cli-reference.md) - Complete CLI documentation

### Community

- **GitHub**: Issues and discussions
- **FOGO Discord**: `#developers` channel
- **Stack Overflow**: Tag questions with `fogo` and `fogo-stake-pool`

### Security

- Report security issues privately to maintainers
- Security audit reports available in repository
- Bug bounty program (if applicable)

This API reference provides comprehensive documentation for all Fogo Stake Pool components. For specific implementation examples, see the individual component guides.
