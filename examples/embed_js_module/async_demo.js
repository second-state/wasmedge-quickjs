import * as std from 'std'

async function simple_val (){
    return "abc"
}

async function wait_simple_val (a){
    let x = await simple_val()
    print("wait_simple_val:",a,':',x)
    return 12345
}

print(wait_simple_val(1))
print(wait_simple_val(2))
