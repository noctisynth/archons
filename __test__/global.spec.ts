import test from 'ava'

import { type Context, defineCommand, run } from 'archons'

test('global option', (t) => {
  const dev = defineCommand({
    meta: {
      name: 'dev',
    },
    options: {},
    callback: (ctx: Context) => {
      t.is(ctx.args.config, 'config.json')
    },
  })
  const main = defineCommand({
    meta: {
      name: 'test',
    },
    options: {
      config: {
        type: 'option',
        global: true,
      },
    },
    subcommands: {
      dev,
    },
  })
  t.notThrows(() => {
    run(main, ['node', 'test.js', 'dev', '--config', 'config.json'])
  })
})
