import * as net from 'wasi_net'
import * as httpx from 'wasi_http'
import { TextDecoder } from 'util'
import { Buffer } from 'buffer'
import { EventEmitter } from 'events'
import process from 'process'
import { validatePort } from 'internal/validators'
import { Readable, Writable } from "stream";
import { isTypedArray } from 'util/types'

const URL = httpx.URL;

export class Request {
    constructor(input, init = {}) {

        let parsedURL

        if (input instanceof Request) {
            parsedURL = new URL(input.url)
        } else {
            parsedURL = new URL(input)
            input = {}
        }

        this.url = parsedURL;

        if (parsedURL.username !== '' || parsedURL.password !== '') {
            throw new TypeError(`${parsedURL} is an url with embedded credentails.`)
        }

        let method = init.method || input.method || 'GET'
        this.method = method.toUpperCase();

        let headers = init.headers || input.headers || {}
        if (!headers.has('Accept')) {
            headers.set('Accept', '*/*')
        }

        this.headers = headers
    }

    get [Symbol.toStringTag]() {
        return 'Request'
    }

    clone() {
        return new Request(this)
    }
}

export class Response {
    #chunked = false;
    #chunkBuff = null;
    #bodyUsed = false

    constructor(resp, buffer, reader, option = {}) {
        this.response = resp
        this.buffer = buffer
        this.reader = reader
        this.url = option.url

        this.headers = resp.headers
        this.statusText = resp.statusText
        this.status = resp.status

        if (typeof (resp.bodyLength) === "number") {
            this.#chunked = false
        } else {
            this.#chunked = true
            this.#chunkBuff = buffer
            this.buffer = new httpx.Buffer()
        }

        this.onChunk = undefined;
    }

    get chunked() {
        return this.#chunked
    }

    get ok() {
        return this.status >= 200 && this.status < 300;
    }

    get bodyUsed() {
        return this.#bodyUsed
    }

    async #readChunk() {
        while (true) {
            let chunk = this.#chunkBuff.parseChunk();

            if (chunk === undefined) {
                let data = await this.reader.read()
                if (data === undefined) {
                    throw new Error('socket is shutdown')
                }
                this.#chunkBuff.write(data)
                continue
            } else if (chunk === null) {
                // end
                return null
            } else if (chunk instanceof ArrayBuffer) {
                return chunk
            } else {
                throw chunk
            }
        }
    }

    async #readBody() {
        while (true) {

            if (this.buffer.byteLength >= this.response.bodyLength) {
                let buf = this.buffer.buffer;
                this.buffer.clear();
                return buf;
            }

            let data = await this.reader.read()
            if (data === undefined) {
                let buf = this.buffer.buffer;
                this.buffer.clear();
                return buf;
            }

            this.buffer.write(data)
        }
    }

    async arrayBuffer() {
        this.#bodyUsed = true;
        if (this.#chunked) {
            while (true) {
                let chunk = await this.#readChunk();
                if (chunk === null) {
                    let buf = this.buffer.buffer;
                    this.buffer.clear();
                    return buf;
                }
                this.buffer.write(chunk)
                if (typeof this.onChunk === 'function') {
                    let onChunk = this.onChunk;
                    onChunk(chunk)
                }
            }
        } else {
            let body = await this.#readBody()
            if (typeof this.onChunk === 'function') {
                let onChunk = this.onChunk;
                onChunk(body)
            }
            return body
        }
    }

    async text() {
        return new TextDecoder().decode(await this.arrayBuffer())
    }

    async json() {
        return JSON.parse(await this.text())
    }

    get [Symbol.toStringTag]() {
        return 'Response'
    }
}

async function wait_response(reader, url) {
    let buf = new httpx.Buffer()
    let resp = undefined
    while (true) {
        let buff = await reader.read()
        if (buff == undefined && resp == undefined) {
            throw new TypeError('Illegal response')
        }
        buf.append(buff)
        resp = buf.parseResponse()
        if (resp instanceof httpx.WasiResponse) {
            return new Response(resp, buf, reader, { url })
        }
    }
}

