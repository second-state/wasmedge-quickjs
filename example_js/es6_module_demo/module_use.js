import { hello as module_def_hello } from './module_def.js'

module_def_hello()

var f = async ()=>{
    let {hello , something} = await import('./module_def1.js')
    await hello()
    console.log("./module_def1.js `something` is ",something)
}

f()