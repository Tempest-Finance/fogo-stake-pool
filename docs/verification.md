# Program Verification Guide

This guide explains how to verify the Fogo Stake Pool program on the Fogo Explorer using reproducible builds.

## Overview

Program verification proves that the deployed on-chain bytecode matches the public source code. This is achieved through:

1. **Reproducible builds** - Using Docker to create deterministic binaries
2. **On-chain verification metadata** - Storing build info in a Program Derived Address (PDA)
3. **Explorer integration** - Explorers query the PDA to display verification status

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) (must be running)
- [Rust](https://rustup.rs/) with Cargo
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) v2.3.0+
- Program upgrade authority keypair

## Installation

Install the `solana-verify` CLI tool:

```bash
cargo install solana-verify
```

Verify installation:

```bash
solana-verify --version
```

## Step 1: Build with Verifiable Build

The verifiable build uses a Docker container to ensure deterministic compilation:

```bash
# Make sure Docker is running
docker ps

# Build the program (this takes ~2-3 minutes)
solana-verify build --library-name spl_stake_pool
```

This creates `target/deploy/spl_stake_pool.so` with a deterministic hash.

### Verify the build hash

```bash
solana-verify get-executable-hash target/deploy/spl_stake_pool.so
```

Example output:
```
43dfafc0a84e41b901fc26ed447be317a9c620588113e9bc42d7d6277c73c39e
```

## Step 2: Deploy the Program

Deploy the verifiable build to Fogo mainnet:

```bash
# Deploy to mainnet
solana program deploy \
  --url https://mainnet.fogo.io \
  --program-id .keys/SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr.json \
  target/deploy/spl_stake_pool.so
```

For testnet:
```bash
solana program deploy \
  --url https://testnet.fogo.io \
  --program-id .keys/SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr.json \
  target/deploy/spl_stake_pool.so
```

### Verify deployment hash matches

```bash
# Get the on-chain program hash
solana-verify get-program-hash \
  -u https://mainnet.fogo.io \
  SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr

# Compare with local build
solana-verify get-executable-hash target/deploy/spl_stake_pool.so
```

Both hashes must match before proceeding.

## Step 3: Upload Verification Metadata

This step builds, verifies, and uploads the verification PDA on-chain:

```bash
solana-verify verify-from-repo \
  -u https://mainnet.fogo.io \
  --program-id SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr \
  --library-name spl_stake_pool \
  --mount-path program \
  https://github.com/Tempest-Finance/fogo-stake-pool
```

### Options explained

| Option | Description |
|--------|-------------|
| `-u` | RPC endpoint (mainnet or testnet) |
| `--program-id` | The deployed program address |
| `--library-name` | Crate name from `Cargo.toml` (`spl_stake_pool`) |
| `--mount-path` | Relative path to program directory (`program`) |
| `--commit-hash` | (Optional) Specific commit to verify against |
| `-k, --keypair` | (Optional) Path to keypair for uploading PDA |
| `-y, --skip-prompt` | Skip confirmation prompt |

### For a specific commit

```bash
solana-verify verify-from-repo \
  -u https://mainnet.fogo.io \
  --program-id SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr \
  --library-name spl_stake_pool \
  --mount-path program \
  --commit-hash abc123def456 \
  https://github.com/Tempest-Finance/fogo-stake-pool
```

## Step 4: (Optional) Submit to Remote Verifier

For additional verification through OtterSec's verification registry:

```bash
# Get your public key
UPLOADER=$(solana-keygen pubkey ~/.config/solana/id.json)

# Submit verification job
solana-verify remote submit-job \
  --program-id SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr \
  --uploader $UPLOADER
```

Check job status:
```bash
solana-verify remote get-status \
  --program-id SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr
```

## Verification Status

After successful verification, the program will show as "Verified" on:

- [Fogoscan](https://fogoscan.com)
- [Fogo Explorer](https://explorer.fogo.io)

## Troubleshooting

### Hash mismatch between local and on-chain

This occurs when the deployed program was built with a different toolchain:

```
Local hash:    43dfafc0a84e41b901fc26ed447be317a9c620588113e9bc42d7d6277c73c39e
On-chain hash: e1e817c3bce0d02552877c877fb258d996ca291beae0357512209d6c3cacf61d
```

**Solution**: Redeploy using `solana-verify build` instead of `make build` or `cargo build-sbf`.

### Docker not running

```
docker: failed to connect to the docker API
```

**Solution**: Start Docker Desktop or the Docker daemon.

### Wrong library name

```
error: no library named 'stake_pool' found
```

**Solution**: Use the exact crate name from `program/Cargo.toml`:
```bash
solana-verify build --library-name spl_stake_pool
```

### Build takes too long

The Docker build can take 2-5 minutes depending on cache state. Set resource limits:

```bash
export SVB_DOCKER_MEMORY_LIMIT=4g
export SVB_DOCKER_CPU_LIMIT=4
solana-verify build --library-name spl_stake_pool
```

## Important Notes

### Why `make build` hashes don't match `solana-verify build`

| Build Method | Environment | Deterministic |
|--------------|-------------|---------------|
| `make build` | Local toolchain | No |
| `cargo build-sbf` | Local toolchain | No |
| `solana-verify build` | Docker container | Yes |

Even minor differences in Rust version, LLVM, or linker settings produce completely different binaries. The Docker-based build ensures everyone gets the exact same output.

### Verification PDA

The verification metadata is stored in a PDA owned by the Otter Verify program. It contains:

- Repository URL
- Commit hash
- Build hash
- Uploader public key

Explorers query this PDA to display verification status.

### Re-verification after upgrades

After each program upgrade, you must:

1. Build with `solana-verify build`
2. Deploy the new binary
3. Run `verify-from-repo` again to update the PDA

## Quick Reference

```bash
# Full verification workflow
solana-verify build --library-name spl_stake_pool

solana program deploy \
  --url https://mainnet.fogo.io \
  --program-id .keys/SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr.json \
  target/deploy/spl_stake_pool.so

solana-verify verify-from-repo \
  -u https://mainnet.fogo.io \
  --program-id SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr \
  --library-name spl_stake_pool \
  --mount-path program \
  https://github.com/Tempest-Finance/fogo-stake-pool
```

## References

- [Solana Verifiable Build](https://github.com/Ellipsis-Labs/solana-verifiable-build)
- [Fogo Documentation](https://docs.fogo.io)
- [OtterSec Verification Registry](https://verify.osec.io)

## See Also

- [multisig.md](./multisig.md) - Squads multisig guide for upgrades and pool management
