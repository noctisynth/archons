import test from 'ava'
import { spawnSync } from 'child_process'

import { Context, defineCommand, run } from '../index'

test('positional option', (t) => {
  const main = defineCommand({
    meta: {
      name: 'test',
    },
    options: {
      foo: {
        type: 'positional',
      },
    },
    callback: (ctx: Context) => {
      t.is(ctx.args.foo, 'foo')
    },
  })
  t.notThrows(() => {
    run(main, ['node', 'test.js', 'foo'])
  })
})

test('required positional option', (t) => {
  const result = spawnSync('node', [`examples/positional_required.cjs`, 'foo'])
  const should_fail = spawnSync('node', [`examples/positional_required.cjs`])
  t.falsy(result.error)
  t.deepEqual(result.status, 0)
  t.not(should_fail.status, 0)
})

test('boolean flag', (t) => {
  const main = defineCommand({
    meta: {
      name: 'test',
    },
    options: {
      verbose: {
        type: 'option',
        action: 'store',
      },
      eq: {
        type: 'option',
        action: 'store',
        alias: ['e'],
      },
    },
    callback: (ctx: Context) => {
      t.is(ctx.args.verbose, ctx.args.eq)
    },
  })
  t.notThrows(() => {
    run(main, ['node', 'test.js', '--verbose', '-e'])
  })
  t.notThrows(() => {
    run(main, ['node', 'test.js'])
  })
})

