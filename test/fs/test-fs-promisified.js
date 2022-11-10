// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import assert from 'assert';
import fs from 'fs';
import path from 'path';
import { promisify } from 'util';

const read = promisify(fs.read);
const write = promisify(fs.write);
const exists = promisify(fs.exists);

const __filename = args[0];

{
  const fd = fs.openSync(__filename, 'r');
  read(fd, Buffer.alloc(1024), 0, 1024, null).then(common.mustCall((obj) => {
    assert.strictEqual(typeof obj.bytesRead, 'number');
    assert(obj.buffer instanceof Buffer);
    fs.closeSync(fd);
  }));
}

import tmpdir from '../common/tmpdir';
tmpdir.refresh();
{
  const filename = path.join(tmpdir.path, 'write-promise.txt');
  const fd = fs.openSync(filename, 'w');
  write(fd, Buffer.from('foobar')).then(common.mustCall((obj) => {
    assert.strictEqual(typeof obj.bytesWritten, 'number');
    assert.strictEqual(obj.buffer.toString(), 'foobar');
    fs.closeSync(fd);
  }));
}

{
  exists(__filename).then(common.mustCall((x) => {
    assert.strictEqual(x, true);
  }));
}
