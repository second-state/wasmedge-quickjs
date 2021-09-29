use crate::*;
use http_req::request::Method;
use http_req::response::Response;
use std::convert::TryFrom;

struct HttpModule;

fn parse_headers(ctx: &mut Context, req: &mut http_req::request::Request, header: &JsObject) {
    if let Ok(header_map) = header.to_map() {
        for (k, v) in header_map {
            if let JsValue::String(v) = ctx.value_to_string(&v) {
                let s = v.to_string();
                req.header(k.as_str(), s.as_str());
            }
        }
    }
}

fn parse_body(ctx: &mut Context, body: JsValue) -> Vec<u8> {
    match body {
        JsValue::String(body) => Vec::from(body.to_string()),
        JsValue::ArrayBuffer(buff) => buff.to_vec(),
        other => {
            if let JsValue::String(s) = ctx.value_to_string(&other) {
                Vec::from(s.to_string())
            } else {
                Vec::new()
            }
        }
    }
}

fn parse_response(
    ctx: &mut Context,
    resp: Result<Response, http_req::error::Error>,
    body: &[u8],
) -> JsValue {
    match resp {
        Ok(o) => {
            let mut obj = ctx.new_object();
            {
                let status = (u16::from(o.status_code())) as i32;
                obj.set("status", status.into());
            }
            {
                let mut header_obj = ctx.new_object();
                for h in o.headers().iter() {
                    header_obj.set(h.0.as_ref(), ctx.new_string(h.1.as_str()).into());
                }
                obj.set("headers", header_obj.into());
            }
            {
                let body = ctx.new_array_buffer(body);
                obj.set("body", body.into());
            }
            obj.into()
        }
        Err(e) => ctx.throw_internal_type_error(e.to_string().as_str()).into(),
    }
}

struct GET;
impl JsFn for GET {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let url = if let Some(JsValue::String(ref s)) = argv.get(0) {
            s.to_string()
        } else {
            return ctx.throw_type_error("url not string\0").into();
        };

        let addr = http_req::uri::Uri::try_from(url.as_str());
        if let Err(e) = addr {
            let e = ctx.new_string(e.to_string().as_str());
            return ctx.throw_error(e.into()).into();
        }
        let addr = addr.unwrap();
        let mut req = http_req::request::Request::new(&addr);
        req.header("Connection", "Close");

        if let Some(JsValue::Object(ref headers)) = argv.get(1) {
            parse_headers(ctx, &mut req, headers)
        }

        let mut write = vec![];
        let resp = req.method(Method::GET).send(&mut write);

        parse_response(ctx, resp, write.as_slice())
    }
}

struct POST;
impl JsFn for POST {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let url = if let Some(JsValue::String(ref s)) = argv.get(0) {
            s.to_string()
        } else {
            return ctx.throw_type_error("url not string\0").into();
        };

        let addr = http_req::uri::Uri::try_from(url.as_str());
        if let Err(e) = addr {
            let e = ctx.new_string(e.to_string().as_str());
            return ctx.throw_error(e.into()).into();
        }
        let addr = addr.unwrap();
        let mut req = http_req::request::Request::new(&addr);
        req.header("Connection", "Close");

        let body = if let Some(body) = argv.get(1) {
            let body = parse_body(ctx, body.clone());
            Some(body)
        } else {
            None
        };

        if let Some(body) = &body {
            let len = body.len();
            req.header("Content-Length", &len);
            req.body(body);
        }

        if let Some(JsValue::Object(ref headers)) = argv.get(2) {
            parse_headers(ctx, &mut req, headers)
        }

        let mut write = vec![];
        let resp = req.method(Method::POST).send(&mut write);

        parse_response(ctx, resp, write.as_slice())
    }
}

struct PUT;
impl JsFn for PUT {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let url = if let Some(JsValue::String(ref s)) = argv.get(0) {
            s.to_string()
        } else {
            return ctx.throw_type_error("url not string\0").into();
        };

