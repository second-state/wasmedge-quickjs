'use strict';

import common from '../common';

import assert from 'assert';
import fs from 'fs';

let __filename = args[0];

const fd = fs.openSync(__filename, 'r');

fs.close(fd, common.mustCall(function(...args) {
  assert.deepStrictEqual(args, [null]);
}));
