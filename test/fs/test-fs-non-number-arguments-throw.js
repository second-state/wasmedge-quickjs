// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';
import assert from 'assert';
import fs from 'fs';
import path from 'path';
import tmpdir from '../common/tmpdir';
const tempFile = path.join(tmpdir.path, 'fs-non-number-arguments-throw');

tmpdir.refresh();
fs.writeFileSync(tempFile, 'abc\ndef');

// A sanity check when using numbers instead of strings
const sanity = 'def';
const saneEmitter = fs.createReadStream(tempFile, { start: 4, end: 6 });

assert.throws(
  () => {
    fs.createReadStream(tempFile, { start: '4', end: 6 });
  },
  {
    code: 'ERR_INVALID_ARG_TYPE',
    name: 'TypeError'
  });

assert.throws(
  () => {
    fs.createReadStream(tempFile, { start: 4, end: '6' });
  },
  {
    code: 'ERR_INVALID_ARG_TYPE',
    name: 'TypeError'
  });

assert.throws(
  () => {
    fs.createWriteStream(tempFile, { start: '4' });
  },
  {
    code: 'ERR_INVALID_ARG_TYPE',
    name: 'TypeError'
  });

saneEmitter.on('data', common.mustCall(function(data) {
  assert.strictEqual(
    sanity, data.toString('utf8'),
    `read ${data.toString('utf8')} instead of ${sanity}`);
}));
