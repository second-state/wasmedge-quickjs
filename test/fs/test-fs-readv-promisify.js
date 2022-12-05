// Copyright Joyent and Node contributors. All rights reserved. MIT license.

'use strict';

import common from '../common';
import fixtures from '../common/fixtures';
import fs from 'fs';
import { promisify } from 'util';
let readv = promisify(fs.readv);
import assert from 'assert';
const filepath = fixtures.path('x.txt');
const fd = fs.openSync(filepath, 'r');

const expected = [Buffer.from('xyz\n')];

readv(fd, expected)
  .then(function({ bytesRead, buffers }) {
    assert.deepStrictEqual(bytesRead, expected[0].length);
    assert.deepStrictEqual(buffers, expected);
  })
  .then(common.mustCall());
