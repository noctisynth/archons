import test from 'ava'
import { spawnSync } from 'child_process'

test('exclusive', (t) => {
  const exclusive = spawnSync('node', [`examples/exclusive.cjs`, '--config', 'config.json'])
  const should_fail = spawnSync('node', [`examples/exclusive.cjs`, '--config', 'config.json', '--other', 'other'])
  t.is(exclusive.error, undefined)
  t.is(exclusive.stderr.length, 0)
  t.deepEqual(exclusive.status ?? 0, 0)
  t.not(should_fail.stderr.length, 0)
})
