# Referral Integration Guide

This guide explains how to integrate referral tracking with the Ignition Stake Pool.

## Memo Format

Add a memo instruction to your transaction using the following prefixes:

| Prefix    | Description                           |
| --------- | ------------------------------------- |
| `ref:`    | Referral code for partner attribution |
| `direct:` | Vote account address for direct stake |

Multiple prefixes can be combined using `&` separator.

**Examples:**

- `ref:xlabstest`
- `ref:xlabstest&direct:VoteAddr1234567890abcdef`

## Implementation

Program ID: `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr`

### JavaScript/TypeScript

```typescript
import { PublicKey, TransactionInstruction } from '@solana/web3.js'

const MEMO_PROGRAM_ID = new PublicKey('MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr')

function createMemoInstruction(memo: string): TransactionInstruction {
  return new TransactionInstruction({
    keys: [],
    programId: MEMO_PROGRAM_ID,
    data: Buffer.from(memo, 'utf-8'),
  })
}

const memoIx = createMemoInstruction('ref:xlabstest')
transaction.add(memoIx)
// ... add stake pool instruction
```

### Rust

```rust
use solana_sdk::{instruction::Instruction, pubkey};

const MEMO_PROGRAM_ID: Pubkey = pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

fn create_memo_instruction(memo: &str) -> Instruction {
    Instruction {
        program_id: MEMO_PROGRAM_ID,
        accounts: vec![],
        data: memo.as_bytes().to_vec(),
    }
}

let memo_ix = create_memo_instruction("ref:xlabstest");
```

## Transaction Structure

1. **Memo instruction** (optional) - `ref:<referral_code>`
2. **Stake pool instruction** - DepositSol, DepositWsolWithSession, WithdrawSol, WithdrawWsolWithSession, WithdrawStake, WithdrawStakeWithSession

> **Note:** The memo instruction must be placed before the stake pool instruction.

## Tracking API

**Base URL:** `https://api.ignition.svt.one`

| Endpoint                               | Description                                 |
| -------------------------------------- | ------------------------------------------- |
| `GET /referral/referrer/{code}`        | Stake info for referrer (by code)           |
| `GET /referral/referrer/{code}/epochs` | Stake info by epochs for referrer (by code) |
