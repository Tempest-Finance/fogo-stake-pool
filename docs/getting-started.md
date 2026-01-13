# Getting Started with Ignition Stake Pool

This guide will help you quickly set up your development environment and using the Ignition Stake Pool program. 
Whether you're building a dApp, managing validators, or integrating staking into your application, this guide covers everything you need to get started.

## Prerequisites

Before you begin, ensure you have the following installed:

### Required Tools

- **Rust**: Version `1.86.0` or later
- **Solana CLI**: Version `2.3.4` or later
- **Node.js**: Version `22.x` or later
- **pnpm**: Version `10.28.0` or later
- **Git**: For cloning the repository

### Installing Tools

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Solana CLI
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash

# Install Node.js (using nvm recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 22
nvm use 22

# Install pnpm
npm install -g pnpm@10.28.0

# Verify installations
rustc --version
solana --version
node --version
pnpm --version
```

## Quick Setup

### 1. Clone and Install

```bash
# Clone the repository
git clone https://github.com/tempest-finance/spl-stake-pool.git
cd spl-stake-pool

# Install all dependencies
pnpm install
```

### 2. Build the Project

```bash
# Build the on-chain program
make build

# Build the CLI tool
make build/cli

# Build the TypeScript SDK
pnpm sdk build
```

### 3. Run Tests

```bash
# Run all tests
make test

# Or run specific tests
pnpm programs:test    # Program tests
pnpm clients:cli:test # CLI tests
pnpm sdk test         # SDK tests
```

## Using the TypeScript SDK

The TypeScript SDK provides a simple interface for interacting with stake pools. This section covers common operations and patterns.

### Installation

```bash
npm install @ignitionfi/spl-stake-pool @solana/web3.js
# or
pnpm add @ignitionfi/spl-stake-pool @solana/web3.js
```

### Basic Setup

```typescript
import { Connection, PublicKey, Keypair, clusterApiUrl } from '@solana/web3.js';
import {
  getStakePoolAccount,
  getStakePoolAccounts,
  STAKE_POOL_PROGRAM_ID,
} from '@ignitionfi/spl-stake-pool';

// Connect to cluster
const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

// Load your keypair
const payer = Keypair.fromSecretKey(
  Buffer.from(JSON.parse(process.env.PRIVATE_KEY || '[]'))
);
```

### Finding Stake Pools

```typescript
// Get all stake pools for a program
const stakePools = await getStakePoolAccounts(
  connection,
  STAKE_POOL_PROGRAM_ID
);

console.log(`Found ${stakePools?.length || 0} stake pools`);

// Get a specific stake pool
const stakePoolAddress = new PublicKey('YOUR_STAKE_POOL_ADDRESS');
const stakePool = await getStakePoolAccount(connection, stakePoolAddress);

console.log('Pool mint:', stakePool.account.data.poolMint.toBase58());
console.log('Total lamports:', stakePool.account.data.totalLamports.toString());
```

### Deposit Operations

#### Deposit SOL

```typescript
import { depositSol } from '@ignitionfi/spl-stake-pool';
import { sendAndConfirmTransaction, Transaction } from '@solana/web3.js';

// Deposit 1 SOL into the stake pool
const lamports = 1_000_000_000; // 1 SOL in lamports
const { instructions, signers } = await depositSol(
  connection,
  stakePoolAddress,
  payer.publicKey, // SOL comes from this account
  lamports
);

// Create and send transaction
const transaction = new Transaction().add(...instructions);
const signature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [payer, ...signers],
  { commitment: 'confirmed' }
);

console.log('Deposit successful:', signature);
```

#### Deposit Existing Stake Account

```typescript
import { depositStake } from '@ignitionfi/spl-stake-pool';

// Deposit an existing stake account into the pool
const validatorVoteAccount = new PublicKey('VALIDATOR_VOTE_ADDRESS');
const stakeAccount = new PublicKey('YOUR_STAKE_ACCOUNT');

const { instructions, signers } = await depositStake(
  connection,
  stakePoolAddress,
  payer.publicKey, // Current stake authority
  validatorVoteAccount,
  stakeAccount
);

const transaction = new Transaction().add(...instructions);
const signature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [payer, ...signers]
);

console.log('Stake deposited:', signature);
```

### Withdraw Operations

#### Withdraw SOL

```typescript
import { withdrawSol } from '@ignitionfi/spl-stake-pool';

// Withdraw 0.5 pool tokens worth of SOL
const poolTokenAmount = 0.5;
const { instructions, signers } = await withdrawSol(
  connection,
  stakePoolAddress,
  payer.publicKey, // Pool token owner
  payer.publicKey, // SOL receiver
  poolTokenAmount
);

const transaction = new Transaction().add(...instructions);
const signature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [payer, ...signers]
);

