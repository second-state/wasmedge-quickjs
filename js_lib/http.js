let _get = globalThis.http_get
let _post = globalThis.http_post
let _put = globalThis.http_put
let _patch = globalThis.http_patch
let _delete = globalThis.http_delete

export function GET(url, header){
    return _get(url, header)
}
export function POST(url, body, header){
    return _post(url, body, header)
}
export function PUT(url, body, header){
    return _put(url, body, header)
}
export function PATCH(url, body, header){
    return _patch(url, body, header)
}
export function DELETE(url, body, header){
    return _delete(url, body, header)
}