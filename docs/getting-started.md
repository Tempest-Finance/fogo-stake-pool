# Getting Started with Ignition Stake Pool

This guide will help you quickly set up your development environment and start working with the Ignition Stake Pool program.

## Prerequisites

Before you begin, ensure you have the following installed:

### Required Tools
- **Rust**: `rustc 1.86.0` or later
- **Solana CLI**: Version `2.3.4` or later
- **Node.js**: `22.x` or later
- **pnpm**: `10.20.0`
- **Git**: For cloning the repository

### Install Rust and Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Solana CLI
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash

# Verify installation
solana --version
```

## Quick Setup

### 1. Clone the Repository

```bash
git clone https://github.com/tempest-finance/spl-stake-pool.git
cd tempest-finance/spl-stake-pool
```

### 2. Install Dependencies

```bash
# Install JavaScript dependencies
pnpm install

# Verify solana version compatibility
pnpm solana:check
```

### 3. Build the Project

```bash
# Build the program
make build
# OR
pnpm programs:build

# Build the CLI tool
make build/cli
# OR
cargo build --bin spl-stake-pool --release

# Build the TypeScript SDK
pnpm sdk build

```

### 4. Run Tests

```bash
# Run all tests
make test

# Run specific component tests
pnpm programs:test       # Program tests
pnpm clients:cli:test    # CLI tests
pnpm sdk test            # SDK tests
pnpm clients:py:test     # Python tests
```

## Your First Stake Pool

### Setup Local Environment

```bash
# Start local solana validator (in a separate terminal)
solana-test-validator

# In another terminal, configure CLI for local development
solana config set --url localhost
solana config set --keypair ~/.config/fogo/id.json

# Airdrop tokens for testing
solana airdrop 10
```

### Create a Stake Pool

```bash
# Generate a new keypair for the pool
solana-keygen new --outfile ./my-pool-keypair.json

# Create the stake pool
./spl-stake-pool create-pool \
  --pool-keypair ./my-pool-keypair.json \
  --validator-list-keypair ./my-validator-list.json \
  --mint-keypair ./my-mint.json \
  --reserve-keypair ./my-reserve.json \
  --max-validators 100 \
  --deposit-fee-numerator 1 \
  --deposit-fee-denominator 1000 \
  --withdrawal-fee-numerator 5 \
  --withdrawal-fee-denominator 1000 \
  --referral-fee 50

# Or using Make command with default parameters
make deploy/localnet
```

### Add Validators to Pool

```bash
# Add a validator to the pool
./spl-stake-pool add-validator \
  --pool-keypair ./my-pool-keypair.json \
  --validator-vote-account <VALIDATOR_VOTE_PUBKEY>
```

### Deposit Stake

```bash
# Create a stake account first
solana create-stake-account ./stake-account.json 1

# Delegate the stake account to a validator
solana delegate-stake ./stake-account.json <VALIDATOR_VOTE_PUBKEY>

# Wait for the stake to activate, then deposit into pool
./spl-stake-pool deposit-stake \
  --pool ./my-pool-keypair.json \
  --stake-account ./stake-account.json \
  --token-receiver <YOUR_TOKEN_ACCOUNT>
```

## Using the TypeScript SDK

### Basic Setup

```typescript
import {
  Connection,
  PublicKey,
  Keypair,
  clusterApiUrl,
} from '@solana/web3.js';
import {
  StakePoolProgram,
  STAKE_POOL_PROGRAM_ID,
} from '@solana/spl-stake-pool';

// Connect to cluster
const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

// Load your keypair
const payer = Keypair.fromSecretKey(
  Buffer.from(JSON.parse(process.env.PRIVATE_KEY || '[]'))
);
```

### Find Stake Pools

```typescript
// Find all stake pools
const stakePools = await connection.getProgramAccounts(STAKE_POOL_PROGRAM_ID, {
  filters: [
    {
      dataSize: 555, // StakePool account size
    },
  ],
});

console.log(`Found ${stakePools.length} stake pools`);
```

### Deposit tokens into Pool

```typescript
// Find stake pool by address
const stakePoolAddress = new PublicKey('YOUR_STAKE_POOL_ADDRESS');
const stakePool = await StakePoolProgram.getStakePoolAccount(
  connection,
  stakePoolAddress
);

// Deposit tokens and receive pool tokens
const lamports = 1_000_000_000; // 1 tokens
const transaction = StakePoolProgram.depositSol({
  stakePool: stakePoolAddress,
  withdrawAuthority: stakePool.account.data.withdrawAuthority,
  depositAuthority: stakePool.account.data.depositAuthority,
  reserveStake: stakePool.account.data.reserveStake,
  lamports,
  destinationPoolAccount: poolTokenAccount,
  managerFeeAccount: stakePool.account.data.managerFeeAccount,
  referralPoolAccount: poolTokenAccount, // Use same account if no referral
  poolMint: stakePool.account.data.poolMint,
});

await connection.sendTransaction(transaction, [payer]);
```

## Using the Web Application

### Start Development Server

```bash
pnpm start
```

- Connecting wallets via Fogo Sessions
- Viewing stake pool information
- Depositing and withdrawing tokens
- Managing stake positions

### Key Features
- **Wallet Integration**: Seamless connection through Fogo Sessions SDK
- **Real-time Data**: Live stake pool metrics and APY calculations
- **Responsive Design**: Works on desktop and mobile devices
- **Transaction History**: Track your stake pool interactions

## Next Steps

### Development

1. **Explore the Architecture**: Read the [Architecture Guide](./architecture.md) to understand the system design
2. **Deep Dive into the Program**: Check out the [Program Guide](./program-guide.md) for detailed instruction documentation
3. **Master the CLI**: Review the [CLI Reference](./cli-reference.md) for advanced operations
4. **SDK Integration**: Follow the [API Reference](./api-reference.md#typescript-sdk) for building applications

### Deployment

1. **Test on Devnet**: Deploy your pool to FOGO devnet for testing
2. **Security Review**: Ensure proper fee settings and validator selection
3. **Mainnet Deployment**: Follow the [Deployment Guide](./deployment.md) for production deployment

### Community

- **Join Discord**: Connect with other developers in the Discord
- **Follow Updates**: Watch the repository for new features and security updates
- **Contribute**: Submit issues and pull requests to improve the codebase

## Common Issues

### Build Failures
- Ensure you're using the correct Rust nightly version: `nightly-2025-02-16`
- Check Solana CLI version matches exactly: `2.3.4`
- Clear cargo cache: `cargo clean` if encountering link errors

### Test Failures
- Ensure local validator is running for integration tests
- Check that all dependencies are installed: `pnpm install`
- Verify network connectivity for devnet/testnet tests

### Transaction Failures
- Ensure sufficient tokens balance for rent and fees
- Check stake account activation status before operations
- Verify validator vote accounts are active and not delinquent

---

You're now ready to start building with Ignition Stake Pool! For detailed API documentation and advanced usage patterns, continue with the other guides in this documentation.
