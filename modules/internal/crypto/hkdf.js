// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import {
  validateFunction,
  validateInteger,
  validateString,
  validateUint32,
} from '../validators';

import { kMaxLength } from '../../buffer';

import {
  getArrayBufferOrView,
  normalizeHashName,
  toBuf,
  validateByteSource,
  kKeyObject,
  getHashes,
} from '../crypto/util';

import {
  createSecretKey,
  isKeyObject,
} from './keys';

import {
  lazyDOMException,
} from '../util';

import {
  isAnyArrayBuffer,
  isArrayBufferView,
} from '../util/types';

import {
  ERR_INVALID_ARG_TYPE,
  ERR_OUT_OF_RANGE,
  ERR_MISSING_OPTION,
  hideStackFrames,
  ERR_CRYPTO_INVALID_DIGEST,
  ERR_CRYPTO_INVALID_KEYLEN,
} from '../errors';

import { hkdf_sync } from "_node:crypto";

const validateParameters = hideStackFrames((hash, key, salt, info, length) => {
  validateString(hash, 'digest');

  key = prepareKey(key);
  salt = validateByteSource(salt, 'salt');
  info = validateByteSource(info, 'info');

  validateInteger(length, 'length', 0, kMaxLength);
  if (info.byteLength > 1024) {
    throw new ERR_OUT_OF_RANGE(
      'info',
      'must not contain more than 1024 bytes',
      info.byteLength);
  }

  if (!getHashes().includes(hash)) {
    throw new ERR_CRYPTO_INVALID_DIGEST(hash);
  }

  if (hash === "sha256" && length > 255 * 32) {
    throw new ERR_CRYPTO_INVALID_KEYLEN()
  } else if (hash === "sha512" && length > 255 * 64) {
    throw new ERR_CRYPTO_INVALID_KEYLEN()
  }

  return {
    hash,
    key,
    salt,
    info,
    length,
  };
});

function prepareKey(key) {
  if (isKeyObject(key))
    return key;

  if (isAnyArrayBuffer(key))
    return getArrayBufferOrView(key);

  key = toBuf(key);

  if (!isArrayBufferView(key)) {
    throw new ERR_INVALID_ARG_TYPE(
      'ikm',
      [
        'string',
        'SecretKeyObject',
        'ArrayBuffer',
        'TypedArray',
        'DataView',
        'Buffer',
      ],
      key);
  }

  return getArrayBufferOrView(key);
}

function hkdf(hash, key, salt, info, length, callback) {
  ({
    hash,
    key,
    salt,
    info,
    length,
  } = validateParameters(hash, key, salt, info, length));

  validateFunction(callback, 'callback');

  setTimeout(() => {
    let result = hkdf_sync(key.buffer ?? key, salt.buffer ?? salt, info.buffer ?? info, length, hash.toUpperCase());
    callback(null, result);
  }, 0);
}

function hkdfSync(hash, key, salt, info, length) {
  ({
    hash,
    key,
    salt,
    info,
    length,
  } = validateParameters(hash, key, salt, info, length));
  let result = hkdf_sync(key.buffer ?? key, salt.buffer ?? salt, info.buffer ?? info, length, hash.toUpperCase());
  return result;
}

async function hkdfDeriveBits(algorithm, baseKey, length) {
  const { hash } = algorithm;
  const salt = getArrayBufferOrView(algorithm.salt, 'algorithm.salt');
  const info = getArrayBufferOrView(algorithm.info, 'algorithm.info');
  if (hash === undefined)
    throw new ERR_MISSING_OPTION('algorithm.hash');

  let byteLength = 512 / 8;
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
    hkdf(
      normalizeHashName(hash.name),
      baseKey[kKeyObject],
      salt,
      info,
      byteLength,
      (err, bits) => {
        if (err) return reject(err);
        resolve(bits);
      });
  });
}

export {
  hkdf,
  hkdfSync,
  hkdfDeriveBits,
};
