# Fogo Stake Pool CLI Reference

This document provides complete reference documentation for the Fogo Stake Pool command-line interface (CLI) tool.

## Installation

### Build from Source

```bash
# Clone the repository
git clone https://github.com/fogo-labs/fogo-program-library.git
cd solana-program-library/stake-pool

# Build the CLI
cargo build --bin spl-stake-pool --release

# The binary will be available at ./spl-stake-pool
```

### Using Make (if available)

```bash
make build/cli
```

### Add to PATH

```bash
# Copy to system bin directory
sudo cp ./spl-stake-pool /usr/local/bin/

# Or create a symlink
ln -sf $(pwd)/target/release/spl-stake-pool /usr/local/bin/spl-stake-pool
```

## Global Options

All commands support these global options:

### Network Configuration

```bash
--url <URL>                    # JSON RPC URL for the cluster
                              # Default: from Solana CLI config
```

**Examples:**
```bash
--url https://api.mainnet.fogo.io    # Mainnet
--url https://api.devnet.fogo.io          # Devnet
--url http://localhost:8899                  # Local validator
```

### Authentication & Authorization

```bash
--manager <KEYPAIR>           # Stake pool manager keypair
--staker <KEYPAIR>            # Stake pool staker keypair
--funding-authority <KEYPAIR> # Funding authority for deposits/withdrawals
--token-owner <KEYPAIR>       # Owner of pool token accounts
--fee-payer <KEYPAIR>         # Transaction fee payer
```

**Keypair Formats:**
- File path: `~/.config/fogo/id.json`
- Ask prompt: `ASK` (will prompt for keypair)
- Hardware wallet: `usb://ledger` (if configured)

### Output & Behavior

```bash
--verbose, -v                 # Show additional information
--output <FORMAT>             # Output format: json, json-compact
--dry-run                     # Simulate transaction without executing
--no-update                   # Skip automatic pool updates
--config <PATH>               # Configuration file path
```

### Compute Budget

```bash
--with-compute-unit-price <PRICE>     # Set compute unit price (micro-lamports)
--with-compute-unit-limit <LIMIT>     # Set compute unit limit or DEFAULT
```

## Pool Management Commands

### create-pool

Creates a new stake pool with the specified configuration.

**Usage:**
```bash
spl-stake-pool create-pool [OPTIONS] <ARGUMENTS>
```

**Required Arguments:**
```bash
--epoch-fee-numerator <NUM>              # Epoch fee numerator
--epoch-fee-denominator <NUM>            # Epoch fee denominator
--withdrawal-fee-numerator <NUM>         # Withdrawal fee numerator
--withdrawal-fee-denominator <NUM>       # Withdrawal fee denominator
--deposit-fee-numerator <NUM>            # Deposit fee numerator
--deposit-fee-denominator <NUM>          # Deposit fee denominator
--referral-fee <PERCENTAGE>              # Referral fee percentage (0-100)
--max-validators <COUNT>                 # Maximum expected validators
```

**Optional Arguments:**
```bash
--pool-keypair <KEYPAIR>                 # Stake pool account keypair
--validator-list-keypair <KEYPAIR>       # Validator list account keypair
--mint-keypair <KEYPAIR>                 # Pool token mint keypair
--reserve-keypair <KEYPAIR>              # Reserve stake account keypair
--deposit-authority <KEYPAIR>            # Custom deposit authority
--unsafe-fees                            # Allow zero fees (not recommended)
--with-token-metadata                    # Create token metadata
--token-name <NAME>                      # Token name for metadata
--token-symbol <SYMBOL>                  # Token symbol for metadata
--token-uri <URI>                        # Token metadata URI
```

**Example:**
```bash
spl-stake-pool create-pool \
  --epoch-fee-numerator 3 \
  --epoch-fee-denominator 100 \
  --withdrawal-fee-numerator 5 \
  --withdrawal-fee-denominator 1000 \
  --deposit-fee-numerator 0 \
  --deposit-fee-denominator 1 \
  --referral-fee 10 \
  --max-validators 100 \
  --pool-keypair ./pool.json \
  --with-token-metadata \
  --token-name "My Stake Pool" \
  --token-symbol "MSP"
```

