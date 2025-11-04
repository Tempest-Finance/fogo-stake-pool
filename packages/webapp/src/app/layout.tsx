'use client'

import type { ReactNode } from 'react'
import { AntdRegistry } from '@ant-design/nextjs-registry'
import { FogoSessionProvider, Network, SessionButton } from '@fogo/sessions-sdk-react'
import { NATIVE_MINT } from '@solana/spl-token'
import Head from 'next/head'
import { APP_DOMAIN, RPC_URL, STAKE_POOL_MINT } from '../constants.ts'
import '@ant-design/v5-patch-for-react-19'

// eslint-disable-next-line react-refresh/only-export-components
export default ({ children }: { children: ReactNode }) => (
  <html lang="en">
    <Head>
      <title>Fogo stake pool</title>
      <meta name="description" content="TODO" />
    </Head>

    <body>
      <AntdRegistry>
        <FogoSessionProvider
          network={Network.Testnet}
          rpc={RPC_URL}
          domain={APP_DOMAIN}
          tokens={[NATIVE_MINT.toString(), STAKE_POOL_MINT.toString()]}
          // defaultRequestedLimits={{
          //   [NATIVE_MINT.toString()]: BigInt(1e15),
          //   [STAKE_POOL_MINT.toString()]: BigInt(1e15),
          // }}
          enableUnlimited
        >
          <header>
            <SessionButton />
          </header>
          <hr />
          <main>
            {children}
          </main>
        </FogoSessionProvider>
      </AntdRegistry>
    </body>
  </html>
)
