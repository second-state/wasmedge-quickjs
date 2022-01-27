import * as std from 'std'
import * as os from 'os'
import * as http from 'http'

args = args.slice(1)
print("Hello", ...args)
setTimeout(() => {
    print('timeout 1s')
}, 1000)


async function test_fetch() {
    try {
        let r = await http.fetch("http://152.136.235.225/get")
        print(await r.json())
    } catch (e) {
        print(e)
    }
}
test_fetch()