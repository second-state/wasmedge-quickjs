import { validateFunction, validateInteger } from "../internal/validators"
import { getValidatedPath, getValidMode, Stats, validateBufferArray, validateEncoding } from "../internal/fs/utils"
import * as binding from "_node:fs"
import * as errors from "../internal/errors"
import { hideStackFrames } from "../internal/errors"
export { fs as constants } from "../internal_binding/constants"
import { fs as constants, fs } from "../internal_binding/constants"
import { Buffer } from 'buffer';
import { promisify } from "../internal/util"
import { cpFn } from "../internal/fs/cp/cp";
import cpSyncFn from "../internal/fs/cp/cp-sync";
import { createWriteStream, WriteStream, createReadStream, ReadStream } from "../internal/fs/stream"
import EventEmitter from "../events"
import { normalize } from "path"
import uv from "../internal_binding/uv"
import { URL } from "../url"

// Ensure that callbacks run in the global context. Only use this function
// for callbacks that are passed to the binding layer, callbacks that are
// invoked from JS already run in the proper scope.
function makeCallback(cb) {
    validateFunction(cb, 'cb');

    return (...args) => Reflect.apply(cb, this, args);
}

function setDefaultValue(dest, def) {
    for (const [key, val] of Object.entries(def)) {
        if (dest[key] === undefined) {
            dest[key] = val;
        }
    }
}

/**
 * @typedef {Object} Stats
 * @property {number | null} dev
 * @property {number | null} ino
 * @property {number | null} mode
 * @property {number | null} nlink
 * @property {number | null} uid
 * @property {number | null} gid
 * @property {number | null} rdev
 * @property {number} size
 * @property {number | null} blksize
 * @property {number | null} blocks
 * @property {Date | null} mtime
 * @property {Date | null} atime
 * @property {Date | null} birthtime
 * @property {Date | null} ctiem
 * @property {number | null} atimeMs
 * @property {number | null} mtimeMs
 * @property {number | null} ctimeMs
 * @property {number | null} birthtimeMs
 * @property {() => boolean} isBlockDevice
 * @property {() => boolean} isCharacterDevice
 * @property {() => boolean} isDirectory
 * @property {() => boolean} isFIFO
 * @property {() => boolean} isFile
 * @property {() => boolean} isSocket
 * @property {() => boolean} isSymbolicLink
 */

/**
 * @typedef {Object} BigIntStats
 * @property {BigInt | null} dev
 * @property {BigInt | null} ino
 * @property {BigInt | null} mode
 * @property {BigInt | null} nlink
 * @property {BigInt | null} uid
 * @property {BigInt | null} gid
 * @property {BigInt | null} rdev
 * @property {BigInt} size
 * @property {BigInt | null} blksize
 * @property {BigInt | null} blocks
 * @property {Date | null} mtime
 * @property {Date | null} atime
 * @property {Date | null} birthtime
 * @property {Date | null} ctiem
 * @property {BigInt | null} atimeMs
 * @property {BigInt | null} mtimeMs
 * @property {BigInt | null} ctimeMs
 * @property {BigInt | null} birthtimeMs
 * @property {BigInt | null} atimeNs
 * @property {BigInt | null} mtimeNs
 * @property {BigInt | null} ctimens
 * @property {BigInt | null} birthtimeNs
 * @property {() => boolean} isBlockDevice
 * @property {() => boolean} isCharacterDevice
 * @property {() => boolean} isDirectory
 * @property {() => boolean} isFIFO
 * @property {() => boolean} isFile
 * @property {() => boolean} isSocket
 * @property {() => boolean} isSymbolicLink
 */

/**
 * @typedef {Object} RawStat
 * @property {number | null} dev
 * @property {number | null} ino
 * @property {number | null} mode
 * @property {number | null} nlink
 * @property {number | null} uid
 * @property {number | null} gid
 * @property {number | null} rdev
 * @property {number} size
 * @property {number | null} blksize
 * @property {number | null} blocks
 * @property {number | null} atime
 * @property {number | null} mtime
 * @property {number | null} birthtime
 */

/**
 * @param {RawStat} origin
 * @return {Stats}
 */
function convertRawStatInfoToNodeStats(origin) {
    return {
        dev: origin.dev,
        ino: origin.ino,
        mode: origin.mode,
        nlink: origin.nlink,
        uid: origin.uid,
        gid: origin.gid,
        rdev: origin.rdev,
        size: origin.size,
        blksize: origin.blksize,
        blocks: origin.blocks,
        mtime: new Date(origin.mtime),
        atime: new Date(origin.atime),
        birthtime: new Date(origin.birthtime),
        mtimeMs: origin.mtime,
        atimeMs: origin.atime,
        birthtimeMs: origin.birthtime,
        isFile: () => origin.is_file,
        isDirectory: () => origin.is_directory,
        isSymbolicLink: () => origin.is_symlink,
        isBlockDevice: () => origin.is_block_device,
        isFIFO: () => false,
        isCharacterDevice: () => origin.is_char_device,
        isSocket: () => origin.is_socket,
        ctime: new Date(origin.mtime),
        ctimeMs: origin.mtime,
    };
}

/**
 * 
 * @param {number | null} number 
 * @returns {BigInt}
 */
function toBigInt(number) {
    if (number === null || number === undefined) return null;
    return BigInt(number);
}

/**
 * 
 * @param {RawStat} origin 
 * @returns {BigIntStats}
 */
function convertRawStatInfoToBigIntNodeStats(
    origin,
) {
    return {
        dev: toBigInt(origin.dev),
        ino: toBigInt(origin.ino),
        mode: toBigInt(origin.mode),
        nlink: toBigInt(origin.nlink),
        uid: toBigInt(origin.uid),
        gid: toBigInt(origin.gid),
        rdev: toBigInt(origin.rdev),
        size: toBigInt(origin.size) || 0n,
        blksize: toBigInt(origin.blksize),
        blocks: toBigInt(origin.blocks),
        mtime: new Date(origin.mtime),
        atime: new Date(origin.atime),
        birthtime: new Date(origin.birthtime),
        mtimeMs: toBigInt(origin.mtime),
        atimeMs: toBigInt(origin.atime),
        birthtimeMs: toBigInt(origin.birthtime),
        mtimeNs: toBigInt(origin.mtime) * 1000000n,
        atimeNs: toBigInt(origin.atime) * 1000000n,
        birthtimeNs: toBigInt(origin.birthtime) * 1000000n,
        isFile: () => origin.is_file,
        isDirectory: () => origin.is_directory,
        isSymbolicLink: () => origin.is_symlink,
        isBlockDevice: () => origin.is_block_device,
        isFIFO: () => false,
        isCharacterDevice: () => origin.is_char_device,
        isSocket: () => origin.is_socket,
        ctime: new Date(origin.mtime),
        ctimeMs: toBigInt(origin.mtime),
        ctimeNs: toBigInt(origin.mtime) * 1000000n,
    };
}


