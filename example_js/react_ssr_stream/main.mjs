import * as React from 'react'

import LazyHome from './component/LazyHome.jsx'
import {renderToPipeableStream} from 'react-dom/server'

import * as net from 'wasi_net'
import * as http from 'wasi_http'

async function handle_client(s){
    let resp = new http.WasiResponse()
    renderToPipeableStream(<LazyHome />).pipe(resp.chunk(s))
}

async function server_start(){
    print('listen 8000...')
    let s = new net.WasiTcpServer(8000)
    for(var i=0;i<100;i++){
        let cs = await s.accept();
        handle_client(cs)
    }
}

server_start()