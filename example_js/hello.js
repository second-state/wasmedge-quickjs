import * as os from 'os';
import * as std from 'std';
import * as process from 'process'

args = args.slice(1);
print('Hello', ...args);

let id = setTimeout(() => {
    print('setTimeout 2s cancel');
}, 2000);

print(id);
clearTimeout(id);

setTimeout(() => {
    print('setTimeout 2s');
}, 2000);

let env = process.env
for (var k in env) {
    print(k, '=', env[k])
}

let thenable = {
    data: 1,
    then(onFulfilled, onRejected) {
        print("then:")
        onFulfilled(2)
    }
}



async function xx() {
    let p = new Promise((r) => {
        nextTick(() => {
            print("nextTick")
            r(1)
        })
    })

    let a = sleep(() => {
        print('timeout 1s');
    }, 1000).then((v) => {
        return thenable;
    });
    let x = await p;
    print("end await p", x);
    let v = await a;
    print("end xx", v);
}

xx()

print('end main')

