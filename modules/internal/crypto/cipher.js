// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

// TODO
const _privateDecrypt = () => { }
const _privateEncrypt = () => { }
const _publicDecrypt = () => { }
const _publicEncrypt = () => { }
const _getCipherInfo = () => { }

import { crypto as crypto_constants } from "../../internal_binding/constants";

const {
  RSA_PKCS1_OAEP_PADDING,
  RSA_PKCS1_PADDING,
} = crypto_constants;

import {
  ERR_CRYPTO_INVALID_STATE,
  ERR_CRYPTO_UNKNOWN_CIPHER,
  ERR_INVALID_ARG_TYPE,
  ERR_INVALID_ARG_VALUE,
} from '../errors';

import {
  validateEncoding,
  validateInt32,
  validateObject,
  validateString,
} from '../validators';

import {
  preparePrivateKey,
  preparePublicOrPrivateKey,
  prepareSecretKey,
} from './keys';

import {
  getDefaultEncoding,
  getArrayBufferOrView,
  getStringOption,
  kHandle,
  getCiphers,
} from './util';

import {
  isArrayBufferView,
} from '../util/types';

import { assert } from '../assert';

import { LazyTransform } from '../streams/lazy_transform';

import { normalizeEncoding } from '../util';

import { StringDecoder } from 'string_decoder';

import { JsCipher as CipherBase } from "_node:crypto";

function rsaFunctionFor(method, defaultPadding, keyType) {
  return (options, buffer) => {
    const { format, type, data, passphrase } =
      keyType === 'private' ?
        preparePrivateKey(options) :
        preparePublicOrPrivateKey(options);
    const padding = options.padding || defaultPadding;
    const { oaepHash, encoding } = options;
    let { oaepLabel } = options;
    if (oaepHash !== undefined)
      validateString(oaepHash, 'key.oaepHash');
    if (oaepLabel !== undefined)
      oaepLabel = getArrayBufferOrView(oaepLabel, 'key.oaepLabel', encoding);
    buffer = getArrayBufferOrView(buffer, 'buffer', encoding);
    return method(data, format, type, passphrase, buffer, padding, oaepHash,
      oaepLabel);
  };
}

const publicEncrypt = rsaFunctionFor(_publicEncrypt, RSA_PKCS1_OAEP_PADDING,
  'public');
const publicDecrypt = rsaFunctionFor(_publicDecrypt, RSA_PKCS1_PADDING,
  'public');
const privateEncrypt = rsaFunctionFor(_privateEncrypt, RSA_PKCS1_PADDING,
  'private');
const privateDecrypt = rsaFunctionFor(_privateDecrypt, RSA_PKCS1_OAEP_PADDING,
  'private');

function getDecoder(decoder, encoding) {
  encoding = normalizeEncoding(encoding);
  decoder = decoder || new StringDecoder(encoding);
  assert(decoder.encoding === encoding, 'Cannot change encoding');
  return decoder;
}

function getUIntOption(options, key) {
  let value;
  if (options && (value = options[key]) != null) {
    if (value >>> 0 !== value)
      throw new ERR_INVALID_ARG_VALUE(`options.${key}`, value);
    return value;
  }
  return -1;
}

function createCipherBase(cipher, credential, options, decipher, iv) {
  const authTagLength = getUIntOption(options, 'authTagLength');
  if (iv === undefined) {
    // this[kHandle].init(cipher, credential, authTagLength);
  } else {
    this[kHandle] = new CipherBase(cipher, credential.buffer ?? credential, iv.buffer ?? iv, authTagLength, decipher);
  }
  this._decoder = null;

  Reflect.apply(LazyTransform, this, [options]);
}

function createCipher(cipher, password, options, decipher) {
  validateString(cipher, 'cipher');
  password = getArrayBufferOrView(password, 'password');

  Reflect.apply(createCipherBase, this, [cipher, password, options, decipher]);
}

