import { EstablishedSessionState, useConnection } from '@fogo/sessions-sdk-react'
import { depositWsolWithSession, withdrawWsolWithSession } from '@solana/spl-stake-pool/src'
import { STAKE_POOL_ADDRESS } from '../constants.ts'
import { solToLamports, txDetails } from '../utils.ts'

export function useStakePool({ sessionState }: { sessionState: EstablishedSessionState }) {
  const connection = useConnection()
  const poolAddress = STAKE_POOL_ADDRESS

  return {
    poolAddress,

    /**
     * Deposit SOL into the stake pool
     * @param amount
     */
    depositSol: async (amount: number) => {
      const lamports = solToLamports(amount)

      const { instructions } = await depositWsolWithSession(
        connection,
        poolAddress,
        sessionState.sessionPublicKey,
        sessionState.walletPublicKey,
        lamports,
        undefined,
        undefined,
        undefined,
        sessionState.payer,
      )

      const res = await sessionState.sendTransaction(instructions)

      await txDetails(connection, res.signature)
    },

    /**
     * Withdraw SOL from the stake pool
     * @param poolTokens
     */
    withdrawSol: async (poolTokens: number) => {
      const { instructions } = await withdrawWsolWithSession(
        connection,
        poolAddress,
        sessionState.sessionPublicKey,
        sessionState.walletPublicKey,
        poolTokens,
      )

      const res = await sessionState.sendTransaction(instructions)

      await txDetails(connection, res.signature)
    },
  }
}