function stat(path, options, callback) {
    if (typeof (options) === "function") {
        callback = options;
        options = {};
    }
    validateFunction(callback, "callback");
    path = getValidatedPath(path);

    setTimeout(() => {
        try {
            statSync(path, options);
            callback();
        } catch (err) {
            callback(err);
        }
    }, 0);
}

/**
 * Synchronously retrieves the `fs.Stats`
 * for the `path`.
 * @param {string | Buffer | URL} path
 * @param {{
 *   bigint?: boolean;
 *   throwIfNoEntry?: boolean;
 *   }} [options]
 * @returns {Stats | BigIntStats}
 */
function statSync(path, options = { bigint: false, throwIfNoEntry: true }) {
    path = getValidatedPath(path);

    setDefaultValue(options, { bigint: false, throwIfNoEntry: true });

    try {
        let stat = binding.statSync(path);
        if (options.bigint === true) {
            return convertRawStatInfoToBigIntNodeStats(stat);
        } else {
            return convertRawStatInfoToNodeStats(stat);
        }
    } catch (err) {
        if (err.kind === "NotFound" && options.throwIfNoEntry === false) {
            return undefined;
        }
        let e = new Error("no such file or directory");
        e.code = "ENOENT";
        throw e;
    }
}

function lstat(path, options, callback) {
    if (typeof (options) === "function") {
        callback = options;
    }
    validateFunction(callback, "callback");
    path = getValidatedPath(path);

    setTimeout(() => {
        try {
            lstatSync(path, options);
            callback();
        } catch (err) {
            callback(err);
        }
    })
}

function lstatSync(path, options = { bigint: false, throwIfNoEntry: true }) {
    path = getValidatedPath(path);

    setDefaultValue(options, { bigint: false, throwIfNoEntry: true });

    try {
        let stat = binding.lstatSync(path);
        if (options.bigint === true) {
            return convertRawStatInfoToBigIntNodeStats(stat);
        } else {
            return convertRawStatInfoToNodeStats(stat);
        }
    } catch (err) {
        if (err.kind === "NotFound" && options.throwIfNoEntry === false) {
            return undefined;
        }
        throw new Error(err.message);
    }
}

function fstat(fd, options, callback) {
    if (typeof (options) === "function") {
        callback = options;
    }
    validateFunction(callback, "callback");

    setTimeout(() => {
        try {
            fstatSync(fd, options);
            callback();
        } catch (err) {
            callback(err);
        }
    })
}

function fstatSync(fd, options = { bigint: false, throwIfNoEntry: true }) {
    validateInteger(fd, "fd");

    setDefaultValue(options, { bigint: false, throwIfNoEntry: true });

    try {
        let stat = binding.fstatSync(fd);
        if (options.bigint === true) {
            return convertRawStatInfoToBigIntNodeStats(stat);
        } else {
            return convertRawStatInfoToNodeStats(stat);
        }
    } catch (err) {
        if (err.kind === "NotFound" && options.throwIfNoEntry === false) {
            return undefined;
        }
        throw new Error(err.message);
    }
}

function access(path, mode = constants.F_OK, callback) {
    if (typeof (mode) === "function") {
        callback = mode;
        mode = constants.F_OK;
    }

    mode = getValidMode(mode, "access");

    path = getValidatedPath(path);
    validateFunction(callback, "callback");

    setTimeout(() => {
        try {
            accessSync(path, mode);
            callback();
        } catch (err) {
            let e = new Error();
            e.stack += err.stack;
            e.code = "ENOENT";
            e.path = path.toString();
            e.message = `ENOENT: no such file or directory, access '${path}'`;
            e.syscall = "access";
            e.errno = uv.UV_ENOENT;
            callback(e);
        }
    })
}

function accessSync(path, mode = constants.F_OK) {
    path = getValidatedPath(path);

    mode = getValidMode(mode, "access");

    try {
        const stat = statSync(path, { throwIfNoEntry: true });
        if ((stat.mode & mode) === mode) {
            return undefined;
        } else {
            throw new Error(`EACCES: permission denied, access '${path}'`);
        }
    } catch (err) {
        let e = new Error();
        e.stack += err.stack;
        e.code = "ENOENT";
        e.path = path.toString();
        e.message = `ENOENT: no such file or directory, access '${path}'`;
        e.syscall = "access";
        e.errno = uv.UV_ENOENT;
        throw e;
    }
}

function exists(path, callback) {

    validateFunction(callback, "callback")
    try {
        path = getValidatedPath(path);
    } catch (err) {
        callback(false);
        return;
    }

    setTimeout(() => {
        callback(existsSync(path));
    }, 0);
}

function existsSync(path) {
    try {
        path = getValidatedPath(path);
        accessSync(path);
        return true;
    } catch (err) {
        return false;
    }
}

function mkdir(path, mode, callback) {
    if (typeof (mode) === "function") {
        callback = mode;
        mode = {};
    }

    validateFunction(callback, "callback");
    setTimeout(() => {
        try {
            mkdirSync(path, mode);
            callback();
        } catch (err) {
            callback(err);
        }
    }, 0);
}

function mkdirSync(path, options = { recursive: false, mode: 0o777 }) {
    path = getValidatedPath(path);

    setDefaultValue(options, { recursive: false, mode: 0o777 });

    try {
        binding.mkdirSync(path, options.recursive, options.mode);
    } catch (err) {
        throw new Error(err.message);
    }
}

// wasi unspported *chown, *chownSync, *chmod, *chmodSync
function fchown(fd, uid, gid, callback) {
    validateFunction(callback);

    callback(undefined);
}

function fchownSync(fd, uid, gid) {
    return undefined;
}

function lchown(path, uid, gid, callback) {
    validateFunction(callback);

    callback(undefined);
}

function lchownSync(path, uid, gid) {
    return undefined;
}

function chown(path, uid, gid, callback) {
    validateFunction(callback);

    callback(undefined);
}

function chownSync(path, uid, gid) {
    return undefined;
}

function chmod(path, mode, callback) {
    validateFunction(callback);

    callback(undefined);
}

function chmodSync(path, mode) {
    return undefined;
}

function lchmod(path, mode, callback) {
    validateFunction(callback);

    callback(undefined);
}

function lchmodSync(path, mode) {
    return undefined;
}

function fchmod(fd, mode, callback) {
    validateFunction(callback, "callback");

    callback(undefined);
}

