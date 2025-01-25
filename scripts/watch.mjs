// @ts-check
import { execSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import path from 'node:path'
import fs from 'node:fs'
import { createSpinner } from '../index.js'

/**
 * @param {string} filename
 */
function isIgnored(filename) {
  for (const pattern of ignored) {
    if (filename.includes(pattern)) {
      return true
    }
  }
  return false
}

/**
 * @param {string} cmd
 * @param {string} msg
 * @param {import("../index.d.ts").ProgressBar} spinner
 */
function tryBuild(cmd, msg, spinner) {
  try {
    spinner.setMessage(`${msg}`)
    execSync(cmd, { stdio: 'inherit' })
    console.clear()
  } catch (e) {
    console.log(e)
    spinner.println('[Archons] Build failed.')
  }
}

const dirPath = path.join(path.dirname(fileURLToPath(import.meta.url)), '..')
const ignored = ['node_modules', '.git', '.github', '.vscode', 'dist', 'build', 'bin', '.md', '.d.ts', 'target', '.cjs']

const spinner = createSpinner();
spinner.setTemplate('{prefix} {spinner} {msg}')
spinner.enableSteadyTick(100)
spinner.setPrefix('[Archons]')
spinner.setMessage('Initial building...')
tryBuild('yarn build:debug', 'Building module...', spinner)
tryBuild('yarn build:examples', 'Building examples...', spinner)
spinner.finishAndClear()
console.log('[Archons] Build complete.\n')
console.log(`[Archons] Watching on ${dirPath} for changes...`)

let lastBuild = Date.now()
fs.watch(dirPath, { recursive: true }, (eventType, filename) => {
  // debounce
  if (Date.now() - lastBuild < 1000) {
    return
  }
  if (filename && !isIgnored(filename)) {
    const spinner = createSpinner()
    spinner.setTemplate('{prefix} {spinner} {msg}')
    spinner.setPrefix('[Archons]')
    spinner.println(`[Archons] File ${filename} was ${eventType}d, rebuilding...`)
    spinner.setMessage('Rebuilding...')
    spinner.enableSteadyTick(100)
    if (filename.endsWith('.rs') || filename.endsWith('.toml')) {
      tryBuild('yarn build:debug', 'Rebuilding module...', spinner)
    }
    tryBuild('yarn build:examples', 'Rebuilding examples...', spinner)
    spinner.println('[Archons] Build complete.\n\n')
    spinner.println('[Archons] Watching for changes...')
    spinner.finishAndClear()
    lastBuild = Date.now()
  }
})
