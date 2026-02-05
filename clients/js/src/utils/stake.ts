import {
  Connection,
  Keypair,
  PublicKey,
  StakeProgram,
  SystemProgram,
  TransactionInstruction,
} from '@solana/web3.js'
import BN from 'bn.js'
import { MINIMUM_ACTIVE_STAKE } from '../constants'

import { getStakePoolProgramId, WithdrawAccount } from '../index'
import {
  Fee,
  StakePool,
  ValidatorList,
  ValidatorListLayout,
  ValidatorStakeInfoStatus,
} from '../layouts'
import { lamportsToSol } from './math'
import { findStakeProgramAddress, findTransientStakeProgramAddress } from './program-address'

export async function getValidatorListAccount(connection: Connection, pubkey: PublicKey) {
  const account = await connection.getAccountInfo(pubkey)
  if (!account) {
    throw new Error('Invalid validator list account')
  }

  return {
    pubkey,
    account: {
      data: ValidatorListLayout.decode(account?.data) as ValidatorList,
      executable: account.executable,
      lamports: account.lamports,
      owner: account.owner,
    },
  }
}

export interface ValidatorAccount {
  type: 'preferred' | 'active' | 'transient'
  voteAddress?: PublicKey | undefined
  stakeAddress: PublicKey
  lamports: BN
}

export interface PrepareWithdrawResult {
  withdrawAccounts: WithdrawAccount[]
  /** Pool tokens that will be withdrawn via delayed unstake */
  delayedAmount: BN
  /** Pool tokens remaining that need instant unstake */
  remainingAmount: BN
}

/** Pre-fetched data to avoid duplicate RPC calls */
export interface PrepareWithdrawPrefetchedData {
  /** Raw validator list account data from getAccountInfo */
  validatorListData: Buffer | null
  /** Rent exemption for stake accounts in lamports */
  minBalanceForRentExemption: number
  /** Minimum stake delegation in lamports */
  stakeMinimumDelegation: number
}

