#!/usr/bin/env zx
import { cliArguments, workingDirectory } from '../utils.mjs'
import 'zx/globals'

// Build the client and run the tests.
cd(path.join(workingDirectory, 'clients', 'js-legacy'))
await $`pnpm install`
await $`pnpm build`
await $`pnpm test ${cliArguments()}`
