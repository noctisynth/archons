import { execSync } from 'child_process'
import { fileURLToPath } from 'url'
import path from 'path'
import fs from 'fs'

function isIgnored(filename) {
  for (const pattern of ignored) {
    if (filename.includes(pattern)) {
      return true
    }
  }
  return false
}

function tryBuild(cmd, msg) {
  try {
    console.log(`[Archons] ${msg}`)
    execSync(cmd, { stdio: 'inherit' })
    console.clear()
  } catch (e) {
    console.error('[Archons] Build failed.')
  }
}

const dirPath = path.join(path.dirname(fileURLToPath(import.meta.url)), '..')
const ignored = ['node_modules', '.git', '.github', '.vscode', 'dist', 'build', 'bin', '.md', '.d.ts', 'target', '.cjs']

console.log('[Archons] Initial building...')
tryBuild('yarn build', 'Building module...')
tryBuild('yarn build:examples', 'Building examples...')
console.log('[Archons] Build complete.\n')
console.log(`[Archons] Watching on ${dirPath} for changes...`)

fs.watch(dirPath, { recursive: true }, (eventType, filename) => {
  if (filename && !isIgnored(filename)) {
    console.log(`[Archons] File ${filename} was ${eventType}d, rebuilding...`)
    if (filename.endsWith('.rs') || filename.endsWith('.toml')) {
      tryBuild('yarn build', 'Rebuilding module...')
    }
    tryBuild('yarn build:examples', 'Rebuilding examples...')
    console.log('[Archons] Build complete.\n')
    console.log('[Archons] Watching for changes...')
  }
})
