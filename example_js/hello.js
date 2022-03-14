import * as os from 'os';
import * as std from 'std';
import * as process from 'process'

args = args.slice(1);
print('Hello', ...args);
setTimeout(() => {
    print('timeout 2s');
}, 2000);

let env = process.env
for(var k in env){
    print(k,'=',env[k])
}
