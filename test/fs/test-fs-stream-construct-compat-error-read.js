// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';
import fs from 'fs';
import assert from 'assert';

import tmpdir from '../common/tmpdir';
tmpdir.refresh();

{
  // Compat error.

  function ReadStream(...args) {
    fs.ReadStream.call(this, ...args);
  }
  Object.setPrototypeOf(ReadStream.prototype, fs.ReadStream.prototype);
  Object.setPrototypeOf(ReadStream, fs.ReadStream);

  ReadStream.prototype.open = common.mustCall(function ReadStream$open() {
    const that = this;
    fs.open(that.path, that.flags, that.mode, (err, fd) => {
      that.emit('error', err);
    });
  });

  const r = new ReadStream('/doesnotexist', { emitClose: true })
    .on('error', common.mustCall((err) => {
      assert.strictEqual(err.code, 'ENOENT');
      assert.strictEqual(r.destroyed, true);
      r.on('close', common.mustCall());
    }));
}
