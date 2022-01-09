import * as React from 'react'

import LazyHome from './component/LazyHome.jsx'
import {renderToPipeableStream} from 'react-dom/server'

import * as net from 'wasi_net'

async function handle_client(s){
    s.write('HTTP/1.1 200 OK\r\n\r\n')
    renderToPipeableStream(<LazyHome />).pipe(s)
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