// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import fs from 'fs';

common.expectWarning(
  'DeprecationWarning',
  'ReadStream.prototype.open() is deprecated', 'DEP0135');
const s = fs.createReadStream('asd')
  // We don't care about errors in this test.
  .on('error', () => {});
s.open();

process.nextTick(() => {
  // Allow overriding open().
  fs.ReadStream.prototype.open = common.mustCall();
  fs.createReadStream('asd');
});
