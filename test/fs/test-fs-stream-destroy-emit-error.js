// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import assert from 'assert';
import fs from 'fs';

import tmpdir from '../common/tmpdir';
tmpdir.refresh();

{
  const stream = fs.createReadStream(__filename);
  stream.on('close', common.mustCall());
  test(stream);
}

{
  const stream = fs.createWriteStream(`${tmpdir.path}/dummy`);
  stream.on('close', common.mustCall());
  test(stream);
}

{
  const stream = fs.createReadStream(__filename, { emitClose: true });
  stream.on('close', common.mustCall());
  test(stream);
}

{
  const stream = fs.createWriteStream(`${tmpdir.path}/dummy2`,
                                      { emitClose: true });
  stream.on('close', common.mustCall());
  test(stream);
}


function test(stream) {
  const err = new Error('DESTROYED');
  stream.on('open', function() {
    stream.destroy(err);
  });
  stream.on('error', common.mustCall(function(err_) {
    assert.strictEqual(err_, err);
  }));
}
