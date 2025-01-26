// @ts-check
import { execSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import path from 'node:path'
import fs from 'node:fs'
import { createSpinner } from '../index.js'
import chalk from 'chalk'

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
  } catch (e) {
    console.log(e)
    spinner.println('[Archons] Build failed.')
  }
}

function createArchonsSpinner() {
  const spinner = createSpinner()
  spinner.setTemplate('{prefix:.green} {spinner:.yellow} {msg}')
  spinner.setPrefix('[Archons]')
  return spinner
}

const dirPath = path.join(path.dirname(fileURLToPath(import.meta.url)), '..')
const ignored = ['node_modules', '.git', '.github', '.vscode', 'dist', 'build', 'bin', '.md', '.d.ts', 'target', '.cjs']
const greenPrefix = chalk.green('[Archons]')

const spinner = createArchonsSpinner();
spinner.enableSteadyTick(100)
spinner.setMessage('Initial building...')
tryBuild('yarn build:debug', 'Building module...', spinner)
tryBuild('yarn build:examples', 'Building examples...', spinner)
spinner.finishAndClear()
console.clear()
console.log(`${greenPrefix} Build complete.\n`)
console.log(`${greenPrefix} Watching on ${chalk.cyan(dirPath)} for changes...`)

let lastBuild = Date.now()
fs.watch(dirPath, { recursive: true }, (eventType, filename) => {
  // debounce
  if (Date.now() - lastBuild < 1000) {
    return
  }
  if (filename && !isIgnored(filename)) {
    const spinner = createArchonsSpinner()
    spinner.println(`${greenPrefix} File ${chalk.cyan(filename)} was ${eventType}d, rebuilding...`)
    spinner.enableSteadyTick(100)
    if (filename.endsWith('.rs') || filename.endsWith('.toml')) {
      tryBuild('yarn build:debug', 'Rebuilding module...', spinner)
    }
    tryBuild('yarn build:examples', 'Rebuilding examples...', spinner)
    console.clear()
    spinner.println(`${greenPrefix} Build complete.\n\n`)
    spinner.println(`${greenPrefix} Watching for changes...`)
    spinner.finishAndClear()
    lastBuild = Date.now()
  }
})
