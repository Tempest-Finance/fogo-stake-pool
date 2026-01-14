# Fogo Stake Pool

[![FOGO](https://img.shields.io/badge/FOGO-ff3d00?logo=lightning&logoColor=white&style=for-the-badge)](https://fogo.io)
[![npm](https://img.shields.io/npm/v/@ignitionfi/fogo-stake-pool?logo=npm&logoColor=white&style=for-the-badge)](https://www.npmjs.com/package/@ignitionfi/fogo-stake-pool)
[![CI](https://img.shields.io/github/actions/workflow/status/Tempest-Finance/fogo-stake-pool/main.yml?logo=githubactions&logoColor=white&style=for-the-badge&label=CI)](https://github.com/Tempest-Finance/fogo-stake-pool/actions/workflows/main.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue?style=for-the-badge)](./LICENSE)

A liquid staking protocol for the FOGO blockchain, forked from [SPL Stake Pool](https://github.com/solana-program/stake-pool).

## Program IDs

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

| Directory      | Description                                    |
| -------------- | ---------------------------------------------- |
| `program/`     | On-chain Solana BPF program (Rust)             |
| `clients/cli/` | Command-line interface                         |
| `clients/js/`  | TypeScript SDK (`@ignitionfi/fogo-stake-pool`) |
| `clients/py/`  | Python client library                          |

## Documentation

- [Getting Started](./docs/getting-started.md)
- [API Reference](./docs/api-reference.md)
- [Program Guide](./docs/program-guide.md)
- [Testnet Integration](./docs/testnet-integration.md)

## Audits

- **Fogo Stake Pool audits**: [`audits/`](./audits/)
- **SPL Stake Pool audits**: [solana-labs/security-audits](https://github.com/solana-labs/security-audits#stake-pool)

## License

Apache 2.0
