# Fogo Stake Pool - Ignition Testnet

## Addresses

| Component              | Address                                       |
| ---------------------- | --------------------------------------------- |
| **Program ID**         | `SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr` |
| **Stake Pool**         | `ign1zuR3YsvLVsEu8WzsyazBA8EVWUxPPHKnhqhoSTB` |
| **Pool Token Mint**    | `iFoGoY5nMWpuMJogR7xjUAWDJtygHDF17zREeP4MKuD` |
| **Reserve Stake**      | _Derived at pool creation_                    |
| **Withdraw Authority** | _Derived from pool address_                   |

## Fees

```
Stake Pool Info
===============
Epoch Fee: 49/1000 of epoch rewards
Stake Withdrawal Fee: 15/10000 of withdrawal amount
SOL Withdrawal Fee: 225/10000 of withdrawal amount
Stake Deposit Fee: 0/0 of deposit amount
SOL Deposit Fee: 0/0 of deposit amount
Stake Deposit Referral Fee: 0% of Stake Deposit Fee
SOL Deposit Referral Fee: 0% of SOL Deposit Fee
```

## SDK integration

See [Getting Started](../getting-started.md#using-the-typescript-sdk) for full documentation.

```bash
npm install @ignitionfi/fogo-stake-pool
```

### Example: Pool Info & Exchange Rate

```typescript
import { getStakePoolAccount } from '@ignitionfi/fogo-stake-pool'
import { Connection, PublicKey } from '@solana/web3.js'

const connection = new Connection('https://testnet.fogo.io')
const stakePool = new PublicKey('ign1zuR3YsvLVsEu8WzsyazBA8EVWUxPPHKnhqhoSTB')

const poolInfo = await getStakePoolAccount(connection, stakePool)
const { totalLamports, poolTokenSupply, poolMint } = poolInfo.account.data

// Exchange rate: how much FOGO per 1 tFOGO
const rate = totalLamports.toNumber() / poolTokenSupply.toNumber()
```
