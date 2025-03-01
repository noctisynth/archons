import { spawnSync } from 'node:child_process'

test('exclusive', () => {
  const exclusive = spawnSync('node', ['examples/exclusive.cjs', '--config', 'config.json'])
  const should_fail = spawnSync('node', ['examples/exclusive.cjs', '--config', 'config.json', '--other', 'other'])
  expect(exclusive.error).toBe(undefined)
  expect(exclusive.stderr.length).toBe(0)
  expect(exclusive.status ?? 0).toEqual(0)
  expect(should_fail.stderr.length).not.toBe(0)
})