function createCipherWithIV(cipher, key, options, decipher, iv) {
  validateString(cipher, 'cipher');
  const encoding = getStringOption(options, 'encoding');
  key = prepareSecretKey(key, encoding);
  iv = iv === null ? null : getArrayBufferOrView(iv, 'iv');
  if (!getCiphers().includes(cipher)) {
    throw new ERR_CRYPTO_UNKNOWN_CIPHER();
  }
  // Zero-sized IV should be rejected in GCM mode. 
  // Wasi-crypto current implemention only support GCM mode,
  // so always check
  if (iv.byteLength === 0) {
    throw new Error("Invalid initialization vector");
  }
  Reflect.apply(createCipherBase, this, [cipher, key, options, decipher, iv]);
}

// The Cipher class is part of the legacy Node.js crypto API. It exposes
// a stream-based encryption/decryption model. For backwards compatibility
// the Cipher class is defined using the legacy function syntax rather than
// ES6 classes.

function Cipher(cipher, password, options) {
  if (!(this instanceof Cipher))
    return new Cipher(cipher, password, options);

  Reflect.apply(createCipher, this, [cipher, password, options, true]);
}

Object.setPrototypeOf(Cipher.prototype, LazyTransform.prototype);
Object.setPrototypeOf(Cipher, LazyTransform);

Cipher.prototype._transform = function _transform(chunk, encoding, callback) {
  this.push(this.update(chunk, encoding));
  callback();
};

Cipher.prototype._flush = function _flush(callback) {
  try {
    this.push(this.final());
  } catch (e) {
    callback(e);
    return;
  }
  callback();
};

Cipher.prototype.update = function update(data, inputEncoding, outputEncoding) {
  const encoding = getDefaultEncoding();
  inputEncoding = inputEncoding || encoding;
  outputEncoding = outputEncoding || encoding;

  if (typeof data === 'string') {
    validateEncoding(data, inputEncoding);
  } else if (!isArrayBufferView(data)) {
    throw new ERR_INVALID_ARG_TYPE(
      'data', ['string', 'Buffer', 'TypedArray', 'DataView'], data);
  }

  let buf = getArrayBufferOrView(data, "data", inputEncoding);
  const ret = this[kHandle].update(buf.buffer ?? buf);

  if (outputEncoding && outputEncoding !== 'buffer') {
    return ""; // current implemented doesn't return anything from update
    // this._decoder = getDecoder(this._decoder, outputEncoding);
    // return this._decoder.write(ret);
  }

  return ret;
};


Cipher.prototype.final = function final(outputEncoding) {
  outputEncoding = outputEncoding || getDefaultEncoding();
  const ret = this[kHandle].final();
  if (outputEncoding && outputEncoding !== 'buffer') {
    return Buffer.from(ret).toString(outputEncoding);
    // this._decoder = getDecoder(this._decoder, outputEncoding);
    // return this._decoder.end(ret);
  }

  return ret;
};


Cipher.prototype.setAutoPadding = function setAutoPadding(ap) {
  if (!this[kHandle].setAutoPadding(!!ap))
    throw new ERR_CRYPTO_INVALID_STATE('setAutoPadding');
  return this;
};

Cipher.prototype.getAuthTag = function getAuthTag() {
  const ret = this[kHandle].getAuthTag();
  if (ret === undefined)
    throw new ERR_CRYPTO_INVALID_STATE('getAuthTag');
  return ret;
};


function setAuthTag(tagbuf, encoding) {
  tagbuf = getArrayBufferOrView(tagbuf, 'buffer', encoding);
  if (!this[kHandle].setAuthTag(tagbuf.buffer ?? tagbuf))
    throw new ERR_CRYPTO_INVALID_STATE('setAuthTag');
  return this;
}

Cipher.prototype.setAAD = function setAAD(aadbuf, options) {
  const encoding = getStringOption(options, 'encoding');
  const plaintextLength = getUIntOption(options, 'plaintextLength');
  aadbuf = getArrayBufferOrView(aadbuf, 'aadbuf', encoding);
  if (!this[kHandle].setAAD(aadbuf.buffer ?? aadbuf, plaintextLength))
    throw new ERR_CRYPTO_INVALID_STATE('setAAD');
  return this;
};

