'use strict';

import { kEmptyObject } from '../util';

import { Buffer, kMaxLength, FastBuffer } from '../../buffer';

import { ERR_INVALID_ARG_TYPE, ERR_OUT_OF_RANGE, ERR_OPERATION_FAILED } from '../errors';

import { validateNumber, validateBoolean, validateFunction, validateInt32, validateObject, validateUint32 } from '../validators';

import { isArrayBufferView, isAnyArrayBuffer, isTypedArray, isFloat32Array, isFloat64Array } from '../util/types';

import { random_fill } from "_node:crypto";

const kMaxInt32 = 2 ** 31 - 1;
const kMaxPossibleLength = Math.min(kMaxLength, kMaxInt32);

function assertOffset(offset, elementSize, length) {
  validateNumber(offset, 'offset');
  offset *= elementSize;

  const maxLength = Math.min(length, kMaxPossibleLength);
  if (Number.isNaN(offset) || offset > maxLength || offset < 0) {
    throw new ERR_OUT_OF_RANGE('offset', `>= 0 && <= ${maxLength}`, offset);
  }

  return offset >>> 0;  // Convert to uint32.
}

function assertSize(size, elementSize, offset, length) {
  validateNumber(size, 'size');
  size *= elementSize;

  if (Number.isNaN(size) || size > kMaxPossibleLength || size < 0) {
    throw new ERR_OUT_OF_RANGE('size',
      `>= 0 && <= ${kMaxPossibleLength}`, size);
  }

  if (size + offset > length) {
    throw new ERR_OUT_OF_RANGE('size + offset', `<= ${length}`, size + offset);
  }

  return size >>> 0;  // Convert to uint32.
}

function randomBytes(size, callback) {
  size = assertSize(size, 1, 0, Infinity);
  if (callback !== undefined) {
    validateFunction(callback, 'callback');
  }

  const buf = new Buffer(size);

  if (callback === undefined) {
    randomFillSync(buf.buffer, 0, size);
    return buf;
  }

  // Keep the callback as a regular function so this is propagated.
  randomFill(buf.buffer, 0, size, function (error) {
    if (error) return Function.prototype.call.call(callback, this, error);
    Function.prototype.call.call(callback, this, null, buf);
  });
}

function randomFillSync(buf, offset = 0, size) {
  if (!isAnyArrayBuffer(buf) && !isArrayBufferView(buf)) {
    throw new ERR_INVALID_ARG_TYPE(
      'buf',
      ['ArrayBuffer', 'ArrayBufferView'],
      buf);
  }

  const elementSize = buf.BYTES_PER_ELEMENT || 1;

  offset = assertOffset(offset, elementSize, buf.byteLength);

  if (size === undefined) {
    size = buf.byteLength - offset;
  } else {
    size = assertSize(size, elementSize, offset, buf.byteLength);
  }

  if (size === 0)
    return buf;

  random_fill(buf.buffer, offset + (buf.byteOffset ?? 0), size);
  return buf;
}

function randomFill(buf, offset, size, callback) {
  if (!isAnyArrayBuffer(buf) && !isArrayBufferView(buf)) {
    throw new ERR_INVALID_ARG_TYPE(
      'buf',
      ['ArrayBuffer', 'ArrayBufferView'],
      buf);
  }

  const elementSize = buf.BYTES_PER_ELEMENT || 1;

  if (typeof offset === 'function') {
    callback = offset;
    offset = 0;
    // Size is a length here, assertSize() call turns it into a number of bytes
    size = buf.length;
  } else if (typeof size === 'function') {
    callback = size;
    size = buf.length - offset;
  } else {
    validateFunction(callback, 'callback');
  }

  offset = assertOffset(offset, elementSize, buf.byteLength);

  if (size === undefined) {
    size = buf.byteLength - offset;
  } else {
    size = assertSize(size, elementSize, offset, buf.byteLength);
  }

  if (size === 0) {
    callback(null, buf);
    return;
  }

  setTimeout(() => {
    random_fill(buf.buffer, offset + (buf.byteOffset ?? 0), size);
    callback(null, buf);
  }, 0);
}

// Largest integer we can read from a buffer.
// e.g.: Buffer.from("ff".repeat(6), "hex").readUIntBE(0, 6);
const RAND_MAX = 0xFFFF_FFFF_FFFF;

// Cache random data to use in randomInt. The cache size must be evenly
// divisible by 6 because each attempt to obtain a random int uses 6 bytes.
const randomCache = new Buffer(6 * 1024);
let randomCacheOffset = randomCache.length;
let asyncCacheFillInProgress = false;
const asyncCachePendingTasks = [];

