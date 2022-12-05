// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';
import common from '../common';

import assert from 'assert';
import fs from 'fs';

const watch = fs.watchFile(__filename, common.mustNotCall());
let triggered;
const listener = common.mustCall(() => {
  triggered = true;
});

triggered = false;
watch.once('stop', listener);  // Should trigger.
watch.stop();
assert.strictEqual(triggered, false);
setImmediate(() => {
  assert.strictEqual(triggered, true);
  watch.removeListener('stop', listener);
});
