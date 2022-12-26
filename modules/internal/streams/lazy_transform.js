// LazyTransform is a special type of Transform stream that is lazily loaded.
// This is used for performance with bi-API-ship: when two APIs are available
// for the stream, one conventional and one non-conventional.
'use strict';

import stream from '../../stream';

import {
  getDefaultEncoding
} from '../crypto/util';

export function LazyTransform(options) {
  this._options = options;
}
Object.setPrototypeOf(LazyTransform.prototype, stream.Transform.prototype);
Object.setPrototypeOf(LazyTransform, stream.Transform);

function makeGetter(name) {
  return function() {
    stream.Transform.call(this, this._options);
    this._writableState.decodeStrings = false;

    if (!this._options || !this._options.defaultEncoding) {
      this._writableState.defaultEncoding = getDefaultEncoding();
    }

    return this[name];
  };
}

function makeSetter(name) {
  return function(val) {
    Object.defineProperty(this, name, {
      __proto__: null,
      value: val,
      enumerable: true,
      configurable: true,
      writable: true
    });
  };
}

Object.defineProperties(LazyTransform.prototype, {
  _readableState: {
    __proto__: null,
    get: makeGetter('_readableState'),
    set: makeSetter('_readableState'),
    configurable: true,
    enumerable: true
  },
  _writableState: {
    __proto__: null,
    get: makeGetter('_writableState'),
    set: makeSetter('_writableState'),
    configurable: true,
    enumerable: true
  }
});
