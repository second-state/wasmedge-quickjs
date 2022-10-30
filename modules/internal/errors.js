import { inspect } from "./util/inspect";

/**
 * 
 * @template T
 * @param {T} fn 
 * @return {T}
 */
export function hideStackFrames(fn) {
    const hidden = "__node_internal_" + fn.name;
    Object.defineProperty(fn, "name", { value: hidden });

    return fn;
}

export class ERR_HTTP_HEADERS_SENT extends Error {
    constructor(x) {
        super(
            `Cannot ${x} headers after they are sent to the client`,
        );
        this.code = "ERR_HTTP_HEADERS_SENT";
    }
}

export class ERR_HTTP_INVALID_HEADER_VALUE extends TypeError {
    constructor(x, y) {
        super(
            `Invalid value "${x}" for header "${y}"`,
        );
        this.code = "ERR_HTTP_INVALID_HEADER_VALUE";
    }
}

export class ERR_HTTP_TRAILER_INVALID extends Error {
    constructor() {
        super(
            `Trailers are invalid with this transfer encoding`,
        );
        this.code = "ERR_HTTP_TRAILER_INVALID";
    }
}

export class ERR_INVALID_HTTP_TOKEN extends TypeError {
    constructor(x, y) {
        super(`${x} must be a valid HTTP token ["${y}"]`);
        this.code = "ERR_INVALID_HTTP_TOKEN";
    }
}

const classRegExp = /^([A-Z][a-z0-9]*)+$/;

const kTypes = [
    "string",
    "function",
    "number",
    "object",
    "Function",
    "Object",
    "boolean",
    "bigint",
    "symbol",
];

function createInvalidArgType(name, expected) {
    expected = Array.isArray(expected) ? expected : [expected];
    let msg = "The ";
    if (name.endsWith(" argument")) {
        msg += `${name} `;
    } else {
        const type = name.includes(".") ? "property" : "argument";
        msg += `"${name}" ${type} `;
    }
    msg += "must be ";

    const types = [];
    const instances = [];
    const other = [];
    for (const value of expected) {
        if (kTypes.includes(value)) {
            types.push(value.toLocaleLowerCase());
        } else if (classRegExp.test(value)) {
            instances.push(value);
        } else {
            other.push(value);
        }
    }

    if (instances.length > 0) {
        const pos = types.indexOf("object");
        if (pos !== -1) {
            types.splice(pos, 1);
            instances.push("Object");
        }
    }

    if (types.length > 0) {
        if (types.length > 2) {
            const last = types.pop();
            msg += `one of type ${types.join(", ")}, or ${last}`;
        } else if (types.length === 2) {
            msg += `one of type ${types[0]} or ${types[1]}`;
        } else {
            msg += `of type ${types[0]}`;
        }
        if (instances.length > 0 || other.length > 0) {
            msg += " or ";
        }
    }

    if (instances.length > 0) {
        if (instances.length > 2) {
            const last = instances.pop();
            msg += `an instance of ${instances.join(", ")}, or ${last}`;
        } else {
            msg += `an instance of ${instances[0]}`;
            if (instances.length === 2) {
                msg += ` or ${instances[1]}`;
            }
        }
        if (other.length > 0) {
            msg += " or ";
        }
    }

    if (other.length > 0) {
        if (other.length > 2) {
            const last = other.pop();
            msg += `one of ${other.join(", ")}, or ${last}`;
        } else if (other.length === 2) {
            msg += `one of ${other[0]} or ${other[1]}`;
        } else {
            if (other[0].toLowerCase() !== other[0]) {
                msg += "an ";
            }
            msg += `${other[0]}`;
        }
    }

    return msg;
}

function invalidArgTypeHelper(input) {
    if (input == null) {
        return ` Received ${input}`;
    }
    if (typeof input === "function" && input.name) {
        return ` Received function ${input.name}`;
    }
    if (typeof input === "object") {
        if (input.constructor && input.constructor.name) {
            return ` Received an instance of ${input.constructor.name}`;
        }
        return ` Received ${inspect(input, { depth: -1 })}`;
    }
    let inspected = inspect(input, { colors: false });
    if (inspected.length > 25) {
        inspected = `${inspected.slice(0, 25)}...`;
    }
    return ` Received type ${typeof input} (${inspected})`;
}

/**
 * 
 * @param {string} val 
 * @returns {string}
 */