function fchmodSync(fd, mode) {
    return undefined;
}

function getValidTime(time, name) {
    if (typeof time === "string") {
        time = Number(time);
    }

    if (
        typeof time === "number" &&
        (Number.isNaN(time) || !Number.isFinite(time))
    ) {
        throw new errors.ERR_INVALID_ARG_TYPE(name, "number | string | Date", time);
    }

    return time;
}

function utimes(path, atime, mtime, callback) {
    validateFunction(callback);

    validateFunction(callback, "callback");
    setTimeout(() => {
        try {
            utimesSync(path, atime, mtime);
            callback();
        } catch (err) {
            callback(err);
        }
    }, 0);
}

function utimesSync(path, atime, mtime) {
    path = getValidatedPath(path);
    atime = getValidTime(atime);
    mtime = getValidTime(mtime);

    try {
        binding.utimeSync(path, atime, mtime);
    } catch (err) {
        throw new Error(err.message);
    }
}

function lutimes(path, atime, mtime, callback) {
    utimes(path, atime, mtime, callback);
}

function lutimesSync(path, atime, mtime) {
    utimesSync(path, atime, mtime);
}

function futimes(fd, atime, mtime, callback) {
    validateFunction(callback, "callback");

    setTimeout(() => {
        try {
            futimesSync(fd, atime, mtime);
            callback();
        } catch (err) {
            callback(err);
        }
    })
}

function futimesSync(fd, atime, mtime) {
    validateInteger(fd, "fd");
    atime = getValidTime(atime, "atime");
    mtime = getValidTime(mtime, "mtime");

    try {
        binding.futimeSync(fd, atime, mtime);
    } catch (err) {
        throw new Error(err.message);
    }
}

function rmdir(path, options, callback) {
    if (typeof (options) === "function") {
        callback = options;
        options = {};
    }
    validateFunction(callback, "callback");
    setTimeout(() => {
        try {
            rmdirSync(path, options, callback);
            callback();
        } catch (err) {
            callback(err);
        }
    }, 0);
}

function rmdirSync(path, options = { maxRetries: 0, recursive: false, retryDelay: 100 }) {
    path = getValidatedPath(path);

    setDefaultValue(options, { maxRetries: 0, recursive: false, retryDelay: 100 });

    try {
        binding.rmdirSync(path, options.recursive);
    } catch (err) {
        throw new Error(err.message);
    }
}

function rm(path, options, callback) {
    if (typeof (options) === "function") {
        callback = options;
        options = {};
    }
    validateFunction(callback, "callback");
    setTimeout(() => {
        try {
            rmSync(path, options);
            callback();
        }
        catch (err) {
            callback(err);
        }
    }, 0);
}

function rmSync(path, options = { force: false, maxRetries: 0, recursive: false, retryDelay: 100 }) {
    path = getValidatedPath(path);

    setDefaultValue(options, { force: false, maxRetries: 0, recursive: false, retryDelay: 100 });

    for (let i = 0; i < options.maxRetries; i++) {
        try {
            binding.rmSync(path, options.recursive, options.force);
        } catch (err) {
            if (i === options.maxRetries - 1) {
                throw new Error(err.message);
            }
            continue;
        }
        break;
    }
}

function rename(oldPath, newPath, callback) {
    validateFunction(callback, "callback");
    setTimeout(() => {
        try {
            renameSync(oldPath, newPath);
            callback();
        } catch (err) {
            callback(err);
        }
    })
}

function renameSync(oldPath, newPath) {
    oldPath = getValidatedPath(oldPath);
    newPath = getValidatedPath(newPath);

    try {
        binding.renameSync(oldPath, newPath);
    } catch (err) {
        throw new Error(err.message);
    }
}

function unlink(path, callback) {
    path = getValidatedPath(path);

    rm(path, callback);
}

function unlinkSync(path) {
    path = getValidatedPath(path);

    rmSync(path);
}

function truncate(path, len, callback) {
    if (typeof (len) === "function") {
        callback = len;
        len = 0;
    }
    validateFunction(callback, "callback");
    setTimeout(() => {
        try {
            truncateSync(path, len);
            callback();
        }
        catch (err) {
            callback(err);
        }
    }, 0);
}

function truncateSync(path, len = 0) {
    validateInteger(len);

    path = getValidatedPath(path);

    try {
        binding.truncateSync(path, len);
    } catch (err) {
        throw new Error(err.message);
    }
}

function ftruncate(fd, len, callback) {
    if (typeof (len) === "function") {
        callback = len;
        len = 0;
    }
    validateFunction(callback, "callback");
    setTimeout(() => {
        try {
            ftruncateSync(fd, len);
            callback();
        }
        catch (err) {
            callback(err);
        }
    }, 0);
}

function ftruncateSync(fd, len = 0) {
    validateInteger(len);

    validateInteger(fd, "fd");

    try {
        binding.ftruncateSync(fd, len);
    } catch (err) {
        throw new Error(err.message);
    }
}

function realpath(path, options = { encoding: "utf8" }, callback) {
    if (typeof (options) === "function") {
        callback = options;
        options = {};
    }
    validateFunction(callback, "callback");
    if (typeof (options) === "string") {
        options = {
            encoding: options
        };
    }
    setDefaultValue(options, {
        encoding: "utf8"
    });
    validateEncoding(options.encoding, "encoding");
    setTimeout(() => {
        try {
            callback(null, realpathSync(path, options));
        } catch (err) {
            callback(err);
        }
    }, 0);
}

function realpathSync(path, options = { encoding: "utf8" }) {
    path = getValidatedPath(path);
    if (typeof (options) === "string") {
        options = {
            encoding: options
        };
    }
    setDefaultValue(options, { encoding: "utf8" });
    validateEncoding(options.encoding, "encoding");
    let useBuffer = options.encoding === "buffer" || options.encoding === "Buffer";
    let stat = lstatSync(path, { throwIfNoEntry: false });
    if (stat != null) {
        if (!stat.isSymbolicLink()) {
            let res = normalize(path);
            if (!useBuffer) {
                return res;
            } else {
                return Buffer.from(res, "utf8");
            }
        }
    }
    try {
        let res = binding.realpathSync(path);
        res = normalize(res);
        if (!useBuffer) {
            return res;
        } else {
            return Buffer.from(res, "utf8");
        }
    } catch (err) {
        throw new Error(err.message);
    }
}

function genId(len) {
    let result = '';
    let characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let charactersLength = characters.length;
    for (let i = 0; i < len; i++) {
        result += characters.charAt(Math.floor(Math.random() *
            charactersLength));
    }
    return result;
}

