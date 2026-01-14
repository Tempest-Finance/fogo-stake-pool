# Fogo Stake Pool Development Guide

This guide provides comprehensive instructions for setting up a development environment, building, testing, and contributing to the Fogo Stake Pool project.

## Development Environment Setup

### Prerequisites

Before you begin development, ensure you have the following tools installed:

#### Required Tools

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Solana CLI
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash

# Verify installation
solana --version
```

#### Verification

Verify your installations:

```bash
# Check Rust version
rustc --version  # Should show 1.86.0
cargo --version

# Check Solana CLI
solana --version 

# Check Node.js and pnpm
node --version
pnpm --version
```

### Project Setup

#### Clone and Initial Setup

```bash
# Clone the repository
git clone https://github.com/Tempest-Finance/fogo-stake-pool.git
cd Tempest-Finance/fogo-stake-pool

# Install dependencies
pnpm install

# Verify solana version compatibility
pnpm solana:check
```

#### Directory Structure

```
stake-pool/
├── program/               # Program
├── clients/
│   ├── cli/               # CLI tool
│   └── py/                # Python client
├── packages/
│   ├── sdk/               # TypeScript SDK
├── scripts/               # Build and utility scripts
├── Makefile               # Build automation
├── Cargo.toml             # Rust workspace
├── package.json           # Node.js workspace
├── pnpm-workspace.yaml    # pnpm configuration
└── rust-toolchain.toml    # Rust version specification
```

## Build System

### Make Commands (Recommended)

The project includes a comprehensive Makefile for common operations:

```bash
# Show all available commands
make help

# Build commands
make build              # Build program
make build/cli          # Build CLI tool

# Testing
make test              # All tests
make test-integration  # Integration tests
make test/unit         # Unit tests

# Code quality
make fmt               # Format with nightly rustfmt
make lint              # Run clippy linter
make check             # Run cargo check

# Deployment
make deploy/localnet   # Deploy to local validator
make deploy/testnet    # Deploy to devnet
make deploy/mainnet    # Deploy to mainnet (with confirmation)

# Cleanup
make clean             # Remove build artifacts
```

### Package Manager Commands

For client libraries and JavaScript packages:

```bash
# Workspace commands (from root)
pnpm programs:build    # Build all programs
pnpm programs:test     # Test all programs
pnpm programs:format   # Format rust code
pnpm programs:lint     # Lint rust code

# Client library commands
pnpm clients:js:test   # Test TypeScript SDK
pnpm clients:js:format # Format JS/TS code
pnpm clients:js:lint   # Lint JS/TS code
pnpm clients:py:test   # Test Python client
pnpm clients:cli:test  # Test CLI

# Individual package commands
pnpm sdk build         # Build TypeScript SDK
pnpm sdk test          # Test TypeScript SDK
```

## Development Workflow

### 1. Program Development

#### Building the Program

```bash
# Standard build
make build
# OR
cargo build-sbf

# Clean build
make clean && make build

# Check for errors without building
make check
```

#### Testing Strategy

```bash
# Unit tests (fast, no validator required)
make test
# OR
cargo test --lib

# Integration tests (requires validator)
make test/integration
# OR
cargo test -p spl-stake-pool

# Run specific test
cargo test test_initialize_stake_pool

# Test with output
cargo test -- --nocapture
```

#### Code Quality

```bash
# Format code
make fmt

# Lint with clippy
make lint

# Fix clippy warnings
cargo clippy --fix --allow-dirty

# Spell check
pnpm rust:spellcheck

# Security audit
pnpm rust:audit
```

### 2. CLI Development

The CLI is located in `clients/cli/`:

```bash
# Build CLI
make build/cli
# OR
cargo build --bin fogo-stake-pool --release

# Test CLI
pnpm clients:cli:test
# OR
cargo test -p fogo-stake-pool-cli

# Run CLI locally
./fogo-stake-pool --help

# Debug mode
cargo run --bin fogo-stake-pool -- --help
```

#### CLI Testing

```bash
# Unit tests
cargo test -p fogo-stake-pool-cli

# Integration tests with local validator
./scripts/client/setup-test-validator.sh
./fogo-stake-pool create-pool --help
```

### 3. TypeScript SDK Development

Located in `clients/js/`:

```bash
# Build SDK
pnpm sdk build
# OR
cd clients/js && pnpm build

# Development mode (watch)
cd clients/js && pnpm dev

# Test SDK
pnpm sdk test
# OR
cd clients/js && pnpm test

# Lint and format
cd clients/js && pnpm lint
cd clients/js && pnpm format
```

#### SDK Testing

```bash
# Run tests with coverage
cd clients/js && pnpm test --coverage

# Run specific test file
cd clients/js && pnpm test src/instruction.test.ts

# Watch mode for development
cd clients/js && pnpm test --watch
```

## Testing Setup

### Local Validator Setup

For integration testing, you need a local FOGO validator:

```bash
# Start local validator
solana-test-validator

