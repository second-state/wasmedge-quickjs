const md5 = require('md5');
console.log(__dirname);
console.log('md5(message)=', md5('message'));
const {sqrt} = require('mathjs');
console.log('sqrt(-4)=', sqrt(-4).toString());
const {run} = require('./es6_code_module.js')
run()