function addNumericalSeparator(val) {
    let res = "";
    let i = val.length;
    const start = val[0] === "-" ? 1 : 0;
    for (; i >= start + 4; i -= 3) {
        res = `_${val.slice(i - 3, i)}${res}`;
    }
    return `${val.slice(0, i)}${res}`;
}

export class ERR_OUT_OF_RANGE extends RangeError {
    code = "ERR_OUT_OF_RANGE";

    /**
     * 
     * @param {string} str 
     * @param {string} range 
     * @param {unknown} input 
     * @param {boolean} replaceDefaultBoolean 
     */
    constructor(
        str,
        range,
        input,
        replaceDefaultBoolean = false,
    ) {
        // assert(range, 'Missing "range" argument');
        let msg = replaceDefaultBoolean
            ? str
            : `The value of "${str}" is out of range.`;
        let received;
        if (Number.isInteger(input) && Math.abs(input) > 2 ** 32) {
            received = addNumericalSeparator(String(input));
        } else if (typeof input === "bigint") {
            received = String(input);
            if (input > 2n ** 32n || input < -(2n ** 32n)) {
                received = addNumericalSeparator(received);
            }
            received += "n";
        } else {
            received = inspect(input);
        }
        msg += ` It must be ${range}. Received ${received}`;

        super(msg);

        const { name } = this;
        // Add the error code to the name to include it in the stack trace.
        this.name = `${name} [${this.code}]`;
        // Access the stack to generate the error message including the error code from the name.
        this.stack;
        // Reset the name to the actual name.
        this.name = name;
    }
}

export class ERR_INVALID_ARG_TYPE_RANGE extends RangeError {
    constructor(name, expected, actual) {
        const msg = createInvalidArgType(name, expected);

        super(`${msg}.${invalidArgTypeHelper(actual)}`);
        this.code = "ERR_INVALID_ARG_TYPE";
    }
}

export class ERR_INVALID_ARG_TYPE extends TypeError {
    /**
     * 
     * @param {string} name 
     * @param {string | string[]} expected 
     * @param {unknown} actual 
     */
    constructor(name, expected, actual) {
        const msg = createInvalidArgType(name, expected);

        super(`${msg}.${invalidArgTypeHelper(actual)}`);
        this.code = "ERR_INVALID_ARG_TYPE";
    }

    static RangeError = ERR_INVALID_ARG_TYPE_RANGE;
}

export class ERR_INVALID_ARG_VALUE_RANGE extends RangeError {
    constructor(name, value, reason = "is invalid") {
        const type = name.includes(".") ? "property" : "argument";
        const inspected = JSON.stringify(value);

        super(`The ${type} '${name}' ${reason}. Received ${inspected}`,);

        this.code = "ERR_INVALID_ARG_VALUE"
    }
}

export class ERR_INVALID_ARG_VALUE extends TypeError {
    constructor(name, value, reason = "is invalid") {
        const type = name.includes(".") ? "property" : "argument";
        const inspected = JSON.stringify(value);

        super(`The ${type} '${name}' ${reason}. Received ${inspected}`,);

        this.code = "ERR_INVALID_ARG_VALUE"
    }
}

export class ERR_INVALID_CHAR extends TypeError {
    constructor(name, field) {
        super(field
            ? `Invalid character in ${name}`
            : `Invalid character in ${name} ["${field}"]`,
        );
        this.code = "ERR_INVALID_CHAR";
    }
}

export class ERR_METHOD_NOT_IMPLEMENTED extends Error {
    constructor(x) {
        super(`The ${x} method is not implemented`);
        this.code = "ERR_METHOD_NOT_IMPLEMENTED";
    }
}

export class ERR_STREAM_CANNOT_PIPE extends Error {
    constructor() {
        super(`Cannot pipe, not readable`);
        this.code = "ERR_STREAM_CANNOT_PIPE";
    }
}

export class ERR_STREAM_ALREADY_FINISHED extends Error {
    constructor(x) {
        super(
            `Cannot call ${x} after a stream was finished`,
        );
        this.code = "ERR_STREAM_ALREADY_FINISHED";
    }
}

export class ERR_STREAM_WRITE_AFTER_END extends Error {
    constructor() {
        super(`write after end`);
        this.code = "ERR_STREAM_WRITE_AFTER_END";
    }
}

export class ERR_STREAM_NULL_VALUES extends TypeError {
    constructor() {
        super(`May not write null values to stream`);
        this.code = "ERR_STREAM_NULL_VALUES";
    }
}

