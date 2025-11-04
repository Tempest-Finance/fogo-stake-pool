#!/usr/bin/env zx
import { cliArguments, workingDirectory } from '../utils.mjs'
import 'zx/globals'

// Check the client using ESLint.
cd(path.join(workingDirectory, 'clients', 'js-legacy'))
await $`pnpm install`
await $`pnpm lint ${cliArguments()}`
