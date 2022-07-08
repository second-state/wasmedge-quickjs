import { createServer, request, fetch } from 'http';

createServer((req, resp) => {
  print("server: req.httpVersion=", req.httpVersion);
  print("server: req.url=", req.url);
  print("server: req.method=", req.method);
  print("server: req.headers=", Object.keys(req.headers));

  req.on('data', (body) => {
    print("server: req.body=", body);
    print()

    resp.write('echo:')
    resp.end(body)
  })
}).listen(8001, () => {
  print('listen 8001 ...\n');
})

async function test_request() {
  let client = request({ href: "http://127.0.0.1:8001/request", method: 'POST' }, (resp) => {
    var data = '';
    resp.on('data', (chunk) => {
      data += chunk;
    })
    resp.on('end', () => {
      print('request client recv:', data)
      print()
    })
  })

  client.end('hello server')
}

async function test_fetch() {
  let resp = await fetch('http://127.0.0.1:8001/fetch', { method: 'POST', body: 'hello server' })
  print('fetch client recv:', await resp.text())
  print()
}

async function run_test() {
  await test_request()
  await test_fetch()
  exit(0)
}

run_test()