export async function fetch(input, init = {}) {
    let url = new httpx.URL(input)
    if (url.username !== '' || url.password != '') {
        throw new TypeError(`${input} is an url with embedded credentails.`)
    }

    let method = init.method || 'GET'
    method = method.toUpperCase();

    let headers = init.headers || {}
    if (!headers['Accept']) {
        headers['Accept'] = '*/*'
    }
    if (!headers['Host']) {
        headers['Host'] = url.host
    }

    var s;
    if (url.scheme == 'https' && net.WasiTlsConn) {
        s = await net.WasiTlsConn.connect(url.host, url.port);
    } else {
        s = await net.WasiTcpConn.connect(url.host, url.port);
    }

    let req = new httpx.WasiRequest()
    req.version = init.version || 'HTTP/1.1'
    req.headers = headers

    let path = url.path
    let query = url.query
    if (query != undefined) {
        req.uri = `${path}?${query}`
    } else {
        req.uri = path
    }

    req.method = method
    req.body = init.body || ''
    s.write(req.encode())
    return await wait_response(s, url)
}

const STATUS_CODES = {
    100: 'Continue',                   // RFC 7231 6.2.1
    101: 'Switching Protocols',        // RFC 7231 6.2.2
    102: 'Processing',                 // RFC 2518 10.1 (obsoleted by RFC 4918)
    103: 'Early Hints',                // RFC 8297 2
    200: 'OK',                         // RFC 7231 6.3.1
    201: 'Created',                    // RFC 7231 6.3.2
    202: 'Accepted',                   // RFC 7231 6.3.3
    203: 'Non-Authoritative Information', // RFC 7231 6.3.4
    204: 'No Content',                 // RFC 7231 6.3.5
    205: 'Reset Content',              // RFC 7231 6.3.6
    206: 'Partial Content',            // RFC 7233 4.1
    207: 'Multi-Status',               // RFC 4918 11.1
    208: 'Already Reported',           // RFC 5842 7.1
    226: 'IM Used',                    // RFC 3229 10.4.1
    300: 'Multiple Choices',           // RFC 7231 6.4.1
    301: 'Moved Permanently',          // RFC 7231 6.4.2
    302: 'Found',                      // RFC 7231 6.4.3
    303: 'See Other',                  // RFC 7231 6.4.4
    304: 'Not Modified',               // RFC 7232 4.1
    305: 'Use Proxy',                  // RFC 7231 6.4.5
    307: 'Temporary Redirect',         // RFC 7231 6.4.7
    308: 'Permanent Redirect',         // RFC 7238 3
    400: 'Bad Request',                // RFC 7231 6.5.1
    401: 'Unauthorized',               // RFC 7235 3.1
    402: 'Payment Required',           // RFC 7231 6.5.2
    403: 'Forbidden',                  // RFC 7231 6.5.3
    404: 'Not Found',                  // RFC 7231 6.5.4
    405: 'Method Not Allowed',         // RFC 7231 6.5.5
    406: 'Not Acceptable',             // RFC 7231 6.5.6
    407: 'Proxy Authentication Required', // RFC 7235 3.2
    408: 'Request Timeout',            // RFC 7231 6.5.7
    409: 'Conflict',                   // RFC 7231 6.5.8
    410: 'Gone',                       // RFC 7231 6.5.9
    411: 'Length Required',            // RFC 7231 6.5.10
    412: 'Precondition Failed',        // RFC 7232 4.2
    413: 'Payload Too Large',          // RFC 7231 6.5.11
    414: 'URI Too Long',               // RFC 7231 6.5.12
    415: 'Unsupported Media Type',     // RFC 7231 6.5.13
    416: 'Range Not Satisfiable',      // RFC 7233 4.4
    417: 'Expectation Failed',         // RFC 7231 6.5.14
    418: 'I\'m a Teapot',              // RFC 7168 2.3.3
    421: 'Misdirected Request',        // RFC 7540 9.1.2
    422: 'Unprocessable Entity',       // RFC 4918 11.2
    423: 'Locked',                     // RFC 4918 11.3
    424: 'Failed Dependency',          // RFC 4918 11.4
    425: 'Too Early',                  // RFC 8470 5.2
    426: 'Upgrade Required',           // RFC 2817 and RFC 7231 6.5.15
    428: 'Precondition Required',      // RFC 6585 3
    429: 'Too Many Requests',          // RFC 6585 4
    431: 'Request Header Fields Too Large', // RFC 6585 5
    451: 'Unavailable For Legal Reasons', // RFC 7725 3
    500: 'Internal Server Error',      // RFC 7231 6.6.1
    501: 'Not Implemented',            // RFC 7231 6.6.2
    502: 'Bad Gateway',                // RFC 7231 6.6.3
    503: 'Service Unavailable',        // RFC 7231 6.6.4
    504: 'Gateway Timeout',            // RFC 7231 6.6.5
    505: 'HTTP Version Not Supported', // RFC 7231 6.6.6
    506: 'Variant Also Negotiates',    // RFC 2295 8.1
    507: 'Insufficient Storage',       // RFC 4918 11.5
    508: 'Loop Detected',              // RFC 5842 7.2
    509: 'Bandwidth Limit Exceeded',
    510: 'Not Extended',               // RFC 2774 7
    511: 'Network Authentication Required' // RFC 6585 6
};

