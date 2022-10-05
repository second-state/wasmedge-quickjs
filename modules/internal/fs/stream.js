// Copyright 2018-2022 the Deno authors. All rights reserved. MIT license.
// Copyright Joyent, Inc. and Node.js contributors. All rights reserved. MIT license.

import { Writable, Readable } from "stream";

const kIsPerformingIO = Symbol('kIsPerformingIO');

const kFs = Symbol('kFs');

export class WriteStreamClass extends Writable {
    fd = null;
    bytesWritten = 0;
    pos = 0;
    [kFs] = { open, write };
    [kIsPerformingIO] = false;
    constructor(path, opts) {
        super(opts);
        this.path = toPathIfFileURL(path);
        this.flags = opts.flags || "w";
        this.mode = opts.mode || 0o666;
        this[kFs] = opts.fs ?? { open, write, close };

        if (opts.encoding) {
            this.setDefaultEncoding(opts.encoding);
        }
    }

    _construct(callback) {
        this[kFs].open(
            this.path.toString(),
            this.flags,
            this.mode,
            (err, fd) => {
                if (err) {
                    callback(err);
                    return;
                }

                this.fd = fd;
                callback();
                this.emit("open", this.fd);
                this.emit("ready");
            },
        );
    }

    _write(
        data,
        _encoding,
        cb,
    ) {
        this[kIsPerformingIO] = true;
        this[kFs].write(
            this.fd,
            data,
            0,
            data.length,
            this.pos,
            (er) => {
                this[kIsPerformingIO] = false;
                if (this.destroyed) {
                    // Tell ._destroy() that it's safe to close the fd now.
                    cb(er);
                    return this.emit(kIoDone, er);
                }

                if (er) {
                    return cb(er);
                }

                this.bytesWritten += bytes;
                cb();
            },
        );

        if (this.pos !== undefined) {
            this.pos += data.length;
        }
    }

    _destroy(err, cb) {
        if (this[kIsPerformingIO]) {
            this.once(kIoDone, (er) => closeStream(this, err || er, cb));
        } else {
            closeStream(this, err, cb);
        }
    }
}

function closeStream(
    stream,
    err,
    cb,
) {
    if (!stream.fd) {
        cb(err);
    } else {
        stream[kFs].close(stream.fd, (er) => {
            cb(er || err);
        });
        stream.fd = null;
    }
}

export function WriteStream(
    path,
    opts,
) {
    return new WriteStreamClass(path, opts);
}

WriteStream.prototype = WriteStreamClass.prototype;

export function createWriteStream(
    path,
    opts,
) {
    return new WriteStreamClass(path, opts);
}

export class ReadStream extends Readable {
    constructor(path, opts) {
        path = path instanceof URL ? fromFileUrl(path) : path;
        const hasBadOptions = opts && (
            opts.fd || opts.start || opts.end || opts.fs
        );
        if (hasBadOptions) {
            notImplemented(
                `fs.ReadStream.prototype.constructor with unsupported options (${JSON.stringify(opts)
                })`,
            );
        }
        const file = Deno.openSync(path, { read: true });
        const buffer = new Uint8Array(16 * 1024);
        super({
            autoDestroy: true,
            emitClose: true,
            objectMode: false,
            read: async function (_size) {
                try {
                    const n = await file.read(buffer);
                    this.push(n ? Buffer.from(buffer.slice(0, n)) : null);
                } catch (err) {
                    this.destroy(err);
                }
            },
            destroy: (err, cb) => {
                try {
                    file.close();
                    // deno-lint-ignore no-empty
                } catch { }
                cb(err);
            },
        });
        this.path = path;
    }
}

export function createReadStream(
    path,
    options,
) {
    return new ReadStream(path, options);
}