function mkdtemp(prefix, options = { encoding: "utf8" }, callback) {
    prefix = getValidatedPath(prefix);

    if (typeof (options) === "string") {
        options = { encoding: options };
    } else if (typeof (options) === "function") {
        callback = options;
        options = { encoding: "utf8" };
    } else {
        setDefaultValue(options, { encoding: "utf8" });
    }
    validateFunction(callback, "callback");
    validateEncoding(options.encoding, "encoding");
    let useBuffer = options.encoding === "buffer" || options.encoding === "Buffer";

    let path = prefix + genId(6);
    mkdir(path, (err) => {
        if (err) {
            callback(err);
        } else if (useBuffer) {
            callback(undefined, Buffer.from(path, "utf8"));
        } else {
            callback(undefined, path);
        }
    })
}

function mkdtempSync(prefix, options = { encoding: "utf8" }) {
    prefix = getValidatedPath(prefix);

    if (typeof (options) === "string") {
        options = { encoding: options };
    } else {
        setDefaultValue(options, { encoding: "utf8" });
    }
    validateEncoding(options.encoding, "encoding");

    let useBuffer = options.encoding === "buffer" || options.encoding === "Buffer";

    let path = prefix + genId(6);
    mkdirSync(path);
    if (useBuffer) {
        return Buffer.from(path, "utf8");
    } else {
        return path;
    }
}

function copyFile(src, dest, mode, callback) {
    if (typeof (mode) === "function") {
        callback = mode;
        mode = 0;
    }
    validateFunction(callback, "callback");
    setTimeout(() => {
        try {
            copyFileSync(src, dest, mode);
            callback();
        } catch (err) {
            callback(err);
        }
    }, 0);
}

function copyFileSync(src, dest, mode = 0) {
    src = getValidatedPath(src);
    dest = getValidatedPath(dest);
    try {
        if (mode ^ constants.COPYFILE_EXCL === constants.COPYFILE_EXCL) {
            if (existsSync(dest)) {
                throw new Error(`EEXIST: file already exists, copyfile '${src}' -> '${dest}'`);
            }
            binding.copyFileSync(src, dest);
        } else {
            binding.copyFileSync(src, dest);
        }
    } catch (err) {
        throw new Error(err.message);
    }
}

function link(existingPath, newPath, callback) {
    existingPath = getValidatedPath(existingPath);
    newPath = getValidatedPath(newPath);
    validateFunction(callback, "callback");

    setTimeout(() => {
        try {
            linkSync(existingPath, newPath);
            callback();
        } catch (err) {
            callback(err);
        }
    }, 0);
}

function linkSync(existingPath, newPath) {
    existingPath = getValidatedPath(existingPath);
    newPath = getValidatedPath(newPath);

    try {
        binding.linkSync(existingPath, newPath);
    } catch (err) {
        throw new Error(err.message);
    }
}

function symlink(target, path, callback) {
    validateFunction(callback, "callback");

    setTimeout(() => {
        try {
            symlinkSync(target, path);
            callback();
        } catch (err) {
            callback(err);
        }
    }, 0);
}

function symlinkSync(target, path) {
    target = getValidatedPath(target);
    path = getValidatedPath(path);

    try {
        binding.symlinkSync(target, path);
    } catch (err) {
        throw new Error(err.message);
    }
}

function close(fd, callback) {
    validateInteger(fd, "fd");
    validateFunction(callback, "callback");

    setTimeout(() => {
        try {
            closeSync(fd);
            callback();
        } catch (err) {
            callback(err);
        }
    })
}

function closeSync(fd) {
    validateInteger(fd, "fd");

    try {
        binding.fcloseSync(fd);
    } catch (err) {
        throw new Error(err.message);
    }
}

function fsync(fd, callback) {
    validateFunction(callback, "callback");

    setTimeout(() => {
        try {
            fsyncSync(fd);
            callback();
        } catch (err) {
            callback(err);
        }
    })
}

function fsyncSync(fd) {
    validateInteger(fd, "fd");


    try {
        binding.fsyncSync(fd);
    } catch (err) {
        throw new Error(err.message);
    }
}

function fdatasync(fd, callback) {
    validateFunction(callback, "callback");

    setTimeout(() => {
        try {
            fdatasyncSync(fd);
            callback();
        } catch (err) {
            callback(err);
        }
    })
}

function fdatasyncSync(fd) {
    validateInteger(fd, "fd");

    try {
        binding.fdatasyncSync(fd);
    } catch (err) {
        throw new Error(err.message);
    }
}

function read(fd, buffer, offset, length, position, callback) {
    if (typeof (buffer) === "function") {
        callback = buffer;
        buffer = Buffer.alloc(16384);
        offset = 0;
        length = buffer.byteLength - offset;
        position = 0;
    } else if (!(buffer instanceof Buffer)) {
        callback = offset;
        let option = buffer;
        buffer = Buffer.alloc(16384);
        offset = option.offset || 0;
        length = option.length || buffer.byteLength - offset;
        if (option.position === null) {
            option.position = -1;
        }
        position = option.position || 0;
    } else if (typeof (offset) === "object") {
        callback = length;
        let option = offset;
        offset = option.offset || 0;
        length = option.length || buffer.byteLength - offset;
        if (option.position === null) {
            option.position = -1;
        }
        position = option.position || 0;
    } else if (typeof (offset) === "function") {
        callback = offset;
        offset = 0;
        length = buffer.byteLength - offset;
        position = 0;
    }

    validateFunction(callback, "callback");
    validateInteger(offset, "offset");
    validateInteger(position, "position");
    validateInteger(length, "length");

    binding.fread(fd, position, length).then((data) => {
        buffer.fill(data, offset, data.byteLength);
        callback(null, data.byteLength, buffer)
    }).catch((e) => {
        callback(e)
    })
}

function readSync(fd, buffer, offset, length, position) {
    if (typeof (offset) === "object" && offset !== null) {
        let option = offset;
        offset = option.offset || 0;
        length = option.length || buffer.byteLength - offset;
        if (option.position === null || typeof(option.position) === "undefined") {
            option.position = -1;
        } else {
            position = option.position;
        }
    }

    if (typeof (offset) !== "number" || offset === Infinity) {
        offset = 0;
        length = buffer.byteLength;
        position = -1;
    } else {
        offset = offset || 0;
        length = length || buffer.byteLength - offset;
        if (position === null) {
            position = -1;
        }
        if (position === null || typeof(position) === "undefined") {
            position = -1;
        }
    }

    validateInteger(offset, "offset");
    
    const nodePositionMin = -9223372036854775808n;
    const nodePositionMax = 9223372036854775807n;
    if (typeof (position) === "bigint") {
        if (position < nodePositionMin || position > nodePositionMax) {
            throw new errors.ERR_OUT_OF_RANGE("position", `>= ${nodePositionMin} && <= ${nodePositionMax}`, position);
        } else if (position < Number.MIN_SAFE_INTEGER || position > Number.MAX_SAFE_INTEGER) {
            let err = new Error();
            err.code = "EFBIG";
            throw err;
        } else {
            position = Number(position);
        }
    } else {
        validateInteger(position, "position");
    }
    validateInteger(length, "length");

    try {
        let data = binding.freadSync(fd, position, length);
        buffer.fill(data, offset, offset + data.byteLength);
        return data.byteLength;
    } catch (err) {
        throw new Error(err.message);
    }
}

