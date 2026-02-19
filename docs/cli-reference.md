# Fogo Stake Pool CLI Reference

Complete reference for the `fogo-stake-pool` command-line tool.

## Installation

```bash
cargo install fogo-stake-pool
```

### Build from Source

```bash
git clone https://github.com/Tempest-Finance/fogo-stake-pool.git
cd fogo-stake-pool
make build/cli
```

The binary is written to `./target/release/fogo-stake-pool`.

## Global Options

All commands support these options:

### Network & Config

| Flag | Description |
|------|-------------|
| `--url <URL>` | JSON RPC URL for the cluster (default: from Solana CLI config) |
| `-C, --config <PATH>` | Solana CLI configuration file path |
| `--program-id <PROGRAM-ID>` | Stake pool program id (default: auto-detected by network) |

**RPC endpoints:**

```
--url https://mainnet.fogo.io       # Mainnet
--url https://testnet.fogo.io       # Testnet
--url http://localhost:8899         # Local validator
```

### Authentication

| Flag | Description |
|------|-------------|
| `--manager <KEYPAIR>` | Stake pool manager keypair |
| `--staker <KEYPAIR>` | Stake pool staker keypair |
| `--funding-authority <KEYPAIR>` | Funding authority for deposits/withdrawals |
| `--token-owner <KEYPAIR>` | Owner of pool token accounts |
| `--fee-payer <KEYPAIR>` | Transaction fee payer |

Keypair formats: file path (`~/.config/solana/id.json`), `ASK` (interactive prompt), or `usb://ledger`.

### Behavior

| Flag | Description |
|------|-------------|
| `-v, --verbose` | Show additional information |
| `--output <FORMAT>` | Output format: `json`, `json-compact` |
| `--dry-run` | Simulate transaction without executing |
| `--no-update` | Skip automatic pool updates |
| `--with-compute-unit-price <PRICE>` | Compute unit price in micro-lamports |
| `--with-compute-unit-limit <LIMIT>` | Compute unit limit (or `DEFAULT` for 200k/instruction) |

### Squads Multisig

| Flag | Description |
|------|-------------|
| `--squads-multisig <PUBKEY>` | Create a Squads v3 proposal instead of executing directly |
| `--squads-auto-approve` | Auto-approve the Squads proposal after creating it |

## Pool Management

### create-pool

Create a new stake pool.

```bash
fogo-stake-pool create-pool \
  --epoch-fee-numerator 3 \
  --epoch-fee-denominator 100 \
  --withdrawal-fee-numerator 5 \
  --withdrawal-fee-denominator 1000 \
  --deposit-fee-numerator 0 \
  --deposit-fee-denominator 1 \
  --referral-fee 10 \
  --max-validators 100
```

**Required arguments:**

| Flag | Description |
|------|-------------|
| `--epoch-fee-numerator/denominator` | Epoch fee fraction |
| `--withdrawal-fee-numerator/denominator` | Withdrawal fee fraction |
| `--deposit-fee-numerator/denominator` | Deposit fee fraction |
| `--referral-fee <PERCENTAGE>` | Referral fee percentage (0–100) |
| `--max-validators <COUNT>` | Maximum validators in the pool |

**Optional arguments:**

| Flag | Description |
|------|-------------|
| `--pool-keypair <KEYPAIR>` | Stake pool account keypair |
| `--validator-list-keypair <KEYPAIR>` | Validator list account keypair |
| `--mint-keypair <KEYPAIR>` | Pool token mint keypair |
| `--reserve-keypair <KEYPAIR>` | Reserve stake account keypair |
| `--deposit-authority <KEYPAIR>` | Custom deposit authority (restricts deposits) |
| `--unsafe-fees` | Allow zero fees |
| `--with-token-metadata` | Create token metadata on pool creation |
| `--token-name <NAME>` | Token name for metadata |
| `--token-symbol <SYMBOL>` | Token symbol for metadata |
| `--token-uri <URI>` | Token metadata URI |

### set-manager

Change the pool manager. Must be signed by the current manager.

```bash
fogo-stake-pool set-manager <POOL_ADDRESS> \
  --new-manager <PUBKEY> \
  --new-fee-receiver <PUBKEY>
```

### set-staker

Change the pool staker. Must be signed by the manager or current staker.

```bash
fogo-stake-pool set-staker <POOL_ADDRESS> --new-staker <PUBKEY>
```

### set-fee

Update pool fees. Must be signed by the manager.

```bash
fogo-stake-pool set-fee <POOL_ADDRESS> <FEE_TYPE> \
  --fee-numerator <NUM> \
  --fee-denominator <NUM>
```

Fee types: `epoch`, `stake-deposit`, `sol-deposit`, `stake-withdrawal`, `sol-withdrawal`

### set-referral-fee

Update referral fee percentage. Must be signed by the manager.

```bash
fogo-stake-pool set-referral-fee <POOL_ADDRESS> <FEE_TYPE> \
  --fee-percentage <PERCENTAGE>
```

Fee types: `stake`, `sol`

### set-funding-authority

Update a funding authority. Must be signed by the manager.

