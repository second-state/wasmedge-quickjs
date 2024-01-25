// Copyright Joyent and Node contributors. All rights reserved. MIT license.

'use strict';
import common from '../common';
if (!common.hasCrypto)
  common.skip('missing crypto');

import { randomFillSync } from 'crypto';
import assert from 'assert';

const ab = new ArrayBuffer(20);
const buf = Buffer.from(ab, 10);

const before = buf.toString('hex');

randomFillSync(buf);

const after = buf.toString('hex');

assert.notStrictEqual(before, after);