function stringToFlags(flags) {
    if (typeof flags === 'number') {
        return flags;
    }

    switch (flags) {
        case 'r': return constants.O_RDONLY;
        case 'rs': // Fall through.
        case 'sr': return constants.O_RDONLY | constants.O_SYNC;
        case 'r+': return constants.O_RDWR;
        case 'rs+': // Fall through.
        case 'sr+': return constants.O_RDWR | constants.O_SYNC;

        case 'w': return constants.O_TRUNC | constants.O_CREAT | constants.O_WRONLY;
        case 'wx': // Fall through.
        case 'xw': return constants.O_TRUNC | constants.O_CREAT | constants.O_WRONLY | constants.O_EXCL;

        case 'w+': return constants.O_TRUNC | constants.O_CREAT | constants.O_RDWR;
        case 'wx+': // Fall through.
        case 'xw+': return constants.O_TRUNC | constants.O_CREAT | constants.O_RDWR | constants.O_EXCL;

        case 'a': return constants.O_APPEND | constants.O_CREAT | constants.O_WRONLY;
        case 'ax': // Fall through.
        case 'xa': return constants.O_APPEND | constants.O_CREAT | constants.O_WRONLY | constants.O_EXCL;
        case 'as': // Fall through.
        case 'sa': return constants.O_APPEND | constants.O_CREAT | constants.O_WRONLY | constants.O_SYNC;

        case 'a+': return constants.O_APPEND | constants.O_CREAT | constants.O_RDWR;
        case 'ax+': // Fall through.
        case 'xa+': return constants.O_APPEND | constants.O_CREAT | constants.O_RDWR | constants.O_EXCL;
        case 'as+': // Fall through.
        case 'sa+': return constants.O_APPEND | constants.O_CREAT | constants.O_RDWR | constants.O_SYNC;
    }

    throw new errors.ERR_INVALID_ARG_VALUE('flags', flags);
}

function openSync(path, flag = "r", mode = 0o666) {
    path = getValidatedPath(path);
    flag = stringToFlags(flag);

    if (mode === null || mode === undefined) {
        mode = 0o666;
    }

    try {
        validateInteger(mode, "mode", 0, 0o777);
    } catch (err) {
        if (typeof (mode) === "string") {
            err.code = "ERR_INVALID_ARG_VALUE";
        }
        throw err;
    }

    try {
        let fd = binding.openSync(path, flag, mode);
        return fd;
    } catch (err) {
        let e = new Error(err.message);
        e.code = "ENOENT";
        throw e;
    }
}

function open(path, flag = "r", mode = 0o666, callback) {
    if (typeof (flag) === "function") {
        callback = flag;
        flag = "r";
        mode = 0o666;
    } else if (typeof (mode) === "function") {
        callback = mode;
        mode = 0o666;
    }

    path = getValidatedPath(path);

    validateFunction(callback, "callback");
    if (mode === null || mode === undefined) {
        mode = 0o666;
    }

    try {
        validateInteger(mode, "mode", 0, 0o777);
    } catch (err) {
        if (typeof (mode) === "string") {
            err.code = "ERR_INVALID_ARG_VALUE";
        }
        throw err;
    }


    setTimeout(() => {
        try {
            fd = openSync(path, flag, mode);
            callback(null, fd);
        } catch (err) {
            callback(err);
        }
    })
}

function readFile(path, option, callback) {
    if (typeof (option) === "function") {
        callback = option;
    }
    let encoding = undefined;
    if (typeof (option) === "string") {
        encoding = option;
        option = {};
    } else {
        option = option || {};
    }
    setDefaultValue(option, {
        encoding: encoding || "",
        flag: "r",
        signal: undefined
    })
    validateEncoding(option.encoding, "encoding");
    validateFunction(callback, "callback");

    let fd;
    if (typeof (path) === "number") {
        fd = path;
    } else {
        try {
            fd = openSync(path, option.flag);
        } catch (err) {
            if (err.message === "File exists.") {
                err.code = "EEXIST";
            }
            callback(err);
            return;
        }
    }
    let stat = fstatSync(fd);
    let len = stat.size;
    let buf = Buffer.alloc(len)

    read(fd, buf, (err, rlen, obuf) => {
        if (typeof (path) !== "number") {
            closeSync(fd);
        }
        if (err) {
            callback(err);
        } else if (option.encoding !== "") {
            callback(err, obuf.slice(0, rlen).toString(option.encoding));
        } else {
            callback(err, obuf.slice(0, rlen));
        }
    })
}


function readFileSync(path, option) {
    let encoding = undefined;
    if (typeof (option) === "string") {
        encoding = option;
        option = {};
    } else {
        option = option || {};
    }
    setDefaultValue(option, {
        encoding: encoding || "",
        flag: "r",
        signal: undefined
    })
    validateEncoding(option.encoding, "encoding");
    let fd;
    if (typeof (path) === "number") {
        fd = path;
    }
    else {
        try {
            fd = openSync(path, option.flag);
        } catch (err) {
            if (err.message === "File exists.") {
                err.code = "EEXIST";
            }
            throw err;
        }
    }
    let stat = fstatSync(fd);
    let len = stat.size;
    let buf = Buffer.alloc(len)
    let rlen = readSync(fd, buf);
    if (typeof (path) !== "number") {
        closeSync(fd);
    }
    if (option.encoding !== "") {
        return buf.slice(0, rlen).toString(option.encoding);
    } else {
        return buf.slice(0, rlen);
    }
}

function readlinkSync(path, option) {
    path = getValidatedPath(path);
    if (typeof (option) === "string") {
        option = {
            encoding: option
        }
    }
    setDefaultValue(option, {
        encoding: "utf8"
    });
    validateEncoding(option.encoding, "encoding");
    try {
        let res = binding.readlinkSync(path);
        if (option.encoding === "buffer" || option.encoding === "Buffer") {
            return Buffer.from(res);
        }
        return res;
    } catch (err) {
        throw new Error(err.message);
    }
}

