import type { Context } from '../index.js'

const { defineCommand, run } = require('../index.js')

test('sub command', () => {
  const cmd = defineCommand({
    meta: {},
    options: {
      foo: {
        type: 'positional',
      },
    },
    callback: (ctx: Context) => {
      expect(ctx.args).toEqual({ foo: 'foo' })
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
  expect(() => {
    run(main, ['node.exe', 'test.js', 'cmd', 'foo'])
  }).not.toThrow()
})
