// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import assert from 'assert';
import fs from 'fs';
import fsPromises from 'fs/promises';

assert.strictEqual(fsPromises, fs.promises);
// assert.strictEqual(fsPromises.constants, fs.constants);