function readlink(path, option, callback) {
    if (typeof (option) === "function") {
        callback = option;
        option = {};
    }
    if (typeof (option) === "string") {
        option = {
            encoding: option
        };
    }
    path = getValidatedPath(path);
    setDefaultValue(option, {
        encoding: "utf8"
    });
    validateEncoding(option.encoding, "encoding");
    setTimeout(() => {
        try {
            let res = readlinkSync(path, option);
            callback(null, res);
        } catch (err) {
            callback(err);
        }
    })
}

function readv(fd, buffer, position, callback) {
    if (typeof (position) === "function") {
        callback = position;
        position = 0;
    }

    validateFunction(callback, "callback");
    validateInteger(position, "position");

    let length = 0;
    for (const buf of buffer) {
        length += buf.byteLength;
    }

    binding.fread(fd, position, length).then((data) => {
        let off = 0;
        for (const buf of buffer) {
            if (buf.byteLength !== 0) {
                buf.fill(data.slice(off, off + buf.byteLength));
                off += buf.byteLength;
            }
        }
        callback(null, data.byteLength, buffer)
    }).catch((e) => {
        callback(e)
    })
}

function readvSync(fd, buffer, position = 0) {
    validateInteger(fd, "fd");
    validateInteger(position, "position");

    let length = 0;
    for (const buf of buffer) {
        length += buf.byteLength;
    }

    let data = binding.freadSync(fd, position, length);
    let off = 0;
    for (const buf of buffer) {
        if (buf.byteLength !== 0) {
            buf.fill(data.slice(off, off + buf.byteLength));
            off += buf.byteLength;
        }
    }
    return data.byteLength;
}

function write(fd, buffer, offset, length, position, callback) {
    let oriStr = null;
    if (typeof (buffer) === "string") {
        oriStr = buffer;
        if (typeof (offset) === "function") {
            callback = offset;
            position = 0;
            buffer = Buffer.from(buffer);
        } else if (typeof (length) === "function") {
            callback = length;
            position = offset;
            buffer = Buffer.from(buffer);
        } else {
            buffer = Buffer.from(buffer, position);
        }
        offset = 0;
        length = buffer.byteLength - offset;
    } else if (typeof (offset) === "object") {
        callback = length;
        let option = offset;
        offset = option.offset || 0;
        length = option.length || buffer.byteLength - offset;
        position = option.position || 0;
    } else if (typeof (offset) === "function") {
        callback = offset;
        offset = 0;
        length = buffer.byteLength - offset;
        position = 0;
    }

    validateFunction(callback, "callback");
    validateInteger(offset, "offset");
    validateInteger(position, "position");
    validateInteger(length, "length");

    binding.fwrite(fd, position, buffer.buffer.slice(offset, offset + length)).then((len) => {
        if (oriStr !== null) {
            callback(null, len, buffer.slice(offset, offset + len));
        } else {
            callback(null, len, oriStr.slice(offset, offset + len));
        }
    }).catch((e) => {
        callback(e);
    })
}

function writeSync(fd, buffer, offset, length, position) {
    if (typeof (buffer) !== "string") {
        if (typeof (buffer) !== "object" || !(buffer instanceof Buffer)) {
            throw new errors.ERR_INVALID_ARG_TYPE("buffer", "buffer | string");
        }
    }

    let oriStr = null;
    if (typeof (buffer) === "string") {
        oriStr = buffer;
        let encoding = length || "utf8";
        buffer = Buffer.from(buffer, encoding);
        position = offset || 0;
        offset = 0;
        length = buffer.byteLength - offset;
    } else if (typeof (offset) === "object") {
        let option = offset;
        offset = option.offset || 0;
        length = option.length || buffer.byteLength - offset;
        position = option.position || 0;
    } else if (typeof (offset) === "number") {
        length = buffer.byteLength - offset;
        position = 0;
    } else {
        offset = offset || 0;
        length = length || buffer.byteLength - offset
        position = position || 0;
    }

    validateInteger(offset, "offset");
    validateInteger(position, "position");
    validateInteger(length, "length");

    try {
        let len = binding.fwriteSync(fd, position, buffer.buffer.slice(offset, offset + length));
        return len;
    } catch (err) {
        throw new Error(err.message);
    }
}

function writeFile(file, data, options, callback) {
    if (typeof (options) === "function") {
        callback = options;
        options = {};
    }
    options = options || {};
    if (typeof (options) === "string") {
        validateEncoding(options, "option");
        options = {
            encoding: options
        };
    } else {
        validateObject(options, "option");
    }
    setDefaultValue(options, {
        encoding: "utf8",
        mode: 0o666,
        flag: "w",
        signal: null
    });
    validateFunction(callback, "callback");
    file = getValidatedPath(file);
    let buffer = typeof (data) === "string" ? Buffer.from(data, options.encoding) : data;
    try {
        let fd = openSync(file, options.flag, options.mode);
        write(fd, buffer, (err) => {
            if (err) {
                callback(err);
            } else {
                callback();
            }
            closeSync(fd);
        })
    } catch (err) {
        callback(err);
    }
}

function writeFileSync(file, data, options = {}) {
    options = options || {};
    if (typeof (options) === "string") {
        validateEncoding(options, "option");
        options = {
            encoding: options
        };
    } else {
        validateObject(options, "option");
    }
    setDefaultValue(options, {
        encoding: "utf8",
        mode: 0o666,
        flag: "w",
        signal: null
    });
    file = getValidatedPath(file);
    let buffer = typeof (data) === "string" ? Buffer.from(data, options.encoding) : data;
    let fd = openSync(file, options.flag, options.mode);
    writeSync(fd, buffer);
    closeSync(fd);
}

function appendFile(file, data, options, callback) {
    if (typeof (options) === "function") {
        callback = options;
        options = {};
    }

    validateFunction(callback, "callback");
    file = getValidatedPath(file);
    if (typeof (options) === "string") {
        options = {
            encoding: options
        };
    }
    setDefaultValue(options, {
        encoding: "utf8",
        mode: 0o666,
        flag: "a",
        signal: null
    });
    validateEncoding(options.encoding, "encoding");
    if (typeof (data) !== "string" && !(data instanceof Buffer)) {
        throw new errors.ERR_INVALID_ARG_TYPE("data", ["string", "buffer"]);
    }

    let buffer = typeof (data) === "string" ? Buffer.from(data, options.encoding) : data;
    try {
        let fd = openSync(file, options.flag, options.mode);
        write(fd, buffer, (err) => {
            if (err) {
                callback(err);
            } else {
                callback();
            }
            closeSync(fd);
        })
    } catch (err) {
        callback(err);
    }
}

