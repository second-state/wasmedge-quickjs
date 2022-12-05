// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import assert from 'assert';
import path from 'path';
import fs from 'fs';
import tmpdir from '../common/tmpdir';
const tmp = tmpdir.path;
tmpdir.refresh();
const filename = path.resolve(tmp, 'truncate-file.txt');

fs.writeFileSync(filename, 'hello world', 'utf8');
const fd = fs.openSync(filename, 'r+');

const msg = 'Using fs.truncate with a file descriptor is deprecated.' +
  ' Please use fs.ftruncate with a file descriptor instead.';


common.expectWarning('DeprecationWarning', msg, 'DEP0081');
fs.truncate(fd, 5, common.mustSucceed(() => {
  assert.strictEqual(fs.readFileSync(filename, 'utf8'), 'hello');
}));

globalThis.commonExitCheck = () => {
  fs.closeSync(fd);
  fs.unlinkSync(filename);
  console.log('ok');
};
