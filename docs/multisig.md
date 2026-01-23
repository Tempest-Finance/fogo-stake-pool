# Squads v3 Multisig Integration

Manage the Fogo Stake Pool with [Squads v3](https://backup.v3.squads.so/) multisig.

## Setup

### Configure Squads UI for Fogo

1. Go to [Squads v3](https://backup.v3.squads.so/)
2. Click **Settings** → **RPC URL**
3. Enter: `https://mainnet.fogo.io` (or `https://testnet.fogo.io`)
4. Save

### Required Information

- **Multisig Address**: Found in Squads UI → Your Squad → Settings
- **Vault Address**: Found in Squads UI → Settings → Vault Address
- **Member Keypair**: A keypair that is a member of the multisig

---

## CLI Usage

### Global Options

| Argument                | Description                              |
| ----------------------- | ---------------------------------------- |
| `--squads-multisig`     | Multisig address. Enables proposal mode. |
| `--squads-auto-approve` | Auto-approve after creating proposal     |

### Basic Example

```bash
fogo-stake-pool set-staker <POOL> <NEW_STAKER> \
  --squads-multisig <MULTISIG> \
  --fee-payer <MEMBER_KEYPAIR> \
  --url https://mainnet.fogo.io
```

When `--squads-multisig` is set, the CLI creates a proposal instead of executing directly.

---

## Transfer Manager to Multisig

The `set_manager` instruction requires both current and new manager to sign. Use this two-step process:

### Step 1: Transfer to Member Keypair

```bash
fogo-stake-pool set-manager <POOL> \
  --new-manager <MEMBER_KEYPAIR> \
  --manager <CURRENT_MANAGER_KEYPAIR> \
  --url https://mainnet.fogo.io
```

### Step 2: Transfer to Vault via Squads

```bash
fogo-stake-pool set-manager <POOL> \
  --new-manager <VAULT_PUBKEY> \
  --manager <MEMBER_KEYPAIR> \
  --fee-payer <MEMBER_KEYPAIR> \
  --squads-multisig <MULTISIG> \
  --url https://mainnet.fogo.io
```

After members approve in Squads UI, execute the proposal.

---

## Common Operations

### Set Fee

```bash
fogo-stake-pool set-fee <POOL> epoch <NUMERATOR> <DENOMINATOR> \
  --squads-multisig <MULTISIG> \
  --fee-payer <MEMBER_KEYPAIR> \
  --url https://mainnet.fogo.io
```

Fee types: `epoch`, `stake-deposit`, `sol-deposit`, `stake-withdrawal`, `sol-withdrawal`

### Set Staker

```bash
fogo-stake-pool set-staker <POOL> <NEW_STAKER_PUBKEY> \
  --squads-multisig <MULTISIG> \
  --fee-payer <MEMBER_KEYPAIR> \
  --url https://mainnet.fogo.io
```

### Set Funding Authority

```bash
fogo-stake-pool set-funding-authority <POOL> <TYPE> <AUTHORITY> \
  --squads-multisig <MULTISIG> \
  --fee-payer <MEMBER_KEYPAIR> \
  --url https://mainnet.fogo.io
```

Types: `stake-deposit`, `sol-deposit`, `sol-withdraw`

### Change Fee Receiver

```bash
fogo-stake-pool set-manager <POOL> \
  --new-fee-receiver <TOKEN_ACCOUNT> \
  --squads-multisig <MULTISIG> \
  --fee-payer <MEMBER_KEYPAIR> \
  --url https://mainnet.fogo.io
```

Note: Fee receiver must be a token account for the pool mint.

---

## Program Upgrades

1. **Build**: `solana-verify build --library-name spl_stake_pool`
2. **Buffer**: `solana program write-buffer --url https://mainnet.fogo.io target/deploy/spl_stake_pool.so`
3. **Transfer authority**: `solana program set-buffer-authority <BUFFER> --new-buffer-authority <VAULT>`
4. **Propose**: In Squads UI → Programs → Enter program ID and buffer → Create upgrade
5. **Approve & Execute**: Members approve, then execute
6. **Verify**: `solana-verify verify-from-repo ...`

---

## Troubleshooting

### "new_manager must be the vault"

Use the two-step transfer process described above.

### "Proposer is not a member"

Your fee-payer keypair must be a multisig member.

### Proposal not visible

Verify RPC is set to `https://mainnet.fogo.io` in Squads settings.

### Simulation failed

- Try without `--no-update` flag
- Check you have sufficient SOL for fees
- Verify you're using the correct authority (manager vs staker)

Use `--dry-run` to debug before executing.

---

## Reference

| Item               | Value                                         |
| ------------------ | --------------------------------------------- |
| Stake Pool Program | `SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr` |
| Squads v3 Program  | `SMPLecH534NA9acpos4G6x7uf3LWbCAwZQE9e8ZekMu` |
| Mainnet RPC        | `https://mainnet.fogo.io`                     |
| Testnet RPC        | `https://testnet.fogo.io`                     |

---

## See Also

- [CLI Reference](./cli-reference.md)
- [Program Verification](./verification.md)
- [Squads v3 Documentation](https://docs.squads.so/)
