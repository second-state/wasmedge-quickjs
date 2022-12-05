// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';

// The following tests validate base functionality for the fs.promises
// FileHandle.appendFile method.

import fs from 'fs';
const { open } = fs.promises;
import path from 'path';
import tmpdir from '../common/tmpdir';
import assert from 'assert';
const tmpDir = tmpdir.path;

tmpdir.refresh();

async function validateAppendBuffer() {
  const filePath = path.resolve(tmpDir, 'tmp-append-file-buffer.txt');
  const fileHandle = await open(filePath, 'a');
  const buffer = Buffer.from('a&Dp'.repeat(100), 'utf8');

  await fileHandle.appendFile(buffer);
  const appendedFileData = fs.readFileSync(filePath);
  assert.deepStrictEqual(appendedFileData, buffer);

  await fileHandle.close();
}

async function validateAppendString() {
  const filePath = path.resolve(tmpDir, 'tmp-append-file-string.txt');
  const fileHandle = await open(filePath, 'a');
  const string = 'x~yz'.repeat(100);

  await fileHandle.appendFile(string);
  const stringAsBuffer = Buffer.from(string, 'utf8');
  const appendedFileData = fs.readFileSync(filePath);
  assert.deepStrictEqual(appendedFileData, stringAsBuffer);

  await fileHandle.close();
}

validateAppendBuffer()
  .then(validateAppendString)
  .then(common.mustCall());
