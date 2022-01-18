import * as std from 'std'
import * as os from 'os'

args = args.slice(1)
print("Hello",...args)
setTimeout(()=>{
    print('timeout 2s')
},2000)