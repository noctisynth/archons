import { type Context, defineCommand, run } from 'archons'

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
