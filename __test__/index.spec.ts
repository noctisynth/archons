import test from 'ava'
import { spawnSync } from 'child_process'

import { Command, defineCommand, run } from '../index'

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
  callback: (_: any) => {},
}

const main = defineCommand(cmd)

test('define command', (t) => {
  t.deepEqual(defineCommand(cmd), cmd)
})

test('run command', (t) => {
  t.notThrows(() => {
    run(main, ['node.exe', 'test.js'])
  })
})

test('run help', (t) => {
  const result = spawnSync('node.exe', [`examples/simple.cjs`, '--help'])
  t.falsy(result.error)
  t.deepEqual(result.status, 0)
})

test('run version', (t) => {
  const version = spawnSync('node.exe', [`examples/simple.cjs`, '--version'])
  const no_version = spawnSync('node.exe', [`examples/no_version.cjs`, '--version'])
  t.falsy(version.error)
  t.deepEqual(version.status, 0)
  t.not(no_version.status, 0)
})
