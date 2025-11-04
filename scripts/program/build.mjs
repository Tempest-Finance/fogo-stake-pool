#!/usr/bin/env zx
import {
  cliArguments,
  getProgramFolders,
  workingDirectory,
} from '../utils.mjs'
import 'zx/globals'

// Build the programs.
for (const folder of getProgramFolders()) {
  const manifestPath = path.join(workingDirectory, folder, 'Cargo.toml')
  await $`cargo-build-sbf --manifest-path ${manifestPath} ${cliArguments()}`
}
