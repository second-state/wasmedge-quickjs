import { ERR_SOCKET_BAD_PORT, ERR_INVALID_ARG_TYPE, ERR_INVALID_CALLBACK, hideStackFrames } from 'internal/errors'

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
function validateBoolean(value, name) {
    if (typeof value !== "boolean") {
        throw new codes.ERR_INVALID_ARG_TYPE(name, "boolean", value);
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
            throw new codes.ERR_INVALID_ARG_TYPE(name, "AbortSignal", signal);
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

export default {
    validatePort, validateFunction, validateString, validateObject, validateAbortSignal, validateCallback
}