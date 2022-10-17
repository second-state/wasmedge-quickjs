// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';
import assert from 'assert';
import fs from 'fs';

const encoding = 'foo-8';
const filename = 'bar.txt';
assert.throws(
  () => fs.readFile(filename, { encoding }, common.mustNotCall()),
  { code: 'ERR_INVALID_ARG_VALUE', name: 'TypeError' }
);
