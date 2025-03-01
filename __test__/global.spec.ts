import { type Context, defineCommand, run } from 'archons'

test('global option', () => {
  const dev = defineCommand({
    meta: {
      name: 'dev',
    },
    options: {},
    callback: (ctx: Context) => {
      expect(ctx.args.config).toBe('config.json')
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
  })
  expect(() => {
    run(main, ['node', 'test.js', 'dev', '--config', 'config.json'])
  }).not.toThrow()
})
