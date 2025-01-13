import { Context, defineCommand, run } from '..'

const spinner = defineCommand({
  meta: {
    name: 'spinner',
  },
  options: {
    'enable-steady-tick': {
      type: 'option',
      action: 'store',
    },
  },
  callback: async (ctx: Context) => {
    const spinner = ctx.createSpinner()
    spinner.setMessage('loading')
    spinner.tick()
    let i = 100
    const interval = ctx.args.interval as number
    if (ctx.args['enable-steady-tick']) {
      spinner.println('Enabled steady tick')
      spinner.enableSteadyTick(interval)
      while (i--) {
        if (i < 30) {
          spinner.setMessage('Disabled steady tick for now')
          spinner.disableSteadyTick()
        }
        await new Promise((resolve) => setTimeout(resolve, interval))
      }
    } else {
      spinner.println('Disabled steady tick')
      while (i--) {
        spinner.tick()
        await new Promise((resolve) => setTimeout(resolve, interval))
      }
    }
    spinner.finishWithMessage('✨ finished')
  },
})

const bar = defineCommand({
  meta: {
    name: 'bar',
  },
  options: {
    clear: {
      type: 'option',
      action: 'store',
      help: 'Clear the progress bar',
      parser: 'boolean',
    },
  },
  callback: async (ctx: Context) => {
    const bar = ctx.createProgressBar(ctx.args.total as number)
    bar.setTemplate('{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>5}/{len:5} {msg}')
    bar.setProgressChars('=>-')
    let i = 100
    const interval = ctx.args.interval as number
    while (i--) {
      bar.inc(1)
      await new Promise((resolve) => setTimeout(resolve, interval))
    }
    if (ctx.args.clear) {
      bar.finishAndClear()
    } else {
      bar.finish()
    }
    console.log('✨ finished')
  },
})

const main = defineCommand({
  meta: {
    name: 'progressbar',
    styled: true,
    subcommandRequired: true,
  },
  options: {
    interval: {
      type: 'option',
      numArgs: '1',
      default: '100',
      global: true,
      help: 'Interval of spinner',
      parser: 'number',
    },
    total: {
      type: 'option',
      numArgs: '1',
      default: '100',
      global: true,
      help: 'Total of progress bar',
      parser: 'number',
    },
  },
  subcommands: {
    spinner,
    bar,
  },
})

run(main)
