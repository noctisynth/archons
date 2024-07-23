import { Context, defineCommand, run } from '..'

const main = defineCommand({
  meta: {
    name: 'test',
  },
  options: {
    foo: {
      type: 'positional',
      required: true,
    },
  },
  callback: (_: Context) => {},
})

run(main)
