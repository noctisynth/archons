import { spawnSync } from 'node:child_process'

test('num_args', () => {
  const result = spawnSync('node', ['examples/num_args.cjs', '--foo', '1', '2', '3'])
  const more_than = spawnSync('node', ['examples/num_args.cjs', '--foo', '1', '2', '3', '4'])
  const less_than = spawnSync('node', ['examples/num_args.cjs', '--foo', '1'])
  expect(result.error).toBe(undefined)
  expect(result.stderr.length).toBe(0)
  expect(result.status ?? 0).toEqual(0)
  expect(more_than.stderr.length).not.toBe(0)
  expect(less_than.stderr.length).not.toBe(0)
})
