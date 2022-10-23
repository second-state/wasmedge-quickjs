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
const isMainThread = true;
const isDumbTerminal = process.env.TERM === 'dumb';

export function mustCall(fn) {
  return typeof (fn) === "function" ? fn : (() => { });
}

export function mustCallAtLeast(fn) {
  return typeof (fn) === "function" ? fn : (() => { });
}

function mustNotCall(msg) {
  const callSite = new Error().stack;
  return function mustNotCall(...args) {
    const argsInfo = args.length > 0 ?
      `\ncalled with arguments: ${args.map((arg) => inspect(arg)).join(', ')}` : '';
    assert.fail(
      `${msg || 'function should not have been called'} at ${callSite}` +
      argsInfo);
  };
}

const _mustNotMutateObjectDeepProxies = new WeakMap();

export function mustNotMutateObjectDeep(original) {
  // Return primitives and functions directly. Primitives are immutable, and
  // proxied functions are impossible to compare against originals, e.g. with
  // `assert.deepEqual()`.
  if (original === null || typeof original !== 'object') {
    return original;
  }

  const cachedProxy = _mustNotMutateObjectDeepProxies.get(original);
  if (cachedProxy) {
    return cachedProxy;
  }

  const _mustNotMutateObjectDeepHandler = {
    __proto__: null,
    defineProperty(target, property, descriptor) {
      assert.fail(`Expected no side effects, got ${inspect(property)} ` +
                  'defined');
    },
    deleteProperty(target, property) {
      assert.fail(`Expected no side effects, got ${inspect(property)} ` +
                  'deleted');
    },
    get(target, prop, receiver) {
      return mustNotMutateObjectDeep(Reflect.get(target, prop, receiver));
    },
    preventExtensions(target) {
      assert.fail('Expected no side effects, got extensions prevented on ' +
                  inspect(target));
    },
    set(target, property, value, receiver) {
      assert.fail(`Expected no side effects, got ${inspect(value)} ` +
                  `assigned to ${inspect(property)}`);
    },
    setPrototypeOf(target, prototype) {
      assert.fail(`Expected no side effects, got set prototype to ${prototype}`);
    }
  };

  const proxy = new Proxy(original, _mustNotMutateObjectDeepHandler);
  _mustNotMutateObjectDeepProxies.set(original, proxy);
  return proxy;
}

export function mustSucceed(fn) {
  return mustCall((err, ...args) => {
    if (err !== null) {
      print(err);
      print(err.stack);
    }
    assert.ifError(err);
    if (typeof (fn) === "function") {
      fn(...args);
    }
  });
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

export function platformTimeout(ms) {
  return ms;
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
  isMainThread,
  mustCall,
  mustCallAtLeast,
  mustNotCall,
  mustNotMutateObjectDeep,
  skip,
  mustSucceed,
  invalidArgTypeHelper,
  platformTimeout
};

export default common;
