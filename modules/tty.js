import internal from "_node:tty"
import fs from "fs"
import { hasColors, getColorDepth } from "./internal/tty";

export function isatty(fd) {
    return Number.isInteger(fd) && fd >= 0 && fd <= 2147483647 && (internal.isatty(fd) ?? false);
}

// net.Socket is unimplemented now, use fs.ReadStream temporarily
export class ReadStream extends fs.ReadStream {
    constructor(fd) {
        super("", { fd });
    }

    get isTTY() {
        return true;
    }

    get isRaw() {
        return false;
    }

    setRawMode(mode) {
        // require tcgetattr and tcsetattr or ioctl, unsupported
        return this;
    }
}

export class WriteStream extends fs.WriteStream {
    constructor(fd) {
        super("", { fd });
    }

    clearLine(dir, callback) {
        if (dir === -1) {
            this.write("\x1b[K"); // clear left
        } else if (dir === 1) {
            this.write("\x1b[1K"); // clear right
        } else if (dir === 2) {
            this.write("\x1b[2K"); // clear all
        }
        if (typeof (callback) === "function") {
            callback();
        }
        return true;
    }

    clearScreenDown(callback) {
        this.write("\x1b[J");
        if (typeof (callback) === "function") {
            callback();
        }
    }

    getWindowSize() {
        // require tcgetwinsize, unsupported
        return [undefined, undefined];
    }

    get columns() {
        return this.getWindowSize[0];
    }

    get rows() {
        return this.getWindowSize[1];
    }

    getColorDepth = getColorDepth;

    hasColors = hasColors;

    get isTTY() {
        return true;
    }

    cursorTo(x, y, callback) {
        this.write(`\x1b[${x};${y}`);
        if (typeof (callback) === "function") {
            callback();
        }
        return true;
    }

    moveCursor(dx, dy, callback) {
        this.write((dx > 0 ? `\x1b[${dx}C` : `\x1b[${-dx}D`) + (dy > 0 ? `\x1b[${dy}B` : `\x1b[${-dy}A`))
        if (typeof (callback) === "function") {
            callback();
        }
        return true;
    }
}

export default {
    isatty
}