export class ERR_STREAM_DESTROYED extends Error {
    constructor(x) {
        super(
            `Cannot call ${x} after a stream was destroyed`,
        );
        this.code = "ERR_STREAM_DESTROYED";
    }
}

export function aggregateTwoErrors(innerError, outerError) {
    if (innerError && outerError && innerError !== outerError) {
        if (Array.isArray(outerError.errors)) {
            // If `outerError` is already an `AggregateError`.
            outerError.errors.push(innerError);
            return outerError;
        }
        // eslint-disable-next-line no-restricted-syntax
        const err = new AggregateError(
            [
                outerError,
                innerError,
            ],
            outerError.message,
        );
        // deno-lint-ignore no-explicit-any
        err.code = outerError.code;
        return err;
    }
    return innerError || outerError;
}

export class ERR_SOCKET_BAD_PORT extends RangeError {
    constructor(name, port, allowZero = true) {
        assert(
            typeof allowZero === "boolean",
            "The 'allowZero' argument must be of type boolean.",
        );

        const operator = allowZero ? ">=" : ">";

        super(
            `${name} should be ${operator} 0 and < 65536. Received ${port}.`,
        );
        this.code = "ERR_SOCKET_BAD_PORT";
    }
}

export class ERR_STREAM_PREMATURE_CLOSE extends Error {
    constructor() {
        super(`Premature close`);
        this.code = "ERR_STREAM_PREMATURE_CLOSE";
    }
}

export class AbortError extends Error {
    constructor() {
        super("The operation was aborted");
        this.code = "ABORT_ERR";
        this.name = "AbortError";
    }
}

export class ERR_INVALID_CALLBACK extends TypeError {
    constructor(object) {
        super(
            `Callback must be a function. Received ${JSON.stringify(object)}`,
        );
        this.code = "ERR_INVALID_CALLBACK";
    }
}

export class ERR_MISSING_ARGS extends TypeError {
    constructor(...args) {
        let msg = "The ";

        const len = args.length;

        const wrap = (a) => `"${a}"`;

        args = args.map((a) =>
            Array.isArray(a) ? a.map(wrap).join(" or ") : wrap(a)
        );

        switch (len) {
            case 1:
                msg += `${args[0]} argument`;
                break;
            case 2:
                msg += `${args[0]} and ${args[1]} arguments`;
                break;
            default:
                msg += args.slice(0, len - 1).join(", ");
                msg += `, and ${args[len - 1]} arguments`;
                break;
        }

        super(`${msg} must be specified`);
        this.code = "ERR_MISSING_ARGS";
    }
}
export class ERR_MISSING_OPTION extends TypeError {
    constructor(x) {
        super(`${x} is required`);
        this.code = "ERR_MISSING_OPTION";
    }
}
export class ERR_MULTIPLE_CALLBACK extends Error {
    constructor() {
        super(`Callback called multiple times`);
        this.code = "ERR_MULTIPLE_CALLBACK";
    }
}

export class ERR_STREAM_PUSH_AFTER_EOF extends Error {
    constructor() {
        super(`stream.push() after EOF`);
        this.code = "ERR_STREAM_PUSH_AFTER_EOF";
    }
}

export class ERR_STREAM_UNSHIFT_AFTER_END_EVENT extends Error {
    constructor() {
        super(
            `stream.unshift() after end event`,
        );
        this.code = "ERR_STREAM_UNSHIFT_AFTER_END_EVENT";
    }
}

export class ERR_UNKNOWN_ENCODING extends TypeError {
    constructor(x) {
        super(`Unknown encoding: ${x}`);
        this.code = "ERR_UNKNOWN_ENCODING";
    }
}

function buildReturnPropertyType(value) {
    if (value && value.constructor && value.constructor.name) {
        return `instance of ${value.constructor.name}`;
    } else {
        return `type ${typeof value}`;
    }
}

export class ERR_INVALID_RETURN_VALUE extends TypeError {
    constructor(input, name, value) {
        super(
            `Expected ${input} to be returned from the "${name}" function but got ${buildReturnPropertyType(value)}.`,
        );
        this.code = "ERR_INVALID_RETURN_VALUE";
    }
}

export class ERR_INCOMPATIBLE_OPTION_PAIR extends TypeError {
    constructor(input, name) {
        super(
            `Option "${input}" cannot be used in combination with option "${name}"`,
        );
        this.code = "ERR_INCOMPATIBLE_OPTION_PAIR";
    }
}

