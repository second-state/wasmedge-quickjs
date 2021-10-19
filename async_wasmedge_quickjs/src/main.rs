mod async_net;
mod wasmedge_async_sock;

use lazy_static::lazy_static;
use std::cell::RefCell;
use wasmedge_quickjs::*;

struct Ctx(pub RefCell<Context>);
unsafe impl Sync for Ctx {}

lazy_static! {
    static ref GLOBAL_CTX: Ctx = Ctx(RefCell::new(Context::new()));
}

fn main() {
    println!("Hello, world!");
}

mod example {
    pub fn echo_server() -> &'static str {
        r#"
        import * as async_net from 'async_net'
        
        function buffToString(buff){
            let uint8_buff = new Uint8Array(buff)
            return String.fromCharCode.apply(null,uint8_buff)
        }
        
        async function echo_handle(s){
            try{
                while(true){
                    print(`wait read on sock(${s})`)
                    let buf = await async_net.read(s)
                    print(`recv on sock(${s}),len=${buf.byteLength}`)
                    if(buf.byteLength == 0){
                        async_net.close(s)
                        return
                    }
                    let recv_str = buffToString(buf)
                    let l = await async_net.write(s,`echo: ${recv_str}`)
                    print(`sock(${s}:${l})=> echo: ${recv_str}`)
                }
            }catch(e){
                print(`echo_handle sock(${s}) ${e}`)
            }
        }
        
        async function main(){
            let port = 8000
            print(`[wasi-js] async echo_server start at ${port}`)
            try{
                let listen_sock = await async_net.tcp_listen(port)
                print(`[wasi-js] listen on sock(${listen_sock})`)
                while(true){
                    let s = await async_net.accept(listen_sock)
                    print(`[wasi-js] accept sock(${s})`)
                    echo_handle(s) // dont await!!!
                }
            }catch(e){
                print(`main loop ${e}`)
            }
        }
        
        main()
        "#
    }

    pub fn chat_server() -> &'static str {
        r#"
        import * as async_net from 'async_net'
        
        function buffToString(buff){
            let uint8_buff = new Uint8Array(buff)
            return String.fromCharCode.apply(null,uint8_buff)
        }
        
        async function chat_handle(connects,s){
            try{
                await async_net.write(s,`yours name:`)
                let name_buf = await async_net.read(s)
                if(name_buf.byteLength <= 0){
                    return
                }
                let name = buffToString(name_buf).trim()
                connects.set(name,s)
                for(let [k,v] of connects){
                    await async_net.write(v,`${name} join!\n`)
                }
                while(true){
                    let buf = await async_net.read(s)
                    if(buf.byteLength == 0){
                        break
                    }                    
                    for(let [k,v] of connects){
                        await async_net.write(v,`${name} => `)
                        await async_net.write(v,buf)
                    }
                }
            }catch(e){
                print(`chat_handle sock(${s}) ${e}`)
            }finally{
                print(`close sock(${s})`)
                async_net.close(s)
                connects.delete(s)
            }
        }
        
        async function main(){
            let port = 8000
            let connects = new Map()
            print(`[wasi-js] async chat_server start at ${port}`)
            try{
                let listen_sock = await async_net.tcp_listen(port)
                print(`[wasi-js] listen on sock(${listen_sock})`)
                while(true){
                    let s = await async_net.accept(listen_sock)
                    print(`[wasi-js] accept sock(${s})`)
                    chat_handle(connects,s) // dont await!!!
                }
            }catch(e){
                print(`main loop ${e}`)
            }
        }
        
        main()
        "#
    }
}

#[no_mangle]
extern "C" fn async_main() {
    println!("Hello, world!");
    let mut ctx = GLOBAL_CTX.0.borrow_mut();
    async_net::init_module(&mut ctx);

    let feat = env!("FEATURE");
    let code = match feat {
        "echo" => example::echo_server(),
        "chat" => example::chat_server(),
        feat => panic!("not support feature {}", feat),
    };
    ctx.eval_module_str(code, "input");
}
