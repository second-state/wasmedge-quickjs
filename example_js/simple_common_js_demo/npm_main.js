// use ncc build a single file
// $ncc build npm_main.js

import * as std from 'std'

var md5 = require('md5');
console.log(__dirname);
console.log('md5(message)=',md5('message'));
const { sqrt } = require('mathjs')
console.log('sqrt(-4)=',sqrt(-4).toString())

print('write file')
let f = std.open('hello.txt','w')
let x = f.puts("hello wasm")
f.flush()
f.close()