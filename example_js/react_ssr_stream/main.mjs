import * as React from 'react';
import { renderToPipeableStream } from 'react-dom/server';
import { createServer } from 'http';

import LazyHome from './component/LazyHome.jsx';

createServer((req, res) => {
  res.setHeader('Content-type', 'text/html; charset=utf-8');
  renderToPipeableStream(<LazyHome />).pipe(res);
}).listen(8001, () => {
  print('listen 8001...');
})