export async function prepareWithdrawAccounts(
  connection: Connection,
  stakePool: StakePool,
  stakePoolAddress: PublicKey,
  amount: BN,
  compareFn?: (a: ValidatorAccount, b: ValidatorAccount) => number,
  skipFee?: boolean,
  allowPartial?: boolean,
  prefetchedData?: PrepareWithdrawPrefetchedData,
): Promise<WithdrawAccount[]>
export async function prepareWithdrawAccounts(
  connection: Connection,
  stakePool: StakePool,
  stakePoolAddress: PublicKey,
  amount: BN,
  compareFn: ((a: ValidatorAccount, b: ValidatorAccount) => number) | undefined,
  skipFee: boolean | undefined,
  allowPartial: true,
  prefetchedData?: PrepareWithdrawPrefetchedData,
): Promise<PrepareWithdrawResult>
export async function prepareWithdrawAccounts(
  connection: Connection,
  stakePool: StakePool,
  stakePoolAddress: PublicKey,
  amount: BN,
  compareFn?: (a: ValidatorAccount, b: ValidatorAccount) => number,
  skipFee?: boolean,
  allowPartial?: boolean,
  prefetchedData?: PrepareWithdrawPrefetchedData,
): Promise<WithdrawAccount[] | PrepareWithdrawResult> {
  const stakePoolProgramId = getStakePoolProgramId(connection.rpcEndpoint)

  // Use prefetched data if available, otherwise fetch from RPC
  let validatorListData: Buffer | null
  let minBalanceForRentExemption: number
  let stakeMinimumDelegation: number

  if (prefetchedData) {
    validatorListData = prefetchedData.validatorListData
    minBalanceForRentExemption = prefetchedData.minBalanceForRentExemption
    stakeMinimumDelegation = prefetchedData.stakeMinimumDelegation
  } else {
    const [validatorListAcc, rentExemption, stakeMinimumDelegationResponse] = await Promise.all([
      connection.getAccountInfo(stakePool.validatorList),
      connection.getMinimumBalanceForRentExemption(StakeProgram.space),
      connection.getStakeMinimumDelegation(),
    ])
    validatorListData = validatorListAcc?.data ?? null
    minBalanceForRentExemption = rentExemption
    stakeMinimumDelegation = Number(stakeMinimumDelegationResponse.value)
  }

  if (!validatorListData) {
    throw new Error('No staked funds available for delayed unstake. Use instant unstake instead.')
  }

  const validatorList = ValidatorListLayout.decode(validatorListData) as ValidatorList

  if (!validatorList?.validators || validatorList?.validators.length === 0) {
    throw new Error('No staked funds available for delayed unstake. Use instant unstake instead.')
  }

  // minBalance = rent + max(stake_minimum_delegation, MINIMUM_ACTIVE_STAKE)
  const minimumDelegation = Math.max(stakeMinimumDelegation, MINIMUM_ACTIVE_STAKE)
  const minBalance = new BN(minBalanceForRentExemption + minimumDelegation)

  // Threshold for has_active_stake check (ceiling division for lamports_per_pool_token)
  const lamportsPerPoolToken = stakePool.totalLamports
    .add(stakePool.poolTokenSupply)
    .sub(new BN(1))
    .div(stakePool.poolTokenSupply)
  const minimumLamportsWithTolerance = minBalance.add(lamportsPerPoolToken)

  const hasActiveStake = validatorList.validators.some(
    v => v.status === ValidatorStakeInfoStatus.Active
      && v.activeStakeLamports.gt(minimumLamportsWithTolerance),
  )
  const hasTransientStake = validatorList.validators.some(
    v => v.status === ValidatorStakeInfoStatus.Active
      && v.transientStakeLamports.gt(minimumLamportsWithTolerance),
  )

  // ValidatorRemoval mode: no validator above threshold
  const isValidatorRemovalMode = !hasActiveStake && !hasTransientStake

  let accounts: ValidatorAccount[] = []

  for (const validator of validatorList.validators) {
    if (validator.status !== ValidatorStakeInfoStatus.Active) {
      continue
    }

    const stakeAccountAddress = await findStakeProgramAddress(
      stakePoolProgramId,
      validator.voteAccountAddress,
      stakePoolAddress,
    )

    // ValidatorRemoval: full balance available; Normal: leave minBalance
    const availableActiveLamports = isValidatorRemovalMode
      ? validator.activeStakeLamports
      : validator.activeStakeLamports.sub(minBalance)

    if (availableActiveLamports.gt(new BN(0))) {
      const isPreferred = stakePool?.preferredWithdrawValidatorVoteAddress?.equals(
        validator.voteAccountAddress,
      )
      accounts.push({
        type: isPreferred ? 'preferred' : 'active',
        voteAddress: validator.voteAccountAddress,
        stakeAddress: stakeAccountAddress,
        lamports: availableActiveLamports,
      })
    }

    const availableTransientLamports = isValidatorRemovalMode
      ? validator.transientStakeLamports
      : validator.transientStakeLamports.sub(minBalance)

    if (availableTransientLamports.gt(new BN(0))) {
      const transientStakeAccountAddress = await findTransientStakeProgramAddress(
        stakePoolProgramId,
        validator.voteAccountAddress,
        stakePoolAddress,
        validator.transientSeedSuffixStart,
      )
      accounts.push({
        type: 'transient',
        voteAddress: validator.voteAccountAddress,
        stakeAddress: transientStakeAccountAddress,
        lamports: availableTransientLamports,
      })
    }
  }

  // Sort from highest to lowest balance
  accounts = accounts.sort(compareFn || ((a, b) => b.lamports.cmp(a.lamports)))

  // Prepare the list of accounts to withdraw from
  const withdrawFrom: WithdrawAccount[] = []
  let remainingAmount = new BN(amount)

  const fee = stakePool.stakeWithdrawalFee
  const inverseFee: Fee = {
    numerator: fee.denominator.sub(fee.numerator),
    denominator: fee.denominator,
  }

  for (const type of ['preferred', 'active', 'transient']) {
    const filteredAccounts = accounts.filter(a => a.type === type)

    for (const { stakeAddress, voteAddress, lamports } of filteredAccounts) {
      let availableForWithdrawal = calcPoolTokensForDeposit(stakePool, lamports)

      if (!skipFee && !inverseFee.numerator.isZero()) {
        availableForWithdrawal = availableForWithdrawal
          .mul(inverseFee.denominator)
          .div(inverseFee.numerator)
      }

      // In ValidatorRemoval mode, must withdraw full validator balance (no partial)
      // Skip if remaining amount is less than full validator balance
      if (isValidatorRemovalMode && remainingAmount.lt(availableForWithdrawal)) {
        continue
      }

      const poolAmount = BN.min(availableForWithdrawal, remainingAmount)
      if (poolAmount.lte(new BN(0))) {
        continue
      }

      withdrawFrom.push({ stakeAddress, voteAddress, poolAmount })
      remainingAmount = remainingAmount.sub(poolAmount)

      if (remainingAmount.isZero()) {
        break
      }
    }

    if (remainingAmount.isZero()) {
      break
    }
  }

  // Not enough stake to withdraw the specified amount
  if (remainingAmount.gt(new BN(0))) {
    if (allowPartial) {
      const delayedAmount = amount.sub(remainingAmount)
      return {
        withdrawAccounts: withdrawFrom,
        delayedAmount,
        remainingAmount,
      }
    }
    const availableAmount = amount.sub(remainingAmount)
    throw new Error(
      `Not enough staked funds for delayed unstake. Requested ${lamportsToSol(amount)} iFOGO, but only ${lamportsToSol(availableAmount)} available. Use instant unstake for the remaining amount.`,
    )
  }

  if (allowPartial) {
    return {
      withdrawAccounts: withdrawFrom,
      delayedAmount: amount,
      remainingAmount: new BN(0),
    }
  }

  return withdrawFrom
}

/**
 * Calculate the pool tokens that should be minted for a deposit of `stakeLamports`
 */
export function calcPoolTokensForDeposit(stakePool: StakePool, stakeLamports: BN): BN {
  if (stakePool.poolTokenSupply.isZero() || stakePool.totalLamports.isZero()) {
    return stakeLamports
  }
  const numerator = stakeLamports.mul(stakePool.poolTokenSupply)
  return numerator.div(stakePool.totalLamports)
}

/**
 * Calculate lamports amount on withdrawal
 */
export function calcLamportsWithdrawAmount(stakePool: StakePool, poolTokens: BN): BN {
  const numerator = poolTokens.mul(stakePool.totalLamports)
  const denominator = stakePool.poolTokenSupply
  if (numerator.lt(denominator)) {
    return new BN(0)
  }
  return numerator.div(denominator)
}

export function newStakeAccount(
  feePayer: PublicKey,
  instructions: TransactionInstruction[],
  lamports: number,
): Keypair {
  // Account for tokens not specified, creating one
  const stakeReceiverKeypair = Keypair.generate()
  console.log(`Creating account to receive stake ${stakeReceiverKeypair.publicKey}`)

  instructions.push(
    // Creating new account
    SystemProgram.createAccount({
      fromPubkey: feePayer,
      newAccountPubkey: stakeReceiverKeypair.publicKey,
      lamports,
      space: StakeProgram.space,
      programId: StakeProgram.programId,
    }),
  )

  return stakeReceiverKeypair
}
