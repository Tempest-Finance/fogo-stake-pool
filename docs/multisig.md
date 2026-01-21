# Squads Multisig Guide

This guide explains how to manage the Fogo Stake Pool using [Squads v3](https://backup.v3.squads.so/) multisig.

## Table of Contents

- [Setup](#setup)
- [Program Upgrades](#program-upgrades)
- [Stake Pool Management](#stake-pool-management)
- [Troubleshooting](#troubleshooting)
- [Reference](#reference)

## Setup

### Configure Squads for Fogo Network

Before using Squads with Fogo, configure the RPC endpoint:

1. Go to [Squads v3](https://backup.v3.squads.so/)
2. Click **Settings** (gear icon)
3. Set **RPC URL** to:
   - Mainnet: `https://mainnet.fogo.io`
   - Testnet: `https://testnet.fogo.io`
4. Save settings

> **Note**: Squads v3 works with Fogo because Fogo is 100% Solana-compatible at the RPC level.

### Prerequisites

- Access to [Squads v3](https://backup.v3.squads.so/)
- Wallet connected as a multisig member
- For upgrades: `solana-verify` CLI and Docker
- Sufficient FOGO for transaction fees

---

## Program Upgrades

### Overview

Program upgrades follow this workflow:

1. Build verifiable program binary
2. Write binary to buffer account
3. Transfer buffer authority to Squads
4. Create upgrade proposal
5. Collect approvals
6. Execute upgrade
7. Upload verification metadata

### Step 1: Build Verifiable Program

```bash
# Ensure Docker is running
docker ps

# Build the program
solana-verify build --library-name spl_stake_pool

# Verify build hash
solana-verify get-executable-hash target/deploy/spl_stake_pool.so
```

### Step 2: Write to Buffer

```bash
solana program write-buffer \
  --url https://mainnet.fogo.io \
  target/deploy/spl_stake_pool.so
```

Save the buffer address from output:

```
Buffer: BuFfEr111111111111111111111111111111111111
```

Verify buffer hash matches local build:

```bash
solana-verify get-buffer-hash \
  -u https://mainnet.fogo.io \
  --buffer-address <BUFFER_ADDRESS>
```

### Step 3: Transfer Buffer Authority

```bash
solana program set-buffer-authority \
  --url https://mainnet.fogo.io \
  <BUFFER_ADDRESS> \
  --new-buffer-authority <SQUADS_VAULT_ADDRESS>
```

### Step 4: Create Upgrade Proposal

1. Open [Squads v3](https://backup.v3.squads.so/#/programs/)
2. Connect wallet and select your multisig
3. Go to **Programs** in sidebar
4. Enter program ID `SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr`
5. Enter the buffer address and buffer refund address
6. Click **Create upgrade**

### Step 5: Collect Approvals

Share proposal link with other signers. Each member should:

1. Verify buffer hash independently
2. Review transaction details
3. Click **Approve**

### Step 6: Execute Upgrade

Once threshold is reached:

1. Click **Execute**
2. Verify upgrade:
   ```bash
   solana-verify get-program-hash \
     -u https://mainnet.fogo.io \
     SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr
   ```

### Step 7: Upload Verification

```bash
solana-verify verify-from-repo \
  -u https://mainnet.fogo.io \
  --program-id SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr \
  --library-name spl_stake_pool \
  --mount-path program \
  https://github.com/Tempest-Finance/fogo-stake-pool
```

---

## Stake Pool Management

For stake pool operations that require multisig approval (like fee changes), the pool's manager authority must be set to the Squads vault address.

### Transfer Manager to Multisig

To transfer the stake pool manager authority to a Squads multisig vault:

```bash
fogo-stake-pool set-manager SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr --new-manager-pubkey 2GUNgc8kGvJhD3iBYf4fkH9cjofvAbdgxE6iwon8sN6v --url https://mainnet.fogo.io --sign-only
```

> **Important**: Once transferred to the multisig, all manager operations will require multisig approval. Ensure you have access to the multisig before transferring.

### Generate Transaction for Squads

Once the manager is a multisig, use the `--sign-only` flag with `--multisig-signer` to generate transactions for Squads import:

```bash
fogo-stake-pool set-fee \
  <POOL_ADDRESS> \
  epoch \
  5 \
  100 \
  --url https://mainnet.fogo.io \
  --sign-only \
  --multisig-signer <SQUADS_VAULT_ADDRESS> \
  --no-update
```

This outputs a base58-encoded transaction string. Copy the output and:

1. Open [Squads v3](https://backup.v3.squads.so/)
2. Select your multisig
3. Click **Import Transaction**
4. Paste the transaction string
5. Review and submit for approval

### Fee Types Reference

Available fee types for `set-fee` command:

- `epoch` - Fee on staking rewards
- `stake-deposit` - Fee on stake deposits
- `sol-deposit` - Fee on SOL deposits
- `stake-withdrawal` - Fee on stake withdrawals
- `sol-withdrawal` - Fee on SOL withdrawals

### Direct Execution (Single Signer)

If the manager is a single keypair (not multisig):

```bash
fogo-stake-pool set-fee \
  <POOL_ADDRESS> \
  epoch \
  5 \
  100 \
  --url https://mainnet.fogo.io
```

---

## Troubleshooting

### RPC Connection Failed

Ensure Squads is configured with correct Fogo RPC:

- Settings → RPC URL → `https://mainnet.fogo.io`

### Transaction Simulation Failed

Common causes:

- Incorrect account ordering
- Missing signer
- Insufficient funds
- Wrong program ID

Check transaction details carefully before submitting.

### Proposal Not Visible

- Ensure all members use same RPC settings
- Refresh the page
- Check wallet is connected to correct multisig

### Buffer Authority Transfer Failed

Verify you're the current buffer authority:

```bash
solana program show --url https://mainnet.fogo.io --buffers
```

### Upgrade Execution Failed

Check:

- Sufficient FOGO for fees
- Buffer account still exists
- Program is upgradeable (not immutable)

```bash
solana program show \
  --url https://mainnet.fogo.io \
  SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr
```

---

## Reference

### Key Addresses

| Item         | Address                                       |
| ------------ | --------------------------------------------- |
| Program ID   | `SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr` |
| Squads Vault | `<YOUR_SQUADS_VAULT_ADDRESS>`                 |

### RPC Endpoints

| Network | URL                       |
| ------- | ------------------------- |
| Mainnet | `https://mainnet.fogo.io` |
| Testnet | `https://testnet.fogo.io` |

### Useful Commands

```bash
# Check program info
solana program show -u https://mainnet.fogo.io SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr

# List buffers
solana program show -u https://mainnet.fogo.io --buffers

# Close unused buffer (reclaim rent)
solana program close -u https://mainnet.fogo.io <BUFFER_ADDRESS>

# Get program hash
solana-verify get-program-hash -u https://mainnet.fogo.io SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr

# Get buffer hash
solana-verify get-buffer-hash -u https://mainnet.fogo.io --buffer-address <BUFFER_ADDRESS>
```

### Security Best Practices

1. **Always use verifiable builds** for program upgrades
2. **Verify buffer hash** before approving upgrade proposals
3. **Document all changes** in proposal descriptions
4. **Test on testnet** before mainnet operations
5. **Review transaction simulation** before executing

---

## See Also

- [verification.md](./verification.md) - Program verification guide
- [Squads v3](https://backup.v3.squads.so/) - Multisig interface
- [Fogo Documentation](https://docs.fogo.io) - Fogo network docs
