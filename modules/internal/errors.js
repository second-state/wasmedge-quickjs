import { inspect } from 'util'

export class ERR_HTTP_HEADERS_SENT extends Error {
    constructor(x) {
        super(
            "ERR_HTTP_HEADERS_SENT",
            `Cannot ${x} headers after they are sent to the client`,
        );
    }
}

export class ERR_HTTP_INVALID_HEADER_VALUE extends TypeError {
    constructor(x, y) {
        super(
            "ERR_HTTP_INVALID_HEADER_VALUE",
            `Invalid value "${x}" for header "${y}"`,
        );
    }
}

export class ERR_HTTP_TRAILER_INVALID extends Error {
    constructor() {
        super(
            "ERR_HTTP_TRAILER_INVALID",
            `Trailers are invalid with this transfer encoding`,
        );
    }
}

export class ERR_INVALID_HTTP_TOKEN extends TypeError {
    constructor(x, y) {
        super("ERR_INVALID_HTTP_TOKEN", `${x} must be a valid HTTP token ["${y}"]`);
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

export class ERR_INVALID_ARG_TYPE_RANGE extends RangeError {
    constructor(name, expected, actual) {
        const msg = createInvalidArgType(name, expected);

        super("ERR_INVALID_ARG_TYPE", `${msg}.${invalidArgTypeHelper(actual)}`);
    }
}

export class ERR_INVALID_ARG_VALUE_RANGE extends RangeError {
    constructor(name, value, reason = "is invalid") {
        const type = name.includes(".") ? "property" : "argument";
        const inspected = inspect(value);

        super(
            "ERR_INVALID_ARG_VALUE",
            `The ${type} '${name}' ${reason}. Received ${inspected}`,
        );
    }
}

export class ERR_INVALID_CHAR extends TypeError {
    constructor(name, field) {
        super(
            "ERR_INVALID_CHAR",
            field
                ? `Invalid character in ${name}`
                : `Invalid character in ${name} ["${field}"]`,
        );
    }
}

export class ERR_METHOD_NOT_IMPLEMENTED extends Error {
    constructor(x) {
        super("ERR_METHOD_NOT_IMPLEMENTED", `The ${x} method is not implemented`);
    }
}

export class ERR_STREAM_CANNOT_PIPE extends Error {
    constructor() {
        super("ERR_STREAM_CANNOT_PIPE", `Cannot pipe, not readable`);
    }
}

export class ERR_STREAM_ALREADY_FINISHED extends Error {
    constructor(x) {
        super(
            "ERR_STREAM_ALREADY_FINISHED",
            `Cannot call ${x} after a stream was finished`,
        );
    }
}

export class ERR_STREAM_WRITE_AFTER_END extends Error {
    constructor() {
        super("ERR_STREAM_WRITE_AFTER_END", `write after end`);
    }
}

export class ERR_STREAM_NULL_VALUES extends TypeError {
    constructor() {
        super("ERR_STREAM_NULL_VALUES", `May not write null values to stream`);
    }
}

export class ERR_STREAM_DESTROYED extends Error {
    constructor(x) {
        super(
            "ERR_STREAM_DESTROYED",
            `Cannot call ${x} after a stream was destroyed`,
        );
    }
}

export class ERR_SOCKET_BAD_PORT extends RangeError {
    constructor(name, port, allowZero = true) {
        assert(
            typeof allowZero === "boolean",
            "The 'allowZero' argument must be of type boolean.",
        );

        const operator = allowZero ? ">=" : ">";

        super(
            "ERR_SOCKET_BAD_PORT",
            `${name} should be ${operator} 0 and < 65536. Received ${port}.`,
        );
    }
}

export function hideStackFrames(fn) {
    const hidden = "__node_internal_" + fn.name;
    Object.defineProperty(fn, "name", { value: hidden });

    return fn;
}