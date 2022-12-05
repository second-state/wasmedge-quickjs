// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';

// Simulate `cat readfile.js | node readfile.js`

if (common.isWindows || common.isAIX)
  common.skip(`No /dev/stdin on ${process.platform}.`);

import assert from 'assert';
import path from 'path';
import fs from 'fs';

if (process.argv[2] === 'child') {
  process.stdout.write(fs.readFileSync('/dev/stdin', 'utf8'));
  return;
}

import tmpdir from '../common/tmpdir';

const filename = path.join(tmpdir.path, '/readfilesync_pipe_large_test.txt');
const dataExpected = 'a'.repeat(999999);
tmpdir.refresh();
fs.writeFileSync(filename, dataExpected);

import { exec } from 'child_process';
const f = JSON.stringify(__filename);
const node = JSON.stringify(process.execPath);
const cmd = `cat ${filename} | ${node} ${f} child`;
exec(
  cmd,
  { maxBuffer: 1000000 },
  common.mustSucceed((stdout, stderr) => {
    assert.strictEqual(stdout, dataExpected);
    assert.strictEqual(stderr, '');
    console.log('ok');
  })
);
