# Fogo Stake Pool

A liquid staking protocol for the FOGO blockchain, forked from [SPL Stake Pool](https://github.com/solana-program/stake-pool).

## Program ID

`SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr`

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
