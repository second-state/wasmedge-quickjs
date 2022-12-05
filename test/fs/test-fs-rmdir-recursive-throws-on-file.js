// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import tmpdir from '../common/tmpdir';
import assert from 'assert';
import fs from 'fs';
import path from 'path';

tmpdir.refresh();

const code = common.isWindows ? 'ENOENT' : 'ENOTDIR';

{
  const filePath = path.join(tmpdir.path, 'rmdir-recursive.txt');
  fs.writeFileSync(filePath, '');
  assert.throws(() => fs.rmdirSync(filePath, { recursive: true }), { code });
}
{
  const filePath = path.join(tmpdir.path, 'rmdir-recursive.txt');
  fs.writeFileSync(filePath, '');
  fs.rmdir(filePath, { recursive: true }, common.mustCall((err) => {
    assert.strictEqual(err.code, code);
  }));
}
{
  const filePath = path.join(tmpdir.path, 'rmdir-recursive.txt');
  fs.writeFileSync(filePath, '');
  assert.rejects(() => fs.promises.rmdir(filePath, { recursive: true }),
                 { code }).then(common.mustCall());
}
