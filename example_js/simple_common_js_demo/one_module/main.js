print('hello one_module');
print('dirname:',__dirname);
let other_module_exports = require('../other_module/main.js')
print('other_module_exports=',other_module_exports)