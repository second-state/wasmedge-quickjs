// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';
import assert from 'assert';
import path from 'path';
import { open, readFile } from 'fs/promises';
import tmpdir from '../common/tmpdir';

tmpdir.refresh();

async function validateTruncate() {
  const text = 'Hello world';
  const filename = path.resolve(tmpdir.path, 'truncate-file.txt');
  const fileHandle = await open(filename, 'w+');

  const buffer = Buffer.from(text, 'utf8');
  await fileHandle.write(buffer, 0, buffer.length);

  assert.strictEqual((await readFile(filename)).toString(), text);

  await fileHandle.truncate(5);
  assert.strictEqual((await readFile(filename)).toString(), 'Hello');

  await fileHandle.close();
}

validateTruncate().then(common.mustCall());
