import * as std from 'std'

async function simple_val (){
    return "abc"
}

export async function wait_simple_val (a){
    let x = await simple_val()
    print("wait_simple_val:",a,':',x)
    return 12345
}