// The Cipheriv class is part of the legacy Node.js crypto API. It exposes
// a stream-based encryption/decryption model. For backwards compatibility
// the Cipheriv class is defined using the legacy function syntax rather than
// ES6 classes.

function Cipheriv(cipher, key, iv, options) {
  if (!(this instanceof Cipheriv))
    return new Cipheriv(cipher, key, iv, options);

  Reflect.apply(createCipherWithIV, this, [cipher, key, options, true, iv]);
}

function addCipherPrototypeFunctions(constructor) {
  constructor.prototype._transform = Cipher.prototype._transform;
  constructor.prototype._flush = Cipher.prototype._flush;
  constructor.prototype.update = Cipher.prototype.update;
  constructor.prototype.final = Cipher.prototype.final;
  constructor.prototype.setAutoPadding = Cipher.prototype.setAutoPadding;
  if (constructor === Cipheriv) {
    constructor.prototype.getAuthTag = Cipher.prototype.getAuthTag;
  } else {
    constructor.prototype.setAuthTag = setAuthTag;
  }
  constructor.prototype.setAAD = Cipher.prototype.setAAD;
}

Object.setPrototypeOf(Cipheriv.prototype, LazyTransform.prototype);
Object.setPrototypeOf(Cipheriv, LazyTransform);
addCipherPrototypeFunctions(Cipheriv);

// The Decipher class is part of the legacy Node.js crypto API. It exposes
// a stream-based encryption/decryption model. For backwards compatibility
// the Decipher class is defined using the legacy function syntax rather than
// ES6 classes.

function Decipher(cipher, password, options) {
  if (!(this instanceof Decipher))
    return new Decipher(cipher, password, options);

  Reflect.apply(createCipher, this, [cipher, password, options, false]);
}

Object.setPrototypeOf(Decipher.prototype, LazyTransform.prototype);
Object.setPrototypeOf(Decipher, LazyTransform);
addCipherPrototypeFunctions(Decipher);

// The Decipheriv class is part of the legacy Node.js crypto API. It exposes
// a stream-based encryption/decryption model. For backwards compatibility
// the Decipheriv class is defined using the legacy function syntax rather than
// ES6 classes.

function Decipheriv(cipher, key, iv, options) {
  if (!(this instanceof Decipheriv))
    return new Decipheriv(cipher, key, iv, options);

  Reflect.apply(createCipherWithIV, this, [cipher, key, options, false, iv]);
}

Object.setPrototypeOf(Decipheriv.prototype, LazyTransform.prototype);
Object.setPrototypeOf(Decipheriv, LazyTransform);
addCipherPrototypeFunctions(Decipheriv);

function getCipherInfo(nameOrNid, options) {
  if (typeof nameOrNid !== 'string' && typeof nameOrNid !== 'number') {
    throw new ERR_INVALID_ARG_TYPE(
      'nameOrNid',
      ['string', 'number'],
      nameOrNid);
  }
  if (typeof nameOrNid === 'number')
    validateInt32(nameOrNid, 'nameOrNid');
  let keyLength, ivLength;
  if (options !== undefined) {
    validateObject(options, 'options');
    ({ keyLength, ivLength } = options);
    if (keyLength !== undefined)
      validateInt32(keyLength, 'options.keyLength');
    if (ivLength !== undefined)
      validateInt32(ivLength, 'options.ivLength');
  }

  const ret = _getCipherInfo({}, nameOrNid, keyLength, ivLength);
  if (ret !== undefined) {
    if (ret.name) ret.name = String.prototype.toLowerCase.call(ret.name);
    if (ret.type) ret.type = String.prototype.toLowerCase.call(ret.type);
  }
  return ret;
}

export {
  // Cipher,
  Cipheriv,
  // Decipher,
  Decipheriv,
  privateDecrypt,
  privateEncrypt,
  publicDecrypt,
  publicEncrypt,
  getCipherInfo,
};
