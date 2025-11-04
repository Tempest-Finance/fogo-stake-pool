#!/usr/bin/env zx
import {
  cliArguments,
  getToolchainArgument,
  popArgument,
  workingDirectory,
} from '../utils.mjs'
import 'zx/globals'

const [folder, ...args] = cliArguments()

// Configure arguments here.
const lintArgs = [
  '-Zunstable-options',
  '--all-targets',
  '--all-features',
  '--',
  '--deny=warnings',
  '--deny=clippy::arithmetic_side_effects',
  ...args,
]

const fix = popArgument(lintArgs, '--fix')
const toolchain = getToolchainArgument('lint')

const manifestPath = path.join(workingDirectory, folder, 'Cargo.toml')

if (fix) {
  await $`cargo ${toolchain} clippy --manifest-path ${manifestPath} --fix ${lintArgs}`
} else {
  await $`cargo ${toolchain} clippy --manifest-path ${manifestPath} ${lintArgs}`
}
