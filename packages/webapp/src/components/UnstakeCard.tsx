'use client'

import { EstablishedSessionState } from '@fogo/sessions-sdk-react'
import { Button, Card, InputNumber } from 'antd'
import { useState } from 'react'
import { useStakePool } from '../hooks'

export function UnstakeCard({ sessionState }: { sessionState: EstablishedSessionState }) {
  const { value, setValue, inProgress, unstake } = useStaking({ sessionState })

  return (
    <Card title="Unstake Card">
      <InputNumber
        style={{ width: 200 }}
        value={value}
        min={0.01}
        max={100}
        step={0.000000001}
        onChange={setValue}
      />
      <Button type="primary" loading={inProgress} onClick={unstake}>Unstake</Button>
    </Card>
  )
}

function useStaking({ sessionState }: { sessionState: EstablishedSessionState }) {
  const [value, setValue] = useState<string | number | null>(0.1)
  const [inProgress, setInProgress] = useState<boolean>(false)
  const { withdrawSol } = useStakePool({ sessionState })

  const unstake = async () => {
    setInProgress(true)
    try {
      await withdrawSol(Number(value))
    } catch (e) {
      console.error(e)
    } finally {
      setInProgress(false)
    }
  }

  return {
    value,
    setValue,
    inProgress,
    setInProgress,
    unstake,
  }
}
