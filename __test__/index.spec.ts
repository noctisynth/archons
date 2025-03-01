import { spawnSync } from 'node:child_process'

import { type Context, type Command, defineCommand, run } from 'archons'

const cmd: Command = {
  meta: {
    name: 'test',
    version: '1.0.0',
    about: 'test command',
  },
  options: {
    foo: {
      type: 'positional',
      action: 'set',
    },
  },
  callback: (_: Context) => {},
}

const main = defineCommand(cmd)

test('define command', () => {
  expect(defineCommand(cmd)).toEqual(cmd)
})

test('run command', () => {
  expect(() => {
    run(main, ['node', 'test.js'])
  }).not.toThrow()
})

test('run help', () => {
  const result = spawnSync('node', ['examples/simple.cjs', '--help'])
  expect(result.error).toBe(undefined)
  expect(result.stderr.length).toBe(0)
  expect(result.status ?? 0).toEqual(0)
})

test('run version', () => {
  const version = spawnSync('node', ['examples/simple.cjs', '--version'])
  const no_version = spawnSync('node', ['examples/no_version.cjs', '--version'])
  expect(version.error).toBe(undefined)
  expect(version.stderr.length).toBe(0)
  expect(version.status ?? 0).toEqual(0)
  expect(no_version.stderr.length).not.toBe(0)
})
