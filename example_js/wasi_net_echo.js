import * as net from 'wasi_net'

async function handle_client(cs){
    print(cs.peer())
    while(true){
        try{
            let d = await cs.read()
            if(d.byteLength<=0){
                break
            }
            let s = newStringFromUTF8(await d)
            print('recv:',s)
            cs.write('echo:'+s)
        }catch(e){
            print(e)
        }
    }
    print('close')
}

async function server_start(){
    print('listen 8000 ...')
    let s = new net.WasiTcpServer(8000)
    for(var i=0;i<100;i++){
        let cs = await s.accept();
        handle_client(cs)
    }
}

server_start()