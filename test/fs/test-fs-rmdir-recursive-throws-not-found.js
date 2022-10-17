// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import tmpdir from '../common/tmpdir';
import assert from 'assert';
import fs from 'fs';
import path from 'path';

tmpdir.refresh();

{
  assert.throws(
    () =>
      fs.rmdirSync(path.join(tmpdir.path, 'noexist.txt'), { recursive: true }),
    {
      code: 'ENOENT',
    }
  );
}
{
  fs.rmdir(
    path.join(tmpdir.path, 'noexist.txt'),
    { recursive: true },
    common.mustCall((err) => {
      assert.strictEqual(err.code, 'ENOENT');
    })
  );
}
{
  assert.rejects(
    () => fs.promises.rmdir(path.join(tmpdir.path, 'noexist.txt'),
                            { recursive: true }),
    {
      code: 'ENOENT',
    }
  ).then(common.mustCall());
}