function appendFileSync(file, data, options = {}) {
    if (typeof (options) === "string") {
        options = {
            encoding: options
        };
    }
    setDefaultValue(options, {
        encoding: "utf8",
        mode: 0o666,
        flag: "a",
        signal: null
    });
    validateEncoding(options.encoding, "encoding");
    if (typeof (data) !== "string" && !(data instanceof Buffer)) {
        throw new errors.ERR_INVALID_ARG_TYPE("data", ["string", "buffer"]);
    }

    let fd = -1;
    if (typeof (file) === "number") {
        fd = file;
    } else {
        file = getValidatedPath(file);
        fd = openSync(file, options.flag, options.mode);
    }

    let buffer = typeof (data) === "string" ? Buffer.from(data, options.encoding) : data;
    writeSync(fd, buffer);
    if (typeof (file) !== "number") {
        closeSync(fd);
    }
}

function writev(fd, buffer, position, callback) {
    if (typeof (position) === "function") {
        callback = position;
        position = 0;
    }

    validateFunction(callback, "callback");
    validateInteger(position, "position");

    let length = 0;
    for (const buf of buffer) {
        length += buf.byteLength;
    }

    let buf = new Blob([...buffer])

    binding.fwrite(fd, position, buf.arrayBuffer()).then((len) => {
        callback(null, len, buffer)
    }).catch((e) => {
        callback(e)
    })
}

function writevSync(fd, buffer, position = 0) {
    validateInteger(position, "position");

    let length = 0;
    for (const buf of buffer) {
        length += buf.byteLength;
    }

    let buf = new Blob([...buffer])

    try {
        let len = binding.fwriteSync(fd, position, buf.arrayBuffer());
        return len;
    } catch (err) {
        throw new Error(err.message);
    }
}

/// The type of the file descriptor or file is unknown or is different from any of the other types specified.
const FILETYPE_UNKNOWN = 0;
/// The file descriptor or file refers to a block device inode.
const FILETYPE_BLOCK_DEVICE = 1;
/// The file descriptor or file refers to a character device inode.
const FILETYPE_CHARACTER_DEVICE = 2;
/// The file descriptor or file refers to a directory inode.
const FILETYPE_DIRECTORY = 3;
/// The file descriptor or file refers to a regular file inode.
const FILETYPE_REGULAR_FILE = 4;
/// The file descriptor or file refers to a datagram socket.
const FILETYPE_SOCKET_DGRAM = 5;
/// The file descriptor or file refers to a byte-stream socket.
const FILETYPE_SOCKET_STREAM = 6;
/// The file refers to a symbolic link inode.
const FILETYPE_SYMBOLIC_LINK = 7;

class Dirent {
    constructor(innerData) {
        this.filetype = innerData.filetype;
        this.name = innerData.name;
    }

    isFile = () => this.filetype === FILETYPE_REGULAR_FILE;
    isDirectory = () => this.filetype === FILETYPE_DIRECTORY;
    isSymbolicLink = () => this.filetype === FILETYPE_SYMBOLIC_LINK;
    isBlockDevice = () => this.filetype === FILETYPE_BLOCK_DEVICE;
    isFIFO = () => false;
    isCharacterDevice = () => this.filetype === FILETYPE_CHARACTER_DEVICE;
    isSocket = () => this.filetype === FILETYPE_SOCKET_DGRAM || this.filetype === FILETYPE_SOCKET_STREAM;
}

class Dir {
    #fd = 0;

    constructor(fd, path) {
        this.#fd = fd;
        this.path = path;
    }

    #dataBuf = []
    #idx = 0;
    #fin = false;
    #cookie = 0;

