import { type Context, defineCommand, run } from 'archons'

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
    console.log(ctx.get('config'))
  },
})

run(main)