```bash
fogo-stake-pool set-funding-authority <POOL_ADDRESS> <AUTHORITY_TYPE> \
  --new-authority <PUBKEY|none>
```

Authority types: `stake-deposit`, `sol-deposit`, `sol-withdraw`

Pass `--new-authority none` to make the authority permissionless.

## Validator Management

### add-validator

Add a validator to the pool. Must be signed by the staker.

```bash
fogo-stake-pool add-validator <POOL_ADDRESS> <VALIDATOR_VOTE_ACCOUNT> [--seed <SEED>]
```

### remove-validator

Remove a validator from the pool. Must be signed by the staker.

```bash
fogo-stake-pool remove-validator <POOL_ADDRESS> <VALIDATOR_VOTE_ACCOUNT>
```

### increase-validator-stake

Move stake from the reserve to a validator. Must be signed by the staker.

```bash
fogo-stake-pool increase-validator-stake <POOL_ADDRESS> <VALIDATOR_VOTE_ACCOUNT> \
  --lamports <AMOUNT> [--transient-stake-seed <SEED>]
```

### decrease-validator-stake

Move stake from a validator back to the reserve. Must be signed by the staker.

```bash
fogo-stake-pool decrease-validator-stake <POOL_ADDRESS> <VALIDATOR_VOTE_ACCOUNT> \
  --lamports <AMOUNT> [--transient-stake-seed <SEED>]
```

### set-preferred-validator

Set the preferred validator for deposits or withdrawals. Must be signed by the staker.

```bash
# Set preferred deposit validator
fogo-stake-pool set-preferred-validator <POOL_ADDRESS> deposit \
  --validator-vote-account <VOTE_ACCOUNT>

# Remove preferred withdraw validator
fogo-stake-pool set-preferred-validator <POOL_ADDRESS> withdraw
```

## User Operations

### deposit-stake

Deposit an active stake account into the pool in exchange for pool tokens.

```bash
fogo-stake-pool deposit-stake <POOL_ADDRESS> <STAKE_ACCOUNT> \
  [--token-receiver <TOKEN_ACCOUNT>] \
  [--referrer <TOKEN_ACCOUNT>]
```

### deposit-all-stake

Deposit all active stake accounts delegated to pool validators.

```bash
fogo-stake-pool deposit-all-stake <POOL_ADDRESS> \
  [--token-receiver <TOKEN_ACCOUNT>] \
  [--referrer <TOKEN_ACCOUNT>]
```

### deposit-sol

Deposit SOL into the pool's reserve in exchange for pool tokens.

```bash
fogo-stake-pool deposit-sol <POOL_ADDRESS> <AMOUNT_IN_SOL> \
  [--token-receiver <TOKEN_ACCOUNT>] \
  [--referrer <TOKEN_ACCOUNT>] \
  [--from <SOURCE_ACCOUNT>]
```

### withdraw-stake

Withdraw stake from the pool by burning pool tokens.

```bash
fogo-stake-pool withdraw-stake <POOL_ADDRESS> <POOL_TOKEN_AMOUNT> \
  [--vote-account <VALIDATOR_VOTE_ACCOUNT>] \
  [--stake-receiver <STAKE_ACCOUNT>] \
  [--pool-account <POOL_TOKEN_ACCOUNT>]
```

### withdraw-sol

Withdraw SOL from the pool's reserve by burning pool tokens.

```bash
fogo-stake-pool withdraw-sol <POOL_ADDRESS> <POOL_TOKEN_AMOUNT> \
  [--sol-receiver <SOL_ACCOUNT>] \
  [--pool-account <POOL_TOKEN_ACCOUNT>]
```

## Information

### list

Show all stake accounts managed by the pool.

```bash
fogo-stake-pool list <POOL_ADDRESS>
```

### list-all

List all stake pools on the network.

```bash
fogo-stake-pool list-all
```

## Maintenance

### update

Update all validator balances after rewards are distributed.

```bash
fogo-stake-pool update <POOL_ADDRESS> [--no-merge] [--force]
```

| Flag | Description |
|------|-------------|
| `--no-merge` | Don't merge transient stakes |
| `--force` | Force update even if recently updated |

## Token Metadata

### create-token-metadata

```bash
fogo-stake-pool create-token-metadata <POOL_ADDRESS> \
  --token-name <NAME> --token-symbol <SYMBOL> --token-uri <URI>
```

### update-token-metadata

```bash
fogo-stake-pool update-token-metadata <POOL_ADDRESS> \
  --token-name <NAME> --token-symbol <SYMBOL> --token-uri <URI>
```

## Troubleshooting

| Problem | Solution |
|---------|----------|
| Transaction timeout | Use `--with-compute-unit-limit DEFAULT` |
| Account not found | Verify correct network (`--url`) |
| Permission denied | Check that the right keypair is specified |
| Pool balances stale | Run `fogo-stake-pool update <POOL>` first |
| Stake account not active | Wait 1–2 epochs for activation |

Enable verbose output for debugging:

```bash
fogo-stake-pool --verbose list <POOL_ADDRESS>
```

Test without executing:

```bash
fogo-stake-pool --dry-run deposit-sol <POOL_ADDRESS> 1.0
```
