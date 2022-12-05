// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import assert from 'assert';
import fs from 'fs/promises';
import fixtures from '../common/fixtures';

const fn = fixtures.path('empty.txt');

fs.readFile(fn)
  .then(assert.ok);

fs.readFile(fn, 'utf8')
  .then(assert.strictEqual.bind(this, ''));

fs.readFile(fn, { encoding: 'utf8' })
  .then(assert.strictEqual.bind(this, ''));
