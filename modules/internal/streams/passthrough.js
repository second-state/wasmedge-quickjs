// Copyright 2018-2022 the Deno authors. All rights reserved. MIT license.
// Copyright Joyent and Node contributors. All rights reserved. MIT license.
// deno-lint-ignore-file

import Transform from "./transform.js";

function PassThrough(options) {
    if (!(this instanceof PassThrough)) {
        return new PassThrough(options);
    }

    Transform.call(this, options);
}

Object.setPrototypeOf(PassThrough.prototype, Transform.prototype);
Object.setPrototypeOf(PassThrough, Transform);

PassThrough.prototype._transform = function (chunk, encoding, cb) {
    cb(null, chunk);
};

export default PassThrough;