// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';
if (!common.isWindows)
  common.skip('Test for Windows only');

import fixtures from '../common/fixtures';

import assert from 'assert';
import fs from 'fs';
import { spawnSync } from 'child_process';

let result;

// Create a subst drive
const driveLetters = 'ABCDEFGHIJKLMNOPQRSTUWXYZ';
let drive;
let i;
for (i = 0; i < driveLetters.length; ++i) {
  drive = `${driveLetters[i]}:`;
  result = spawnSync('subst', [drive, fixtures.fixturesDir]);
  if (result.status === 0)
    break;
}
if (i === driveLetters.length)
  common.skip('Cannot create subst drive');

// Schedule cleanup (and check if all callbacks where called)
process.on('exit', function() {
  spawnSync('subst', ['/d', drive]);
});

// test:
const filename = `${drive}\\empty.js`;
const filenameBuffer = Buffer.from(filename);

result = fs.realpathSync(filename);
assert.strictEqual(result, filename);

result = fs.realpathSync(filename, 'buffer');
assert(Buffer.isBuffer(result));
assert(result.equals(filenameBuffer));

fs.realpath(filename, common.mustSucceed((result) => {
  assert.strictEqual(result, filename);
}));

fs.realpath(filename, 'buffer', common.mustSucceed((result) => {
  assert(Buffer.isBuffer(result));
  assert(result.equals(filenameBuffer));
}));
