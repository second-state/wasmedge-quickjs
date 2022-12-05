// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import assert from 'assert';
import fs from 'fs';

[Infinity, -Infinity, NaN].forEach((input) => {
  assert.throws(
    () => {
      fs._toUnixTimestamp(input);
    },
    {
      code: 'ERR_INVALID_ARG_TYPE',
      name: 'TypeError'
    });
});

assert.throws(
  () => {
    fs._toUnixTimestamp({});
  },
  {
    code: 'ERR_INVALID_ARG_TYPE',
    name: 'TypeError'
  });

const okInputs = [1, -1, '1', '-1', Date.now()];
okInputs.forEach((input) => {
  fs._toUnixTimestamp(input);
});
