'use client'

import type { EstablishedSessionState } from '@fogo/sessions-sdk-react'
import { AnchorProvider, BN, Wallet } from '@coral-xyz/anchor'
import { ExampleProgram } from '@fogo/sessions-idls'
import { TransactionResult } from '@fogo/sessions-sdk'
import { useConnection } from '@fogo/sessions-sdk-react'
import { getAssociatedTokenAddressSync, getMint, NATIVE_MINT } from '@solana/spl-token'
import { PublicKey } from '@solana/web3.js'
import { useCallback, useState } from 'react'

enum StateType {
  Base,
  Running,
  Error,
  Complete,
}

const State = {
  Base: () => ({ type: StateType.Base as const }),
  Running: () => ({ type: StateType.Running as const }),
  Complete: (result: TransactionResult) => ({ type: StateType.Complete as const, result }),
  ErrorState: (error: unknown) => ({
    type: StateType.Error as const,
    error,
  }),
}
type State = ReturnType<typeof State[keyof typeof State]>

class AlreadyInProgressError extends Error {
  constructor() {
    super('Can\'t run async hook when already in progress')
    this.name = 'AlreadyInProgressError'
  }
}

export function TransferButton({ sessionState }: { sessionState: EstablishedSessionState }) {
  const { state, execute } = useTrade(sessionState, 0.001, NATIVE_MINT)

  return (
    <button onClick={execute} disabled={state.type === StateType.Running}>
      Transfer 0.001 FOGO
    </button>
  )
}

function useTrade(sessionState: EstablishedSessionState,
  amount: number,
  mint: PublicKey) {
  const [state, setState] = useState<State>(State.Base())
  const connection = useConnection()

  const execute = useCallback(() => {
    if (state.type === StateType.Running) {
      throw new AlreadyInProgressError()
    }
    setState(State.Running())
    getMint(connection, mint)
      .then(({ decimals }) => (
        new ExampleProgram(
          new AnchorProvider(connection, {} as Wallet, {}),
        ).methods.exampleTransfer(new BN(amount * 10 ** decimals)).accountsPartial({
          signerOrSession: sessionState.sessionPublicKey,
          sink: getAssociatedTokenAddressSync(mint, sessionState.payer),
          userTokenAccount: getAssociatedTokenAddressSync(
            mint,
            sessionState.walletPublicKey,
          ),
          mint,
        }).instruction()
      ))
      .then(instruction => sessionState.sendTransaction([instruction]))
      .then(result => setState(State.Complete(result)))
      .catch((error: unknown) => {
        console.error(error)
        setState(State.ErrorState(error))
        throw error
      })
  }, [state, sessionState, amount, mint, setState])

  return { state, execute }
}
