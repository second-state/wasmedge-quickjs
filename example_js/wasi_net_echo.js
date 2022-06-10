import * as net from 'wasi_net';
import { TextDecoder } from 'util'

async function handle_client(cs) {
  print(cs.peer());
  try {
    while (true) {
      let d = await cs.read();
      if (d == undefined || d.byteLength <= 0) {
        print(d)
        break;
      }
      let s = new TextDecoder().decode(d);
      print('recv:', s);
      cs.write('echo:' + s);
    }
  } catch (e) {
    print(e);
  }
  print('close');
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
    print(e)
  }
}

server_start();