### set-manager

Changes the pool manager. Must be signed by the current manager.

**Usage:**
```bash
spl-stake-pool set-manager <POOL_ADDRESS> \
  --new-manager <PUBKEY> \
  --new-fee-receiver <PUBKEY>
```

**Example:**
```bash
spl-stake-pool set-manager So11111111111111111111111111111111111111112 \
  --new-manager 9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM \
  --new-fee-receiver HLmqeL62xR1QoZ1HKKbXRrdN1p3phKpxRMb2VVopvBBz
```

### set-staker

Changes the pool staker. Must be signed by manager or current staker.

**Usage:**
```bash
spl-stake-pool set-staker <POOL_ADDRESS> \
  --new-staker <PUBKEY>
```

### set-fee

Updates pool fees. Must be signed by the manager.

**Usage:**
```bash
spl-stake-pool set-fee <POOL_ADDRESS> <FEE_TYPE> \
  --fee-numerator <NUM> \
  --fee-denominator <NUM>
```

**Fee Types:**
- `epoch` - Management fee on rewards
- `stake-deposit` - Fee on stake deposits
- `sol-deposit` - Fee on tokens deposits
- `stake-withdrawal` - Fee on stake withdrawals
- `sol-withdrawal` - Fee on tokens withdrawals

**Example:**
```bash
# Update withdrawal fee to 0.3%
spl-stake-pool set-fee So11111111111111111111111111111111111111112 stake-withdrawal \
  --fee-numerator 3 \
  --fee-denominator 1000
```

### set-referral-fee

Updates referral fees. Must be signed by the manager.

**Usage:**
```bash
spl-stake-pool set-referral-fee <POOL_ADDRESS> <FEE_TYPE> \
  --fee-percentage <PERCENTAGE>
```

**Fee Types:**
- `stake` - Referral fee for stake deposits
- `sol` - Referral fee for tokens deposits

### set-funding-authority

Updates funding authorities. Must be signed by the manager.

**Usage:**
```bash
spl-stake-pool set-funding-authority <POOL_ADDRESS> <AUTHORITY_TYPE> \
  --new-authority <PUBKEY|none>
```

**Authority Types:**
- `stake-deposit` - Stake deposit authority
- `sol-deposit` - tokens deposit authority
- `sol-withdraw` - tokens withdraw authority

**Example:**
```bash
# Remove tokens deposit authority (make permissionless)
spl-stake-pool set-funding-authority So11111111111111111111111111111111111111112 sol-deposit \
  --new-authority none
```

## Validator Management Commands

### add-validator

Adds a validator to the stake pool. Must be signed by the staker.

**Usage:**
```bash
spl-stake-pool add-validator <POOL_ADDRESS> <VALIDATOR_VOTE_ACCOUNT> \
  [--seed <SEED>]
```

**Example:**
```bash
spl-stake-pool add-validator So11111111111111111111111111111111111111112 \
  Vote111111111111111111111111111111111111111 \
  --seed 42
```

### remove-validator

Removes a validator from the stake pool. Must be signed by the staker.

**Usage:**
```bash
spl-stake-pool remove-validator <POOL_ADDRESS> <VALIDATOR_VOTE_ACCOUNT>
```

### increase-validator-stake

Increases stake on a validator from the reserve. Must be signed by the staker.

**Usage:**
```bash
spl-stake-pool increase-validator-stake <POOL_ADDRESS> <VALIDATOR_VOTE_ACCOUNT> \
  --lamports <AMOUNT> \
  [--transient-stake-seed <SEED>]
```

**Example:**
```bash
# Increase stake by 10 tokens
spl-stake-pool increase-validator-stake So11111111111111111111111111111111111111112 \
  Vote111111111111111111111111111111111111111 \
  --lamports 10000000000
```

### decrease-validator-stake

Decreases stake on a validator to the reserve. Must be signed by the staker.

**Usage:**
```bash
spl-stake-pool decrease-validator-stake <POOL_ADDRESS> <VALIDATOR_VOTE_ACCOUNT> \
  --lamports <AMOUNT> \
  [--transient-stake-seed <SEED>]
```

### set-preferred-validator

Sets preferred validator for deposits or withdrawals. Must be signed by the staker.

