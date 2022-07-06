import { ERR_SOCKET_BAD_PORT } from 'internal/errors'

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