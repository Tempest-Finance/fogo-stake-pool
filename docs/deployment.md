# Fogo Stake Pool Deployment Guide

This guide provides comprehensive instructions for deploying Fogo Stake Pool programs and applications across different FOGO networks and environments.

## Overview

The Fogo Stake Pool project supports deployment to:

- **Testnet**: FOGO's testing network
- **Mainnet**: FOGO's production network

## Prerequisites

### Required Tools

```bash
# FOGO CLI (Solana-compatible)
solana --version

# Rust toolchain
rustc --version

# Built program artifacts
ls target/deploy/spl_stake_pool.so
```

### Network Configuration

```bash
# Check current CLI configuration
solana config get

# Configure for specific network
solana config set --url https://testnet.fogo.io     # Testnet
solana config set --url https://mainnet.fogo.io     # Mainnet
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

## Testnet Deployment

### Configure for Testnet

```bash
# Set CLI to FOGO testnet
solana config set --url https://testnet.fogo.io
solana config set --keypair ~/.config/fogo/testnet-deployer.json

# Verify connection
solana cluster-version
solana epoch-info
```

### Get Testnet Tokens

```bash
# Airdrop tokens for deployment costs
solana airdrop 5

# Verify balance (deployment typically costs ~2-3 tokens)
solana balance
```

### Deploy to Testnet

```bash
# Method 1: Using Makefile (recommended)
make deploy CLUSTER=testnet

# Method 2: Manual deployment
make build
solana program deploy \
    --url https://testnet.fogo.io \
    --program-id .keys/SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr.json \
    target/deploy/spl_stake_pool.so

# Verify deployment
solana program show SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr --url https://testnet.fogo.io
```

### Testnet Testing

```bash
# Create stake pool on testnet
./fogo-stake-pool create-pool \
    --url https://testnet.fogo.io \
    --epoch-fee-numerator 3 \
    --epoch-fee-denominator 100 \
    --withdrawal-fee-numerator 5 \
    --withdrawal-fee-denominator 1000 \
    --deposit-fee-numerator 0 \
    --deposit-fee-denominator 1 \
    --referral-fee 10 \
    --max-validators 50

# List all pools on testnet
./fogo-stake-pool list-all --url https://testnet.fogo.io
```

## Mainnet Deployment

### Pre-Deployment Checklist

Before deploying to mainnet, ensure:

- [ ] Code has been thoroughly tested on testnet
- [ ] Security audit completed (if applicable)
- [ ] All tests pass (`make test`)
- [ ] Code formatted and linted (`make fmt lint`)
- [ ] Sufficient tokens for deployment (~5-10 tokens recommended)
- [ ] Program keypair is secure and backed up
- [ ] Deployment plan approved by stakeholders

### Configure for Mainnet

```bash
# Set CLI to FOGO mainnet
solana config set --url https://mainnet.fogo.io
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
make deploy CLUSTER=mainnet

# This will:
# 1. Build the program
# 2. Show a warning about mainnet deployment
# 3. Ask for confirmation
# 4. Deploy if confirmed
# 5. Verify deployment

# Alternative: Manual deployment with extra verification
make build

# Final deployment command
solana program deploy \
    --url https://mainnet.fogo.io \
    --program-id .keys/SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr.json \
    target/deploy/spl_stake_pool.so \
    --with-compute-unit-price 1000

# Verify successful deployment
solana program show SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr --url https://mainnet.fogo.io
```

### Post-Deployment Verification

```bash
# Verify program deployment
solana program show SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr

# Test basic functionality (if safe)
./fogo-stake-pool list-all

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
make upgrade CLUSTER=testnet

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
cd clients/js
pnpm build
pnpm test

# Publish to npm (maintainers only)
pnpm publish
```

### CLI Tool Distribution

```bash
# Build release binary
make build/cli

# Create distribution package
tar -czf fogo-stake-pool-cli.tar.gz -C target/release fogo-stake-pool

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
POOLS=$(./fogo-stake-pool list-all --url $RPC_URL --output json 2>/dev/null)
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
solana airdrop 5  # For testnet
# For mainnet: transfer tokens from another account
```

#### Program Account Already Exists

```bash
# Error: program account already exists
# Solution: Use upgrade instead of deploy
make upgrade CLUSTER=testnet
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

# FOGO RPC endpoints:
# - https://testnet.fogo.io (Testnet)
# - https://mainnet.fogo.io (Mainnet)
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
3. **Gradual Rollout**: Test thoroughly on testnet before mainnet
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

This deployment guide covers all aspects of deploying Fogo Stake Pool across different networks.
For development setup, see the [Development Guide](./development.md).
