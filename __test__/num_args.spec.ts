import test from 'ava'
import { spawnSync } from 'child_process'

test('num_args', (t) => {
  const result = spawnSync('node', [`examples/num_args.cjs`, '--foo', '1', '2', '3'])
  const more_than = spawnSync('node', [`examples/num_args.cjs`, '--foo', '1', '2', '3', '4'])
  const less_than = spawnSync('node', [`examples/num_args.cjs`, '--foo', '1'])
  t.is(result.error, undefined)
  t.is(result.stderr.length, 0)
  t.deepEqual(result.status ?? 0, 0)
  t.not(more_than.stderr.length, 0)
  t.not(less_than.stderr.length, 0)
})
