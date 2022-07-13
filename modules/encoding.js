import { text_encode, text_decode, text_encode_into } from '_encoding'

function isError(e) {
    return isObject(e) &&
        (objectToString(e) === '[object Error]' || e instanceof Error);
}

function isObject(arg) {
    return typeof arg === 'object' && arg !== null;
}

function isUndefined(arg) {
    return arg === void 0;
}

export class TextEncoder {

    get encoding() {
        return 'utf-8'
    }

    encode(input) {
        let arr = text_encode(input, 'utf-8');
        if (isUndefined(arr)) {
            return new Uint8Array()
        } else {
            return new Uint8Array(arr)
        }
    }

    encodeInto(src, dest) {
        if (dest instanceof Uint8Array) {
            return text_encode_into(src, 'utf8', dest.buffer, dest.byteOffset)
        } else {
            throw new TypeError('The "dest" argument must be an instance of Uint8Array.')
        }
    }
}

export class TextDecoder {
    #encoding = 'utf-8';
    #fatal = undefined;
    #ignoreBOM = undefined;

    constructor(encoding, options) {
        let { fatal, ignoreBOM } = options || {};
        this.#fatal = fatal ? true : false;
        this.#ignoreBOM = ignoreBOM;
        encoding = encoding || 'utf-8';

        let exist = [
            'utf8', 'utf-8', 'gbk', 'gb18030', 'hz-gb-2312', 'big5', 'euc-jp', 'iso-2022-jp',
            'utf-16be', 'utf-16le', 'x-user-defined', 'ibm866',
            'iso-8859-2', 'iso-8859-3', 'iso-8859-4', 'iso-8859-5', 'iso-8859-6', 'iso-8859-7', 'iso-8859-8',
            'iso-8859-8i', 'iso-8859-10', 'iso-8859-13', 'iso-8859-14', 'iso-8859-15', 'iso-8859-16',
            'windows-874', 'windows-1250', 'windows-1251', 'windows-1252', 'windows-1253', 'windows-1254',
            'windows-1255', 'windows-1256', 'windows-1257', 'windows-1258', ''
        ].indexOf(encoding);

        if (exist < 0) {
            throw new RangeError(`The "${encoding}" encoding is not supported`);
        } else {
            this.#encoding = encoding
        }
    }

    get encoding() {
        return this.#encoding
    }

    get fatal() {
        return this.#fatal
    }

    decode(input) {
        if (typeof input != 'undefined') {
            let ret;
            if (input.buffer instanceof ArrayBuffer) {
                ret = text_decode(input.buffer, this.encoding, this.fatal)
            } else if (input instanceof ArrayBuffer) {
                ret = text_decode(input, this.encoding, this.fatal)
            }
            if (isError(ret)) {
                throw new TypeError(`The encoded data was not valid for encoding ${this.encoding}`)
            }
            return ret
        }
        throw new TypeError('The "input" argument must be an instance of ArrayBuffer or ArrayBufferView.')

    }
}

globalThis.TextDecoder = TextDecoder;
globalThis.TextEncoder = TextEncoder;