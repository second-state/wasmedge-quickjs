// Copyright Joyent and Node contributors. All rights reserved. MIT license.
// Flags: --expose-gc --no-warnings --expose-internals
'use strict';

import common from '../common';
import assert from 'assert';
import path from 'path';
import { internalBinding } from 'internal/test/binding';
const fs = internalBinding('fs');
import { stringToFlags } from 'internal/fs/utils';

// Verifies that the FileHandle object is garbage collected and that a
// warning is emitted if it is not closed.

let fdnum;
{
  const ctx = {};
  fdnum = fs.openFileHandle(path.toNamespacedPath(__filename),
                            stringToFlags('r'), 0o666, undefined, ctx).fd;
  assert.strictEqual(ctx.errno, undefined);
}

const deprecationWarning =
  'Closing a FileHandle object on garbage collection is deprecated. ' +
  'Please close FileHandle objects explicitly using ' +
  'FileHandle.prototype.close(). In the future, an error will be ' +
  'thrown if a file descriptor is closed during garbage collection.';

common.expectWarning({
  'internal/test/binding': [
    'These APIs are for internal testing only. Do not use them.',
  ],
  'Warning': [
    `Closing file descriptor ${fdnum} on garbage collection`,
  ],
  'DeprecationWarning': [[deprecationWarning, 'DEP0137']]
});

global.gc();

setTimeout(() => {}, 10);
