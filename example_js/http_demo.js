import {GET,POST,PUT,PATCH,DELETE} from 'http'

if(true){
    print('get-test')
    let r = GET("http://152.136.235.225/get?a=123",{"a":"b","c":[1,2,3]})
    print(r.status)
    let headers = r.headers
    print(JSON.stringify(headers))
    let body = r.body;
    let body_str = new Uint8Array(body)
    print(String.fromCharCode.apply(null,body_str))
}

if(true){
    print('post-test')
    let r1 = POST("http://152.136.235.225/post?a=123","haha=1",{"a":"b","c":[1,2,3]})
    print(r1.status)
    let headers1 = r1.headers
    print(JSON.stringify(headers1))
    let body1 = r1.body
    let body_str1 = new Uint8Array(body1)
    print(String.fromCharCode.apply(null,body_str1))
}

if(true){
    print('put-test')
    let r = PUT("http://152.136.235.225/put?a=123","haha=1",{"a":"b","c":[1,2,3]})
    print(r.status)
    let headers = r.headers
    print(JSON.stringify(headers))
    let body = r.body
    let body_str = new Uint8Array(body)
    print(String.fromCharCode.apply(null,body_str))
}

if(true){
    print('patch-test')
    let r = PATCH("http://152.136.235.225/patch?a=123","haha=1",{"a":"b","c":[1,2,3]})
    print(r.status)
    let headers = r.headers
    print(JSON.stringify(headers))
    let body = r.body
    let body_str = new Uint8Array(body)
    print(String.fromCharCode.apply(null,body_str))
}

if(true){
    print('delete-test')
    let r = DELETE("http://152.136.235.225/delete?a=123","haha=1");
    print(r.status)
    let headers = r.headers
    print(JSON.stringify(headers))
    let body = r.body;
    let body_str = new Uint8Array(body)
    print(String.fromCharCode.apply(null,body_str))
}
