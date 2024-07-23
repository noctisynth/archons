import { Context, defineCommand, run } from '..'

const dev = defineCommand({
  meta: {
    name: 'dev',
  },
  options: {},
  callback: (ctx: Context) => {
    console.log(ctx.args.config)
  },
})
const main = defineCommand({
  meta: {
    name: 'test',
  },
  options: {
    config: {
      type: 'option',
      global: true,
    },
  },
  subcommands: {
    dev,
  },
  callback: (ctx: Context) => {
    console.log(ctx.args.config)
  },
})

run(main)