export const captureStackTrace = hideStackFrames(
    function captureStackTrace(err) {
        // Error.captureStackTrace is only available in V8
        const e = new Error();
        Object.defineProperties(err, {
            stack: {
                configurable: true,
                writable: true,
                get: () => e.stack
            }
        })
        return err;
    },
);

const captureLargerStackTrace = hideStackFrames(
    function captureLargerStackTrace(err) {
        captureStackTrace(err);

        return err;
    },
);


/**
 * All error instances in Node have additional methods and properties
 * This export class is meant to be extended by these instances abstracting native JS error instances
 */
export class NodeErrorAbstraction extends Error {
    /**
     * @type {string}
     */
    code;

    /**
     * 
     * @param {string} name 
     * @param {string} code 
     * @param {string} message 
     */
    constructor(name, code, message) {
        super(message);
        this.code = code;
        this.name = name;
        //This number changes depending on the name of this class
        //20 characters as of now
        this.stack = this.stack && `${name} [${this.code}]${this.stack.slice(20)}`;
    }

    toString() {
        return `${this.name} [${this.code}]: ${this.message}`;
    }
}

const kIsNodeError = Symbol("kIsNodeError");

/**
 * @typedef {Object} NodeSystemErrorCtx
 * @property {string} code
 * @property {string} syscall
 * @property {string} message
 * @property {number} errno
 * @property {string=} path
 * @property {string=} dest
 */

class NodeSystemError extends NodeErrorAbstraction {
    /**
     * 
     * @param {string} key 
     * @param {NodeSystemErrorCtx} context 
     * @param {string} msgPrefix 
     */
    constructor(key, context, msgPrefix) {
        let message = `${msgPrefix}: ${context.syscall} returned ` +
            `${context.code} (${context.message})`;

        if (context.path !== undefined) {
            message += ` ${context.path}`;
        }
        if (context.dest !== undefined) {
            message += ` => ${context.dest}`;
        }

        super("SystemError", key, message);
        // captureLargerStackTrace(this);

        Object.defineProperties(this, {
            [kIsNodeError]: {
                value: true,
                enumerable: false,
                writable: false,
                configurable: true,
            },
            info: {
                value: context,
                enumerable: true,
                configurable: true,
                writable: false,
            },
            errno: {
                get() {
                    return context.errno;
                },
                set: (value) => {
                    context.errno = value;
                },
                enumerable: true,
                configurable: true,
            },
            syscall: {
                get() {
                    return context.syscall;
                },
                set: (value) => {
                    context.syscall = value;
                },
                enumerable: true,
                configurable: true,
            },
        });

        if (context.path !== undefined) {
            Object.defineProperty(this, "path", {
                get() {
                    return context.path;
                },
                set: (value) => {
                    context.path = value;
                },
                enumerable: true,
                configurable: true,
            });
        }

        if (context.dest !== undefined) {
            Object.defineProperty(this, "dest", {
                get() {
                    return context.dest;
                },
                set: (value) => {
                    context.dest = value;
                },
                enumerable: true,
                configurable: true,
            });
        }
    }

    toString() {
        return `${this.name} [${this.code}]: ${this.message}`;
    }
}

/**
 * 
 * @param {string} key 
 * @param {string} msgPrfix 
 */
function makeSystemErrorWithCode(key, msgPrfix) {
    return class NodeError extends NodeSystemError {
        /**
         * 
         * @param {NodeSystemErrorCtx} ctx 
         */
        constructor(ctx) {
            super(key, ctx, msgPrfix);
        }
    };
}

export const ERR_FS_EISDIR = makeSystemErrorWithCode(
    "ERR_FS_EISDIR",
    "Path is a directory",
);

export const ERR_FS_CP_DIR_TO_NON_DIR = makeSystemErrorWithCode('ERR_FS_CP_DIR_TO_NON_DIR',
    'Cannot overwrite directory with non-directory');
export const ERR_FS_CP_EEXIST = makeSystemErrorWithCode('ERR_FS_CP_EEXIST', 'Target already exists');
export const ERR_FS_CP_EINVAL = makeSystemErrorWithCode('ERR_FS_CP_EINVAL', 'Invalid src or dest');
export const ERR_FS_CP_FIFO_PIPE = makeSystemErrorWithCode('ERR_FS_CP_FIFO_PIPE', 'Cannot copy a FIFO pipe');
export const ERR_FS_CP_NON_DIR_TO_DIR = makeSystemErrorWithCode('ERR_FS_CP_NON_DIR_TO_DIR',
    'Cannot overwrite non-directory with directory');
