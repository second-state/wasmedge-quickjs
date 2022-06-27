import * as net from 'wasi_net'
import * as httpx from 'wasi_http'
import { TextDecoder } from 'util'

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
    constructor(resp, buffer, reader, option = {}) {
        this.response = resp
        this.buffer = buffer
        this.reader = reader
        this.url = option.url

        this.headers = resp.headers
        this.statusText = resp.statusText
        this.status = resp.status
    }

    get ok() {
        return this.status >= 200 && this.status < 300;
    }

    async body() {
        while (true) {
            let data = await this.reader.read()

            if (data === undefined) {
                return this.buffer.buffer
            }
            this.buffer.append(data)
        }
    }

    async text() {
        return new TextDecoder().decode(await this.body())
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
        if (buff.byteLength === undefined && resp == undefined) {
            throw new TypeError('Illegal response')
        }
        buf.append(buff)
        resp = buf.parseResponse()
        if (resp instanceof httpx.WasiResponse) {
            let body_length = resp.bodyLength
            if (typeof (body_length) === "number") {
                return new Response(resp, buf, reader, { url })
            } else {
                // todo support
                throw new TypeError('no support chuncked')
            }
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

    let addr = net.nsloopup(url.host, `${url.port}`)[0];

    let s = await net.WasiTcpConn.connect(addr)
    let req = new httpx.WasiRequest()
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