**Usage:**
```bash
spl-stake-pool set-preferred-validator <POOL_ADDRESS> <VALIDATOR_TYPE> \
  [--validator-vote-account <VOTE_ACCOUNT>]
```

**Validator Types:**
- `deposit` - Preferred validator for deposits
- `withdraw` - Preferred validator for withdrawals

**Examples:**
```bash
# Set preferred deposit validator
spl-stake-pool set-preferred-validator So11111111111111111111111111111111111111112 deposit \
  --validator-vote-account Vote111111111111111111111111111111111111111

# Remove preferred withdraw validator
spl-stake-pool set-preferred-validator So11111111111111111111111111111111111111112 withdraw
```

## User Operations

### deposit-stake

Deposits a stake account into the pool in exchange for pool tokens.

**Usage:**
```bash
spl-stake-pool deposit-stake <POOL_ADDRESS> <STAKE_ACCOUNT> \
  [--token-receiver <TOKEN_ACCOUNT>] \
  [--referrer <TOKEN_ACCOUNT>] \
  [--pool-account <POOL_TOKEN_ACCOUNT>]
```

**Example:**
```bash
spl-stake-pool deposit-stake So11111111111111111111111111111111111111112 \
  Stake11111111111111111111111111111111111111 \
  --token-receiver 7GgPYjS5Dza89wV6FpZ23kUJRG5vbQ1GM25ezspYFSoE
```

### deposit-all-stake

Deposits all stake accounts owned by the user into the pool.

**Usage:**
```bash
spl-stake-pool deposit-all-stake <POOL_ADDRESS> \
  [--token-receiver <TOKEN_ACCOUNT>] \
  [--referrer <TOKEN_ACCOUNT>]
```

### deposit-sol

Deposits tokens directly into the pool's reserve in exchange for pool tokens.

**Usage:**
```bash
spl-stake-pool deposit-sol <POOL_ADDRESS> <AMOUNT> \
  [--token-receiver <TOKEN_ACCOUNT>] \
  [--referrer <TOKEN_ACCOUNT>] \
  [--from <SOURCE_ACCOUNT>]
```

**Example:**
```bash
# Deposit 5 tokens
spl-stake-pool deposit-sol So11111111111111111111111111111111111111112 5.0 \
  --token-receiver 7GgPYjS5Dza89wV6FpZ23kUJRG5vbQ1GM25ezspYFSoE
```

### withdraw-stake

Withdraws stake from the pool by burning pool tokens.

**Usage:**
```bash
spl-stake-pool withdraw-stake <POOL_ADDRESS> <POOL_TOKEN_AMOUNT> \
  [--vote-account <VALIDATOR_VOTE_ACCOUNT>] \
  [--stake-receiver <STAKE_ACCOUNT>] \
  [--pool-account <POOL_TOKEN_ACCOUNT>]
```

**Example:**
```bash
# Withdraw 100 pool tokens worth of stake
spl-stake-pool withdraw-stake So11111111111111111111111111111111111111112 100.0 \
  --vote-account Vote111111111111111111111111111111111111111
```

### withdraw-sol

Withdraws tokens directly from the pool's reserve by burning pool tokens.

**Usage:**
```bash
spl-stake-pool withdraw-sol <POOL_ADDRESS> <POOL_TOKEN_AMOUNT> \
  [--sol-receiver <SOL_ACCOUNT>] \
  [--pool-account <POOL_TOKEN_ACCOUNT>]
```

**Example:**
```bash
# Withdraw tokens equivalent to 50 pool tokens
spl-stake-pool withdraw-sol So11111111111111111111111111111111111111112 50.0
```

## Information Commands

### list

Shows all stake accounts managed by the pool.

**Usage:**
```bash
spl-stake-pool list <POOL_ADDRESS>
```

**Output includes:**
- Validator vote accounts
- Stake account addresses
- Active stake amounts
- Transient stake amounts
- Last update epochs

### list-all

Lists information about all stake pools.

**Usage:**
```bash
spl-stake-pool list-all
```

**Output includes:**
- Pool addresses
- Manager and staker addresses
- Total value locked (TVL)
- Pool token supply
- Fee information

