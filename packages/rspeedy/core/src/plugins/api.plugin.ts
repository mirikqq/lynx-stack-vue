// Copyright 2024 The Lynx Authors. All rights reserved.
// Licensed under the Apache License Version 2.0 that can be found in the
// LICENSE file in the root directory of this source tree.

import { logger } from '@rsbuild/core'
import type { RsbuildPlugin } from '@rsbuild/core'

import type { ExposedAPI } from '../api.js'
import type { Config } from '../config/index.js'
import { debug } from '../debug.js'
import { version } from '../version.js'

const sAPI = Symbol.for('rspeedy.api')

export function pluginAPI(config: Config): RsbuildPlugin {
  return {
    name: 'lynx:rsbuild:plugin-api',
    setup(api) {
      api.expose<ExposedAPI>(sAPI, {
        config,
        debug,
        async exit(code) {
          const { exit } = await import('../cli/exit.js')
          return exit(code)
        },
        logger,
        version,
      })
    },
  }
}
