'use client'

import { BN } from '@coral-xyz/anchor'
import { useConnection } from '@fogo/sessions-sdk-react'
import { getStakePoolAccount, StakePool } from '@ignitionfi/spl-stake-pool'
import { PublicKey, StakeProgram } from '@solana/web3.js'
import { useCallback, useState } from 'react'
import { STAKE_POOL_ADDRESS } from '../constants.ts'

export function PoolInfo() {
  const { state, fetchData } = usePoolInfo()

  return (
    <div>
      <hr />
      <h4>Pool Info</h4>
      <button type="button" onClick={fetchData}>
        Fetch pool data
      </button>
      {state
        && (
          <div>
            <div>
              poolAddress:
              {state.poolAddress}
            </div>
            <div>
              poolMint:
              {state.poolMint}
            </div>
            <div>
              reserveStakeAccount:
              {state.reserveStakeAccount}
            </div>
            <div>
              poolTokenSupply:
              {state.poolTokenSupply}
            </div>
            <div>
              totalLamports:
              {state.totalLamports}
            </div>
            <div>
              exchangeRate:
              {state.exchangeRate}
            </div>
            <div>
              reserveStakeBalance:
              {state.reserveStakeBalance}
            </div>
            <div>
              minRentBalance:
              {state.minRentBalance}
            </div>
            <div>
              fee epoch:
              {state.fees.epochFee}
            </div>
            <div>
              fee withdrawal:
              {state.fees.withdrawalFee}
            </div>
          </div>
        )}
      <hr />
    </div>
  )
}

function usePoolInfo() {
  const [state, setState] = useState<StakePoolData | null>(null)
  const connection = useConnection()

  const fetchData = useCallback(async () => {
    const poolAddress = STAKE_POOL_ADDRESS
    try {
      const stakePoolAccount = await getStakePoolAccount(
        connection,
        new PublicKey(poolAddress),
      )
      const stakePool = stakePoolAccount?.account.data as StakePool

      const [reserveStake, minRentBalance] = await Promise.all([
        connection.getAccountInfo(
          stakePool.reserveStake,
        ),
        connection.getMinimumBalanceForRentExemption(
          StakeProgram.space,
        ),
      ])

      const reserveStakeBalance = reserveStake?.lamports ?? 0

      const data = {
        poolAddress,
        ...handleStakePoolData(stakePool),
        reserveStakeBalance,
        minRentBalance,
        connectionLost: false,
      }

      console.log(data)

      setState(data)
    } catch (error) {
      console.error(`Unable to update pool info: ${error}`)
      setState(null)
    }
  }, [connection])

  return { state, fetchData }
}

type StakePoolFees = {
  epochFee: number
  stakeDepositFee: number
  solDepositFee: number
  withdrawalFee: number
  solWithdrawalFee: number
  nextStakeWithdrawalFee: number
  nextSolWithdrawalFee: number
  nextEpochFee: number
  solReferralFee: number
  stakeReferralFee: number
}

type StakePoolData = {
  poolAddress: string
  poolMint: string
  reserveStakeAccount: string
  poolTokenSupply: number
  totalLamports: number
  exchangeRate: number
  fees: StakePoolFees
  reserveStakeBalance: number
  minRentBalance: number
  connectionLost: boolean
}

function divideBnToNumber(numerator: BN, denominator: BN): number {
  if (denominator.isZero()) {
    return 0
  }
  const quotient = numerator.div(denominator).toNumber()
  const rem = numerator.umod(denominator)
  const gcd = rem.gcd(denominator)
  return quotient + rem.div(gcd).toNumber() / denominator.div(gcd).toNumber()
}

function handleStakePoolData(data: StakePool) {
  const fees = {
    epochFee: divideBnToNumber(
      data.epochFee.numerator,
      data.epochFee.denominator,
    ),
    stakeDepositFee: divideBnToNumber(
      data.stakeDepositFee.numerator,
      data.stakeDepositFee.denominator,
    ),
    withdrawalFee: divideBnToNumber(
      data.stakeWithdrawalFee.numerator,
      data.stakeWithdrawalFee.denominator,
    ),
    solWithdrawalFee: divideBnToNumber(
      data.solWithdrawalFee.numerator,
      data.solWithdrawalFee.denominator,
    ),
    solDepositFee: divideBnToNumber(
      data.solDepositFee.numerator,
      data.solDepositFee.denominator,
    ),
    nextStakeWithdrawalFee: 0,
    nextSolWithdrawalFee: 0,
    nextEpochFee: 0,
    solReferralFee: data.solReferralFee ?? 0,
    stakeReferralFee: data.stakeReferralFee ?? 0,
  }
  if (data.nextEpochFee?.numerator) {
    fees.nextEpochFee = divideBnToNumber(
      data.nextEpochFee.numerator,
      data.nextEpochFee.denominator,
    )
  }
  if (data.nextStakeWithdrawalFee?.numerator) {
    fees.nextStakeWithdrawalFee = divideBnToNumber(
      data.nextStakeWithdrawalFee.numerator,
      data.nextStakeWithdrawalFee.denominator,
    )
  }
  if (data.nextSolWithdrawalFee?.numerator) {
    fees.nextSolWithdrawalFee = divideBnToNumber(
      data.nextSolWithdrawalFee.numerator,
      data.nextSolWithdrawalFee.denominator,
    )
  }

  let exchangeRate = 0
  exchangeRate = data.poolTokenSupply.isZero() || data.totalLamports.isZero() ? 1 : divideBnToNumber(data.poolTokenSupply, data.totalLamports)

  return {
    poolMint: data.poolMint.toBase58(),
    reserveStakeAccount: data.reserveStake.toBase58(),
    poolTokenSupply: data.poolTokenSupply.toNumber(),
    totalLamports: data.totalLamports.toNumber(),
    exchangeRate,
    fees,
  }
}
