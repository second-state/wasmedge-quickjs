import * as React from 'react';
import {renderToPipeableStream} from 'react-dom/server';
import * as http from 'wasi_http';
import * as net from 'wasi_net';

import LazyHome from './component/LazyHome.jsx';

async function handle_client(s) {
  let resp = new http.WasiResponse();
  renderToPipeableStream(<LazyHome />).pipe(resp.chunk(s));
}

async function server_start() {
  print('listen 8001...');
  let s = new net.WasiTcpServer(8001);
  for (var i = 0; i < 100; i++) {
    let cs = await s.accept();
    handle_client(cs);
  }
}

server_start();
