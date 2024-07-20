import test from 'ava'

import { Command, defineCommand, run } from '../index'

test('define command', (t) => {
  const cmd: Command = {
    meta: {
      name: 'test',
      version: '1.0.0',
      about: 'test command',
    },
    options: {
      foo: {
        type: 'positional',
      },
    },
    callback: (ctx: any) => {
      console.log(ctx)
    },
  }
  t.deepEqual(defineCommand(cmd), cmd)
})
