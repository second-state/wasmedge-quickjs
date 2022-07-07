import { ERR_SOCKET_BAD_PORT, ERR_INVALID_ARG_TYPE, hideStackFrames } from 'internal/errors'

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

export default {
    validatePort, validateFunction, validateString
}