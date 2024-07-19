import test from 'ava'

import { defineCommand } from '../index'

test('define command', (t) => {
  const cmd = {
    meta: {},
    options: {},
    callback: (ctx: any) => {
      console.log(ctx)
    },
  }
  t.deepEqual(defineCommand(cmd), cmd)
})
