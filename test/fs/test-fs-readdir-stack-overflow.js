// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import assert from 'assert';
import fs from 'fs';

function recurse() {
  fs.readdirSync('.');
  recurse();
}

assert.throws(
  () => recurse(),
  {
    name: 'RangeError',
    message: 'Maximum call stack size exceeded'
  }
);
