import { createServer } from 'http';

createServer((req, resp) => {
  print("server: req.httpVersion=", req.httpVersion);
  print("server: req.url=", req.url);
  print("server: req.method=", req.method);
  print("server: req.headers=", Object.keys(req.headers));

  req.on('data', (body) => {
    print("server: req.body=", body);
    print()

    resp.end(body)
  })
}).listen(8080, () => {
  print('listen 8080 ...\n');
})
