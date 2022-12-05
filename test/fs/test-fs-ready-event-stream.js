// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import assert from 'assert';
import fs from 'fs';
import path from 'path';
import tmpdir from '../common/tmpdir';

const __filename = args[0];

const readStream = fs.createReadStream(__filename);
assert.strictEqual(readStream.pending, true);
readStream.on('ready', common.mustCall(() => {
  assert.strictEqual(readStream.pending, false);
}));

const writeFile = path.join(tmpdir.path, 'write-fsreadyevent.txt');
tmpdir.refresh();
const writeStream = fs.createWriteStream(writeFile, { autoClose: true });
assert.strictEqual(writeStream.pending, true);
writeStream.on('ready', common.mustCall(() => {
  assert.strictEqual(writeStream.pending, false);
  writeStream.end();
}));
