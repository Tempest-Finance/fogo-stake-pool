import { BN } from '@coral-xyz/anchor'
import { Connection, LAMPORTS_PER_SOL } from '@solana/web3.js'

const SOL_DECIMALS = Math.log10(LAMPORTS_PER_SOL)

/**
 * Convert SOL to lamports.
 * @param amount
 */
export function solToLamports(amount: number | string): number {
  if (Number.isNaN(amount)) {
    return Number(0)
  }
  return new BN(Number(amount).toFixed(SOL_DECIMALS).replace('.', '')).toNumber()
}

/**
 * Fetch and log transaction details for a given signature.
 * @param connection
 * @param signature
 */
export async function txDetails(connection: Connection, signature: string) {
  const txDetails = await connection.getTransaction(signature, {
    commitment: 'confirmed',
    maxSupportedTransactionVersion: 0,
  })

  if (txDetails?.meta?.logMessages) {
    console.log(`Transaction Logs:`, txDetails.meta.logMessages)
  } else {
    console.warn('Transaction details not available yet')
  }

  console.log('Transaction Signature:', signature)
  console.log('Compute Units Consumed:', txDetails?.meta?.computeUnitsConsumed)
}
