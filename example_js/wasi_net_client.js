import * as net from 'wasi_net';
import { TextDecoder } from 'util'

async function connect_test() {
  try {
    let ss = await net.connect('127.0.0.1:8000')
    ss.write('hello');
    let msg = await ss.read();
    print('recv:', new TextDecoder().decode(msg));
  } catch (e) {
    print('catch:', e);
  }
}

connect_test();
