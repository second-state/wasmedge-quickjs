import {
    ERR_SOCKET_BAD_PORT,
    ERR_INVALID_ARG_TYPE,
    ERR_INVALID_CALLBACK,
    ERR_OUT_OF_RANGE,
    hideStackFrames,
    ERR_INVALID_ARG_VALUE
} from './errors'

export function validatePort(port, name = "Port", allowZero = true) {
    if (
        (typeof port !== "number" && typeof port !== "string") ||
        (typeof port === "string" &&
            port.trim().length === 0) ||
        +port !== (+port >>> 0) ||
        port > 0xFFFF ||
        (port === 0 && !allowZero)
    ) {
        throw new ERR_SOCKET_BAD_PORT(name, port, allowZero);
    }

    return port;
}

export const validateFunction = hideStackFrames(
    (value, name) => {
        if (typeof value !== "function") {
            throw new ERR_INVALID_ARG_TYPE(name, "Function", value);
        }
    },
);

export function validateString(value, name) {
    if (typeof value !== "string") {
        throw new ERR_INVALID_ARG_TYPE(name, "string", value);
    }
}

/**
 * @param {unknown} value
 * @param {string} name
 */
export function validateBoolean(value, name) {
    if (typeof value !== "boolean") {
        throw new ERR_INVALID_ARG_TYPE(name, "boolean", value);
    }
}


/**
 * @param {unknown} signal
 * @param {string} name
 */
export const validateAbortSignal = hideStackFrames(
    (signal, name) => {
        if (
            signal !== undefined &&
            (signal === null ||
                typeof signal !== "object" ||
                !("aborted" in signal))
        ) {
            throw new ERR_INVALID_ARG_TYPE(name, "AbortSignal", signal);
        }
    },
);

export const validateObject = hideStackFrames((value, name, options) => {
    const useDefaultOptions = options == null;
    const allowArray = useDefaultOptions ? false : options.allowArray;
    const allowFunction = useDefaultOptions ? false : options.allowFunction;
    const nullable = useDefaultOptions ? false : options.nullable;
    if (
        (!nullable && value === null) ||
        (!allowArray && Array.isArray(value)) ||
        (typeof value !== "object" && (
            !allowFunction || typeof value !== "function"
        ))
    ) {
        throw new ERR_INVALID_ARG_TYPE(name, "Object", value);
    }
});

export const validateCallback = hideStackFrames((callback) => {
    if (typeof callback !== "function") {
        throw new ERR_INVALID_CALLBACK(callback);
    }
});

/**
 * @param {number} value
 * @returns {boolean}
 */
function isInt32(value) {
    return value === (value | 0);
}

/**
 * @param {unknown} value
 * @returns {boolean}
 */
function isUint32(value) {
    return value === (value >>> 0);
}

export const validateInt32 = hideStackFrames(
    (value, name, min = -2147483648, max = 2147483647) => {
        // The defaults for min and max correspond to the limits of 32-bit integers.
        if (!isInt32(value)) {
            if (typeof value !== "number") {
                throw new ERR_INVALID_ARG_TYPE(name, "number", value);
            }

            if (!Number.isInteger(value)) {
                throw new ERR_OUT_OF_RANGE(name, "an integer", value);
            }

            throw new ERR_OUT_OF_RANGE(name, `>= ${min} && <= ${max}`, value);
        }

        if (value < min || value > max) {
            throw new ERR_OUT_OF_RANGE(name, `>= ${min} && <= ${max}`, value);
        }
    },
);

export const validateUint32 = hideStackFrames(
    (value, name, positive) => {
        if (!isUint32(value)) {
            if (typeof value !== "number") {
                throw new ERR_INVALID_ARG_TYPE(name, "number", value);
            }
            if (!Number.isInteger(value)) {
                throw new ERR_OUT_OF_RANGE(name, "an integer", value);
            }
            const min = positive ? 1 : 0;
            // 2 ** 32 === 4294967296
            throw new ERR_OUT_OF_RANGE(
                name,
                `>= ${min} && < 4294967296`,
                value,
            );
        }
        if (positive && value === 0) {
            throw new ERR_OUT_OF_RANGE(name, ">= 1 && < 4294967296", value);
        }
    },
);

export const validateInteger = hideStackFrames(
    (
        value,
        name,
        min = Number.MIN_SAFE_INTEGER,
        max = Number.MAX_SAFE_INTEGER,
    ) => {
        if (typeof value !== "number") {
            throw new ERR_INVALID_ARG_TYPE(name, "number", value);
        }
        if (!Number.isInteger(value)) {
            throw new ERR_OUT_OF_RANGE(name, "an integer", value);
        }
        if (value < min || value > max) {
            throw new ERR_OUT_OF_RANGE(name, `an integer >= ${min} && <= ${max}`, value);
        }
    },
);

export const getValidMode = hideStackFrames((mode, type) => {
    let min = kMinimumAccessMode;
    let max = kMaximumAccessMode;
    let def = F_OK;
    if (type === "copyFile") {
        min = kMinimumCopyMode;
        max = kMaximumCopyMode;
        def = mode || kDefaultCopyMode;
    } else {
        // assert(type === "access");
    }
    if (mode == null) {
        return def;
    }
    if (Number.isInteger(mode) && mode >= min && mode <= max) {
        return mode;
    }
    if (typeof mode !== "number") {
        throw new ERR_INVALID_ARG_TYPE("mode", "integer", mode);
    }
    throw new ERR_OUT_OF_RANGE(
        "mode",
        `an integer >= ${min} && <= ${max}`,
        mode,
    );
});

