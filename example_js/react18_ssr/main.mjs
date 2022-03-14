import * as React from 'react';
import { renderToPipeableStream } from 'react-dom/server';
import * as http from 'wasi_http';
import * as net from 'wasi_net';
import * as std from 'std';

import App from './component/App.js';
import { DataProvider } from './component/data.js'

let assets = {
  'main.js': '/main.js',
  'main.css': '/main.css',
};

const css = std.loadFile('./public/main.css')

function createServerData() {
  let done = false;
  let promise = null;
  return {
    read() {
      if (done) {
        return;
      }
      if (promise) {
        throw promise;
      }
      promise = new Promise(resolve => {
        setTimeout(() => {
          done = true;
          promise = null;
          resolve();
        }, 2000);
      });
      throw promise;
    },
  };
}

async function handle_client(cs) {
  print('open:', cs.peer());
  let buffer = new http.Buffer();

  while (true) {
    try {
      let d = await cs.read();
      if (d == undefined || d.byteLength <= 0) {
        return;
      }
      buffer.append(d);
      let req = buffer.parseRequest();
      if (req instanceof http.WasiRequest) {
        handle_req(cs, req);
        break;
      }
    } catch (e) {
      print(e);
    }
  }
  print('end:', cs.peer());
}

async function handle_req(s, req) {
  print('uri:', req.uri)
  let resp = new http.WasiResponse();
  if (req.uri == '/main.css') {
    resp.headers = {
      "Content-Type": "text/css; charset=utf-8"
    }
    let r = resp.encode(css);
    s.write(r);
  } else {
    resp.headers = {
      "Content-Type": "text/html; charset=utf-8"
    }
    let data = createServerData()
    renderToPipeableStream(
      <DataProvider data={data}>
        <App assets={assets} />
      </DataProvider>
    ).pipe(resp.chunk(s));
  }
}

async function server_start() {
  print('listen 8002...');
  try {
    let s = new net.WasiTcpServer(8002);
    for (var i = 0; ; i++) {
      let cs = await s.accept();
      handle_client(cs);
    }
  } catch (e) {
    print(e)
  }
}

server_start();
