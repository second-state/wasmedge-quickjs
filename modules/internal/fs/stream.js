// Copyright 2018-2022 the Deno authors. All rights reserved. MIT license.
// Copyright Joyent, Inc. and Node.js contributors. All rights reserved. MIT license.

import { Writable, Readable } from "stream";
import { validateEncoding } from "./utils";
import { URL } from "url";
import { toPathIfFileURL } from "../url";
import fs, { open, write, close, statSync } from "../../fs";
import { validateInteger } from "../validators";
import { nextTick } from "../../process";

const kIsPerformingIO = Symbol('kIsPerformingIO');

const kFs = Symbol('kFs');

function notImplemented(msg) {
    throw new Error(msg);
}

export class WriteStreamClass extends Writable {
    fd = null;
    bytesWritten = 0;
    pos = 0;
    [kFs] = {
        open: fs.open,
        write: fs.write
    };
    [kIsPerformingIO] = false;
    constructor(path, opts) {
        super(opts);
        if (typeof (opts) === "string") {
            validateEncoding(opts, "encoding");
        }
        if (opts.encoding) {
            validateEncoding(opts.encoding, "encoding");
            this.setDefaultEncoding(opts.encoding);
        }
        if (opts.start) {
            validateInteger(opts.start, "start");
        }
        this.pending = true;
        this.path = toPathIfFileURL(path);
        this.flags = opts.flags || "w";
        this.mode = opts.mode || 0o666;
        this[kFs] = opts.fs ?? {
            open: fs.open, write: fs.write, close: fs.close
        };
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
                this.pending = false;
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
        if (opts && opts.start) {
            validateInteger(opts.start, "start");
        }
        if (opts && opts.end) {
            validateInteger(opts.end, "end");
        }
        const hasBadOptions = opts && (
            opts.start || opts.end || opts.fs
        );
        if (opts === null || typeof (opts) === "undefined") {
            opts = "utf8";
        }
        if (typeof (opts) === "string") {
            validateEncoding(opts, "encoding");
        } else {
            validateEncoding(opts.encoding || "utf8", "encoding");
        }
        // skip
        if (hasBadOptions && false) {
            notImplemented(
                `fs.ReadStream.prototype.constructor with unsupported options (${JSON.stringify(opts)
                })`,
            );
        }
        const buffer = Buffer.alloc(16 * 1024);
        let curPos = 0;
        if (opts.fd) {
            setTimeout(() => {
                if (this.file === undefined) {
                    this.file = opts.fd;
                    this.pending = false;
                    this.emit("ready");
                }
            }, 0);
        } else {
            fs.promises.open(path, fs.constants.O_RDONLY).then(f => {
                if (this.file === undefined) {
                    this.file = f;
                    this.pending = false;
                    this.emit("ready");
                }
            });
        }
        super({
            autoDestroy: true,
            emitClose: true,
            objectMode: false,
            read: async function (_size) {
                try {
                    if (this.file === undefined) {
                        if (opts.fd) {
                            this.file = new fs.FileHandle(opts.fd, path);
                        } else {
                            this.file = new fs.FileHandle(fs.openSync(path, fs.constants.O_RDONLY), path);
                        }
                        this.pending = false;
                        this.emit("ready");
                    }
                    opts.end = opts.end || fs.fstatSync(this.file.fd).size;
                    opts.start = opts.start || 0;
                    const n = await this.file.read(buffer, 0, opts.end - opts.start - curPos + 1, curPos === 0 ? opts.start : -1);
                    curPos += n;
                    if (n === 0) {
                        this.emit("end");
                    } else {
                        this.push(n ? Buffer.from(buffer.slice(0, n)) : null);
                    }
                } catch (err) {
                    this.destroy(err);
                }
            },
            destroy: (err, cb) => {
                try {
                    this.file.close();
                    // deno-lint-ignore no-empty
                } catch { }
                cb(err);
            },
        });
        this.pending = true;
        this.path = path;
    }
}

export function createReadStream(
    path,
    options,
) {
    return new ReadStream(path, options);
}

