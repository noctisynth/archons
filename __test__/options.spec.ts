import { spawnSync } from 'node:child_process'

import { type Context, defineCommand, run } from '../index.js'

test('positional option', () => {
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
      expect(ctx.args.foo).toBe('foo')
    },
  })
  expect(() => {
    run(main, ['node', 'test.js', 'foo'])
  }).not.toThrow()
})

test('required positional option', () => {
  const result = spawnSync('node', ['examples/positional_required.cjs', 'foo'])
  const should_fail = spawnSync('node', ['examples/positional_required.cjs'])
  expect(result.error).toBe(undefined)
  expect(result.stderr.length).toBe(0)
  expect(result.status ?? 0).toEqual(0)
  expect(should_fail.stderr.length).not.toBe(0)
})

test('boolean flag', () => {
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
      expect(ctx.args.verbose).toBe(ctx.args.eq)
    },
  })
  expect(() => {
    run(main, ['node', 'test.js', '--verbose', '-e'])
  }).not.toThrow()
  expect(() => {
    run(main, ['node', 'test.js'])
  }).not.toThrow()
})
