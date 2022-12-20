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
} from '../crypto/util';

import {
  createSecretKey,
  isKeyObject,
} from '../keys';

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
} from '../errors';

const validateParameters = hideStackFrames((hash, key, salt, info, length) => {
  validateString(hash, 'digest');
  key = prepareKey(key);
  salt = validateByteSource(salt, 'salt');
  info = validateByteSource(info, 'info');

  validateInteger(length, 'length', 0, kMaxLength);

  if (info.byteLength > 1024) {
    throw ERR_OUT_OF_RANGE(
      'info',
      'must not contain more than 1024 bytes',
      info.byteLength);
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
    return createSecretKey(key);

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

  return createSecretKey(key);
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

  const job = new HKDFJob(kCryptoJobAsync, hash, key, salt, info, length);

  job.ondone = (error, bits) => {
    if (error) return Function.prototype.call.call(callback, job, error);
    Function.prototype.call.call(callback, job, null, bits);
  };

  job.run();
}

function hkdfSync(hash, key, salt, info, length) {
  ({
    hash,
    key,
    salt,
    info,
    length,
  } = validateParameters(hash, key, salt, info, length));

  const job = new HKDFJob(kCryptoJobSync, hash, key, salt, info, length);
  const { 0: err, 1: bits } = job.run();
  if (err !== undefined)
    throw err;

  return bits;
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

module.exports = {
  hkdf,
  hkdfSync,
  hkdfDeriveBits,
};
