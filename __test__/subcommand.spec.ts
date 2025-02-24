import test from 'ava'

import { Context, defineCommand, run } from '..'

test('sub command', (t) => {
  const cmd = defineCommand({
    meta: {},
    options: {
      foo: {
        type: 'positional',
      },
    },
    callback: (ctx: Context) => {
      t.deepEqual(ctx.args, { foo: 'foo' })
    },
  })
  const main = defineCommand({
    meta: {
      name: 'test',
    },
    options: {},
    subcommands: {
      cmd,
    },
  })
  t.notThrows(() => {
    run(main, ['node.exe', 'test.js', 'cmd', 'foo'])
  })
})
