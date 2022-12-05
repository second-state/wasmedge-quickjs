// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import assert from 'assert';
import fs from 'fs';
import path from 'path';

import tmpdir from '../common/tmpdir';
tmpdir.refresh();

// O_WRONLY without O_CREAT shall fail with ENOENT
const pathNE = path.join(tmpdir.path, 'file-should-not-exist');
assert.throws(
  () => fs.openSync(pathNE, fs.constants.O_WRONLY),
  (e) => e.code === 'ENOENT'
);
