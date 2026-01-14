# Fogo Stake Pool Deployment Guide

This guide provides comprehensive instructions for deploying Fogo Stake Pool programs and applications across different FOGO networks and environments.

## Overview

The Fogo Stake Pool project supports deployment to multiple environments:

- **Local Development**: Local FOGO validator for testing
- **Testnet**: FOGO's testing network (via Fogo infrastructure)
- **Mainnet**: FOGO's production network

## Prerequisites

### Required Tools

```bash
# Solana CLI
solana --version

# Rust toolchain
rustc --version

# Built program artifacts
ls target/deploy/spl_stake_pool.so
```

### Network Configuration

```bash
# Check current Solana CLI configuration
solana config get

# Configure for specific network (examples)
solana config set --url https://testnet.fogo.io     # Testnet
solana config set --url http://localhost:8899       # Local
```

## Program Deployment

### Program ID and Keypairs

The Fogo Stake Pool program uses a deterministic Program ID:

```bash
# Program details
PROGRAM_ID="SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr"
PROGRAM_KEYPAIR=".keys/SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr.json"
PROGRAM_BINARY="target/deploy/spl_stake_pool.so"
```

### Generating Program Keypair

If you need to create a new program keypair:

```bash
# Create keys directory
mkdir -p .keys

# Generate program keypair with specific address (advanced)
# For existing program, use the existing keypair
cp /path/to/existing/SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr.json .keys/

# Verify keypair matches expected program ID
solana-keygen pubkey .keys/SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr.json
```

### Building for Deployment

```bash
# Clean and build release version
make clean
make build

# Verify build artifacts
ls -lh target/deploy/spl_stake_pool.so

# Check program size (should be under 1MB for efficient deployment)
du -h target/deploy/spl_stake_pool.so
```

## Local Deployment

### Setting Up Local Validator

```bash
# Start local validator with necessary features
solana-test-validator \
    --ledger .ledger \
    --bind-address 0.0.0.0 \
    --rpc-port 8899 \
    --faucet-port 9900 \
    --reset

# In another terminal, configure for local development
solana config set --url http://localhost:8899
solana config set --keypair ~/.config/fogo/id.json

# Verify connection
solana cluster-version
solana balance
```

### Deploy to Local Network

```bash
# Method 1: Using Makefile (recommended)
make deploy/localnet

# Method 2: Direct deployment
make build
solana program deploy \
    --url http://localhost:8899 \
    --program-id .keys/SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr.json \
    target/deploy/spl_stake_pool.so

# Verify deployment
solana program show SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr --url http://localhost:8899
```

### Local Testing

```bash
# Airdrop tokens for testing
solana airdrop 10

# Create test stake pool using CLI
./spl-stake-pool create-pool \
    --epoch-fee-numerator 3 \
    --epoch-fee-denominator 100 \
    --withdrawal-fee-numerator 5 \
    --withdrawal-fee-denominator 1000 \
    --deposit-fee-numerator 0 \
    --deposit-fee-denominator 1 \
    --referral-fee 10 \
    --max-validators 100

# Test basic operations
./spl-stake-pool list-all
```

## Devnet Deployment

### Configure for Devnet

```bash
# Set Solana CLI to devnet
solana config set --url https://api.devnet.fogo.io
solana config set --keypair ~/.config/fogo/devnet-deployer.json

# Verify connection
solana cluster-version
solana epoch-info
```

### Get Devnet tokens

```bash
# Airdrop tokens for deployment costs
solana airdrop 5

# Verify balance (deployment typically costs ~2-3 tokens)
solana balance
```

### Deploy to Devnet

```bash
# Method 1: Using Makefile
make deploy/testnet

# Method 2: Manual deployment
make build
solana program deploy \
    --url https://api.devnet.fogo.io \
    --program-id .keys/SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr.json \
    target/deploy/spl_stake_pool.so

# Verify deployment
solana program show SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr --url https://api.devnet.fogo.io
```

### Devnet Testing

