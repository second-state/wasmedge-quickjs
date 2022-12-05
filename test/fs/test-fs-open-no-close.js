// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

// Refs: https://github.com/nodejs/node/issues/34266
// Failing to close a file should not keep the event loop open.

import common from '../common';
import assert from 'assert';

import fs from 'fs';

import process from 'process';

const debuglog = (arg) => {
  console.log(new Date().toLocaleString(), arg);
};

import tmpdir from '../common/tmpdir';
tmpdir.refresh();

let openFd;

fs.open(`${tmpdir.path}/dummy`, 'wx+', common.mustCall((err, fd) => {
  debuglog('fs open() callback');
  assert.ifError(err);
  openFd = fd;
}));
debuglog('waiting for callback');

// test_fs's runner will invoke this
globalThis._onExit = common.mustCall(() => {
  if (openFd) {
    fs.closeSync(openFd);
  }
});
