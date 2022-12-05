// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import fs from 'fs';

if (!common.isOSX) {
  common.skip('this tests works only on MacOS');
}

import assert from 'assert';

fs.readdir(
  Buffer.from('/dev'),
  { withFileTypes: true, encoding: 'buffer' },
  common.mustCall((e, d) => {
    assert.strictEqual(e, null);
  })
);