const METHODS = [
    'GET',
    'POST',
    'PUT',
    'DELETE',
    'CONNECT',
    'HEAD',
    'OPTIONS',
    'TRACE',
    'PATCH'
];

function chunkToU8(chunk) {
    if (typeof chunk === "string") {
        return Buffer.from(chunk);
    }
    if (isTypedArray(chunk)) {
        return Buffer.from(chunk);
    }
    return chunk;
}

class ClientRequest extends Writable {
    body = null;
    constructor(opts, cb) {
        super();
        this.opts = opts;
        this.cb = cb
        this.body = new httpx.Buffer()
    }

    // deno-lint-ignore no-explicit-any
    _write(chunk, _enc, cb) {
        this.body.write(chunkToU8(chunk)?.buffer)
        cb()
    }

    async _final() {
        try {
            const opts = { body: this.body, method: this.opts.method, headers: this.opts.headers };
            const mayResponse = await fetch(this._createUrlStrFromOptions(this.opts), opts)
            const res = new IncomingMessageForClient(mayResponse);
            this.emit("response", res);
            this.cb?.(res);
        } catch (e) {
            this.emit('error', e)
        }
    }

    abort() {
        this.destroy();
    }

    _createCustomClient() {
        return Promise.resolve(undefined);
    }

    // deno-lint-ignore no-explicit-any
    _createUrlStrFromOptions(opts) {
        if (opts.href) {
            return opts.href;
        } else {
            const {
                auth,
                protocol,
                host,
                hostname,
                path,
                port,
            } = opts;
            return `${protocol}//${auth ? `${auth}@` : ""}${host ?? hostname}${port ? `:${port}` : ""}${path}`;
        }
    }

    get [Symbol.toStringTag]() {
        return 'Request'
    }
}

export class IncomingMessageForClient extends Readable {
    constructor(response) {
        super();
        this.response = response;
    }

    async _read(_size) {
        try {
            this.response.onChunk = (chunk) => {
                this.push(Buffer.from(chunk));
            }

            const _ = await this.response.arrayBuffer();
            this.emit('end')
        } catch (e) {
            // deno-lint-ignore no-explicit-any
            this.destroy(e);
        }
    }

    get headers() {
        if (this.response) {
            return Object.fromEntries(this.response.headers.entries());
        }
        return {};
    }

    get trailers() {
        return {};
    }

    get statusCode() {
        return this.response?.status || 0;
    }

    get statusMessage() {
        return this.response?.statusText || "";
    }
}

