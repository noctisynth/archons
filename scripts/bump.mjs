import { execSync } from 'child_process';

execSync('npx changeset version', { stdio: 'inherit' });
execSync('npx napi version', { stdio: 'inherit' });
execSync('yarn install', { stdio: 'inherit' });
