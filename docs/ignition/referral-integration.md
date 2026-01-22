# Referral Integration Guide

This guide explains how to integrate referral tracking with the Ignition Stake Pool.

## Overview

Partners can track their users' staking activity by adding a memo to transactions. This allows attribution of deposits and withdrawals to specific referral sources.

## Memo Format

Add a memo instruction to your transaction with the following format:

```
ref:<referral_code>
```

**Example:** `ref:xlabstest`

### Combined Memos

Multiple memo types can be combined using `&` separator in any order:

| Prefix    | Description                              |
| --------- | ---------------------------------------- |
| `ref:`    | Referral code for partner attribution    |
| `direct:` | Wallet address for direct stake tracking |

```
ref:<referral_code>&direct:<wallet_address>
```

**Example:** `ref:xlabstest&direct:ABC123walletAddress`

The backend parses memos like this:

```typescript
const memos = memo.split('&')
const memoDirectStake = memos.find(m => m.startsWith('direct:'))
const memoReferralStake = memos.find(m => m.startsWith('ref:'))
```

## Implementation

### Using SPL Memo Program

Program ID: `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr`

#### JavaScript/TypeScript

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

// Simple referral
const memoIx = createMemoInstruction('ref:xlabstest')

// Combined memo
const combinedMemoIx = createMemoInstruction('ref:xlabstest&direct:UserWalletAddress')

transaction.add(memoIx)
// ... add stake pool instruction
```

#### Rust

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

// Usage
let memo_ix = create_memo_instruction("ref:xlabstest");
```

## Tracking API

Query referral statistics using the Ignition API:

**Base URL:** `https://api.ignition.svt.one`

### Endpoints

| Endpoint                               | Description                                 |
| -------------------------------------- | ------------------------------------------- |
| `GET /referral/referrer/{code}`        | Stake info for referrer (by code)           |
| `GET /referral/referrer/{code}/epochs` | Stake info by epochs for referrer (by code) |

## Transaction Structure

A typical transaction with referral tracking:

1. **Memo instruction** (optional) - `ref:<referral_code>`
2. **Stake pool instruction** - DepositSol, DepositWsolWithSession, WithdrawSol, WithdrawWsolWithSession, WithdrawStake, WithdrawStakeWithSession

The memo instruction should be placed before the stake pool instruction in the transaction.
