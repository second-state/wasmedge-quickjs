import * as net from 'wasi_net';
import { TextDecoder } from 'util'
import { nextTick, exit } from 'process';

async function handle_client(cs) {
  print('server accept:', cs.peer());
  try {
    while (true) {
      let d = await cs.read();
      if (d == undefined || d.byteLength <= 0) {
        break;
      }
      let s = new TextDecoder().decode(d);
      print('server recv:', s);
      cs.write('echo:' + s);
    }
  } catch (e) {
    print('server handle_client error:', e);
  }
  print('server: conn close');
}

async function server_start() {
  print('listen 8000 ...');
  try {
    let s = new net.WasiTcpServer(8000);
    for (var i = 0; i < 100; i++) {
      let cs = await s.accept();
      handle_client(cs);
    }
  } catch (e) {
    print('server accept error:', e)
  }
}

server_start();

async function connect_test() {
  try {
    let ss = await net.WasiTcpConn.connect('127.0.0.1', 8000)
    ss.write('hello');
    let msg = await ss.read() || "";
    print('client recv:', new TextDecoder().decode(msg));
  } catch (e) {
    print('client catch:', e);
  } finally {
    nextTick(() => {
      exit(0)
    })
  }
}

connect_test();
