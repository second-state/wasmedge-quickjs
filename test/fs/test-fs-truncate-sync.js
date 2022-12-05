// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import assert from 'assert';
import path from 'path';
import fs from 'fs';
import tmpdir from '../common/tmpdir';
const tmp = tmpdir.path;

tmpdir.refresh();

const filename = path.resolve(tmp, 'truncate-sync-file.txt');

fs.writeFileSync(filename, 'hello world', 'utf8');

const fd = fs.openSync(filename, 'r+');

fs.truncateSync(fd, 5);
assert(fs.readFileSync(fd).equals(Buffer.from('hello')));

fs.closeSync(fd);
fs.unlinkSync(filename);