console.log('Withdrawal successful:', signature);
```

#### Withdraw Stake

```typescript
import { withdrawStake } from '@ignitionfi/spl-stake-pool';

// Withdraw stake from the pool
const poolTokenAmount = 1.0;
const { instructions, signers, stakeReceiver } = await withdrawStake(
  connection,
  stakePoolAddress,
  payer.publicKey, // Pool token owner
  poolTokenAmount
);

const transaction = new Transaction().add(...instructions);
const signature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [payer, ...signers]
);

console.log('Stake withdrawn to:', stakeReceiver?.toBase58());
console.log('Transaction:', signature);
```

### Session-Based Operations (Gasless Transactions)

The SDK supports session-based operations for gasless transactions using the Fogo Sessions SDK. 
This is particularly useful for web applications where you want to provide a seamless user experience.

#### Setup with Fogo Sessions

```typescript
import { useSession } from '@fogo/sessions-sdk-react';
import {
  depositWsolWithSession,
  withdrawWsolWithSession,
} from '@ignitionfi/spl-stake-pool';

// In your React component
function StakePoolComponent() {
  const { sessionState } = useSession();

  // Session must be established
  if (sessionState.status !== 'established') {
    return <div>Connecting...</div>;
  }

  return <StakeInterface sessionState={sessionState} />;
}
```

#### Deposit with Session

```typescript
import { depositWsolWithSession } from '@ignitionfi/spl-stake-pool';

async function depositWithSession(
  connection,
  stakePoolAddress,
  sessionState,
  amount
) {
  const lamports = amount * 1_000_000_000; // Convert SOL to lamports

  // Create deposit instructions using session
  const { instructions } = await depositWsolWithSession(
    connection,
    stakePoolAddress,
    sessionState.sessionPublicKey, // Session signer
    sessionState.walletPublicKey, // User's wallet
    lamports,
    undefined, // destinationTokenAccount (auto-created)
    undefined, // referrerTokenAccount (optional)
    undefined, // depositAuthority (optional)
    sessionState.payer // Session payer
  );

  // Send using session
  const result = await sessionState.sendTransaction(instructions);

  console.log('Signature:', result.signature);
  return result;
}
```

#### Withdraw with Session

```typescript
import { withdrawWsolWithSession } from '@ignitionfi/spl-stake-pool';

async function withdrawWithSession(
  connection,
  stakePoolAddress,
  sessionState,
  poolTokenAmount
) {
  // Create withdraw instructions using session
  const { instructions } = await withdrawWsolWithSession(
    connection,
    stakePoolAddress,
    sessionState.sessionPublicKey, // Session signer
    sessionState.walletPublicKey, // User's wallet
    poolTokenAmount
  );

  // Send using session
  const result = await sessionState.sendTransaction(instructions);

  console.log('Signature:', result.signature);
  return result;
}
```

#### Withdraw Stake with Session

```typescript
import {
  withdrawStakeWithSession,
  findNextUserStakeSeed,
  getUserStakeAccounts
} from '@ignitionfi/spl-stake-pool';

async function withdrawStakeWithSessionExample(
  connection,
  stakePoolAddress,
  sessionState,
  poolTokenAmount
) {
  // Find the next available user stake seed
  const nextSeed = await findNextUserStakeSeed(
    connection,
    STAKE_POOL_PROGRAM_ID,
    sessionState.walletPublicKey
  );

  // Create withdraw stake instructions using session
  const { instructions, stakeAccountPubkeys, userStakeSeeds } = await withdrawStakeWithSession(
    connection,
    stakePoolAddress,
    sessionState.sessionPublicKey, // Session signer
    sessionState.walletPublicKey, // User's wallet
    sessionState.payer, // Payer for stake account rent
    poolTokenAmount,
    nextSeed // Starting seed for user stake PDAs
  );

  // Send using session
  const result = await sessionState.sendTransaction(instructions);

  console.log('Stake accounts created:', stakeAccountPubkeys.map(p => p.toBase58()));
  console.log('Seeds used:', userStakeSeeds);
  return result;
}

// Fetch user's stake accounts after withdrawal
async function getUserStakes(connection, userPubkey) {
  const stakes = await getUserStakeAccounts(
    connection,
    STAKE_POOL_PROGRAM_ID,
    userPubkey
  );

  stakes.forEach(stake => {
    console.log(`Stake ${stake.seed}: ${stake.lamports} lamports, state: ${stake.state}`);
  });

  return stakes;
}
```

### Validator Management Operations

#### Add Validator to Pool

```typescript
import { addValidatorToPool } from '@ignitionfi/spl-stake-pool';

const validatorVoteAccount = new PublicKey('VALIDATOR_VOTE_ADDRESS');

