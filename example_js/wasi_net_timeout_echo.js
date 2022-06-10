import * as net from 'wasi_net';
import { TextDecoder } from 'util'


async function handle_client(cs) {
  print(cs.peer());
  let timeout_millis = 5000;
  while (true) {
    try {
      let d = await cs.read(timeout_millis);
      if (d.byteLength <= 0) {
        break;
      }
      let s = new TextDecoder().decode(d)
      print('recv:', s);
      cs.write('echo:' + s);
    } catch (e) {
      print('handle_client err:', e);
      break;
    }
  }
  print('close');
}

async function server_start() {
  print('listen 8000 ...');
  let s = new net.WasiTcpServer(8000);
  let timeout_millis = 5000;
  for (var i = 0; i < 10; i++) {
    try {
      let cs = await s.accept(timeout_millis);
      handle_client(cs);
    } catch (e) {
      print('accept err:', e);
    }
  }
}


server_start();
