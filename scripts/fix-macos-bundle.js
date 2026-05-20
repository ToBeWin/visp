import { existsSync } from 'node:fs';
import { spawnSync } from 'node:child_process';

const appPath = process.argv[2] ?? 'src-tauri/target/release/bundle/macos/Visp.app';
const bundleId = process.argv[3] ?? 'live.visp.translator';

if (process.platform !== 'darwin') {
  console.log('Skipping macOS bundle signing on non-macOS platform.');
  process.exit(0);
}

if (!existsSync(appPath)) {
  console.log(`Skipping macOS bundle signing; app bundle not found: ${appPath}`);
  process.exit(0);
}

const result = spawnSync(
  'codesign',
  ['--force', '--deep', '--sign', '-', '--identifier', bundleId, appPath],
  { stdio: 'inherit' },
);

if (result.status !== 0) {
  process.exit(result.status ?? 1);
}

console.log(`Re-signed ${appPath} with identifier ${bundleId}`);
