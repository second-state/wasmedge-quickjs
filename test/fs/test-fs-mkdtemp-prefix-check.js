// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import assert from 'assert';
import fs from 'fs';

const prefixValues = [undefined, null, 0, true, false, 1];

function fail(value) {
  assert.throws(
    () => {
      fs.mkdtempSync(value, {});
    },
    {
      code: 'ERR_INVALID_ARG_TYPE',
      name: 'TypeError'
    });
}

function failAsync(value) {
  assert.throws(
    () => {
      fs.mkdtemp(value, common.mustNotCall());
    },
    {
      code: 'ERR_INVALID_ARG_TYPE',
      name: 'TypeError'
    });
}

prefixValues.forEach((prefixValue) => {
  fail(prefixValue);
  failAsync(prefixValue);
});
