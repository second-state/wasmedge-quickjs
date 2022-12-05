// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import fixtures from '../common/fixtures';
import assert from 'assert';
import fs from 'fs';
const filepath = fixtures.path('x.txt');
const fd = fs.openSync(filepath, 'r');
const fsPromises = fs.promises;

const buffer = new Uint8Array();

assert.throws(
  () => fs.readSync(fd, buffer, 0, 10, 0),
  {
    code: 'ERR_INVALID_ARG_VALUE',
    message: /is empty and cannot be written/
  }
);

assert.throws(
  () => fs.read(fd, buffer, 0, 1, 0, common.mustNotCall()),
  {
    code: 'ERR_INVALID_ARG_VALUE',
    message: /is empty and cannot be written/
  }
);

(async () => {
  const filehandle = await fsPromises.open(filepath, 'r');
  assert.rejects(
    () => filehandle.read(buffer, 0, 1, 0),
    {
      code: 'ERR_INVALID_ARG_VALUE',
      message: /is empty and cannot be written/
    }
  );
})().then(common.mustCall());
