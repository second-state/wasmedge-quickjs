import { promisify, deprecate } from "./internal/util.js";
import { debuglog } from "./internal/util/debuglog.js";
import types from "util/types";
import { Buffer } from "buffer";
import { ERR_INVALID_ARG_TYPE } from "./internal/errors.js";
import * as encoding from 'encoding';

export const debuglog = debuglog
export const promisify = promisify;
export const deprecate = deprecate;

class NodeFalsyValueRejectionError extends Error {
    reason;
    code = "ERR_FALSY_VALUE_REJECTION";
    constructor(reason) {
        super("Promise was rejected with falsy value");
        this.reason = reason;
    }
}

class NodeInvalidArgTypeError extends TypeError {
    code = "ERR_INVALID_ARG_TYPE";
    constructor(argumentName) {
        super(`The ${argumentName} argument must be of type function.`);
    }
}

function callbackify(original) {
    if (typeof original !== "function") {
        throw new NodeInvalidArgTypeError('"original"');
    }

    const callbackified = function (_this, ...args) {
        const maybeCb = args.pop();
        if (typeof maybeCb !== "function") {
            throw new NodeInvalidArgTypeError("last");
        }
        const cb = (...args) => {
            maybeCb.apply(_this, args);
        };
        original.apply(_this, args).then(
            (ret) => {
                nextTick(cb.bind(_this, null, ret));
            },
            (rej) => {
                rej = rej || new NodeFalsyValueRejectionError(rej);
                nextTick(cb.bind(_this, rej));
            },
        );
    };

    const descriptors = Object.getOwnPropertyDescriptors(original);
    // It is possible to manipulate a functions `length` or `name` property. This
    // guards against the manipulation.
    if (typeof descriptors.length.value === "number") {
        descriptors.length.value++;
    }
    if (typeof descriptors.name.value === "string") {
        descriptors.name.value += "Callbackified";
    }
    Object.defineProperties(callbackified, descriptors);
    return callbackified;
}

export function isArray(value) {
    return Array.isArray(value);
}

export function isBoolean(value) {
    return typeof value === "boolean" || value instanceof Boolean;
}

export function isNull(value) {
    return value === null;
}

export function isNullOrUndefined(value) {
    return value === null || value === undefined;
}

export function isNumber(value) {
    return typeof value === "number" || value instanceof Number;
}

export function isString(value) {
    return typeof value === "string" || value instanceof String;
}

export function isSymbol(value) {
    return typeof value === "symbol";
}

export function isUndefined(value) {
    return value === undefined;
}

export function isObject(value) {
    return value !== null && typeof value === "object";
}

export function isError(e) {
    return e instanceof Error;
}

export function isFunction(value) {
    return typeof value === "function";
}

export function isRegExp(value) {
    return types.isRegExp(value);
}

export function isDate(value) {
    return types.isDate(value);
}

export function isPrimitive(value) {
    return (
        value === null || (typeof value !== "object" && typeof value !== "function")
    );
}

export function isBuffer(value) {
    return Buffer.isBuffer(value);
}

export function _extend(target, source) {
    // Don't do anything if source isn't an object
    if (source === null || typeof source !== "object") return target;

    const keys = Object.keys(source);
    let i = keys.length;
    while (i--) {
        target[keys[i]] = source[keys[i]];
    }
    return target;
}

export function inherits(ctor, superCtor) {
    if (ctor === undefined || ctor === null) {
        throw new ERR_INVALID_ARG_TYPE("ctor", "Function", ctor);
    }

    if (superCtor === undefined || superCtor === null) {
        throw new ERR_INVALID_ARG_TYPE("superCtor", "Function", superCtor);
    }

    if (superCtor.prototype === undefined) {
        throw new ERR_INVALID_ARG_TYPE(
            "superCtor.prototype",
            "Object",
            superCtor.prototype,
        );
    }
    Object.defineProperty(ctor, "super_", {
        value: superCtor,
        writable: true,
        configurable: true,
    });
    Object.setPrototypeOf(ctor.prototype, superCtor.prototype);
}

export const TextDecoder = encoding.TextDecoder;
export const TextEncoder = encoding.TextEncoder;

function pad(n) {
    return n.toString().padStart(2, "0");
}

const months = [
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
];

function timestamp() {
    const d = new Date();
    const t = [
        pad(d.getHours()),
        pad(d.getMinutes()),
        pad(d.getSeconds()),
    ].join(":");
    return `${(d.getDate())} ${months[(d).getMonth()]} ${t}`;
}

export function log(...args) {
    console.log(timestamp(), '-', ...args);
}

export default {
    isArray,
    isBoolean,
    isNull,
    isNullOrUndefined,
    isNumber,
    isString,
    isSymbol,
    isUndefined,
    isObject,
    isError,
    isFunction,
    isRegExp,
    isDate,
    isPrimitive,
    isBuffer,
    _extend,
    deprecate,
    callbackify,
    promisify,
    inherits,
    types,
    TextDecoder,
    TextEncoder,
    log,
    debuglog,
};