// Generates an integer in [min, max) range where min is inclusive and max is
// exclusive.
function randomInt(min, max, callback) {
  // Detect optional min syntax
  // randomInt(max)
  // randomInt(max, callback)
  const minNotSpecified = typeof max === 'undefined' ||
    typeof max === 'function';

  if (minNotSpecified) {
    callback = max;
    max = min;
    min = 0;
  }

  const isSync = typeof callback === 'undefined';
  if (!isSync) {
    validateFunction(callback, 'callback');
  }
  if (!Number.isSafeInteger(min)) {
    throw new ERR_INVALID_ARG_TYPE('min', 'a safe integer', min);
  }
  if (!Number.isSafeInteger(max)) {
    throw new ERR_INVALID_ARG_TYPE('max', 'a safe integer', max);
  }
  if (max <= min) {
    throw new ERR_OUT_OF_RANGE(
      'max', `greater than the value of "min" (${min})`, max
    );
  }

  // First we generate a random int between [0..range)
  const range = max - min;

  if (!(range <= RAND_MAX)) {
    throw new ERR_OUT_OF_RANGE(`max${minNotSpecified ? '' : ' - min'}`,
      `<= ${RAND_MAX}`, range);
  }

  // For (x % range) to produce an unbiased value greater than or equal to 0 and
  // less than range, x must be drawn randomly from the set of integers greater
  // than or equal to 0 and less than randLimit.
  const randLimit = RAND_MAX - (RAND_MAX % range);

  // If we don't have a callback, or if there is still data in the cache, we can
  // do this synchronously, which is super fast.
  while (isSync || (randomCacheOffset < randomCache.length)) {
    if (randomCacheOffset === randomCache.length) {
      // This might block the thread for a bit, but we are in sync mode.
      randomFillSync(randomCache);
      randomCacheOffset = 0;
    }

    const x = randomCache.readUIntBE(randomCacheOffset, 6);
    randomCacheOffset += 6;

    if (x < randLimit) {
      const n = (x % range) + min;
      if (isSync) return n;
      process.nextTick(callback, undefined, n);
      return;
    }
  }

  // At this point, we are in async mode with no data in the cache. We cannot
  // simply refill the cache, because another async call to randomInt might
  // already be doing that. Instead, queue this call for when the cache has
  // been refilled.
  Array.prototype.push.call(asyncCachePendingTasks, { min, max, callback });
  asyncRefillRandomIntCache();
}

function asyncRefillRandomIntCache() {
  if (asyncCacheFillInProgress)
    return;

  asyncCacheFillInProgress = true;
  randomFill(randomCache, (err) => {
    asyncCacheFillInProgress = false;

    const tasks = asyncCachePendingTasks;
    const errorReceiver = err && Array.prototype.shift.call(tasks);
    if (!err)
      randomCacheOffset = 0;

    // Restart all pending tasks. If an error occurred, we only notify a single
    // callback (errorReceiver) about it. This way, every async call to
    // randomInt has a chance of being successful, and it avoids complex
    // exception handling here.
    Array.prototype.forEach.call(Array.prototype.splice.call(tasks, 0), (task) => {
      randomInt(task.min, task.max, task.callback);
    });

    // This is the only call that might throw, and is therefore done at the end.
    if (errorReceiver)
      errorReceiver.callback(err);
  });
}

function lazyDOMException(msg, name) {
    let e = new Error(msg)
    e.name = name;
    return e;
}

// Really just the Web Crypto API alternative
// to require('crypto').randomFillSync() with an
// additional limitation that the input buffer is
// not allowed to exceed 65536 bytes, and can only
// be an integer-type TypedArray.
function getRandomValues(data) {
  if (!isTypedArray(data) ||
    isFloat32Array(data) ||
    isFloat64Array(data)) {
    // Ordinarily this would be an ERR_INVALID_ARG_TYPE. However,
    // the Web Crypto API and web platform tests expect this to
    // be a DOMException with type TypeMismatchError.
    throw lazyDOMException(
      'The data argument must be an integer-type TypedArray',
      'TypeMismatchError');
  }
  if (data.byteLength > 65536) {
    throw lazyDOMException(
      'The requested length exceeds 65,536 bytes',
      'QuotaExceededError');
  }
  randomFillSync(data, 0);
  return data;
}

// Implements an RFC 4122 version 4 random UUID.
// To improve performance, random data is generated in batches
// large enough to cover kBatchSize UUID's at a time. The uuidData
// buffer is reused. Each call to randomUUID() consumes 16 bytes
// from the buffer.

const kBatchSize = 128;
let uuidData;
let uuidNotBuffered;
let uuidBatch = 0;

let hexBytesCache;
function getHexBytes() {
  if (hexBytesCache === undefined) {
    hexBytesCache = new Array(256);
    for (let i = 0; i < hexBytesCache.length; i++) {
      const hex = Number.prototype.toString.call(i, 16);
      hexBytesCache[i] = String.prototype.padStart.call(hex, 2, '0');
    }
  }
  return hexBytesCache;
}

