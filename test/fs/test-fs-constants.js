// Copyright Joyent and Node contributors. All rights reserved. MIT license.

'use strict';

import fs from 'fs';
import assert from 'assert';

// Check if the two constants accepted by chmod() on Windows are defined.
assert.notStrictEqual(fs.constants.S_IRUSR, undefined);
assert.notStrictEqual(fs.constants.S_IWUSR, undefined);
