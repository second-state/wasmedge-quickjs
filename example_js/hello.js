import * as std from 'std'

args = args.slice(1)
print("\n\n\nHello",...args)

export var a = 3;

let x = eval("1+a")
print("x:",x)

async function xx(){
    return "abc"
}

async function zz(a){
    let x = await xx()
    print("zz:",a,':',x)
    return 12345
}

export function kk(x){
    print('kk:',x)
    return 12345
}


print(zz(1))
print(zz(2))