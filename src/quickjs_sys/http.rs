use super::*;
use std::convert::TryFrom;

pub fn add_http(ctx: &mut Context) {
    let mut g = ctx.get_global();
    let mut http_obj = ctx.new_object();
    http_obj.set("get", ctx.new_function("http_get", Some(get)));
    http_obj.set("post", ctx.new_function("http_post", Some(post)));
    http_obj.set("put", ctx.new_function("http_put", Some(put)));
    http_obj.set("patch", ctx.new_function("http_patch", Some(patch)));
    http_obj.set("delete", ctx.new_function("http_delete", Some(delete)));

    g.set("http", http_obj);
}

unsafe fn parse_response(
    ctx: *mut JSContext,
    r: Result<http_req::response::Response, http_req::error::Error>,
    body: &[u8],
) -> JSValue {
    match r {
        Ok(o) => {
            let obj = JS_NewObject(ctx);
            {
                let status = JS_NewInt32_real(ctx, u16::from(o.status_code()) as i32);
                set(ctx, "status", obj, status);
            }
            {
                let header_obj = JS_NewObject(ctx);
                for h in o.headers().iter() {
                    set(ctx, h.0.as_ref(), header_obj, new_string(ctx, h.1.as_str()));
                }
                set(ctx, "headers", obj, header_obj);
            }
            {
                let body = new_array_buff(ctx, body);
                set(ctx, "body", obj, body);
            }
            obj
        }
        Err(e) => JS_ThrowInternalError(ctx, make_c_string(e.to_string()).as_ptr()),
    }
}

unsafe fn parse_headers(
    ctx: *mut JSContext,
    req: &mut http_req::request::Request,
    header: JSValue,
) -> Result<(), JSValue> {
    let tag = JS_ValueGetTag_real(header);
    if tag != JS_TAG_NULL && tag != JS_TAG_UNDEFINED {
        let obj = deserialize_object(ctx, header);
        if let Err(e) = obj {
            return Err(JS_ThrowInternalError(
                ctx,
                make_c_string(e.as_str()).as_ptr(),
            ));
        }
        let obj = obj.unwrap();

        for (k, v) in obj {
            req.header(
                k.as_str(),
                to_string(ctx, JS_ToString(ctx, v.v)).unwrap().as_str(),
            );
        }
    }
    Ok(())
}

unsafe fn parse_body(ctx: *mut JSContext, body: JSValue) -> Vec<u8> {
    if JS_IsString_real(body) >= 1 {
        return Vec::from(to_string(ctx, body).unwrap());
    }
    let mut len = 0u32;
    let ptr = JS_GetArrayBuffer(ctx, &mut len as *mut u32, body);
    if !ptr.is_null() && len > 0 {
        return Vec::from_raw_parts(ptr, len as usize, (len + 1) as usize).to_vec();
    }
    return Vec::from(to_string(ctx, JS_ToString(ctx, body)).unwrap());
}

pub extern "C" fn get(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    unsafe {
        if argc < 1 {
            return JS_ThrowInternalError(ctx, "url is undefined\0".as_ptr() as *const i8);
        }
        let url_value = *argv.offset(0);
        let url = to_string(ctx, url_value).unwrap();

        let addr = http_req::uri::Uri::try_from(url.as_str()).unwrap();
        let mut req = http_req::request::Request::new(&addr);
        req.header("Connection", "Close");

        if argc >= 2 {
            let header = *argv.offset(1);
            if let Err(e) = parse_headers(ctx, &mut req, header) {
                return e;
            };
        }

        let mut write = Vec::new();
        let r = req.method(http_req::request::Method::GET).send(&mut write);

        parse_response(ctx, r, write.as_slice())
    }
}

