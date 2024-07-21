import { defineCommand, run, type Context } from '../index';

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
    verbose: {
      type: 'flag',
      parser: 'boolean',
      help: 'Enable verbose output',
    },
  },
  subcommands: {
    dev,
  },
})

run(main)
