// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import fs from 'fs';

import tmpdir from '../common/tmpdir';

import assert from 'assert';
import { fork } from 'child_process';

// Run in a child process because 'out' is opened twice, blocking the tmpdir
// and preventing cleanup.
if (process.argv[2] !== 'child') {
  // Parent

  tmpdir.refresh();

  // Run test
  const child = fork(__filename, ['child'], { stdio: 'inherit' });
  child.on('exit', common.mustCall(function(code) {
    assert.strictEqual(code, 0);
  }));

  return;
}

// Child

common.expectWarning(
  'DeprecationWarning',
  'WriteStream.prototype.open() is deprecated', 'DEP0135');
const s = fs.createWriteStream(`${tmpdir.path}/out`);
s.open();

process.nextTick(() => {
  // Allow overriding open().
  fs.WriteStream.prototype.open = common.mustCall();
  fs.createWriteStream('asd');
});
