// Copyright Joyent and Node contributors. All rights reserved. MIT license.

'use strict';

import common from '../common';

// The following tests validate base functionality for the fs.promises
// FileHandle.stat method.

import { open } from 'fs/promises';
import path from 'path';
import tmpdir from '../common/tmpdir';
import assert from 'assert';

tmpdir.refresh();

async function validateStat() {
  const filePath = path.resolve(tmpdir.path, 'tmp-read-file.txt');
  const fileHandle = await open(filePath, 'w+');
  const stats = await fileHandle.stat();
  assert.ok(stats.mtime instanceof Date);
  await fileHandle.close();
}

validateStat()
  .then(common.mustCall());
