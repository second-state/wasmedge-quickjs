// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';

if (common.isIBMi)
  common.skip('IBMi does not support `fs.watch()`');

import fs from 'fs';

const watcher = fs.watch(__filename, common.mustNotCall());

watcher.unref();

setTimeout(
  common.mustCall(() => {
    watcher.ref();
    watcher.unref();
  }),
  common.platformTimeout(100)
);
