// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import assert from 'assert';
import path from 'path';

// Refs: https://github.com/nodejs/node/issues/13683

const relativePath = path.posix.relative('a/b/c', '../../x');
assert.match(relativePath, /^(\.\.\/){3,5}x$/);
