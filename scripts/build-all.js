#!/usr/bin/env node

import { platform } from 'node:os';
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const script =
  platform() === 'win32'
    ? path.join(__dirname, 'build-all.ps1')
    : path.join(__dirname, 'build-all.sh');

const command =
  platform() === 'win32'
    ? 'powershell'
    : 'bash';

const args =
  platform() === 'win32'
    ? ['-ExecutionPolicy', 'Bypass', '-File', script]
    : [script];

const child = spawn(command, args, {
  stdio: 'inherit',
  cwd: path.resolve(__dirname, '..'),
});

child.on('exit', (code) => {
  process.exit(code ?? 1);
});
