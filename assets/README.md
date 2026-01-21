# iFOGO Token Assets

Token metadata and assets for Ignition Staked FOGO (iFOGO).

## Files

| File             | Description                  |
| ---------------- | ---------------------------- |
| `ifogo.svg`      | iFOGO token logo (128x128)   |
| `metadata.json`  | Token metadata               |
| `paymaster.toml` | Fogo paymaster configuration |

## URLs

**Image:**

```
https://raw.githubusercontent.com/Tempest-Finance/fogo-stake-pool/refs/heads/main/assets/ifogo.svg
```

**Metadata:**

```
https://raw.githubusercontent.com/Tempest-Finance/fogo-stake-pool/refs/heads/main/assets/metadata.json
```

## Create Token Metadata

To create on-chain metadata, use the stake pool CLI:

```bash
fogo-stake-pool create-token-metadata \
  --stake-pool ign1zuR3YsvLVsEu8WzsyazBA8EVWUxPPHKnhqhoSTB \
  --name "Ignition Staked FOGO" \
  --symbol "iFOGO" \
  --uri "https://raw.githubusercontent.com/Tempest-Finance/fogo-stake-pool/refs/heads/main/assets/metadata.json"
```

## Update Token Metadata

To update on-chain metadata, use the stake pool CLI:

```bash
fogo-stake-pool update-token-metadata \
  --stake-pool ign1zuR3YsvLVsEu8WzsyazBA8EVWUxPPHKnhqhoSTB \
  --uri "https://raw.githubusercontent.com/Tempest-Finance/fogo-stake-pool/refs/heads/main/assets/metadata.json"
```
