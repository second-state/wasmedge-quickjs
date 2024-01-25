// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import { Buffer } from 'buffer';

import {
  validateFunction,
  validateInteger,
  validateInt32,
  validateUint32,
} from '../validators';

import {
  ERR_CRYPTO_SCRYPT_INVALID_PARAMETER,
  ERR_CRYPTO_SCRYPT_NOT_SUPPORTED,
} from '../errors';

import {
  getArrayBufferOrView,
  getDefaultEncoding,
} from './util';

import { scrypt_sync } from "_node:crypto";

const defaults = {
  N: 16384,
  r: 8,
  p: 1,
  maxmem: 32 << 20,  // 32 MiB, matches SCRYPT_MAX_MEM.
};

function scrypt(password, salt, keylen, options, callback = defaults) {
  if (callback === defaults) {
    callback = options;
    options = defaults;
  }

  options = check(password, salt, keylen, options);
  const { N, r, p, maxmem } = options;
  ({ password, salt, keylen } = options);

  validateFunction(callback, 'callback');
  const encoding = getDefaultEncoding();
  setTimeout(() => {
    let result = scrypt_sync(password.buffer ?? password, salt.buffer ?? salt, N, r, p, keylen);
    const buf = Buffer.from(result);
    if (encoding === 'buffer') {
      callback(null, buf);
    } else {
      callback(null, buf.toString(encoding));
    }
  }, 0);
}

function scryptSync(password, salt, keylen, options = defaults) {
  options = check(password, salt, keylen, options);
  const { N, r, p, maxmem } = options;
  ({ password, salt, keylen } = options);

  let result = scrypt_sync(password.buffer ?? password, salt.buffer ?? salt, N, r, p, keylen);

  const buf = Buffer.from(result);
  const encoding = getDefaultEncoding();
  return encoding === 'buffer' ? buf : buf.toString(encoding);
}

function check(password, salt, keylen, options) {
  /*if (ScryptJob === undefined)
    throw new ERR_CRYPTO_SCRYPT_NOT_SUPPORTED();*/

  password = getArrayBufferOrView(password, 'password');
  salt = getArrayBufferOrView(salt, 'salt');
  validateInt32(keylen, 'keylen', 0);

  let { N, r, p, maxmem } = defaults;
  if (options && options !== defaults) {
    const has_N = options.N !== undefined;
    if (has_N) {
      N = options.N;
      validateUint32(N, 'N');
    }
    if (options.cost !== undefined) {
      if (has_N) throw new ERR_CRYPTO_SCRYPT_INVALID_PARAMETER();
      N = options.cost;
      validateUint32(N, 'cost');
    }
    const has_r = (options.r !== undefined);
    if (has_r) {
      r = options.r;
      validateUint32(r, 'r');
    }
    if (options.blockSize !== undefined) {
      if (has_r) throw new ERR_CRYPTO_SCRYPT_INVALID_PARAMETER();
      r = options.blockSize;
      validateUint32(r, 'blockSize');
    }
    const has_p = options.p !== undefined;
    if (has_p) {
      p = options.p;
      validateUint32(p, 'p');
    }
    if (options.parallelization !== undefined) {
      if (has_p) throw new ERR_CRYPTO_SCRYPT_INVALID_PARAMETER();
      p = options.parallelization;
      validateUint32(p, 'parallelization');
    }
    if (options.maxmem !== undefined) {
      maxmem = options.maxmem;
      validateInteger(maxmem, 'maxmem', 0);
    }
    if (N === 0) N = defaults.N;
    if (r === 0) r = defaults.r;
    if (p === 0) p = defaults.p;
    if (maxmem === 0) maxmem = defaults.maxmem;
  }

  if (Math.log2(N) % 1 !== 0 || N <= 1) {
    throw new ERR_CRYPTO_SCRYPT_INVALID_PARAMETER();
  }

  let blen = p * 128 * r
  let vlen = 32 * r * (N + 2) * 4
  if (vlen + blen > maxmem || 128 * N * r > maxmem || N >= 2 ** (r * 16) || p > (2 ** 30 - 1) / r) {
    throw new ERR_CRYPTO_SCRYPT_INVALID_PARAMETER();
  }

  return { password, salt, keylen, N, r, p, maxmem };
}

export {
  scrypt,
  scryptSync,
};
