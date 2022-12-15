// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';


import { Buffer } from 'buffer';

import { validateFunction, validateInteger, validateString, validateUint32 } from '../validators';

import { ERR_CRYPTO_INVALID_DIGEST, ERR_MISSING_OPTION } from '../errors';

import { getArrayBufferOrView, getDefaultEncoding, normalizeHashName, kKeyObject } from './util';

import { lazyDOMException } from '../util';

import { pbkdf2_sync } from "_node:crypto";

export function pbkdf2(password, salt, iterations, keylen, digest, callback) {
  if (typeof digest === 'function') {
    callback = digest;
    digest = undefined;
  }

  ({ password, salt, iterations, keylen, digest } =
    check(password, salt, iterations, keylen, digest));

  validateFunction(callback, 'callback');

  if (!["SHA256", "SHA512"].includes(digest.toUpperCase())) {
    throw new ERR_CRYPTO_INVALID_DIGEST(digest);
  }

  const encoding = getDefaultEncoding();

  setTimeout(() => {
    let result = pbkdf2_sync(password.buffer ?? password, salt.buffer ?? salt, iterations, keylen, digest.toUpperCase());
    const buf = Buffer.from(result);
    if (encoding === 'buffer') {
      callback(null, buf);
    } else {
      callback(null, buf.toString(encoding));
    }
  }, 0);
}

export function pbkdf2Sync(password, salt, iterations, keylen, digest) {
  ({ password, salt, iterations, keylen, digest } =
    check(password, salt, iterations, keylen, digest));

  if (!["SHA256", "SHA512"].includes(digest.toUpperCase())) {
    throw new ERR_CRYPTO_INVALID_DIGEST(digest);
  }

  let result = pbkdf2_sync(password.buffer ?? password, salt.buffer ?? salt, iterations, keylen, digest.toUpperCase());

  const buf = Buffer.from(result);
  const encoding = getDefaultEncoding();
  return encoding === 'buffer' ? buf : buf.toString(encoding);
}

function check(password, salt, iterations, keylen, digest) {
  validateString(digest, 'digest');

  password = getArrayBufferOrView(password, 'password');
  salt = getArrayBufferOrView(salt, 'salt');
  validateUint32(iterations, 'iterations', true);
  validateUint32(keylen, 'keylen');

  return { password, salt, iterations, keylen, digest };
}

export async function pbkdf2DeriveBits(algorithm, baseKey, length) {
  const { iterations } = algorithm;
  let { hash } = algorithm;
  const salt = getArrayBufferOrView(algorithm.salt, 'algorithm.salt');
  if (hash === undefined)
    throw new ERR_MISSING_OPTION('algorithm.hash');
  validateInteger(iterations, 'algorithm.iterations');
  if (iterations === 0)
    throw lazyDOMException(
      'iterations cannot be zero',
      'OperationError');

  hash = normalizeHashName(hash.name);

  const raw = baseKey[kKeyObject].export();

  let byteLength = 64;  // the default
  if (length !== undefined) {
    if (length === 0)
      throw lazyDOMException('length cannot be zero', 'OperationError');
    if (length === null)
      throw lazyDOMException('length cannot be null', 'OperationError');
    validateUint32(length, 'length');
    if (length % 8) {
      throw lazyDOMException(
        'length must be a multiple of 8',
        'OperationError');
    }
    byteLength = length / 8;
  }

  return new Promise((resolve, reject) => {
    pbkdf2(raw, salt, iterations, byteLength, hash, (err, result) => {
      if (err) return reject(err);
      resolve(result.buffer);
    });
  });
}

export default {
  pbkdf2,
  pbkdf2Sync,
  pbkdf2DeriveBits,
};
