import { type Context, defineCommand, run } from 'archons'

const main = defineCommand({
  meta: {
    name: 'test',
  },
  options: {
    config: {
      type: 'option',
      exclusive: true,
    },
    other: {
      type: 'option',
    },
  },
  callback: async (ctx: Context) => {
    console.log(ctx)
  },
})

run(main)
