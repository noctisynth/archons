import { defineCommand, run, type Context } from '../index'

const main = defineCommand({
  meta: {
    name: 'simple',
  },
  options: {},
  callback: (_: Context) => {},
})

run(main)
