const { defineCommand, run } = require('./index.js')

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
  callback: (ctx) => {
    console.log(ctx);
  }
})

const main = defineCommand({
  meta: {
    name: 'simple',
    version: '0.0.1',
    about: 'A simple command line tool',
    alias: ['dev']
  },
  options: {
    verbose: {
      type: 'flag',
      parser: 'boolean',
    },
  },
  subcommands: {
    dev,
  },
})

run(main)
