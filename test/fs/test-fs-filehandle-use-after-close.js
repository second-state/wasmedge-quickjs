// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import assert from 'assert';
import fs from 'fs/promises';

(async () => {
  const filehandle = await fs.open(__filename);

  assert.notStrictEqual(filehandle.fd, -1);
  await filehandle.close();
  assert.strictEqual(filehandle.fd, -1);

  // Open another file handle first. This would typically receive the fd
  // that `filehandle` previously used. In earlier versions of Node.js, the
  // .stat() call would then succeed because it still used the original fd;
  // See https://github.com/nodejs/node/issues/31361 for more details.
  const otherFilehandle = await fs.open(process.execPath);

  assert.rejects(() => filehandle.stat(), {
    code: 'EBADF',
    syscall: 'fstat'
  });

  await otherFilehandle.close();
})().then(common.mustCall());
