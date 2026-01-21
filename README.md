# Fogo Stake Pool

[![FOGO](https://img.shields.io/badge/FOGO-grey?logo=lightning&style=for-the-badge)](https://fogo.io)
[![npm](https://img.shields.io/npm/v/@ignitionfi/fogo-stake-pool?logo=npm&logoColor=white&style=for-the-badge)](https://www.npmjs.com/package/@ignitionfi/fogo-stake-pool)
[![CI](https://img.shields.io/github/actions/workflow/status/Tempest-Finance/fogo-stake-pool/main.yml?logo=githubactions&logoColor=white&style=for-the-badge&label=CI)](https://github.com/Tempest-Finance/fogo-stake-pool/actions/workflows/main.yml)

A liquid staking protocol for the FOGO blockchain, forked from [SPL Stake Pool](https://github.com/solana-program/stake-pool).

## Program ID

| Network | Address                                       |
| ------- | --------------------------------------------- |
| Mainnet | `SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr` |
| Testnet | `SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr` |

## Quick Start

```bash
# Build the program
make build

# Run tests
make test

# See all commands
make help
```

## Components

| Directory       | Description                                                              |
| --------------- | ------------------------------------------------------------------------ |
| `program/`      | On-chain Solana BPF program (Rust)                                       |
| `interface/`    | State types for deserializing accounts (lightweight, no processor)       |
| `clients/rust/` | Generated Rust client via [Codama](https://github.com/codama-idl/codama) |
| `clients/cli/`  | Command-line interface for stake pool operations                         |
| `clients/js/`   | TypeScript client (`@ignitionfi/fogo-stake-pool`)                        |
| `clients/py/`   | Python client library                                                    |

## Installation

### TypeScript/JavaScript

```bash
npm install @ignitionfi/fogo-stake-pool
# or
pnpm add @ignitionfi/fogo-stake-pool
```

### Rust

```toml
# For deserializing stake pool accounts (lightweight)
[dependencies]
fogo-stake-pool-interface = "0.1"

# For building transactions with generated instructions
[dependencies]
fogo-stake-pool-client = "0.1"
```

## Documentation

- [Getting Started](./docs/getting-started.md) — First steps with the stake pool
- [CLI Reference](./docs/cli-reference.md) — Command-line tool usage
- [API Reference](./docs/api-reference.md) — SDK and instruction documentation
- [Program Guide](./docs/program-guide.md) — On-chain program architecture
- [Deployment](./docs/deployment.md) — Deploying and upgrading the program
- [Multisig Operations](./docs/multisig.md) — Managing the pool with multisig
- [Testnet Integration](./docs/testnet-integration.md) — Testing on FOGO testnet

## Audits

- **Fogo Stake Pool audits**: [`audits/`](./audits/)
- **SPL Stake Pool audits**: [solana-labs/security-audits](https://github.com/solana-labs/security-audits#stake-pool)

## Development

```bash
# Format code
make fmt

# Run linter
make lint

# Build CLI
make build/cli

# Generate IDL and clients
make generate-clients
```

## License

Apache 2.0