    #fetch() {
        if (this.#idx === this.#dataBuf.length && !this.#fin) {
            let data = binding.freaddirSync(this.#fd, this.#cookie);
            this.#dataBuf.push(...data.res);
            this.#fin = data.fin;
            this.#cookie = data.cookie;
        }
        return !(this.#idx === this.#dataBuf.length && this.#fin);
    }

    close(callback) {
        if (callback) {
            close(fd, callback);
        } else {
            return fs.promises.close(fd);
        }
    }

    closeSync() {
        closeSync(fd);
    }

    read(callback) {
        if (callback) {
            try {
                if (!this.#fetch()) {
                    callback(null, null);
                }
                callback(null, new Dirent(this.#dataBuf[this.#idx++]));
            } catch (err) {
                callback(err);
            }
        } else {
            return new Promise((resolve, reject) => {
                try {
                    if (!this.#fetch()) {
                        resolve(null);
                    }
                    resolve(new Dirent(this.#dataBuf[this.#idx++]));
                } catch (err) {
                    reject(err);
                }
            })
        }
    }

    readSync() {
        if (!this.#fetch()) {
            return null;
        }
        return new Dirent(this.#dataBuf[this.#idx++]);
    }

    async *[Symbol.asyncIterator]() {
        try {
            let p = this.read();
            while (p) {
                yield p;
                p = this.read();
            }
        } finally {
            await this.close();
        }
    }
}

function opendir(path, options, callback) {
    if (typeof (options) === "function") {
        callback = options;
    }

    path = getValidatedPath(path);
    validateFunction(callback, "callback");

    setTimeout(() => {
        try {
            let fd = openSync(path);
            callback(null, new Dir(fd, path));
        } catch (err) {
            callback(err);
        }
    }, 0);
}

function opendirSync(path, options) {
    path = getValidatedPath(path);

    let fd = openSync(path);
    return new Dir(fd, path);
}

function readdir(path, options, callback) {
    if (typeof (options) === "function") {
        callback = options;
        options = {};
    }
    if (typeof (options) === "string") {
        options = {
            encoding: options
        };
    }
    setDefaultValue(options, {
        encoding: "utf8",
        withFileTypes: false
    });
    validateEncoding(options.encoding, "encoding");
    path = getValidatedPath(path);
    validateFunction(callback, "callback");

    setTimeout(async () => {
        try {
            let data = [];
            let dir = opendirSync(path);
            for await (const p of dir) {
                data.push(options.withFileTypes ? p : p.name);
            }
            callback(null, data);
        } catch (err) {
            callback(err);
        }
    }, 0);
}

function readdirSync(path, options) {
    path = getValidatedPath(path);
    if (typeof (options) === "string") {
        options = {
            encoding: options
        };
    }
    setDefaultValue(options, {
        encoding: "utf8",
        withFileTypes: false
    });
    validateEncoding(options.encoding, "encoding");
    let data = [];
    let dir = opendirSync(path);
    let p = dir.readSync();
    while (p) {
        data.push(options.withFileTypes ? p : p.name);
        p = dir.readSync();
    }
    return data;
}

function watch() {
    throw new Error(`'watch' is unsupported`);
}

function unwatch() {
    throw new Error(`'unwatch' is unsupported`);
}

function watchFile() {
    throw new Error(`'watchFile' is unsupported`);
}

function getOwnPropertyValueOrDefault(options, key, defaultValue) {
    return options == null || !Object.prototype.hasOwnProperty(options, key) ?
        defaultValue :
        options[key];
}

/**
 * @callback validateObject
 * @param {*} value
 * @param {string} name
 * @param {{
 *   allowArray?: boolean,
 *   allowFunction?: boolean,
 *   nullable?: boolean
 * }} [options]
 */

/** @type {validateObject} */
const validateObject = hideStackFrames(
    (value, name, options = null) => {
        const allowArray = getOwnPropertyValueOrDefault(options, 'allowArray', false);
        const allowFunction = getOwnPropertyValueOrDefault(options, 'allowFunction', false);
        const nullable = getOwnPropertyValueOrDefault(options, 'nullable', false);
        if ((!nullable && value === null) ||
            (!allowArray && Array.isArray(value)) ||
            (typeof value !== 'object' && (
                !allowFunction || typeof value !== 'function'
            ))) {
            throw new errors.ERR_INVALID_ARG_TYPE(name, 'Object', value);
        }
    });


const validateCpOptions = hideStackFrames((options) => {
    if (options === undefined)
        return { ...defaultCpOptions };
    validateObject(options, 'options');
    options = { ...defaultCpOptions, ...options };
    validateBoolean(options.dereference, 'options.dereference');
    validateBoolean(options.errorOnExist, 'options.errorOnExist');
    validateBoolean(options.force, 'options.force');
    validateBoolean(options.preserveTimestamps, 'options.preserveTimestamps');
    validateBoolean(options.recursive, 'options.recursive');
    validateBoolean(options.verbatimSymlinks, 'options.verbatimSymlinks');
    if (options.dereference === true && options.verbatimSymlinks === true) {
        throw new errors.ERR_INCOMPATIBLE_OPTION_PAIR('dereference', 'verbatimSymlinks');
    }
    if (options.filter !== undefined) {
        validateFunction(options.filter, 'options.filter');
    }
    return options;
});

/**
 * Synchronously copies `src` to `dest`. `src` can be a file, directory, or
 * symlink. The contents of directories will be copied recursively.
 * @param {string | URL} src
 * @param {string | URL} dest
 * @param {object} [options]
 * @returns {void}
 */
function cpSync(src, dest, options) {
    options = validateCpOptions(options);
    src = getValidatedPath(src, 'src');
    dest = getValidatedPath(dest, 'dest');
    cpSyncFn(src, dest, options);
}

/**
 * Asynchronously copies `src` to `dest`. `src` can be a file, directory, or
 * symlink. The contents of directories will be copied recursively.
 * @param {string | URL} src
 * @param {string | URL} dest
 * @param {object} [options]
 * @param {() => any} callback
 * @returns {void}
 */
function cp(src, dest, options, callback) {
    if (typeof options === 'function') {
        callback = options;
        options = undefined;
    }
    callback = makeCallback(callback);
    options = validateCpOptions(options);
    src = getValidatedPath(src, 'src');
    dest = getValidatedPath(dest, 'dest');
    cpFn(src, dest, options, callback);
}

class FileHandle extends EventEmitter {
    fd = 0;
    #path = "";
    constructor(fd, path) {
        this.fd = fd;
        this.#path = path;
    }
    appendFile(data, options) {
        let encoding = options.encoding || "utf8";
        return promisify(write)(this.fd, typeof (data) === "string" ? data : data.toString(encoding));
    }
    chown() {
        return new Promise((res, rej) => { res(undefined); });
    }

    close() {
        this.emit("close");
        return promisify(close)(this.fd);
    }

    createReadStream(options) {
        return createReadStream(this.#path, options);
    }

    createWriteStream(options) {
        return createWriteStream(this.#path, options);
    }

    dataSync() {
        return promisify(fdatasync)(this.fd);
    }

    read(...args) {
        return promisify(read)(this.fd, ...args);
    }

    readFile(options) {
        return promisify(readFile)(this.#path, options)
    }

    readv(buffers, position) {
        return promisify(readv)(this.fd, buffers, position);
    }

    stat(options) {
        return promisify(fstat)(this.fd, options);
    }

    sync() {
        return promisify(fsync)(this.fd);
    }

    truncate(len) {
        return promisify(ftruncate)(this.fd, len);
    }

    utimes(atime, mtime) {
        return promisify(futimes)(this.fd, atime, mtime);
    }

    write(...args) {
        return promisify(write)(this.fd, ...args);
    }

    writeFile(data, options) {
        return promisify(writeFile)(this.#path, data, options);
    }

    writev(buffers, position) {
        return promisify(writev)(this.fd, buffers, position);
    }
}

export {
    stat,
    statSync,
    lstat,
    lstatSync,
    fstat,
    fstatSync,
    access,
    accessSync,
    exists,
    existsSync,
    mkdir,
    mkdirSync,
    fchown,
    fchownSync,
    chown,
    chownSync,
    lchown,
    lchownSync,
    rmdir,
    rmdirSync,
    rm,
    rmSync,
    fchmod,
    fchmodSync,
    lchmod,
    lchmodSync,
    chmod,
    chmodSync,
    futimes,
    futimesSync,
    lutimes,
    lutimesSync,
    utimes,
    utimesSync,
    rename,
    renameSync,
    unlink,
    unlinkSync,
    truncate,
    truncateSync,
    ftruncate,
    ftruncateSync,
    realpath,
    realpathSync,
    mkdtemp,
    mkdtempSync,
    copyFile,
    copyFileSync,
    link,
    linkSync,
    symlink,
    symlinkSync,
    close,
    closeSync,
    fdatasync,
    fdatasyncSync,
    fsync,
    fsyncSync,
    read,
    readSync,
    open,
    openSync,
    readFile,
    readFileSync,
    readlink,
    readlinkSync,
    readv,
    readvSync,
    write,
    writeSync,
    writeFile,
    writeFileSync,
    appendFile,
    appendFileSync,
    writev,
    writevSync,
    opendir,
    opendirSync,
    Dir,
    Dirent,
    readdir,
    readdirSync,
    watch,
    watchFile,
    unwatch,
    cp,
    cpSync,
    createWriteStream,
    WriteStream,
    createReadStream,
    ReadStream,
    FileHandle,
}