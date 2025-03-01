import { defineCommand, run, type Context } from 'archons';

const dev = defineCommand({
  meta: {
    name: 'dev',
    about: 'Run development server',
  },
  options: {
    port: {
      type: 'option',
      parser: 'number',
      default: '3000',
    },
  },
  callback: (ctx: Context) => {
    console.log(ctx);
  }
})

const main = defineCommand({
  meta: {
    name: 'simple',
    version: '0.0.1',
    about: 'A simple command line tool',
    styled: true,
  },
  options: {
    name: {
      type: 'positional',
      required: true,
      help: 'Name of the person to greet',
    },
    verbose: {
      type: 'option',
      parser: 'boolean',
      action: 'store',
      help: 'Enable verbose output',
      global: true
    },
  },
  subcommands: {
    dev,
  },
  callback: (ctx: Context) => {
    console.log(ctx);
  }
})

run(main)
