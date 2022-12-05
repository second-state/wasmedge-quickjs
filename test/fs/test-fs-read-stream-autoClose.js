// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';
import fs from 'fs';
import path from 'path';
import assert from 'assert';
import tmpdir from '../common/tmpdir';
const writeFile = path.join(tmpdir.path, 'write-autoClose.txt');
tmpdir.refresh();

const file = fs.createWriteStream(writeFile, { autoClose: true });

file.on('finish', common.mustCall(() => {
  assert.strictEqual(file.destroyed, false);
}));
file.end('asd');
