'use client'

import { isEstablished, useSession } from '@fogo/sessions-sdk-react'
import { PoolInfo } from '../components/PoolInfo'
import { StakeCard } from '../components/StakeCard'
import { UnstakeCard } from '../components/UnstakeCard'

export default () => {
  const sessionState = useSession()
  if (isEstablished(sessionState)) {
    return (
      <>
        <div>
          Address:
          {sessionState.walletPublicKey.toString()}
        </div>
        <div>
          sessionPublicKey:
          {sessionState.sessionPublicKey.toString()}
        </div>
        <PoolInfo />
        <StakeCard sessionState={sessionState} />
        <UnstakeCard sessionState={sessionState} />
      </>
    )
  } else {
    return 'Session not established'
  }
}
