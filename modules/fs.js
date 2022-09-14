import { validateFunction } from "./internal/validators"
import { getValidatedPath, getValidMode, Stats } from "./internal/fs/utils"
import * as binding from "_node:fs"
import * as errors from "./internal/errors"
export { fs as constants } from "./internal_binding/constants"
import { fs as constants } from "./internal_binding/constants"

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


function stat() {
    // TODO
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
        throw new Error(err.message);
    }
}

function lstat() {
    // TODO
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

function access(path, mode = constants.F_OK, callback) {
    // TODO
}

function accessSync(path, mode = constants.F_OK) {
    path = getValidatedPath(path);

    try {
        const stat = statSync(path, { throwIfNoEntry: true });
        if ((stat.mode & mode) === mode) {
            return undefined;
        } else {
            throw new Error(`EACCES: permission denied, access '${path}'`);
        }
    } catch (err) {
        throw err;
    }
}

function exists(path, callback) {
    // TODO
}

function existsSync(path) {
    path = getValidatedPath(path);

    try {
        accessSync(path);
        return true;
    } catch (err) {
        return false;
    }
}

function mkdir(path, mode, callback) {
    // TODO
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

function fchown(fd, uid, gid, callback) {
    validateFunction(callback);

    callback(undefined);
}

function fchownSync(fd, uid, gid, callback) {
    return undefined;
}

function lchown(fd, uid, gid, callback) {
    validateFunction(callback);

    callback(undefined);
}

function lchownSync(fd, uid, gid, callback) {
    return undefined;
}

function chown(fd, uid, gid, callback) {
    validateFunction(callback);

    callback(undefined);
}

function chownSync(fd, uid, gid, callback) {
    return undefined;
}

function rmdir(path, options, callback) {
    // TODO
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
    // TODO
}

function rmSync(path, options = { force: false, maxRetries: 0, recursive: false, retryDelay: 100 }) {
    path = getValidatedPath(path);

    setDefaultValue(options, { force: false, maxRetries: 0, recursive: false, retryDelay: 100 });

    try {
        binding.rmSync(path, options.recursive, options.force);
    } catch (err) {
        throw new Error(err.message);
    }
}

export default {
    stat,
    statSync,
    lstat,
    lstatSync,
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
    rmSync
}

export {
    stat,
    statSync,
    lstat,
    lstatSync,
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
    rmSync
}
