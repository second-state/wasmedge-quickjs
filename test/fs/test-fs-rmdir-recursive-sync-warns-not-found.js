// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import tmpdir from '../common/tmpdir';
import assert from 'assert';
import fs from 'fs';
import path from 'path';

tmpdir.refresh();

{
  // Should warn when trying to delete a nonexistent path
  common.expectWarning(
    'DeprecationWarning',
    'In future versions of Node.js, fs.rmdir(path, { recursive: true }) ' +
      'will be removed. Use fs.rm(path, { recursive: true }) instead',
    'DEP0147'
  );
  assert.throws(
    () => fs.rmdirSync(path.join(tmpdir.path, 'noexist.txt'),
                       { recursive: true }),
    { code: 'ENOENT' }
  );
}