        let addr = http_req::uri::Uri::try_from(url.as_str());
        if let Err(e) = addr {
            let e = ctx.new_string(e.to_string().as_str());
            return ctx.throw_error(e.into()).into();
        }
        let addr = addr.unwrap();
        let mut req = http_req::request::Request::new(&addr);
        req.header("Connection", "Close");

        let body = if let Some(body) = argv.get(1) {
            let body = parse_body(ctx, body.clone());
            Some(body)
        } else {
            None
        };

        if let Some(body) = &body {
            let len = body.len();
            req.header("Content-Length", &len);
            req.body(body);
        }

        if let Some(JsValue::Object(ref headers)) = argv.get(2) {
            parse_headers(ctx, &mut req, headers)
        }

        let mut write = vec![];
        let resp = req.method(Method::PUT).send(&mut write);

        parse_response(ctx, resp, write.as_slice())
    }
}

struct PATCH;
impl JsFn for PATCH {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let url = if let Some(JsValue::String(ref s)) = argv.get(0) {
            s.to_string()
        } else {
            return ctx.throw_type_error("url not string\0").into();
        };

        let addr = http_req::uri::Uri::try_from(url.as_str());
        if let Err(e) = addr {
            let e = ctx.new_string(e.to_string().as_str());
            return ctx.throw_error(e.into()).into();
        }
        let addr = addr.unwrap();
        let mut req = http_req::request::Request::new(&addr);
        req.header("Connection", "Close");

        let body = if let Some(body) = argv.get(1) {
            let body = parse_body(ctx, body.clone());
            Some(body)
        } else {
            None
        };

        if let Some(body) = &body {
            let len = body.len();
            req.header("Content-Length", &len);
            req.body(body);
        }

        if let Some(JsValue::Object(ref headers)) = argv.get(2) {
            parse_headers(ctx, &mut req, headers)
        }

        let mut write = vec![];
        let resp = req.method(Method::PATCH).send(&mut write);

        parse_response(ctx, resp, write.as_slice())
    }
}

struct DELETE;
impl JsFn for DELETE {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let url = if let Some(JsValue::String(ref s)) = argv.get(0) {
            s.to_string()
        } else {
            return ctx.throw_type_error("url not string\0").into();
        };

        let addr = http_req::uri::Uri::try_from(url.as_str());
        if let Err(e) = addr {
            let e = ctx.new_string(e.to_string().as_str());
            return ctx.throw_error(e.into()).into();
        }
        let addr = addr.unwrap();
        let mut req = http_req::request::Request::new(&addr);
        req.header("Connection", "Close");

        let body = if let Some(body) = argv.get(1) {
            let body = parse_body(ctx, body.clone());
            Some(body)
        } else {
            None
        };

        if let Some(body) = &body {
            let len = body.len();
            req.header("Content-Length", &len);
            req.body(body);
        }

        if let Some(JsValue::Object(ref headers)) = argv.get(2) {
            parse_headers(ctx, &mut req, headers)
        }

        let mut write = vec![];
        let resp = req.method(Method::DELETE).send(&mut write);

        parse_response(ctx, resp, write.as_slice())
    }
}

impl ModuleInit for HttpModule {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let get = ctx.new_function::<GET>("GET");
        m.add_export("GET\0", get.into());
        let post = ctx.new_function::<POST>("POST");
        m.add_export("POST\0", post.into());
        let put = ctx.new_function::<PUT>("PUT");
        m.add_export("PUT\0", put.into());
        let patch = ctx.new_function::<PATCH>("PATCH");
        m.add_export("PATCH\0", patch.into());
        let delete = ctx.new_function::<DELETE>("DELETE");
        m.add_export("DELETE\0", delete.into());
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module(
        "http\0",
        HttpModule,
        &["GET\0", "POST\0", "PUT\0", "PATCH\0", "DELETE\0"],
    );
}
