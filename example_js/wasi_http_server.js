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
