import {HttpServer} from 'http'
let http_server = new HttpServer('0.0.0.0:8000')
print('listen on 0.0.0.0:8000')
while(true){
    http_server.accept((request)=>{
        let body = request.body
        let body_str = String.fromCharCode.apply(null,new Uint8Array(body))
        print(JSON.stringify(request),'\n body_str:',body_str)

        return {
            status:200,
            header:{'Content-Type':'application/json'},
            body:'echo:'+body_str
        }
    });
}