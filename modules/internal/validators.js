import {
    ERR_SOCKET_BAD_PORT,
    ERR_INVALID_ARG_TYPE,
    ERR_INVALID_CALLBACK,
    ERR_OUT_OF_RANGE,
    hideStackFrames
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
    getValidMode
}