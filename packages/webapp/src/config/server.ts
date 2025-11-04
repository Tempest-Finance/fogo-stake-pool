import { Network } from '@fogo/sessions-sdk-react'

function getNetwork() {
  switch (process.env.NETWORK) {
    case 'testnet': {
      return Network.Testnet
    }
    case 'mainnet': {
      return Network.Mainnet
    }
    default:
      return Network.Testnet
  }
}

export const NETWORK = getNetwork()

function getProviderConfig() {
  if (NETWORK === undefined) {
    return {
      addressLookupTableAddress:
                process.env.ADDRESS_LOOKUP_TABLE_ADDRESS
                ?? '93QGBU8ZHuvyKSvDFeETsdek1KQs4gqk3mEVKG8UxoX3',
      domain: process.env.FOGO_SESSIONS_DOMAIN,
      rpc: process.env.RPC ?? 'http://127.0.0.1:8899',
      paymaster: process.env.PAYMASTER ?? 'http://localhost:4000',
    }
  } else if (
    process.env.PAYMASTER === undefined
    || process.env.RPC === undefined
  ) {
    return {
      network: NETWORK,
      rpc: process.env.RPC,
      addressLookupTableAddress: process.env.ADDRESS_LOOKUP_TABLE_ADDRESS,
      domain: process.env.FOGO_SESSIONS_DOMAIN ?? 'https://sessions-example.fogo.io',
    }
  } else {
    return {
      rpc: process.env.RPC,
      paymaster: process.env.PAYMASTER,
      addressLookupTableAddress: process.env.ADDRESS_LOOKUP_TABLE_ADDRESS,
      domain: process.env.FOGO_SESSIONS_DOMAIN ?? 'https://sessions-example.fogo.io',
    }
  }
}

export const PROVIDER_CONFIG = getProviderConfig()