```bash
# Create stake pool on devnet
./spl-stake-pool create-pool \
    --url https://api.devnet.fogo.io \
    --epoch-fee-numerator 3 \
    --epoch-fee-denominator 100 \
    --withdrawal-fee-numerator 5 \
    --withdrawal-fee-denominator 1000 \
    --deposit-fee-numerator 0 \
    --deposit-fee-denominator 1 \
    --referral-fee 10 \
    --max-validators 50

# List all pools on devnet
./spl-stake-pool list-all --url https://api.devnet.fogo.io
```

## Testnet Deployment (Fogo)

### Configure for Fogo Testnet

```bash
# Set Solana CLI to Fogo testnet
solana config set --url https://testnet.fogo.io
solana config set --keypair ~/.config/fogo/fogo-testnet.json

# Verify connection
solana cluster-version
```

### Get Testnet tokens

```bash
# Request tokens from Fogo faucet
# Visit faucet website or use CLI if available
solana airdrop 10 --url https://testnet.fogo.io

# Verify balance
solana balance --url https://testnet.fogo.io
```

### Deploy to Testnet

```bash
# Deploy using Makefile
make deploy/testnet

# Verify deployment
solana program show SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr --url https://testnet.fogo.io
```

## Mainnet Deployment

### Pre-Deployment Checklist

Before deploying to mainnet, ensure:

- [ ] Code has been thoroughly tested on devnet/testnet
- [ ] Security audit completed (if applicable)
- [ ] All tests pass (`make test`)
- [ ] Code formatted and linted (`make fmt lint`)
- [ ] Sufficient tokens for deployment (~5-10 tokens recommended)
- [ ] Program keypair is secure and backed up
- [ ] Deployment plan approved by stakeholders

### Configure for Mainnet

```bash
# Set Solana CLI to mainnet
solana config set --url https://api.mainnet.fogo.io
solana config set --keypair ~/.config/fogo/mainnet-deployer.json

# Verify configuration
solana config get
solana cluster-version

# Check balance (ensure sufficient tokens)
solana balance
```

### Secure Deployment Process

```bash
# Deploy to mainnet (with confirmation prompt)
make deploy/mainnet

# This will:
# 1. Build the program
# 2. Show a warning about mainnet deployment
# 3. Ask for confirmation
# 4. Deploy if confirmed
# 5. Verify deployment

# Alternative: Manual deployment with extra verification
make build
make verify-build

# Final deployment command
solana program deploy \
    --url https://api.mainnet.fogo.io \
    --program-id .keys/SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr.json \
    target/deploy/spl_stake_pool.so \
    --with-compute-unit-price 1000

# Verify successful deployment
solana program show SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr --url https://api.mainnet.fogo.io
```

### Post-Deployment Verification

```bash
# Verify program deployment
solana program show SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr

# Test basic functionality (if safe)
./spl-stake-pool list-all

# Monitor program usage
solana logs SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr
```

## Program Upgrades

### Upgrade Process

```bash
# Build new version
make build

# Upgrade existing program
make upgrade CLUSTER=mainnet
# OR
make upgrade CLUSTER=devnet
# OR
make upgrade CLUSTER=localnet

# Verify upgrade
solana program show SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr
```

### Upgrade Authority Management

```bash
# Check current upgrade authority
solana program show SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr | grep "Upgrade Authority"

# Change upgrade authority (if needed)
solana program set-upgrade-authority \
    SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr \
    --new-upgrade-authority <NEW_AUTHORITY_PUBKEY>

# Make program immutable (permanent - cannot be undone!)
solana program set-upgrade-authority \
    SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr \
    --final
```

## Client Application Deployment

### TypeScript SDK

The SDK is published to npm and doesn't require separate deployment:

```bash
# Build and test SDK
cd packages/sdk
pnpm build
pnpm test

# Publish to npm (maintainers only)
pnpm publish
```

### Web Application


