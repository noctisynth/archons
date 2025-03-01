import { execSync } from 'node:child_process';

execSync('npx changeset version', { stdio: 'inherit' });
execSync('npx napi version', { stdio: 'inherit' });
execSync('pnpm install', { stdio: 'inherit' });
