// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';
import assert from 'assert';
import fs from 'fs';

[false, 1, [], {}, null, undefined].forEach((i) => {
  assert.throws(
    () => fs.rmdir(i, common.mustNotCall()),
    {
      code: 'ERR_INVALID_ARG_TYPE',
      name: 'TypeError'
    }
  );
  assert.throws(
    () => fs.rmdirSync(i),
    {
      code: 'ERR_INVALID_ARG_TYPE',
      name: 'TypeError'
    }
  );
});
