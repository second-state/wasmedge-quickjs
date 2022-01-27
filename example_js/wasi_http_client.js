import * as http from 'wasi_http';
import * as net from 'wasi_net';

async function handle_response(s) {
  let buf = new http.Buffer();
  let resp = undefined;
  while (true) {
    buf.append(await s.read());
    if (resp == undefined) {
      resp = buf.parseResponse();
    }
    if (resp instanceof http.WasiResponse) {
      let resp_length = resp.bodyLength;
      if (typeof (resp_length) === "number") {
        if (buf.length >= resp.bodyLength) {
          print('resp.body');
          print(newStringFromUTF8(buf.buffer));
          break;
        }
      } else {
        throw new Error('no support');
      }
    }
  }
}

async function get_test() {
  try {
    let ss = await net.connect('152.136.235.225:80');
    let req = new http.WasiRequest();
    req.headers = { 'Host': '152.136.235.225' };
    req.uri = '/get?a=123';
    req.method = 'GET';
    ss.write(req.encode());
    print('wait get');
    await handle_response(ss);
    print('get end');

  } catch (e) {
    print('catch:', e);
  }
}

async function post_test() {
  try {
    let ss = await net.connect('152.136.235.225:80');
    let req = new http.WasiRequest();
    req.headers = {
      'Host': '152.136.235.225'
    }
    req.uri = '/post?a=123';
    req.method = 'POST';
    req.body = 'hello';
    ss.write(req.encode());
    print('wait post');
    await handle_response(ss);
    print('post end');

  } catch (e) {
    print('catch:', e);
  }
}

get_test();
post_test();
