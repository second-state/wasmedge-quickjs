// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';
import fs from 'fs';
import assert from 'assert';

assert.throws(function() {
  fs.write(null, Buffer.allocUnsafe(1), 0, 1, common.mustNotCall());
}, /TypeError/);

assert.throws(function() {
  fs.write(null, '1', 0, 1, common.mustNotCall());
}, /TypeError/);
