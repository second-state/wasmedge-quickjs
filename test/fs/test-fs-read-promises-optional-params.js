// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';
import fixtures from '../common/fixtures';
import fs from 'fs';
import { promisify } from 'util';
const read = promisify(fs.read);
import assert from 'assert';
const filepath = fixtures.path('x.txt');
const fd = fs.openSync(filepath, 'r');

const expected = Buffer.from('xyz\n');
const defaultBufferAsync = Buffer.alloc(16384);
const bufferAsOption = Buffer.allocUnsafe(expected.byteLength);

read(fd, common.mustNotMutateObjectDeep({}))
  .then(function({ bytesRead, buffer }) {
    assert.strictEqual(bytesRead, expected.byteLength);
    assert.deepStrictEqual(defaultBufferAsync.byteLength, buffer.byteLength);
  })
  .then(common.mustCall());

read(fd, bufferAsOption, common.mustNotMutateObjectDeep({ position: 0 }))
  .then(function({ bytesRead, buffer }) {
    assert.strictEqual(bytesRead, expected.byteLength);
    assert.deepStrictEqual(bufferAsOption.byteLength, buffer.byteLength);
  })
  .then(common.mustCall());