# In another terminal, configure CLI
solana config set --url localhost
solana config set --keypair ~/.config/solana/id.json

# Airdrop tokens for testing
solana airdrop 10

# Deploy program to local validator
make deploy/localnet
```

### Test Environment Configuration

Create test configuration files:

```bash
# Create test keypairs
mkdir -p .keys
solana-keygen new --outfile .keys/test-pool.json
solana-keygen new --outfile .keys/test-validator.json

# Set environment variables for tests
export STAKE_POOL_PROGRAM_ID=SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr
```

### Running Comprehensive Tests

```bash
# Setup test validator with program deployed
./scripts/client/setup-test-validator.sh

# Run all tests
make test

# Run tests for specific components
cargo test -p spl-stake-pool          # Program tests
cargo test -p fogo-stake-pool-cli      # CLI tests
pnpm sdk test                         # SDK tests
pnpm clients:py:test                  # Python tests
```

## Debugging and Development Tools

### Logging and Debugging

```bash
# Enable detailed logging
export RUST_LOG=solana_runtime::system_instruction_processor=trace,solana_runtime::message_processor=debug,solana_bpf_loader=debug

# Program logs
solana logs SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr

# Transaction debugging
solana confirm <TRANSACTION_SIGNATURE> -v

# Account inspection
solana account <ACCOUNT_ADDRESS>
```

### Development Utilities

```bash
# Check program size
ls -lh target/deploy/spl_stake_pool.so

# Verify program deployment
solana program show SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr

# Benchmark program performance
cargo bench

# Memory usage analysis
cargo tree --duplicates
```

## Performance and Optimization

### Build Optimization

```bash
# Release build with optimizations
cargo build-sbf --release

# Minimize program size
cargo build-sbf --release -- --strip-debug

# Check binary size
ls -lh target/deploy/spl_stake_pool.so
```

### Testing Performance

```bash
# Benchmark tests
cargo bench

# Memory usage profiling
cargo test --release -- --test-threads=1 --nocapture
```

## Contributing Guidelines

### Code Style

```bash
# Format code before committing
make fmt

# Run full quality check
make lint && make check && make test
```

### Commit Guidelines

Follow conventional commits:

```bash
git commit -m "feat(program): add new instruction for validator management"
git commit -m "fix(cli): resolve account parsing issue"
git commit -m "docs(sdk): update API examples"
git commit -m "test(integration): add comprehensive pool creation tests"
```

### Pull Request Process

1. **Branch Naming**: Use descriptive names
   ```bash
   git checkout -b feat/add-validator-rebalancing
   git checkout -b fix/cli-account-parsing
   git checkout -b docs/update-sdk-examples
   ```

2. **Testing Requirements**:
   ```bash
   # Ensure all tests pass
   make test
   pnpm sdk test
   pnpm clients:py:test
   ```

3. **Code Quality Checks**:
   ```bash
   make fmt lint check
   pnpm clients:js:lint
   pnpm rust:spellcheck
   ```

4. **Documentation**: Update relevant docs in `docs/` directory

## Troubleshooting

### Debug Techniques

#### Program Debugging

```rust
// Add debug prints (remove before production)
msg!("Debug: account balance = {}", account.lamports);

// Use conditional compilation
#[cfg(feature = "debug")]
msg!("Debug info: {:?}", data);
```

#### Transaction Analysis

```bash
# Analyze failed transaction
solana confirm <TX_SIGNATURE> -v

# Check program logs
solana logs SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr --commitment confirmed
```

#### Account Inspection

```bash
# Inspect account data
solana account <ACCOUNT_ADDRESS> --output json

# Check account owner
solana account <ACCOUNT_ADDRESS> | grep Owner
```

### Performance Debugging

```bash
# Check compute unit usage
# Add to tests to monitor compute consumption
cargo test test_name -- --nocapture

# Profile memory usage
valgrind --tool=massif ./target/debug/program_test

# Analyze binary size
cargo bloat --release --crates
```

## Advanced Development Topics

### Custom Program Builds

```bash
# Build with specific features
cargo build-sbf --features "debug,testing"

# Cross-compilation for different targets
cargo build-sbf --target sbf-solana-solana
```

### Continuous Integration

The project includes GitHub Actions workflows:

```yaml
# .github/workflows/rust.yml
# - Rust formatting check
# - Clippy linting
# - Unit and integration tests
# - Security audit
```

## Resources and References

### Documentation
- [FOGO Docs](https://docs.fogo.io/)
- [Anchor Framework](https://www.anchor-lang.com/)

### Tools
- [Blockchain Explorer](https://explorer.fogo.io/)
- [Block Explorer](https://explorer.fogo.io/)
- [FogoScan](https://fogoscan.com/)

### Community
- [Discord](https://discord.gg/FogoChain)
- [Stack Overflow `fogo` tag](https://stackoverflow.com/questions/tagged/fogo)

This development guide provides everything needed to contribute effectively to the Fogo Stake Pool project. For deployment procedures, see the [Deployment Guide](./deployment.md).
