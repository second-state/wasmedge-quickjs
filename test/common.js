// Copyright Joyent, Inc. and other Node contributors.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit
// persons to whom the Software is furnished to do so, subject to the
// following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN
// NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.

/* eslint-disable node-core/crypto-check */
'use strict';

import { inspect } from "internal/util/inspect";

import assert, { AssertionError } from "assert";
import process from "process";

const isWindows = process.platform === 'win32';
const isAIX = process.platform === 'aix';
const isSunOS = process.platform === 'sunos';
const isFreeBSD = process.platform === 'freebsd';
const isOpenBSD = process.platform === 'openbsd';
const isLinux = process.platform === 'linux';
const isOSX = process.platform === 'darwin';
const isPi = false;

const isDumbTerminal = process.env.TERM === 'dumb';

export function mustCall(fn) {
  return fn;
}

export function mustCallAtLeast(fn) {
  return fn;
}

export function mustNotCall() {
  return () => {
    assert(false, "mustNotCall");
  };
}

export function mustNotMutateObjectDeep(obj = {}) {
  return obj;
}

export function mustSucceed(fn) {
  return (err, ...args) => {
    assert.equal(err, null);
    fn(...args)
  };
}

export function invalidArgTypeHelper(input) {
  if (input == null) {
    return ` Received ${input}`;
  }
  if (typeof input === 'function' && input.name) {
    return ` Received function ${input.name}`;
  }
  if (typeof input === 'object') {
    if (input.constructor?.name) {
      return ` Received an instance of ${input.constructor.name}`;
    }
    return ` Received ${inspect(input, { depth: -1 })}`;
  }

  let inspected = inspect(input, { colors: false });
  if (inspected.length > 28) { inspected = `${inspected.slice(inspected, 0, 25)}...`; }

  return ` Received type ${typeof input} (${inspected})`;
}

export function skip(msg) {
  print("skip, ", msg);
}

const common = {
  isDumbTerminal,
  isFreeBSD,
  isLinux,
  isOpenBSD,
  isOSX,
  isPi,
  isSunOS,
  isWindows,
  isAIX,
  mustCall,
  mustCallAtLeast,
  mustNotCall,
  mustNotMutateObjectDeep,
  skip,
  mustSucceed,
  invalidArgTypeHelper
};

export default common;
