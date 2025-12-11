# Ignition Stake Pool - FOGO Testnet

## Addresses

| Component              | Address                                        |
| ---------------------- | ---------------------------------------------- |
| **Program ID**         | `SP1s4uFeTAX9jsXXmwyDs1gxYYf7cdDZ8qHUHVxE1yr`  |
| **Stake Pool**         | `PooLqqJBxaeCe7UXdsFZ7G5DqtNLCoAqj25sKGKWoCV`  |
| **Pool Token Mint**    | `5VM3GV1V5G64ozKZLDEfpJCAPGrAMPzBzYENMsF2tmHU` |
| **Reserve Stake**      | `C2ogdtPNHbovAsPL2R5f8F5pRpw8bZYf8negTjSgXzPH` |
| **Withdraw Authority** | `6PzP8dmEp8FxKV6TKhgPy9xoWymGoG8HUx2bgA2qwojt` |

## Fees

```
Epoch Fee: 7/100 of epoch rewards
Stake Withdrawal Fee: 7/100 of withdrawal amount
SOL Withdrawal Fee: 7/100 of withdrawal amount
Stake Deposit Fee: 0/0 of deposit amount
SOL Deposit Fee: 0/0 of deposit amount
```

## SDK integration

```bash
npm install @ignitionfi/spl-stake-pool
```

### Example: Pool Info & Exchange Rate

```typescript
import { getStakePoolAccount } from '@ignitionfi/spl-stake-pool'
import { Connection, PublicKey } from '@solana/web3.js'

const connection = new Connection('https://testnet.fogo.io')
const stakePool = new PublicKey('PooLqqJBxaeCe7UXdsFZ7G5DqtNLCoAqj25sKGKWoCV')

const poolInfo = await getStakePoolAccount(connection, stakePool)
const { totalLamports, poolTokenSupply, poolMint } = poolInfo.account.data

// Exchange rate: how much FOGO per 1 tFOGO
const rate = totalLamports.toNumber() / poolTokenSupply.toNumber()
```
