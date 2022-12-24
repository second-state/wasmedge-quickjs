// Copyright 2018-2022 the Deno authors. All rights reserved. MIT license.
// Copyright Joyent, Inc. and Node.js contributors. All rights reserved. MIT license.

import { ERR_CRYPTO_FIPS_FORCED, ERR_CRYPTO_TIMING_SAFE_EQUAL_LENGTH, ERR_INVALID_ARG_TYPE } from "./internal/errors";
import { crypto as constants } from "./internal_binding/constants";
import { getOptionValue } from "./internal/options";
import { isAnyArrayBuffer, isArrayBufferView } from "./internal/util/types";
import {
  timing_safe_equal,
} from "_node:crypto";
function timingSafeEqual(a, b) {
  if (!isAnyArrayBuffer(a) && !isArrayBufferView(a)) {
    throw new ERR_INVALID_ARG_TYPE("buf1", ["ArrayBuffer", "Buffer", "TypedArray", "DataView"], a);
  }
  if (!isAnyArrayBuffer(b) && !isArrayBufferView(b)) {
    throw new ERR_INVALID_ARG_TYPE("buf2", ["ArrayBuffer", "Buffer", "TypedArray", "DataView"], b);
  }
  if (a.byteLength != b.byteLength) {
    throw new ERR_CRYPTO_TIMING_SAFE_EQUAL_LENGTH();
  }
  return timing_safe_equal(a.buffer, b.buffer);
}

import {
  checkPrime,
  checkPrimeSync,
  generatePrime,
  generatePrimeSync,
  randomBytes,
  randomFill,
  randomFillSync,
  randomInt,
  randomUUID,
} from "./internal/crypto/random";
import { pbkdf2, pbkdf2Sync } from "./internal/crypto/pbkdf2";
import { scrypt, scryptSync } from "./internal/crypto/scrypt";
import { hkdf, hkdfSync } from "./internal/crypto/hkdf";
/*import {
  generateKey,
  generateKeyPair,
  generateKeyPairSync,
  generateKeySync,
} from "./internal/crypto/keygen";
import {
  createPrivateKey,
  createPublicKey,
  createSecretKey,
  KeyObject,
} from "./internal/crypto/keys";
import {
  DiffieHellman,
  diffieHellman,
  DiffieHellmanGroup,
  ECDH,
} from "./internal/crypto/diffiehellman";
import {
  Cipheriv,
  Decipheriv,
  getCipherInfo,
  privateDecrypt,
  privateEncrypt,
  publicDecrypt,
  publicEncrypt,
} from "./internal/crypto/cipher";

import {
  Sign,
  signOneShot,
  Verify,
  verifyOneShot,
} from "./internal/crypto/sig";
import { createHash, Hash, Hmac } from "./internal/crypto/hash";
import { X509Certificate } from "./internal/crypto/x509";
*/import {
  getCiphers,
  getCurves,
  getHashes,
  secureHeapUsed,
  setEngine,
} from "./internal/crypto/util";/*
import Certificate from "./internal/crypto/certificate";
*/
const webcrypto = undefined;
const fipsForced = getOptionValue("--force-fips");
/*
function createCipheriv(cipher, key, iv, options) {
  return new Cipheriv(cipher, key, iv, options);
}

function createDecipheriv(algorithm, key, iv, options) {
  return new Decipheriv(algorithm, key, iv, options);
}

function createDiffieHellman(sizeOrKey, keyEncoding, generator, generatorEncoding) {
  return new DiffieHellman(
    sizeOrKey,
    keyEncoding,
    generator,
    generatorEncoding,
  );
}

function createDiffieHellmanGroup(name) {
  return new DiffieHellmanGroup(name);
}

function createECDH(curve) {
  return new ECDH(curve);
}

function createHmac(hmac, key, options) {
  return new Hmac(hmac, key, options);
}

function createSign(algorithm, options) {
  return new Sign(algorithm, options);
}

function createVerify(algorithm, options) {
  return new Verify(algorithm, options);
}
*/
function setFipsForced(val) {
  if (val) {
    return;
  }

  throw new ERR_CRYPTO_FIPS_FORCED();
}

function getFipsForced() {
  return 1;
}

Object.defineProperty(constants, "defaultCipherList", {
  value: getOptionValue("--tls-cipher-list"),
});
/*
const getDiffieHellman = createDiffieHellmanGroup;
*/
function getFipsCrypto() {
  throw new Error("crypto.getFipsCrypto is unimplemented")
}
function setFipsCrypto(_val) {
  throw new Error("crypto.setFipsCrypto is unimplemented")
}
const getFips = fipsForced ? getFipsForced : getFipsCrypto;
const setFips = fipsForced ? setFipsForced : setFipsCrypto;
/*
const sign = signOneShot;
const verify = verifyOneShot;
*/
export default {
  /*Certificate,*/
  checkPrime,
  checkPrimeSync,/*
  Cipheriv,
  constants,
  createCipheriv,
  createDecipheriv,
  createDiffieHellman,
  createDiffieHellmanGroup,
  createECDH,
  createHash,
  createHmac,
  createPrivateKey,
  createPublicKey,
  createSecretKey,
  createSign,
  createVerify,
  Decipheriv,
  DiffieHellman,
  diffieHellman,
  DiffieHellmanGroup,
  ECDH,
  generateKey,
  generateKeyPair,
  generateKeyPairSync,
  generateKeySync,*/
  generatePrime,
  generatePrimeSync,/*
  getCipherInfo,*/
  getCiphers,
  getCurves,/*
  getDiffieHellman,*/
  getFips,
  getHashes,/*
  Hash,*/
  hkdf,
  hkdfSync,/*
  Hmac,
  KeyObject,*/
  pbkdf2,
  pbkdf2Sync,/*
  privateDecrypt,
  privateEncrypt,
  publicDecrypt,
  publicEncrypt,*/
  randomBytes,
  randomFill,
  randomFillSync,
  randomInt,
  randomUUID,
  scrypt,
  scryptSync,
  secureHeapUsed,
  setEngine,
  setFips,/*
  Sign,
  sign,*/
  timingSafeEqual,
  /*Verify,
  verify,
  webcrypto,
  X509Certificate,*/
};

export {
  /*Certificate,*/
  checkPrime,
  checkPrimeSync,/*
  Cipheriv,*/
  constants,
  /*createCipheriv,
  createDecipheriv,
  createDiffieHellman,
  createDiffieHellmanGroup,
  createECDH,
  createHash,
  createHmac,
  createPrivateKey,
  createPublicKey,
  createSecretKey,
  createSign,
  createVerify,
  Decipheriv,
  DiffieHellman,
  diffieHellman,
  DiffieHellmanGroup,
  ECDH,
  generateKey,
  generateKeyPair,
  generateKeyPairSync,
  generateKeySync,*/
  generatePrime,
  generatePrimeSync,/*
  getCipherInfo,*/
  getCiphers,
  getCurves,/*
  getDiffieHellman,*/
  getFips,
  getHashes,/*
  Hash,*/
  hkdf,
  hkdfSync,/*
  Hmac,
  KeyObject,*/
  pbkdf2,
  pbkdf2Sync,/*
  privateDecrypt,
  privateEncrypt,
  publicDecrypt,
  publicEncrypt,*/
  randomBytes,
  randomFill,
  randomFillSync,
  randomInt,
  randomUUID,
  scrypt,
  scryptSync,
  secureHeapUsed,
  setEngine,
  setFips,
  /*Sign,
  sign,*/
  timingSafeEqual,
  /*Verify,
  verify,*/
  webcrypto,
  /*X509Certificate,*/
};