function serializeUUID(buf, offset = 0) {
  const kHexBytes = getHexBytes();
  // xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
  return kHexBytes[buf[offset]] +
    kHexBytes[buf[offset + 1]] +
    kHexBytes[buf[offset + 2]] +
    kHexBytes[buf[offset + 3]] +
    '-' +
    kHexBytes[buf[offset + 4]] +
    kHexBytes[buf[offset + 5]] +
    '-' +
    kHexBytes[(buf[offset + 6] & 0x0f) | 0x40] +
    kHexBytes[buf[offset + 7]] +
    '-' +
    kHexBytes[(buf[offset + 8] & 0x3f) | 0x80] +
    kHexBytes[buf[offset + 9]] +
    '-' +
    kHexBytes[buf[offset + 10]] +
    kHexBytes[buf[offset + 11]] +
    kHexBytes[buf[offset + 12]] +
    kHexBytes[buf[offset + 13]] +
    kHexBytes[buf[offset + 14]] +
    kHexBytes[buf[offset + 15]];
}

function getBufferedUUID() {
  // uuidData ??= secureBuffer(16 * kBatchSize);
  uuidData ??= new Uint8Array(16 * kBatchSize);
  if (uuidData === undefined)
    throw new ERR_OPERATION_FAILED('Out of memory');

  if (uuidBatch === 0) randomFillSync(uuidData);
  uuidBatch = (uuidBatch + 1) % kBatchSize;
  return serializeUUID(uuidData, uuidBatch * 16);
}

function getUnbufferedUUID() {
  // uuidNotBuffered ??= secureBuffer(16);
  uuidNotBuffered ??= new Uint8Array(16);
  if (uuidNotBuffered === undefined)
    throw new ERR_OPERATION_FAILED('Out of memory');
  randomFillSync(uuidNotBuffered);
  return serializeUUID(uuidNotBuffered);
}

function randomUUID(options) {
  if (options !== undefined)
    validateObject(options, 'options');
  const {
    disableEntropyCache = false,
  } = options || kEmptyObject;

  validateBoolean(disableEntropyCache, 'options.disableEntropyCache');

  return disableEntropyCache ? getUnbufferedUUID() : getBufferedUUID();
}

function generatePrime(size, options, callback) {
  validateInt32(size, 'size', 1);
  if (typeof options === 'function') {
    callback = options;
    options = kEmptyObject;
  }
  validateFunction(callback, 'callback');

  throw new Error("crypto.generatePrime is unimplemented");
}

function generatePrimeSync(size, options = kEmptyObject) {
  validateInt32(size, 'size', 1);

  throw new Error("crypto.generatePrimeSync is unimplemented");

}

function unsignedBigIntToBuffer(bigint, name) {
  if (bigint < 0) {
    throw new ERR_OUT_OF_RANGE(name, '>= 0', bigint);
  }

  const hex = bigint.toString(16);
  const padded = hex.padStart(hex.length + (hex.length % 2), 0);
  return Buffer.from(padded, 'hex');
}

function checkPrime(candidate, options = kEmptyObject, callback) {
  if (typeof candidate === 'bigint')
    candidate = unsignedBigIntToBuffer(candidate, 'candidate');
  if (!isAnyArrayBuffer(candidate) && !isArrayBufferView(candidate)) {
    throw new ERR_INVALID_ARG_TYPE(
      'candidate',
      [
        'ArrayBuffer',
        'TypedArray',
        'Buffer',
        'DataView',
        'bigint',
      ],
      candidate
    );
  }
  if (typeof options === 'function') {
    callback = options;
    options = kEmptyObject;
  }
  validateFunction(callback, 'callback');
  validateObject(options, 'options');
  const {
    checks = 0,
  } = options;

  validateUint32(checks, 'options.checks');

  throw new Error("crypto.checkPrime is unimplemented");

}

function checkPrimeSync(candidate, options = kEmptyObject) {
  if (typeof candidate === 'bigint')
    candidate = unsignedBigIntToBuffer(candidate, 'candidate');
  if (!isAnyArrayBuffer(candidate) && !isArrayBufferView(candidate)) {
    throw new ERR_INVALID_ARG_TYPE(
      'candidate',
      [
        'ArrayBuffer',
        'TypedArray',
        'Buffer',
        'DataView',
        'bigint',
      ],
      candidate
    );
  }
  validateObject(options, 'options');
  const {
    checks = 0,
  } = options;

  validateUint32(checks, 'options.checks');

  throw new Error("crypto.checkPrimeSync is unimplemented");
}

export {
  checkPrime,
  checkPrimeSync,
  randomBytes,
  randomFill,
  randomFillSync,
  randomInt,
  getRandomValues,
  randomUUID,
  generatePrime,
  generatePrimeSync,
}

export default {
  checkPrime,
  checkPrimeSync,
  randomBytes,
  randomFill,
  randomFillSync,
  randomInt,
  getRandomValues,
  randomUUID,
  generatePrime,
  generatePrimeSync,
};
