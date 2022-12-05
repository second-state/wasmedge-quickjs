// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';
import fs from 'fs';
const fsPromises = fs.promises;
import path from 'path';
import tmpdir from '../common/tmpdir';
import assert from 'assert';
const tmpDir = tmpdir.path;

tmpdir.refresh();

const dest = path.resolve(tmpDir, 'tmp.txt');
const buffer = Buffer.from('012'.repeat(2 ** 10));

(async () => {
  for (const Constructor of [Uint8Array, Uint16Array, Uint32Array]) {
    const { BYTES_PER_ELEMENT = 1 } = Constructor;
    const array = new Constructor(buffer.buffer, buffer.byteOffset, buffer.byteLength / BYTES_PER_ELEMENT);
    await fsPromises.writeFile(dest, array);
    const data = await fsPromises.readFile(dest);
    assert.deepStrictEqual(data, buffer);
  }
})().then(common.mustCall());