export class ServerResponse extends Writable {
    statusCode = undefined;
    statusMessage = undefined;
    #headers = {};
    headersSent = false;
    #conn;
    #firstChunk = null;
    #_end = false;

    constructor(conn) {
        super({
            autoDestroy: true,
            defaultEncoding: "utf-8",
            emitClose: true,
            write: (chunk, _encoding, cb) => {
                if (!this.headersSent) {
                    if (this.#firstChunk === null) {
                        this.#firstChunk = chunk;
                        if (!this.#_end) {
                            this.respond(false, this.#firstChunk);
                            this.#firstChunk = null;
                        }
                        return cb();
                    } else {
                        this.respond(false, this.#firstChunk);
                        this.#firstChunk = null;
                    }
                }

                this.#conn.write(chunk);
                return cb();
            },
            final: (cb) => {
                if (this.#firstChunk) {
                    this.respond(true, this.#firstChunk);
                } else if (!this.headersSent) {
                    this.respond(true);
                }
                if (this.#conn.connection == 'close') {
                    this.#conn.close()
                } else {
                    this.#conn.end();
                }
                return cb();
            },
            destroy: (err, cb) => {
                // if (err) {
                //     controller.error(err);
                // }
                return cb(null);
            },
        });
        this.#conn = conn;
    }

    setHeader(name, value) {
        this.#headers[name.toLowerCase()] = value;
        return this;
    }

    getHeader(name) {
        return this.#headers[name.toLowerCase()];
    }
    removeHeader(name) {
        return delete this.#headers[name.toLowerCase()];
    }
    getHeaderNames() {
        return Array.from(Object.keys(this.#headers));
    }
    hasHeader(name) {
        return this.#headers[name.toLowerCase()] != undefined;
    }

    writeHead(status, headers) {
        this.statusCode = status;
        for (const k in headers) {
            this.#headers[k.toLowerCase()] = headers[k];
        }
        return this;
    }

    #ensureHeaders(singleChunk) {
        if (this.statusCode === undefined) {
            this.statusCode = 200;
            this.statusMessage = "OK";
        }
        if (typeof singleChunk === "string" && !this.hasHeader("content-type")) {
            this.setHeader("content-type", "text/plain;charset=UTF-8");
        }
    }

    respond(final, singleChunk) {

        this.headersSent = true;
        this.#ensureHeaders(singleChunk);
        if (final) {
            this.#conn.respondWith(
                singleChunk, {
                headers: this.#headers,
                status: this.statusCode,
                statusText: this.statusMessage,
            }
            ).catch(() => {
                // ignore this error
            });
        } else {
            this.#conn.chunk({
                headers: this.#headers,
                status: this.statusCode,
                statusText: this.statusMessage,
            });
            this.#conn.write(singleChunk)
        }
    }

    // deno-lint-ignore no-explicit-any
    end(chunk, encoding, cb) {
        if (!this.headersSent) {
            if (!chunk && this.hasHeader("transfer-encoding")) {
                // FIXME(bnoordhuis) Node sends a zero length chunked body instead, i.e.,
                // the trailing "0\r\n", but respondWith() just hangs when I try that.
                this.setHeader("content-length", "0");
                this.removeHeader("transfer-encoding");
            }
        }
        this.#_end = true;

        // @ts-expect-error The signature for cb is stricter than the one implemented here
        return super.end(chunk, encoding, cb);
    }
}

export class IncomingMessageForServer extends Readable {
    #req;
    url;

    constructor(req, conn) {
        // Check if no body (GET/HEAD/OPTIONS/...)
        let value = req.body;
        super({
            autoDestroy: true,
            emitClose: true,
            objectMode: false,
            read: async function (_size) {
                if (!value) {
                    this.push(null);
                } else {
                    this.push(Buffer.from(value));
                    value = null;
                }
            },
            destroy: (err, cb) => {
                conn.close();
                cb(err);
            },
        });
        this.#req = req;
        this.url = req.uri;
    }

    get aborted() {
        return false;
    }
    get httpVersion() {
        return this.#req.version;
    }

