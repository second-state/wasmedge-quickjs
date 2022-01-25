import * as net from 'wasi_net'
import * as http from 'wasi_http'

async function handle_client(cs, handler_req) {
    print('open:', cs.peer())
    let buffer = new http.Buffer()

    while (true) {
        try {
            let d = await cs.read()
            if (d.byteLength <= 0) {
                return
            }
            buffer.append(d)
            let req = buffer.parseRequest()
            if (req instanceof http.WasiRequest) {
                handler_req(cs, req)
                break
            }
        } catch (e) {
            print(e)
        }
    }
    print('close:', cs.peer())
}

function handler_req(cs, req) {
    print("version=", req.version)
    print("uri=", req.uri)
    print("method=", req.method)
    print("headers=", Object.keys(req.headers))
    print("body=", newStringFromUTF8(req.body))

    let resp = new http.WasiResponse()
    let body = 'echo:' + newStringFromUTF8(req.body)
    let r = resp.encode(body);
    cs.write(r)
}

async function server_start() {
    print('listen 8000 ...')
    let s = new net.WasiTcpServer(8000)
    for (var i = 0; i < 100; i++) {
        let cs = await s.accept();
        try {
            handle_client(cs, handler_req)
        } catch (e) {
            print(e)
        }
    }
}

server_start()