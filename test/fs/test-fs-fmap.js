// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import assert from 'assert';
import fs from 'fs';
import { join } from 'path';

const {
  O_CREAT = 0,
  O_RDONLY = 0,
  O_TRUNC = 0,
  O_WRONLY = 0,
  UV_FS_O_FILEMAP = 0
} = fs.constants;

import tmpdir from '../common/tmpdir';
tmpdir.refresh();

// Run this test on all platforms. While UV_FS_O_FILEMAP is only available on
// Windows, it should be silently ignored on other platforms.

const filename = join(tmpdir.path, 'fmap.txt');
const text = 'Memory File Mapping Test';

const mw = UV_FS_O_FILEMAP | O_TRUNC | O_CREAT | O_WRONLY;
const mr = UV_FS_O_FILEMAP | O_RDONLY;

fs.writeFileSync(filename, text, { flag: mw });
const r1 = fs.readFileSync(filename, { encoding: 'utf8', flag: mr });
assert.strictEqual(r1, text);