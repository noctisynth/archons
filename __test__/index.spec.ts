import test from 'ava'
import { spawnSync } from 'node:child_process'

import { type Context, type Command, defineCommand, run } from '../index.js'

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

test('define command', (t) => {
  t.deepEqual(defineCommand(cmd), cmd)
})

test('run command', (t) => {
  t.notThrows(() => {
    run(main, ['node', 'test.js'])
  })
})

test('run help', (t) => {
  const result = spawnSync('node', ['examples/simple.cjs', '--help'])
  t.is(result.error, undefined)
  t.is(result.stderr.length, 0)
  t.deepEqual(result.status ?? 0, 0)
})

test('run version', (t) => {
  const version = spawnSync('node', ['examples/simple.cjs', '--version'])
  const no_version = spawnSync('node', ['examples/no_version.cjs', '--version'])
  t.is(version.error, undefined)
  t.is(version.stderr.length, 0)
  t.deepEqual(version.status ?? 0, 0)
  t.not(no_version.stderr.length, 0)
})
