use crate::*;
use std::convert::TryFrom;
use wasmedge_http_req::request::{Method, Request};
use wasmedge_http_req::response::Response;
use wasmedge_http_req::uri::Uri;

struct HttpModule;

fn parse_headers(ctx: &mut Context, req: &mut Request, header: &JsObject) {
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
    resp: Result<Response, wasmedge_http_req::error::Error>,
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

        let addr = Uri::try_from(url.as_str());
        if let Err(e) = addr {
            let e = ctx.new_string(e.to_string().as_str());
            return ctx.throw_error(e.into()).into();
        }
        let addr = addr.unwrap();
        let mut req = Request::new(&addr);
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

        let addr = Uri::try_from(url.as_str());
        if let Err(e) = addr {
            let e = ctx.new_string(e.to_string().as_str());
            return ctx.throw_error(e.into()).into();
        }
        let addr = addr.unwrap();
        let mut req = Request::new(&addr);
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

        let addr = Uri::try_from(url.as_str());
        if let Err(e) = addr {
            let e = ctx.new_string(e.to_string().as_str());
            return ctx.throw_error(e.into()).into();
        }
        let addr = addr.unwrap();
        let mut req = Request::new(&addr);
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

        let addr = Uri::try_from(url.as_str());
        if let Err(e) = addr {
            let e = ctx.new_string(e.to_string().as_str());
            return ctx.throw_error(e.into()).into();
        }
        let addr = addr.unwrap();
        let mut req = Request::new(&addr);
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

        let addr = Uri::try_from(url.as_str());
        if let Err(e) = addr {
            let e = ctx.new_string(e.to_string().as_str());
            return ctx.throw_error(e.into()).into();
        }
        let addr = addr.unwrap();
        let mut req = Request::new(&addr);
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

mod http_server {
    use crate::*;

    use bytecodec::bytes::{BytesEncoder, RemainingBytesDecoder};
    use bytecodec::io::{IoDecodeExt, IoEncodeExt};
    use bytecodec::{DecodeExt, Encode};
    use httpcodec::{
        HeaderField, HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, ResponseEncoder,
        StatusCode,
    };
    use std::io::{BufReader, ErrorKind, Read, Write};
    use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream, ToSocketAddrs};

    pub struct HttpServer {
        listener: TcpListener,
    }

    impl HttpServer {
        pub fn new<A: ToSocketAddrs>(addrs: A) -> std::io::Result<Self> {
            let listener = TcpListener::bind(addrs)?;
            Ok(HttpServer { listener })
        }

        fn handle<F, E>(
            &self,
            stream: &mut TcpStream,
            mut handler_impl: F,
        ) -> Result<Vec<u8>, String>
        where
            F: FnMut(Request<Vec<u8>>) -> Result<Response<Vec<u8>>, E>,
            E: Into<Box<dyn std::error::Error>>,
        {
            let mut decoder = RequestDecoder::<
                httpcodec::BodyDecoder<bytecodec::bytes::RemainingBytesDecoder>,
            >::default();
            let mut buff = [0u8; 1024];
            let mut data = Vec::new();

            loop {
                let n = stream.read(&mut buff).map_err(|e| e.to_string())?;
                data.extend_from_slice(&buff[0..n]);
                if n < 1024 {
                    break;
                }
            }

            let req = decoder
                .decode_from_bytes(data.as_slice())
                .map_err(|e| format!("{:?}", e))?;

            let r = handler_impl(req).map_err(|e| format!("{:?}", e.into()))?;
            let mut encoder =
                ResponseEncoder::new(httpcodec::BodyEncoder::new(BytesEncoder::new()));
            encoder.start_encoding(r).map_err(|e| format!("{:?}", e))?;

            let mut write_buf = vec![];
            encoder
                .encode_all(&mut write_buf)
                .map_err(|e| format!("{:?}", e))?;
            Ok(write_buf)
        }

        pub fn accept<F, E>(&self, handler_impl: F) -> Result<(), String>
        where
            F: FnMut(Request<Vec<u8>>) -> Result<Response<Vec<u8>>, E>,
            E: Into<Box<dyn std::error::Error>>,
        {
            let (mut stream, _) = self.listener.accept().map_err(|e| e.to_string())?;
            match self.handle(&mut stream, handler_impl) {
                Ok(buf) => {
                    stream.write(buf.as_slice()).map_err(|e| e.to_string())?;
                    stream.shutdown(Shutdown::Both);
                    Ok(())
                }
                Err(e) => {
                    let resp = Response::new(
                        HttpVersion::V1_0,
                        StatusCode::new(500).unwrap(),
                        ReasonPhrase::new("").unwrap(),
                        "",
                    );

                    let buf = resp.to_string();
                    stream.write(buf.as_bytes()).map_err(|e| e.to_string())?;
                    stream.shutdown(Shutdown::Both);
                    Err(e)
                }
            }
        }
    }

    pub struct HttpServerDef;
    impl JsClassDef<HttpServer> for HttpServerDef {
        const CLASS_NAME: &'static str = "HttpServer\0";
        const CONSTRUCTOR_ARGC: u8 = 0;

        fn constructor(_ctx: &mut Context, argv: &[JsValue]) -> Option<HttpServer> {
            if let JsValue::String(addrs) = argv.get(0)? {
                let addrs = addrs.to_string();
                HttpServer::new(addrs).ok()
            } else {
                None
            }
        }

        fn proto_init(p: &mut JsClassProto<HttpServer, Self>) {
            struct FnAccept;
            impl JsMethod<HttpServer> for FnAccept {
                const NAME: &'static str = "accept\0";
                const LEN: u8 = 1;

                fn call(ctx: &mut Context, this_val: &mut HttpServer, argv: &[JsValue]) -> JsValue {
                    if let Some(JsValue::Function(f)) = argv.get(0) {
                        match this_val.accept(|r| -> Result<Response<Vec<u8>>, String> {
                            let mut request_obj = ctx.new_object();
                            request_obj.set("method", ctx.new_string(r.method().as_str()).into());
                            request_obj
                                .set("path", ctx.new_string(r.request_target().as_str()).into());

                            {
                                let mut header_obj = ctx.new_object();
                                let header = r.header();
                                for field in header.fields() {
                                    header_obj
                                        .set(field.name(), ctx.new_string(field.value()).into());
                                }
                                request_obj.set("header", header_obj.into());
                            }
                            {
                                let body = r.body();
                                if !body.is_empty() {
                                    let array_buff = ctx.new_array_buffer(body.as_slice());
                                    request_obj.set("body", array_buff.into());
                                } else {
                                    request_obj.set("body", JsValue::Null);
                                }
                            }

                            let mut arg = [request_obj.into()];
                            let r = f.call(&mut arg);

                            if let JsValue::Object(response_obj) = r {
                                let status = if let JsValue::Int(s) = response_obj.get("status") {
                                    s
                                } else {
                                    200
                                };
                                let mut resp = match response_obj.get("body") {
                                    JsValue::String(s) => Response::new(
                                        HttpVersion::V1_0,
                                        StatusCode::new(status as u16).unwrap(),
                                        ReasonPhrase::new("").unwrap(),
                                        Vec::from(s.to_string()),
                                    ),
                                    JsValue::ArrayBuffer(buf) => Response::new(
                                        HttpVersion::V1_0,
                                        StatusCode::new(status as u16).unwrap(),
                                        ReasonPhrase::new("").unwrap(),
                                        buf.to_vec(),
                                    ),
                                    other => {
                                        if let JsValue::String(other_str) =
                                            ctx.value_to_string(&other)
                                        {
                                            Response::new(
                                                HttpVersion::V1_0,
                                                StatusCode::new(status as u16).unwrap(),
                                                ReasonPhrase::new("").unwrap(),
                                                Vec::from(other_str.to_string()),
                                            )
                                        } else {
                                            Response::new(
                                                HttpVersion::V1_0,
                                                StatusCode::new(500).unwrap(),
                                                ReasonPhrase::new("").unwrap(),
                                                vec![],
                                            )
                                        }
                                    }
                                };

                                if let JsValue::Object(header_obj) = response_obj.get("header") {
                                    if let Ok(m) = header_obj.to_map() {
                                        let mut header = resp.header_mut();
                                        for (k, v) in m {
                                            if let JsValue::String(v) = v {
                                                if let Ok(f) = HeaderField::new(
                                                    k.as_str(),
                                                    v.to_string().as_str(),
                                                ) {
                                                    header.add_field(f);
                                                }
                                            }
                                        }
                                    };
                                };
                                Ok(resp)
                            } else {
                                Ok(Response::new(
                                    HttpVersion::V1_0,
                                    StatusCode::new(400).unwrap(),
                                    ReasonPhrase::new("").unwrap(),
                                    vec![],
                                ))
                            }
                        }) {
                            Ok(_) => JsValue::UnDefined,
                            Err(e) => ctx.throw_internal_type_error(e.as_str()).into(),
                        }
                    } else {
                        ctx.throw_type_error("'callback' is not a function").into()
                    }
                }
            }

            p.add_function(FnAccept);
        }
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

        let class_ctor = ctx.register_class(http_server::HttpServerDef);
        m.add_export(http_server::HttpServerDef::CLASS_NAME, class_ctor);
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module(
        "http\0",
        HttpModule,
        &[
            "GET\0",
            "POST\0",
            "PUT\0",
            "PATCH\0",
            "DELETE\0",
            http_server::HttpServerDef::CLASS_NAME,
        ],
    );
}
