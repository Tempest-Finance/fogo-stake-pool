import type { NextConfig } from 'next'

const config: NextConfig = {
  output: 'export', // SPA
  headers: async () => [
    {
      source: '/paymaster',
      headers: [
        {
          key: 'Access-Control-Allow-Origin',
          value: '*',
        },
        {
          key: 'Access-Control-Allow-Methods',
          value: 'POST',
        },
        {
          key: 'Access-Control-Allow-Headers',
          value: 'Content-Type',
        },
      ],
    },
  ],
}

export default config