const { instructions } = await addValidatorToPool(
  connection,
  stakePoolAddress,
  validatorVoteAccount
);

const transaction = new Transaction().add(...instructions);
const signature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [payer] // Must be pool staker
);

console.log('Validator added:', signature);
```

#### Remove Validator from Pool

```typescript
import { removeValidatorFromPool } from '@ignitionfi/spl-stake-pool';

const { instructions } = await removeValidatorFromPool(
  connection,
  stakePoolAddress,
  validatorVoteAccount
);

const transaction = new Transaction().add(...instructions);
const signature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [payer] // Must be pool staker
);

console.log('Validator removed:', signature);
```

#### Increase Validator Stake

```typescript
import { increaseValidatorStake } from '@ignitionfi/spl-stake-pool';

const lamports = 5_000_000_000; // 5 SOL to add to validator

const { instructions } = await increaseValidatorStake(
  connection,
  stakePoolAddress,
  validatorVoteAccount,
  lamports
);

const transaction = new Transaction().add(...instructions);
const signature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [payer] // Must be pool staker
);

console.log('Validator stake increased:', signature);
```

#### Decrease Validator Stake

```typescript
import { decreaseValidatorStake } from '@ignitionfi/spl-stake-pool';

const lamports = 2_000_000_000; // 2 SOL to remove from validator

const { instructions } = await decreaseValidatorStake(
  connection,
  stakePoolAddress,
  validatorVoteAccount,
  lamports
);

const transaction = new Transaction().add(...instructions);
const signature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [payer] // Must be pool staker
);

console.log('Validator stake decreased:', signature);
```

### Pool Maintenance and Information

#### Update Stake Pool

```typescript
import { updateStakePool } from '@ignitionfi/spl-stake-pool';

// Update pool after epoch change
const stakePool = await getStakePoolAccount(connection, stakePoolAddress);

const { updateListInstructions, finalInstructions } = await updateStakePool(
  connection,
  stakePool,
  false // noMerge flag
);

// Send update list instructions first (may need multiple transactions)
for (const instruction of updateListInstructions) {
  const tx = new Transaction().add(instruction);
  await sendAndConfirmTransaction(connection, tx, [payer]);
}

// Send final instructions
const finalTx = new Transaction().add(...finalInstructions);
await sendAndConfirmTransaction(connection, finalTx, [payer]);

console.log('Stake pool updated');
```

#### Get Stake Pool Information

```typescript
import { stakePoolInfo } from '@ignitionfi/spl-stake-pool';

// Get comprehensive pool information
const info = await stakePoolInfo(connection, stakePoolAddress);

console.log('Pool address:', info.address);
console.log('Manager:', info.manager);
console.log('Total lamports:', info.totalLamports);
console.log('Pool token supply:', info.poolTokenSupply);
console.log('Current validators:', info.details.currentNumberOfValidators);
console.log('Max validators:', info.details.maxNumberOfValidators);
console.log('Update required:', info.details.updateRequired);

// List all validators
info.validatorList.forEach((validator, index) => {
  console.log(`Validator ${index + 1}:`, validator.voteAccountAddress);
  console.log('  Active stake:', validator.activeStakeLamports);
  console.log('  Transient stake:', validator.transientStakeLamports);
});
```

## Next Steps

Now that you've set up your development environment and created your first stake pool, explore these resources to deepen your understanding:

### Learn More

- **[Architecture Guide](./architecture.md)**: Understand the system design and data structures
- **[Program Guide](./program-guide.md)**: Detailed documentation of all on-chain instructions
- **[CLI Reference](./cli-reference.md)**: Complete command-line tool documentation
- **[API Reference](./api-reference.md)**: Full TypeScript SDK API documentation

### Deploy to Production

- **[Deployment Guide](./deployment.md)**: Step-by-step guide for deploying to testnet and mainnet
- **Security**: Review fee settings and validator selection before mainnet deployment
- **Monitoring**: Set up alerts and health checks for your stake pool

### Get Help

- **Issues**: Report bugs at [GitHub Issues](https://github.com/tempest-finance/spl-stake-pool/issues)
- **Community**: Join the Discord for developer support
- **Contributing**: Submit pull requests to improve the codebase

## Troubleshooting

### Transaction Errors

**"Insufficient funds" error:**
- Check your SOL balance: `solana balance`
- Account for rent exemption (≈0.00203 SOL per stake account)
- Ensure you have enough for transaction fees

**"Stake account not active" error:**
- Wait for stake activation (usually 1-2 epochs)
- Check stake status: `solana stake-account <STAKE_ADDRESS>`

**"Validator not found" error:**
- Verify the validator is in the pool's validator list
- Check validator vote account is active: `solana validators`

---

You're now ready to build with Ignition Stake Pool! For detailed API documentation and advanced patterns, explore the other guides in this documentation.