    get headers() {
        return this.#req.headers;
    }
    get method() {
        return this.#req.method;
    }
}

class HttpConn {

    #chunk = undefined;
    #connection = 'close';
    #version = "HTTP/1.1";
    #chunkBuffer = undefined;
    #respHeaders;

    constructor(socket) {
        this.socket = socket
    }

    get connection() {
        return this.#connection;
    }

    get version() {
        return this.#version
    }

    async nextRequest() {
        let buffer = new httpx.Buffer();
        while (true) {
            let d = await this.socket.read();
            if (d == undefined || d.byteLength <= 0) {
                return null;
            }
            buffer.append(d);
            try {
                let req = buffer.parseRequest();
                if (req instanceof httpx.WasiRequest) {
                    this.#version = req.version;
                    if (this.#version == "HTTP/1.1") {
                        this.#connection = (req.getHeader('connection') ?? "keep-alive").toLowerCase()
                    } else if (this.#version == "HTTP/1.0") {
                        this.#connection = (req.getHeader('connection') ?? "close").toLowerCase()
                    }
                    return req
                }
            } catch (e) {
                return null;
            }
        }
    }

    respondWith(body, resp_header) {
        if (!this.socket) {
            return
        }
        if (body) {
            body = chunkToU8(body)?.buffer;
        }
        let resp = new httpx.WasiResponse()
        resp.version = this.#version;
        resp.headers = resp_header.headers;
        resp.status = resp_header.status;
        resp.statusText = resp_header.statusText;
        this.socket.write(resp.encode(body))
    }

    chunk(resp_header) {
        if (this.#version == "HTTP/1.1") {
            let resp = new httpx.WasiResponse()
            resp.version = this.#version;
            resp.headers = resp_header.headers;
            resp.status = resp_header.status;
            resp.statusText = resp_header.statusText;
            this.#chunk = resp.chunk(this.socket)
        } else {
            this.#chunkBuffer = new httpx.Buffer();
            this.#respHeaders = resp_header;
        }
    }

    write(chunk) {
        if (chunk) {
            let conn = this.#chunkBuffer ?? this.#chunk ?? this.socket;
            conn?.write(chunkToU8(chunk).buffer);
        }
    }

    end(chunk) {
        if (this.#chunk) {
            this.#chunk.end(chunk)
            this.#chunk = undefined
            return
        }

        if (this.#chunkBuffer && chunk) {
            this.#chunkBuffer.write(chunk)
        }

        if (this.#chunkBuffer) {
            this.respondWith(this.#chunkBuffer, this.#respHeaders);
            this.#chunkBuffer = null;
        }
    }

    close() {
        this.end()
        this.socket = undefined
    }
}


export function Server(handler) {
    return new ServerImpl(handler);
}


function _normalizeArgs(args) {
    let arr;

    if (args.length === 0) {
        arr = [{}, null];

        return arr;
    }

    const arg0 = args[0];
    let options = {};

    if (typeof arg0 === "object" && arg0 !== null) {
        // (options[...][, cb])
        options = arg0;
    } else {
        // ([port][, host][...][, cb])
        options.port = arg0;

        if (args.length > 1 && typeof args[1] === "string") {
            options.host = args[1];
        }
    }

    const cb = args[args.length - 1];

    if (typeof cb !== "function") {
        arr = [options, null];
    } else {
        arr = [options, cb];
    }

    return arr;
}


class ServerImpl extends EventEmitter {
    #httpConnections = new Set();
    #listener = undefined;
    #listening = false;

    constructor(handler) {
        super();

        if (handler !== undefined) {
            this.on("request", handler);
        }
    }

