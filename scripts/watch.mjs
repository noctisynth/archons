import { execSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import path from 'node:path'
import fs from 'node:fs'

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
tryBuild('yarn build:debug', 'Building module...')
tryBuild('yarn build:examples', 'Building examples...')
console.log('[Archons] Build complete.\n')
console.log(`[Archons] Watching on ${dirPath} for changes...`)

let lastBuild = Date.now()
fs.watch(dirPath, { recursive: true }, (eventType, filename) => {
  // debounce
  if (Date.now() - lastBuild < 1000) {
    return
  }
  if (filename && !isIgnored(filename)) {
    console.log(`[Archons] File ${filename} was ${eventType}d, rebuilding...`)
    if (filename.endsWith('.rs') || filename.endsWith('.toml')) {
      tryBuild('yarn build:debug', 'Rebuilding module...')
    }
    tryBuild('yarn build:examples', 'Rebuilding examples...')
    console.log('[Archons] Build complete.\n')
    console.log('[Archons] Watching for changes...')
    lastBuild = Date.now()
  }
})