```bash
pnpm build

# Deploy to static hosting
# The 'out' directory contains the built site

# Example deployment platforms:
# - Vercel: vercel --prod
# - Netlify: netlify deploy --prod --dir out
# - AWS S3: aws s3 sync out/ s3://your-bucket/
# - GitHub Pages: Push 'out' contents to gh-pages branch
```

#### Environment Configuration

Create production environment files:

```bash
NEXT_PUBLIC_RPC_ENDPOINT=https://api.mainnet.fogo.io
NEXT_PUBLIC_STAKE_POOL_ADDRESS=SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr
NEXT_PUBLIC_SESSIONS_PROGRAM_ID=Your_Sessions_Program_ID
```

### CLI Tool Distribution

```bash
# Build release binary
make build/cli

# Create distribution package
tar -czf spl-stake-pool-cli.tar.gz -C target/release spl-stake-pool

# Or create platform-specific builds
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-apple-darwin
```

## Monitoring and Maintenance

### Program Monitoring

```bash
# Monitor program logs
solana logs SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr --commitment confirmed

# Check program account info
solana program show SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr

# Monitor program usage
solana program dump SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr program-dump.so
```

### Health Checks

```bash
# Basic health check script
#!/bin/bash
PROGRAM_ID="SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr"
RPC_URL="https://testnet.fogo.io"

# Check if program exists
if solana program show $PROGRAM_ID --url $RPC_URL > /dev/null 2>&1; then
    echo "✅ Program is deployed and accessible"
else
    echo "❌ Program not found or inaccessible"
    exit 1
fi

# Check if any stake pools exist
POOLS=$(./spl-stake-pool list-all --url $RPC_URL --output json 2>/dev/null)
if [ $? -eq 0 ] && [ "$(echo $POOLS | jq length)" -gt 0 ]; then
    echo "✅ Stake pools are operational"
else
    echo "⚠️  No stake pools found or error accessing pools"
fi
```

## Troubleshooting

### Common Deployment Issues

#### Insufficient Funds

```bash
# Error: insufficient funds for deployment
solana balance
# Solution: Add more tokens
solana airdrop 5  # For devnet/testnet
# For mainnet: transfer tokens from another account
```

#### Program Account Already Exists

```bash
# Error: program account already exists
# Solution: Use upgrade instead of deploy
make upgrade CLUSTER=devnet
```

#### Invalid Program Keypair

```bash
# Error: invalid program keypair
# Solution: Verify keypair matches expected program ID
solana-keygen pubkey .keys/SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr.json
# Should output: SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr
```

#### Network Connection Issues

```bash
# Check network connectivity
solana cluster-version --url <RPC_URL>

# Try different RPC endpoints
# Mainnet alternatives:
# - https://testnet.fogo.io
# - https://mainnet.fogo.io
```

#### Program Size Limits

```bash
# Check program size
ls -lh target/deploy/spl_stake_pool.so

# If too large, optimize build
cargo build-sbf --release
```

### Rollback Procedures

If an upgrade fails or causes issues:

```bash
# Deploy previous version (if available)
solana program deploy \
    --program-id SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr \
    target/deploy/spl_stake_pool_v_previous.so

# Or restore from backup
solana program deploy \
    --program-id SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr \
    backup/spl_stake_pool_known_good.so
```

## Security Considerations

### Deployment Security

1. **Keypair Security**: Store program keypairs securely
2. **Multi-signature**: Use multi-sig for upgrade authority on mainnet
3. **Gradual Rollout**: Test thoroughly on devnet before mainnet
4. **Monitoring**: Set up alerts for program activity
5. **Backup Plans**: Have rollback procedures ready

### Access Control

```bash
# Limit upgrade authority to multi-sig
solana program set-upgrade-authority \
    SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr \
    --new-upgrade-authority <MULTISIG_PUBKEY>

# Eventually make immutable for maximum security
solana program set-upgrade-authority \
    SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr \
    --final
```

This deployment guide covers all aspects of deploying Fogo Stake Pool across different networks. For development setup, see the [Development Guide](./development.md).
