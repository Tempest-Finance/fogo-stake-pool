'use client'

import { EstablishedSessionState } from '@fogo/sessions-sdk-react'
import { Button, Card, InputNumber } from 'antd'
import { useState } from 'react'
import { useStakePool } from '../hooks'

export function StakeCard({ sessionState }: { sessionState: EstablishedSessionState }) {
  const { value, setValue, inProgress, stake } = useStaking({ sessionState })

  return (
    <Card title="Stake Card">
      <InputNumber
        style={{ width: 200 }}
        value={value}
        min={0.01}
        max={100}
        step={0.000000001}
        onChange={setValue}
      />
      <Button type="primary" loading={inProgress} onClick={stake}>Stake</Button>
    </Card>
  )
}

function useStaking({ sessionState }: { sessionState: EstablishedSessionState }) {
  const [value, setValue] = useState<string | number | null>(0.1)
  const [inProgress, setInProgress] = useState<boolean>(false)
  const { depositSol } = useStakePool({ sessionState })

  const stake = async () => {
    setInProgress(true)
    try {
      await depositSol(Number(value))
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
    stake,
  }
}
