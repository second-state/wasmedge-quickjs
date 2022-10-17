// Copyright Joyent and Node contributors. All rights reserved. MIT license.

'use strict';

// This test ensures that writeSync throws for invalid data input.

import assert from 'assert';
import fs from 'fs';

[
  true, false, 0, 1, Infinity, () => {}, {}, [], undefined, null,
].forEach((value) => {
  assert.throws(
    () => fs.writeSync(1, value),
    { message: /"buffer"/, code: 'ERR_INVALID_ARG_TYPE' }
  );
});
