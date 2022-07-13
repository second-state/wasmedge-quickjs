import * as React from 'react';
import { renderToPipeableStream } from 'react-dom/server';
import { createServer } from 'http';
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

createServer((req, res) => {
  print(req.url)
  if (req.url == '/main.css') {
    res.setHeader('Content-Type', 'text/css; charset=utf-8')
    res.end(css)
  } else if (req.url == '/favicon.ico') {
    res.end()
  } else {
    res.setHeader('Content-type', 'text/html');

    res.on('error', (e) => {
      print('res error', e)
    })
    let data = createServerData()
    print('createServerData')

    const stream = renderToPipeableStream(
      <DataProvider data={data}>
        <App assets={assets} />
      </DataProvider>, {
      onShellReady: () => {
        stream.pipe(res)
      },
      onShellError: (e) => {
        print('onShellError:', e)
      }
    }
    );
  }
}).listen(8002, () => {
  print('listen 8002...')
})