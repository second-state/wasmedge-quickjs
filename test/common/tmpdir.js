'use strict';

import { rmSync as _rmSync, realpathSync, mkdirSync, readdirSync } from 'fs';
import { resolve, join } from 'path';
const isMainThread = undefined;

function rmSync(pathname) {
  _rmSync(pathname, { maxRetries: 3, recursive: true, force: true });
}

import process from 'process';

const testRoot = process.env.NODE_TEST_DIR ?
  realpathSync(process.env.NODE_TEST_DIR) : resolve(__dirname, '..');

// Using a `.` prefixed name, which is the convention for "hidden" on POSIX,
// gets tools to ignore it by default or by simple rules, especially eslint.
const tmpdirName = '.tmp.' +
  (process.env.TEST_SERIAL_ID || process.env.TEST_THREAD_ID || '0');
const tmpPath = join(testRoot, tmpdirName);

let firstRefresh = true;
function refresh() {
  rmSync(tmpPath);
  mkdirSync(tmpPath);

  if (firstRefresh) {
    firstRefresh = false;
    // Clean only when a test uses refresh. This allows for child processes to
    // use the tmpdir and only the parent will clean on exit.
    process.on('exit', onexit);
  }
}

function onexit() {
  // Change directory to avoid possible EBUSY
  if (isMainThread)
    process.chdir(testRoot);

  try {
    rmSync(tmpPath);
  } catch (e) {
    console.error('Can\'t clean tmpdir:', tmpPath);

    const files = readdirSync(tmpPath);
    console.error('Files blocking:', files);

    if (files.some((f) => f.startsWith('.nfs'))) {
      // Warn about NFS "silly rename"
      console.error('Note: ".nfs*" might be files that were open and ' +
                    'unlinked but not closed.');
      console.error('See http://nfs.sourceforge.net/#faq_d2 for details.');
    }

    console.error();
    throw e;
  }
}

export default {
  path: tmpPath,
  refresh
};