export const ERR_FS_CP_SOCKET = makeSystemErrorWithCode('ERR_FS_CP_SOCKET', 'Cannot copy a socket file');
export const ERR_FS_CP_SYMLINK_TO_SUBDIRECTORY = makeSystemErrorWithCode('ERR_FS_CP_SYMLINK_TO_SUBDIRECTORY',
    'Cannot overwrite symlink in subdirectory of self');
export const ERR_FS_CP_UNKNOWN = makeSystemErrorWithCode('ERR_FS_CP_UNKNOWN', 'Cannot copy an unknown file type');

export class ERR_FS_INVALID_SYMLINK_TYPE extends Error {
    /**
     * 
     * @param {string} x 
     */
    constructor(x) {
        super(
            "ERR_FS_INVALID_SYMLINK_TYPE",
            `Symlink type must be one of "dir", "file", or "junction". Received "${x}"`,
        );
    }
}

/**
 * 
 * @param {number} name 
 * @returns {[string, string]}
 */
function uvErrmapGet(name) {
    return errorMap.get(name);
}

const uvUnmappedError = ["UNKNOWN", "unknown error"];

/**
 * This creates an error compatible with errors produced in the C++
 * function UVException using a context object with data assembled in C++.
 * The goal is to migrate them to ERR_* errors later when compatibility is
 * not a concern.
 */
export const uvException = hideStackFrames(
    /**
     * 
     * @param {NodeSystemErrorCtx} ctx 
     * @returns 
     */
    function uvException(ctx) {
        const { 0: code, 1: uvmsg } = uvErrmapGet(ctx.errno) || uvUnmappedError;

        let message = `${code}: ${ctx.message || uvmsg}, ${ctx.syscall}`;

        let path;
        let dest;

        if (ctx.path) {
            path = ctx.path.toString();
            message += ` '${path}'`;
        }
        if (ctx.dest) {
            dest = ctx.dest.toString();
            message += ` -> '${dest}'`;
        }


        const err = new Error(message);

        for (const prop of Object.keys(ctx)) {
            if (prop === "message" || prop === "path" || prop === "dest") {
                continue;
            }

            err[prop] = ctx[prop];
        }

        err.code = code;

        if (path) {
            err.path = path;
        }

        if (dest) {
            err.dest = dest;
        }

        return captureLargerStackTrace(err);
    }
);

export function isErrorStackTraceLimitWritable() {
    // Do no touch Error.stackTraceLimit as V8 would attempt to install
    // it again during deserialization.
    if (false && import('v8').startupSnapshot.isBuildingSnapshot()) {
        return false;
    }

    const desc = Object.getOwnPropertyDescriptor(Error, 'stackTraceLimit');
    if (desc === undefined) {
        return Object.isExtensible(Error);
    }

    return Object.prototype.hasOwnProperty(desc, 'writable') ?
        desc.writable :
        desc.set !== undefined;
}

export class ERR_UNAVAILABLE_DURING_EXIT extends Error {
    constructor() {
        super(
            "ERR_UNAVAILABLE_DURING_EXIT",
            `Cannot call function in process exit handler`,
        );
    }
}

export class ERR_ASSERT_SNAPSHOT_NOT_SUPPORTED extends TypeError {
    constructor() {
        super(
            "ERR_ASSERT_SNAPSHOT_NOT_SUPPORTED",
            `Snapshot is not supported in this context`,
        );
    }
}

export class ERR_AMBIGUOUS_ARGUMENT extends TypeError {
    constructor(arg, msg) {
        super(
            "ERR_AMBIGUOUS_ARGUMENT",
            `The ${arg} argument is ambiguous. ${msg}`,
        );
    }
}

export class ERR_DIR_CLOSED extends Error {
    constructor() {
        super("Directory handle was closed");
        this.code = "ERR_DIR_CLOSED";
    }
}

export class ERR_DIR_CONCURRENT_OPERATION extends Error {
    constructor() {
        super(
            "Cannot do synchronous work on directory handle with concurrent asynchronous operations",
        );
        this.code = "ERR_DIR_CONCURRENT_OPERATION";
    }
}

export class ERR_FS_FILE_TOO_LARGE extends RangeError {
    constructor(x) {
        super(
            `File size (${x}) is greater than 2 GB`,
        );
        this.code = "ERR_FS_FILE_TOO_LARGE";
    }
}