pub extern "C" fn post(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    unsafe {
        if argc < 1 {
            return JS_ThrowInternalError(ctx, "url is undefined\0".as_ptr() as *const i8);
        }
        let url_value = *argv.offset(0);
        let url = to_string(ctx, url_value).unwrap();

        let addr = http_req::uri::Uri::try_from(url.as_str()).unwrap();
        let mut req = http_req::request::Request::new(&addr);
        req.header("Connection", "Close");

        let body = {
            if argc >= 2 {
                let body = *argv.offset(1);
                Some(parse_body(ctx, body))
            } else {
                None
            }
        };
        if let Some(ref body) = body {
            req.header("Content-Length", &body.len());
            req.body(body);
        }

        if argc >= 3 {
            let header = *argv.offset(2);
            if let Err(e) = parse_headers(ctx, &mut req, header) {
                return e;
            };
        }

        let mut write = Vec::new();
        let r = req.method(http_req::request::Method::POST).send(&mut write);
        parse_response(ctx, r, write.as_slice())
    }
}

pub extern "C" fn put(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    unsafe {
        if argc < 1 {
            return JS_ThrowInternalError(ctx, "url is undefined\0".as_ptr() as *const i8);
        }
        let url_value = *argv.offset(0);
        let url = to_string(ctx, url_value).unwrap();

        let addr = http_req::uri::Uri::try_from(url.as_str()).unwrap();
        let mut req = http_req::request::Request::new(&addr);
        req.header("Connection", "Close");

        let body = {
            if argc >= 2 {
                let body = *argv.offset(1);
                Some(parse_body(ctx, body))
            } else {
                None
            }
        };
        if let Some(ref body) = body {
            req.header("Content-Length", &body.len());
            req.body(body);
        }

        if argc >= 3 {
            let header = *argv.offset(2);
            if let Err(e) = parse_headers(ctx, &mut req, header) {
                return e;
            };
        }

        let mut write = Vec::new();
        let r = req.method(http_req::request::Method::PUT).send(&mut write);
        parse_response(ctx, r, write.as_slice())
    }
}

pub extern "C" fn patch(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    unsafe {
        if argc < 1 {
            return JS_ThrowInternalError(ctx, "url is undefined\0".as_ptr() as *const i8);
        }
        let url_value = *argv.offset(0);
        let url = to_string(ctx, url_value).unwrap();

        let addr = http_req::uri::Uri::try_from(url.as_str()).unwrap();
        let mut req = http_req::request::Request::new(&addr);
        req.header("Connection", "Close");

        let body = {
            if argc >= 2 {
                let body = *argv.offset(1);
                Some(parse_body(ctx, body))
            } else {
                None
            }
        };
        if let Some(ref body) = body {
            req.header("Content-Length", &body.len());
            req.body(body);
        }

        if argc >= 3 {
            let header = *argv.offset(2);
            if let Err(e) = parse_headers(ctx, &mut req, header) {
                return e;
            };
        }

        let mut write = Vec::new();
        let r = req
            .method(http_req::request::Method::PATCH)
            .send(&mut write);
        parse_response(ctx, r, write.as_slice())
    }
}

pub extern "C" fn delete(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    unsafe {
        if argc < 1 {
            return JS_ThrowInternalError(ctx, "url is undefined\0".as_ptr() as *const i8);
        }
        let url_value = *argv.offset(0);
        let url = to_string(ctx, url_value).unwrap();

        let addr = http_req::uri::Uri::try_from(url.as_str()).unwrap();
        let mut req = http_req::request::Request::new(&addr);
        req.header("Connection", "Close");

        let body = {
            if argc >= 2 {
                let body = *argv.offset(1);
                Some(parse_body(ctx, body))
            } else {
                None
            }
        };
        if let Some(ref body) = body {
            req.header("Content-Length", &body.len());
            req.body(body);
        }

        if argc >= 3 {
            let header = *argv.offset(2);
            if let Err(e) = parse_headers(ctx, &mut req, header) {
                return e;
            };
        }

        let mut write = Vec::new();
        let r = req
            .method(http_req::request::Method::DELETE)
            .send(&mut write);
        parse_response(ctx, r, write.as_slice())
    }
}
