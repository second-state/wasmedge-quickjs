// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import fs from 'fs';
import assert from 'assert';
import path from 'path';
import tmpdir from '../common/tmpdir';
const file = path.join(tmpdir.path, 'read_stream_filehandle_worker.txt');
const input = 'hello world';
import { Worker, isMainThread, workerData } from 'worker_threads';

if (isMainThread || !workerData) {
  tmpdir.refresh();
  fs.writeFileSync(file, input);

  fs.promises.open(file, 'r').then((handle) => {
    handle.on('close', common.mustNotCall());
    new Worker(__filename, {
      workerData: { handle },
      transferList: [handle]
    });
  });
  fs.promises.open(file, 'r').then(async (handle) => {
    try {
      fs.createReadStream(null, { fd: handle });
      assert.throws(() => {
        new Worker(__filename, {
          workerData: { handle },
          transferList: [handle]
        });
      }, {
        code: 25,
        name: 'DataCloneError',
      });
    } finally {
      await handle.close();
    }
  });
} else {
  let output = '';

  const handle = workerData.handle;
  handle.on('close', common.mustCall());
  const stream = fs.createReadStream(null, { fd: handle });

  stream.on('data', common.mustCallAtLeast((data) => {
    output += data;
  }));

  stream.on('end', common.mustCall(() => {
    handle.close();
    assert.strictEqual(output, input);
  }));

  stream.on('close', common.mustCall());
}
