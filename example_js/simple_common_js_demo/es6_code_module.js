import * as std from 'std';

export function run() {
    print('write file');
    let f = std.open('hello.txt', 'w');
    let x = f.puts('hello wasm');
    f.flush();
    f.close();
}