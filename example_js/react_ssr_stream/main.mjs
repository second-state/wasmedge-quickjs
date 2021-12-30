import * as React from 'react'

import LazyHome from './component/LazyHome.jsx'
import {renderToPipeableStream} from 'react-dom/server'

import {tcp_listen,WasiSock} from 'wasi_net'

tcp_listen(8000,{
    on_connect(conn){
        print('accept',conn.fd,conn.peer)
        let s = new WasiSock(conn.fd)
        s.write('HTTP/1.1 200 OK\r\n\r\n')
        renderToPipeableStream(<LazyHome />).pipe(s)
    },
})
print('listen 8000...')