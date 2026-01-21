#!/usr/bin/env zx
import { renderVisitor as renderJavaScriptVisitor } from '@codama/renderers-js'
import { renderVisitor as renderRustVisitor } from '@codama/renderers-rust'
import { JsonMap, parse as parseToml } from '@iarna/toml'
import * as c from 'codama'
import 'zx/globals'

// @ts-expect-error zx globals
export const workingDirectory = (await $`pwd`.quiet()).toString().trim()

// Load the auto-generated IDL from Codama macros.
const idlPath = path.join(workingDirectory, 'interface', 'idl.json')
if (!fs.existsSync(idlPath)) {
  console.error(`IDL not found at: ${idlPath}`)
  console.error('Run: cargo run --bin generate-idl --features codama -p fogo-stake-pool-interface')
  process.exit(1)
}

const idl = JSON.parse(fs.readFileSync(idlPath, 'utf-8'))
const codama = c.createFromRoot(idl)

// Rename the program from interface name to proper name.
codama.update(
  c.updateProgramsVisitor({
    fogoStakePoolInterface: { name: 'fogoStakePool' },
  }),
)

// Remove only the FutureEpoch type which uses generic T that Codama can't handle.
codama.update(
  c.deleteNodesVisitor([
    '[definedTypeNode]futureEpoch',
  ]),
)

// Add StakePool and ValidatorList as account types for client generation.
codama.update(
  c.bottomUpTransformerVisitor([
    {
      select: '[programNode]',
      transform: (node) => {
        c.assertIsNode(node, 'programNode')
        return {
          ...node,
          accounts: [
            ...node.accounts,
            c.accountNode({
              name: 'stakePoolAccount',
              data: c.structTypeNode([
                c.structFieldTypeNode({
                  name: 'data',
                  type: c.definedTypeLinkNode('stakePool'),
                }),
              ]),
            }),
            c.accountNode({
              name: 'validatorListAccount',
              data: c.structTypeNode([
                c.structFieldTypeNode({
                  name: 'data',
                  type: c.definedTypeLinkNode('validatorList'),
                }),
              ]),
            }),
          ],
        }
      },
    },
  ]),
)

// Render JavaScript.
const jsClient = path.join(workingDirectory, 'clients', 'js-new')
if (fs.existsSync(jsClient)) {
  void codama.accept(
    renderJavaScriptVisitor(path.join(jsClient, 'src', 'generated'), {
      prettierOptions: JSON.parse(
        fs.readFileSync(path.join(jsClient, '.prettierrc.json'), 'utf-8'),
      ),
    }),
  )
}

// Render Rust client.
const rustClient = path.join(workingDirectory, 'clients', 'rust')
if (fs.existsSync(rustClient)) {
  codama.accept(
    renderRustVisitor(path.join(rustClient, 'src', 'generated'), {
      formatCode: true,
      crateFolder: rustClient,
      anchorTraits: false,
      toolchain: getToolchainArgument('format'),
      linkOverrides: {
        definedTypes: {
          lockup: 'solana_program::stake::state',
          podU64: 'spl_pod::primitives',
          podU32: 'spl_pod::primitives',
        },
      },
      traitOptions: {
        baseDefaults: [
          'borsh::BorshSerialize',
          'borsh::BorshDeserialize',
          'serde::Serialize',
          'serde::Deserialize',
          'Clone',
          'Debug',
          'PartialEq',
        ],
      },
    }),
  )
  console.log('✓ Generated Rust client')
}

// Helper functions for reading Cargo metadata.

function getCargo(folder?: string): JsonMap {
  return parseToml(
    fs.readFileSync(
      path.resolve(workingDirectory, path.join(folder || '.', 'Cargo.toml')),
      'utf8',
    ),
  )
}

function getToolchainArgument(operation: string): string {
  const cargo = getCargo()
  const metadata = (cargo?.workspace as JsonMap)?.metadata as JsonMap | undefined
  const toolchains = metadata?.toolchains as JsonMap | undefined
  const channel = toolchains?.[operation] as string | undefined
  return channel ? `+${channel}` : ''
}
