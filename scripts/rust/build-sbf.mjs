#!/usr/bin/env zx
import {
  cliArguments,
  workingDirectory,
} from '../utils.mjs'
import 'zx/globals'

const [folder, ...args] = cliArguments()
const manifestPath = path.join(workingDirectory, folder, 'Cargo.toml')
await $`cargo-build-sbf --manifest-path ${manifestPath} ${args}`
