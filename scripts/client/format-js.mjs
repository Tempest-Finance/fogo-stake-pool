#!/usr/bin/env zx
import { cliArguments, workingDirectory } from '../utils.mjs'
import 'zx/globals'

// Format the client using Prettier.
cd(path.join(workingDirectory, 'clients', 'js-legacy'))
await $`pnpm install`
await $`pnpm format ${cliArguments()}`