    listen(...args) {
        // TODO(bnoordhuis) Delegate to net.Server#listen().
        const normalized = _normalizeArgs(args);
        const options = normalized[0];
        const cb = normalized[1];

        if (cb != null) {
            // @ts-ignore change EventEmitter's sig to use CallableFunction
            this.once("listening", cb);
        }

        let port = 0;
        if (typeof options.port === "number" || typeof options.port === "string") {
            validatePort(options.port, "options.port");
            port = options.port | 0;
        }

        // TODO(bnoordhuis) Node prefers [::] when host is omitted,
        // we on the other hand default to 0.0.0.0.
        // const hostname = options.host ?? "";

        this.#listener = new net.WasiTcpServer(port);
        this.#listening = true;
        this.#listenLoop();

        return this;
    }

    async #listenLoop() {
        const go = async (httpConn) => {
            try {
                for (; ;) {
                    let request = null;
                    try {
                        // Note: httpConn.nextRequest() calls httpConn.close() on error.
                        request = await httpConn.nextRequest();
                    } catch {
                        // Connection closed.
                        // TODO(bnoordhuis) Emit "clientError" event on the http.Server
                        // instance? Node emits it when request parsing fails and expects
                        // the listener to send a raw 4xx HTTP response on the underlying
                        // net.Socket but we don't have one to pass to the listener.
                    }
                    if (request === null) {
                        break;
                    }
                    const req = new IncomingMessageForServer(request, httpConn);
                    const res = new ServerResponse(httpConn);
                    this.emit("request", req, res);
                }
            } finally {
                this.#httpConnections.delete(httpConn);
                httpConn.close()
            }
        };

        const listener = this.#listener;

        if (listener !== undefined) {
            this.emit("listening");

            try {
                while (this.#listening) {
                    let tcp_conn = await listener.accept();
                    try {
                        let httpConn = new HttpConn(tcp_conn);
                        this.#httpConnections.add(httpConn);
                        go(httpConn);
                    } catch {
                        continue;
                    }
                }
            } catch (e) {
                this.emit('error', e)
            } finally {
                this.listening = false
            }
        }
    }

    get listening() {
        return this.#listening !== undefined;
    }

    close(cb) {
        const listening = this.#listening;

        if (typeof cb === "function") {
            if (listening) {
                this.once("close", cb);
            } else {
                this.once("close", function close() {
                    cb(new ERR_SERVER_NOT_RUNNING());
                });
            }
        }

        process.nextTick(() => this.emit("close"));

        if (listening) {
            this.#listener.close();
            this.#listener = undefined;
            this.#listening = false;

            for (const httpConn of this.#httpConnections) {
                try {
                    httpConn.close();
                } catch {
                    // Already closed.
                }
            }

            this.#httpConnections.clear();
        }

        return this;
    }

    get listening() {
        return this.#listener
    }

    address() {
        const addr = this.#listener.addr;
        return {
            port: addr.port,
            address: addr.hostname,
        };
    }
}

Server.prototype = ServerImpl.prototype;

export function createServer(handler) {
    return Server(handler);
}

function urlToHttpOptions(url) {
    // deno-lint-ignore no-explicit-any
    const options = {
        protocol: url.protocol,
        hostname: typeof url.hostname === "string" &&
            url.hostname.startsWith("[")
            ? url.hostname.slice(1, -1)
            : url.hostname,
        path: url.path,
        href: url.href,
    };
    if (url.port !== "") {
        options.port = url.port;
    }
    if (url.username || url.password) {
        options.auth = `${url.username}:${url.password}`;
    }
    return options;
}

export function request(...args) {
    let options = {};
    if (typeof args[0] === "string") {
        options = urlToHttpOptions(new URL(args.shift()));
    } else if (args[0] instanceof URL) {
        options = urlToHttpOptions(args.shift());
    }
    if (args[0] && typeof args[0] !== "function") {
        Object.assign(options, args.shift());
    }
    args.unshift(options);
    return new ClientRequest(args[0], args[1]);
}

export function get(...args) {
    const req = request(args[0], args[1], args[2]);
    req.end();
    return req;
}

export default {
    ClientRequest,
    STATUS_CODES,
    METHODS,
    createServer,
    Server,
    IncomingMessage: IncomingMessageForServer,
    ServerResponse,
    request,
    get,
};