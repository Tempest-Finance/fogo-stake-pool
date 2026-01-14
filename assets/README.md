# iFOGO Token Assets

Token metadata and assets for Ignition Staked FOGO (iFOGO).

## Files

| File             | Description                  |
| ---------------- | ---------------------------- |
| `ifogo.svg`      | iFOGO token logo (128x128)   |
| `metadata.json`  | Token metadata               |
| `paymaster.toml` | Fogo paymaster configuration |

## IPFS URLs

**Image:**

```
https://tomato-firm-turkey-182.mypinata.cloud/ipfs/bafkreige6nrqie3qooknxjyscgxk6xoleb5ofa2sap3c3hhrrb6azhc76q
```

**Metadata:**

```
https://tomato-firm-turkey-182.mypinata.cloud/ipfs/bafkreihhxpiuh6i5uox4wfiiywos7bw4ptveft4ph5vgytc5fbw3svvwve
```

## Create Token Metadata

To create on-chain metadata, use the stake pool CLI:

```bash
fogo-stake-pool create-token-metadata \
  --stake-pool ign1zuR3YsvLVsEu8WzsyazBA8EVWUxPPHKnhqhoSTB \
  --name "Ignition Staked FOGO" \
  --symbol "iFOGO" \
  --uri "https://tomato-firm-turkey-182.mypinata.cloud/ipfs/bafkreihhxpiuh6i5uox4wfiiywos7bw4ptveft4ph5vgytc5fbw3svvwve"
```

## Update Token Metadata

To update on-chain metadata, use the stake pool CLI:

```bash
fogo-stake-pool update-token-metadata \
  --stake-pool ign1zuR3YsvLVsEu8WzsyazBA8EVWUxPPHKnhqhoSTB \
  --name "Ignition Staked FOGO" \
  --symbol "iFOGO" \
  --uri "https://tomato-firm-turkey-182.mypinata.cloud/ipfs/bafkreihhxpiuh6i5uox4wfiiywos7bw4ptveft4ph5vgytc5fbw3svvwve"
```
