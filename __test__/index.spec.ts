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
    run(main, ['node', 'test.js'])
  })
})

test('run help', (t) => {
  const result = spawnSync('node', [`examples/simple.cjs`, '--help'])
  console.log(result.stderr.length)
  t.is(result.error, undefined)
  t.is(result.stderr.length, 0)
  t.deepEqual(result.status ?? 0, 0)
})

import { execSync } from 'child_process'

test('run version', (t) => {
  const version = spawnSync('node', [`examples/simple.cjs`, '--version'])
  const no_version = spawnSync('node', [`examples/no_version.cjs`, '--version'])
  console.log(version.stderr.toString())
  execSync('./test.sh', { stdio: 'inherit' })
  t.is(version.error, undefined)
  t.is(version.stderr.length, 0)
  t.deepEqual(version.status ?? 0, 0)
  t.not(no_version.stderr.length, 0)
})
