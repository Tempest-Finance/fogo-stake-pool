# Ignition Stake Pool Documentation

Welcome to the comprehensive documentation for the Ignition Stake Pool program - a FOGO blockchain program for creating and managing pools of staked tokens.

## Overview

The Ignition Stake Pool program allows users to pool their stake accounts and receive liquid tokens in return, enabling better capital efficiency and democratizing access to staking rewards on FOGO.

**Program ID:** `SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr`

## Key Features

- **Liquid Staking**: Convert staked tokens into transferable pool tokens
- **Validator Management**: Efficient management of large validator sets using optimized data structures
- **Fee Protection**: Built-in safeguards against malicious fee increases
- **Multi-Client Support**: Rust CLI, TypeScript SDK, and Python bindings
- **Transient Operations**: Secure handling of deposits and withdrawals through temporary accounts

## Documentation Structure

### Getting Started
- [Quick Start Guide](./getting-started.md) - Set up your development environment and run your first commands
- [Development Setup](./development.md) - Complete development environment configuration, build commands, and testing

### Architecture & Design
- [System Architecture](./architecture.md) - Detailed system architecture and design decisions
- [Program Guide](./program-guide.md) - On-chain program documentation, instructions, accounts, and PDAs

### Client Libraries & Tools
- [CLI Reference](./cli-reference.md) - Complete command-line interface documentation
- [API Reference](./api-reference.md#typescript-sdk) - JavaScript/TypeScript SDK usage with examples

### Operations & Deployment
- [Deployment Guide](./deployment.md) - Deploy to different Fogo networks
- [API Reference](./api-reference.md) - Comprehensive API reference for all components

## Project Components

### Core Program (`program/`)
The main FOGO blockchain program written in Rust that handles all on-chain operations including stake pool creation, deposits, withdrawals, and validator management.

### Command Line Interface (`clients/cli/`)
A comprehensive CLI tool for interacting with stake pools, supporting all program operations from pool creation to complex validator management tasks.

### TypeScript SDK (`packages/sdk/`)
A JavaScript/TypeScript client library providing type-safe interfaces to the stake pool program, suitable for Node.js backends and integration services.

### Python Bindings (`clients/py/`)
Python client library for stake pool operations, useful for data analysis and backend integrations.

## Quick Navigation

| Component | Documentation | Source Code |
|-----------|--------------|-------------|
| FOGO Program | [Program Guide](./program-guide.md) | [`program/`](../program/) |
| CLI Tool | [CLI Reference](./cli-reference.md) | [`clients/cli/`](../clients/cli/) |
| TypeScript SDK | [API Reference](./api-reference.md#typescript-sdk) | [`packages/sdk/`](../packages/sdk/) |
| Python Client | [API Reference](./api-reference.md#python-client) | [`clients/py/`](../clients/py/) |

## Key Concepts

### Stake Pool Mechanics
- **Pool Tokens**: Liquid representation of staked tokens with proportional rewards
- **Validator Lists**: Efficiently managed using big_vec data structure for large validator sets
- **Reserve Account**: Maintains liquidity for immediate withdrawals
- **Transient Accounts**: Temporary accounts for secure deposit/withdrawal operations

### Security Features
- **Fee Protection**: Maximum 3/2 ratio increase per epoch prevents malicious fee changes
- **Minimum Stakes**: 1,000,000 lamports minimum active stake per validator
- **Batch Limits**: Maximum 4 validators per update instruction to respect compute limits
- **PDA Security**: All authority accounts use program-derived addresses

## External Resources

- **FOGO Documentation**: https://docs.fogo.io/

## Contributing

This project follows standard open-source contribution guidelines. See the main repository for detailed contribution instructions, coding standards, and security practices.

## Support & Community

- **GitHub Issues**: Report bugs and request features
- **Discord**: Join the #developers channel for community support

---

*This documentation is maintained alongside the codebase. For the most up-to-date information, always refer to the source code and inline documentation.*
