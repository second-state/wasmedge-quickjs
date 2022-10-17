// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import assert from 'assert';
import fixtures from '../common/fixtures';
import fs from 'fs';
import path from 'path';
import stream from 'stream';
import tmpdir from '../common/tmpdir';
const firstEncoding = 'base64';
const secondEncoding = 'latin1';

const examplePath = fixtures.path('x.txt');
const dummyPath = path.join(tmpdir.path, 'x.txt');

tmpdir.refresh();

const exampleReadStream = fs.createReadStream(examplePath, {
  encoding: firstEncoding
});

const dummyWriteStream = fs.createWriteStream(dummyPath, {
  encoding: firstEncoding
});

exampleReadStream.pipe(dummyWriteStream).on('finish', function() {
  const assertWriteStream = new stream.Writable({
    write: function(chunk, enc, next) {
      const expected = Buffer.from('xyz\n');
      assert(chunk.equals(expected));
    }
  });
  assertWriteStream.setDefaultEncoding(secondEncoding);
  fs.createReadStream(dummyPath, {
    encoding: secondEncoding
  }).pipe(assertWriteStream);
});
