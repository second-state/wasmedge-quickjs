// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';
import assert from 'assert';
import path from 'path';
import fs from 'fs';
import tmpdir from '../common/tmpdir';
const d = path.join(tmpdir.path, 'dir');

tmpdir.refresh();

// Make sure the directory does not exist
assert(!fs.existsSync(d));
// Create the directory now
fs.mkdirSync(d);
// Make sure the directory exists
assert(fs.existsSync(d));
// Try creating again, it should fail with EEXIST
assert.throws(function() {
  fs.mkdirSync(d);
}, /EEXIST: file already exists, mkdir/);
// Remove the directory now
fs.rmdirSync(d);
// Make sure the directory does not exist
assert(!fs.existsSync(d));

// Similarly test the Async version
fs.mkdir(d, 0o666, common.mustSucceed(() => {
  fs.mkdir(d, 0o666, common.mustCall(function(err) {
    assert.strictEqual(this, undefined);
    assert.ok(err, 'got no error');
    assert.match(err.message, /^EEXIST/);
    assert.strictEqual(err.code, 'EEXIST');
    assert.strictEqual(err.path, d);

    fs.rmdir(d, assert.ifError);
  }));
}));