## Maintenance Commands

### update

Updates all balances in the pool after validators receive rewards.

**Usage:**
```bash
spl-stake-pool update <POOL_ADDRESS> \
  [--no-merge] \
  [--force]
```

**Options:**
- `--no-merge` - Don't merge transient stakes (for testing)
- `--force` - Force update even if recently updated

**Example:**
```bash
spl-stake-pool update So11111111111111111111111111111111111111112
```

## Token Metadata Commands

### create-token-metadata

Creates metadata for the pool token.

**Usage:**
```bash
spl-stake-pool create-token-metadata <POOL_ADDRESS> \
  --token-name <NAME> \
  --token-symbol <SYMBOL> \
  --token-uri <URI>
```

### update-token-metadata

Updates existing token metadata.

**Usage:**
```bash
spl-stake-pool update-token-metadata <POOL_ADDRESS> \
  --token-name <NAME> \
  --token-symbol <SYMBOL> \
  --token-uri <URI>
```

## Advanced Usage

### Using Custom Program ID

```bash
export SPL_STAKE_POOL_PROGRAM_ID=YourCustomProgramId11111111111111111111111111
spl-stake-pool list-all
```

### Batch Operations

For operations involving multiple transactions, the CLI automatically handles:
- Transaction retry logic
- Proper ordering of dependent operations
- Compute budget management
- Progress reporting

### Error Handling

Common error scenarios and solutions:

**Insufficient Balance:**
```bash
# Error: Fee payer has insufficient balance
# Solution: Add more tokens to fee payer account
solana airdrop 1.0  # On devnet/testnet
```

**Stake Account Not Ready:**
```bash
# Error: Stake account not active
# Solution: Wait for stake account to activate (1-2 epochs)
solana stakes <STAKE_ACCOUNT>  # Check activation status
```

**Pool Needs Update:**
```bash
# Error: Pool balances are stale
# Solution: Update pool first
spl-stake-pool update <POOL_ADDRESS>
```

### Configuration File

Create a configuration file to avoid repeating common options:

```json
{
  "json_rpc_url": "https://api.devnet.fogo.io",
  "keypair_path": "~/.config/fogo/id.json",
  "commitment": "confirmed"
}
```

Use with:
```bash
spl-stake-pool --config ./my-config.json list-all
```

## Integration Examples

### Shell Scripts

```bash
#!/bin/bash
# Example: Automated pool updates

POOL_ADDRESS="So11111111111111111111111111111111111111112"
MANAGER_KEY="~/.config/fogo/manager.json"

# Update pool
spl-stake-pool update $POOL_ADDRESS --manager $MANAGER_KEY

# Check if any validators need rebalancing
spl-stake-pool list $POOL_ADDRESS --output json | \
  jq '.accounts[] | select(.transient_stake_lamports > 0)'
```

### Integration with CI/CD

```bash
# In automated deployment pipeline
spl-stake-pool create-pool \
  --epoch-fee-numerator 3 \
  --epoch-fee-denominator 100 \
  --withdrawal-fee-numerator 5 \
  --withdrawal-fee-denominator 1000 \
  --deposit-fee-numerator 0 \
  --deposit-fee-denominator 1 \
  --referral-fee 10 \
  --max-validators 100 \
  --manager $MANAGER_KEYPAIR \
  --dry-run  # Test first
```

## Troubleshooting

### Common Issues

1. **Transaction Timeout**: Increase compute unit limit or use `--with-compute-unit-limit DEFAULT`
2. **Account Not Found**: Ensure correct network (mainnet/devnet/testnet)
3. **Permission Denied**: Verify correct keypair is specified for the operation
4. **Insufficient Funds**: Check fee payer balance and add more tokens if needed

### Debug Mode

Enable verbose output for troubleshooting:
```bash
spl-stake-pool --verbose list <POOL_ADDRESS>
```

### Dry Run Mode

Test transactions without executing:
```bash
spl-stake-pool --dry-run deposit-sol <POOL_ADDRESS> 1.0
```

This CLI reference provides comprehensive documentation for all Fogo Stake Pool operations. For programmatic integration, see the [API Reference](./api-reference.md#typescript-sdk) and [API Reference](./api-reference.md).
