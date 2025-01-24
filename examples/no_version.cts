import { defineCommand, run, type Context } from '..'

const main = defineCommand({
  meta: {
    name: 'simple',
  },
  options: {},
  callback: (_: Context) => {},
})

run(main)
