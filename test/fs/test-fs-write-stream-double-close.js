// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';
import assert from 'assert';
import fs from 'fs';
import path from 'path';

import tmpdir from '../common/tmpdir';
tmpdir.refresh();

{
  const s = fs.createWriteStream(path.join(tmpdir.path, 'rw'));

  s.close(common.mustCall());
  s.close(common.mustCall());
}

{
  const s = fs.createWriteStream(path.join(tmpdir.path, 'rw2'));

  let emits = 0;
  s.on('close', () => {
    emits++;
  });

  s.close(common.mustCall(() => {
    assert.strictEqual(emits, 1);
    s.close(common.mustCall(() => {
      assert.strictEqual(emits, 1);
    }));
    process.nextTick(() => {
      s.close(common.mustCall(() => {
        assert.strictEqual(emits, 1);
      }));
    });
  }));
}

{
  const s = fs.createWriteStream(path.join(tmpdir.path, 'rw'), {
    autoClose: false
  });

  s.close(common.mustCall());
  s.close(common.mustCall());
}
