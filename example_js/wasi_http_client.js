import * as net from 'wasi_net'
import * as http from 'wasi_http'

async function handle_response(s){
    let buf = new http.Buffer()
    while(true){
        buf.append(await s.read())
        let resp = buf.parseResponse()
        if(resp instanceof http.WasiResponse){
            print('resp.body')
            print(newStringFromUTF8(resp.body))
            break
        }
    }
}

async function get_test(){
    try{
        let ss = await net.connect('152.136.235.225:80')
        let req = new http.WasiRequest()
        req.headers = {'Host':'152.136.235.225'}
        req.uri='/get?a=123'
        req.method = 'GET'
        ss.write(req.encode())
        print('wait get')
        await handle_response(ss)
        print('get end')

    }catch(e){
        print('catch:',e)
    }
}

async function post_test(){
    try{
        let ss = await net.connect('152.136.235.225:80')
        let req = new http.WasiRequest()
        req.headers = {
            'Host':'152.136.235.225'
        }
        req.uri='/post?a=123'
        req.method = 'POST'
        req.body = 'hello'
        ss.write(req.encode())
        print('wait post')
        await handle_response(ss)
        print('post end')

    }catch(e){
        print('catch:',e)
    }
}

get_test()
post_test()