/**
 * @callback validateNumber
 * @param {*} value
 * @param {string} name
 * @param {number} [min]
 * @param {number} [max]
 * @returns {asserts value is number}
 */

/** @type {validateNumber} */
export function validateNumber(value, name, min = undefined, max) {
    if (typeof value !== 'number')
        throw new ERR_INVALID_ARG_TYPE(name, 'number', value);

    if ((min != null && value < min) || (max != null && value > max) ||
        ((min != null || max != null) && Number.isNaN(value))) {
        throw new ERR_OUT_OF_RANGE(
            name,
            `${min != null ? `>= ${min}` : ''}${min != null && max != null ? ' && ' : ''}${max != null ? `<= ${max}` : ''}`,
            value);
    }
}

/**
 * @callback validateArray
 * @param {*} value
 * @param {string} name
 * @param {number} [minLength]
 * @returns {asserts value is any[]}
 */

/** @type {validateArray} */
export const validateArray = hideStackFrames((value, name, minLength = 0) => {
    if (!Array.isArray(value)) {
        throw new ERR_INVALID_ARG_TYPE(name, 'Array', value);
    }
    if (value.length < minLength) {
        const reason = `must be longer than ${minLength}`;
        throw new ERR_INVALID_ARG_VALUE(name, value, reason);
    }
});

/**
 * @callback validateOneOf
 * @template T
 * @param {T} value
 * @param {string} name
 * @param {T[]} oneOf
 */

/** @type {validateOneOf} */
export const validateOneOf = hideStackFrames((value, name, oneOf) => {
    if (!Array.prototype.includes.call(oneOf, value)) {
        const allowed = Array.prototype.join.call(
            Array.prototype.map.call(oneOf, (v) =>
                (typeof v === 'string' ? `'${v}'` : String(v))),
            ', ');
        const reason = 'must be one of: ' + allowed;
        throw new ERR_INVALID_ARG_VALUE(name, value, reason);
    }
});

// Return undefined if there is no match.
// Move the "slow cases" to a separate function to make sure this function gets
// inlined properly. That prioritizes the common case.
function normalizeEncoding(enc) {
    if (enc == null || enc === 'utf8' || enc === 'utf-8') return 'utf8';
    return slowCases(enc);
}

function slowCases(enc) {
    switch (enc.length) {
        case 4:
            if (enc === 'UTF8') return 'utf8';
            if (enc === 'ucs2' || enc === 'UCS2') return 'utf16le';
            enc = `${enc}`.toLowerCase();
            if (enc === 'utf8') return 'utf8';
            if (enc === 'ucs2') return 'utf16le';
            break;
        case 3:
            if (enc === 'hex' || enc === 'HEX' ||
                `${enc}`.toLowerCase() === 'hex')
                return 'hex';
            break;
        case 5:
            if (enc === 'ascii') return 'ascii';
            if (enc === 'ucs-2') return 'utf16le';
            if (enc === 'UTF-8') return 'utf8';
            if (enc === 'ASCII') return 'ascii';
            if (enc === 'UCS-2') return 'utf16le';
            enc = `${enc}`.toLowerCase();
            if (enc === 'utf-8') return 'utf8';
            if (enc === 'ascii') return 'ascii';
            if (enc === 'ucs-2') return 'utf16le';
            break;
        case 6:
            if (enc === 'base64') return 'base64';
            if (enc === 'latin1' || enc === 'binary') return 'latin1';
            if (enc === 'BASE64') return 'base64';
            if (enc === 'LATIN1' || enc === 'BINARY') return 'latin1';
            enc = `${enc}`.toLowerCase();
            if (enc === 'base64') return 'base64';
            if (enc === 'latin1' || enc === 'binary') return 'latin1';
            break;
        case 7:
            if (enc === 'utf16le' || enc === 'UTF16LE' ||
                `${enc}`.toLowerCase() === 'utf16le')
                return 'utf16le';
            break;
        case 8:
            if (enc === 'utf-16le' || enc === 'UTF-16LE' ||
                `${enc}`.toLowerCase() === 'utf-16le')
                return 'utf16le';
            break;
        case 9:
            if (enc === 'base64url' || enc === 'BASE64URL' ||
                `${enc}`.toLowerCase() === 'base64url')
                return 'base64url';
            break;
        default:
            if (enc === '') return 'utf8';
    }
}


/**
 * @param {string} data
 * @param {string} encoding
 */
export function validateEncoding(data, encoding) {
    const normalizedEncoding = normalizeEncoding(encoding);
    const length = data.length;

    if (normalizedEncoding === 'hex' && length % 2 !== 0) {
        throw new ERR_INVALID_ARG_VALUE('encoding', encoding,
            `is invalid for data of length ${length}`);
    }
}

export default {
    validatePort,
    validateFunction,
    validateString,
    validateBoolean,
    validateObject,
    validateAbortSignal,
    validateCallback,
    validateInteger,
    validateNumber,
    validateArray,
    getValidMode,
    validateOneOf,
    validateEncoding
}