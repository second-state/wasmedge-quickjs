// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import {
  getArrayBufferOrView,
  getDefaultEncoding,
  getStringOption,
  jobPromise,
  normalizeAlgorithm,
  normalizeHashName,
  validateMaxBufferLength,
  kHandle,
  getHashes,
} from '../crypto/util';

import {
  prepareSecretKey,
} from '../crypto/keys';

import {
  lazyDOMException,
} from '../util';

import {
  Buffer,
} from '../../buffer';

import {
  ERR_CRYPTO_HASH_FINALIZED,
  ERR_CRYPTO_HASH_UPDATE_FAILED,
  ERR_INVALID_ARG_TYPE,
} from '../errors';

import {
  validateEncoding,
  validateString,
  validateUint32,
} from '../validators';

import {
  isArrayBufferView,
} from '../util/types';

import { LazyTransform } from '../streams/lazy_transform';

const kState = Symbol('kState');
const kFinalized = Symbol('kFinalized');

import {
  JsHash as _Hash,
  JsHmac as _Hmac,
} from "_node:crypto";

function Hash(algorithm, options) {
  if (!(this instanceof Hash))
    return new Hash(algorithm, options);
  if (!(algorithm instanceof _Hash)) {
    validateString(algorithm, 'algorithm');
    if (!getHashes().includes(algorithm.toLowerCase())) {
      throw new Error("Digest method not supported");
    }
  }
  const xofLen = typeof options === 'object' && options !== null ?
    options.outputLength : undefined;
  if (xofLen !== undefined)
    validateUint32(xofLen, 'options.outputLength');
  this[kHandle] = new _Hash(algorithm, xofLen);
  this[kState] = {
    [kFinalized]: false
  };
  Reflect.apply(LazyTransform, this, [options]);
}

Object.setPrototypeOf(Hash.prototype, LazyTransform.prototype);
Object.setPrototypeOf(Hash, LazyTransform);

Hash.prototype.copy = function copy(options) {
  const state = this[kState];
  if (state[kFinalized])
    throw new ERR_CRYPTO_HASH_FINALIZED();

  return new Hash(this[kHandle], options);
};

Hash.prototype._transform = function _transform(chunk, encoding, callback) {
  this.update(chunk, encoding);
  callback();
};

Hash.prototype._flush = function _flush(callback) {
  this.push(this.digest());
  callback();
};

Hash.prototype.update = function update(data, encoding) {
  encoding = encoding || getDefaultEncoding();

  const state = this[kState];
  if (state[kFinalized])
    throw new ERR_CRYPTO_HASH_FINALIZED();

  if (typeof data === 'string') {
    validateEncoding(data, encoding);
  } else if (!isArrayBufferView(data)) {
    throw new ERR_INVALID_ARG_TYPE(
      'data', ['string', 'Buffer', 'TypedArray', 'DataView'], data);
  }
  let buffer = getArrayBufferOrView(data, "data", encoding);
  if (!this[kHandle].update(buffer.buffer ?? buffer))
    throw new ERR_CRYPTO_HASH_UPDATE_FAILED();
  return this;
};


Hash.prototype.digest = function digest(outputEncoding) {
  const state = this[kState];
  if (state[kFinalized])
    throw new ERR_CRYPTO_HASH_FINALIZED();
  outputEncoding = outputEncoding || getDefaultEncoding();

  // Explicit conversion for backward compatibility.
  const ret = Buffer.from(this[kHandle].digest());
  state[kFinalized] = true;
  return outputEncoding === 'buffer' ? ret : ret.toString(outputEncoding);
};

function Hmac(hmac, key, options) {
  if (!(this instanceof Hmac))
    return new Hmac(hmac, key, options);
  validateString(hmac, 'hmac');
  if (!getHashes().includes(hmac.toLowerCase())) {
    throw new Error("Digest method not supported");
  }
  const encoding = getStringOption(options, 'encoding');
  key = prepareSecretKey(key, encoding);
  if (key.export !== undefined) {
    key = key.export();
  }
  this[kHandle] = new _Hmac(hmac, key.buffer ?? key);
  this[kState] = {
    [kFinalized]: false
  };
  Reflect.apply(LazyTransform, this, [options]);
}

Object.setPrototypeOf(Hmac.prototype, LazyTransform.prototype);
Object.setPrototypeOf(Hmac, LazyTransform);

Hmac.prototype.update = Hash.prototype.update;

Hmac.prototype.digest = function digest(outputEncoding) {
  const state = this[kState];
  outputEncoding = outputEncoding || getDefaultEncoding();

  if (state[kFinalized]) {
    const buf = Buffer.from('');
    return outputEncoding === 'buffer' ? buf : buf.toString(outputEncoding);
  }

  // Explicit conversion for backward compatibility.
  const ret = Buffer.from(this[kHandle].digest());
  state[kFinalized] = true;
  return outputEncoding === 'buffer' ? ret : ret.toString(outputEncoding);;
};

Hmac.prototype._flush = Hash.prototype._flush;
Hmac.prototype._transform = Hash.prototype._transform;

// Implementation for WebCrypto subtle.digest()

export {
  Hash,
  Hmac,
};
