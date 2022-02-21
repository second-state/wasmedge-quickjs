use super::core::request::HttpRequest;
use super::core::ParseError;
use crate::event_loop::AsyncTcpConn;
use crate::internal_module::httpx::core::response::{BodyLen, HttpResponse};
use crate::internal_module::httpx::core::Version::V1_1;
use crate::internal_module::wasi_net_module::WasiTcpConn;
use crate::{
    Context, JsClassDef, JsClassGetterSetter, JsClassProto, JsFn, JsMethod, JsModuleDef, JsObject,
    JsValue, ModuleInit,
};
use std::collections::HashMap;
use std::io::BufReader;
use std::str::FromStr;

struct Buffer;
impl JsClassDef<Vec<u8>> for Buffer {
    const CLASS_NAME: &'static str = "Buffer\0";
    const CONSTRUCTOR_ARGC: u8 = 0;

    fn constructor(_ctx: &mut Context, argv: &[JsValue]) -> Result<Vec<u8>, JsValue> {
        if let Some(JsValue::ArrayBuffer(s)) = argv.get(0) {
            Ok(s.as_ref().to_vec())
        } else {
            Ok(vec![])
        }
    }

    fn proto_init(p: &mut JsClassProto<Vec<u8>, Self>) {
        struct Append;
        impl JsMethod<Vec<u8>> for Append {
            const NAME: &'static str = "append\0";
            const LEN: u8 = 1;

            fn call(_ctx: &mut Context, this_val: &mut Vec<u8>, argv: &[JsValue]) -> JsValue {
                if let Some(JsValue::ArrayBuffer(data)) = argv.get(0) {
                    this_val.extend_from_slice(data.as_ref());
                    JsValue::Bool(true)
                } else {
                    JsValue::Bool(false)
                }
            }
        }
        p.add_function::<Append>();

        struct ParseRequest;
        impl JsMethod<Vec<u8>> for ParseRequest {
            const NAME: &'static str = "parseRequest\0";
            const LEN: u8 = 0;

            fn call(ctx: &mut Context, this_val: &mut Vec<u8>, _argv: &[JsValue]) -> JsValue {
                match HttpRequest::parse(this_val.as_slice()) {
                    Ok(req) => WasiRequest::gen_js_obj(ctx, req),
                    Err(ParseError::Pending) => JsValue::UnDefined,
                    Err(e) => ctx.new_error(format!("{:?}", e).as_str()),
                }
            }
        }
        p.add_function::<ParseRequest>();

        struct ParseResponse;
        impl JsMethod<Vec<u8>> for ParseResponse {
            const NAME: &'static str = "parseResponse\0";
            const LEN: u8 = 0;

            fn call(ctx: &mut Context, this_val: &mut Vec<u8>, _argv: &[JsValue]) -> JsValue {
                match HttpResponse::parse(this_val.as_slice()) {
                    Ok((resp, n)) => {
                        *this_val = this_val[n..].to_vec();
                        WasiResponseDef::gen_js_obj(ctx, WasiResponse(resp))
                    }
                    Err(ParseError::Pending) => JsValue::UnDefined,
                    Err(e) => ctx.new_error(format!("{:?}", e).as_str()),
                }
            }
        }
        p.add_function::<ParseResponse>();

        struct Buffer;
        impl JsClassGetterSetter<Vec<u8>> for Buffer {
            const NAME: &'static str = "buffer\0";

            fn getter(ctx: &mut Context, this_val: &mut Vec<u8>) -> JsValue {
                ctx.new_array_buffer(this_val.as_slice()).into()
            }

            fn setter(_ctx: &mut Context, _this_val: &mut Vec<u8>, _val: JsValue) {}
        }
        p.add_getter_setter::<Buffer>();

        struct Length;
        impl JsClassGetterSetter<Vec<u8>> for Length {
            const NAME: &'static str = "length\0";

            fn getter(_ctx: &mut Context, this_val: &mut Vec<u8>) -> JsValue {
                JsValue::Int(this_val.len() as i32)
            }

            fn setter(_ctx: &mut Context, _this_val: &mut Vec<u8>, _val: JsValue) {}
        }
        p.add_getter_setter::<Length>();
    }
}

struct WasiRequest;
impl JsClassDef<HttpRequest> for WasiRequest {
    const CLASS_NAME: &'static str = "WasiRequest\0";
    const CONSTRUCTOR_ARGC: u8 = 0;

    fn constructor(_ctx: &mut Context, _argv: &[JsValue]) -> Result<HttpRequest, JsValue> {
        use super::core::request;
        use super::core::*;
        Ok(HttpRequest {
            method: Method::Get,
            version: Version::V1_0,
            resource: request::Resource::Path(Default::default()),
            headers: Default::default(),
            body: vec![],
        })
    }

    fn proto_init(p: &mut JsClassProto<HttpRequest, Self>) {
        struct Body;
        impl JsClassGetterSetter<HttpRequest> for Body {
            const NAME: &'static str = "body\0";

            fn getter(ctx: &mut Context, this_val: &mut HttpRequest) -> JsValue {
                ctx.new_array_buffer(this_val.body.as_slice()).into()
            }

            fn setter(_ctx: &mut Context, this_val: &mut HttpRequest, val: JsValue) {
                match val {
                    JsValue::String(s) => {
                        this_val.body = Vec::from(s.to_string());
                    }
                    JsValue::Object(obj) => {
                        if let Some(v) = Buffer::opaque(&obj) {
                            this_val.body = v.to_vec();
                        }
                    }
                    JsValue::ArrayBuffer(buf) => {
                        this_val.body = buf.to_vec();
                    }
                    _ => {}
                }
            }
        }
        p.add_getter_setter::<Body>();

        struct Headers;
        impl JsClassGetterSetter<HttpRequest> for Headers {
            const NAME: &'static str = "headers\0";

            fn getter(ctx: &mut Context, this_val: &mut HttpRequest) -> JsValue {
                let mut headers = ctx.new_object();
                for (k, v) in &this_val.headers {
                    headers.set(k.as_str(), ctx.new_string(v.as_str()).into());
                }
                headers.into()
            }

            fn setter(ctx: &mut Context, this_val: &mut HttpRequest, val: JsValue) {
                if let JsValue::Object(headers) = val {
                    if let Ok(h) = headers.to_map() {
                        let mut new_header = HashMap::new();
                        for (k, v) in h {
                            if let JsValue::String(v_str) = ctx.value_to_string(&v) {
                                new_header.insert(k, v_str.to_string());
                            }
                        }
                        this_val.headers = new_header;
                    }
                }
            }
        }
        p.add_getter_setter::<Headers>();

        struct Method;
        impl JsClassGetterSetter<HttpRequest> for Method {
            const NAME: &'static str = "method\0";

            fn getter(ctx: &mut Context, this_val: &mut HttpRequest) -> JsValue {
                ctx.new_string(&format!("{:?}", this_val.method)).into()
            }

            fn setter(_ctx: &mut Context, this_val: &mut HttpRequest, val: JsValue) {
                if let JsValue::String(method) = val {
                    let method = method.to_string().to_uppercase();
                    if let Ok(m) = super::core::Method::from_str(method.as_str()) {
                        this_val.method = m;
                    }
                }
            }
        }
        p.add_getter_setter::<Method>();

        struct Version;
        impl JsClassGetterSetter<HttpRequest> for Version {
            const NAME: &'static str = "version\0";

            fn getter(ctx: &mut Context, this_val: &mut HttpRequest) -> JsValue {
                ctx.new_string(&format!("{}", this_val.version)).into()
            }

            fn setter(_ctx: &mut Context, this_val: &mut HttpRequest, val: JsValue) {
                if let JsValue::String(version) = val {
                    let version = version.to_string();
                    if let Ok(m) = super::core::Version::from_str(version.as_str()) {
                        this_val.version = m;
                    }
                }
            }
        }
        p.add_getter_setter::<Version>();

        struct URI;
        impl JsClassGetterSetter<HttpRequest> for URI {
            const NAME: &'static str = "uri\0";

            fn getter(ctx: &mut Context, this_val: &mut HttpRequest) -> JsValue {
                ctx.new_string(format!("{}", this_val.resource).as_str())
                    .into()
            }

            fn setter(_ctx: &mut Context, this_val: &mut HttpRequest, val: JsValue) {
                if let JsValue::String(uri) = val {
                    let uri = uri.to_string();
                    let uri = super::core::request::Resource::Path(uri);
                    this_val.resource = uri;
                }
            }
        }
        p.add_getter_setter::<URI>();

        struct Encode;
        impl JsMethod<HttpRequest> for Encode {
            const NAME: &'static str = "encode\0";
            const LEN: u8 = 0;

            fn call(ctx: &mut Context, this_val: &mut HttpRequest, _argv: &[JsValue]) -> JsValue {
                let mut buf = Vec::from(format!("{}", this_val));
                buf.extend_from_slice(this_val.body.as_slice());
                ctx.new_array_buffer(buf.as_slice()).into()
            }
        }
        p.add_function::<Encode>();
    }
}

struct WasiResponse(HttpResponse);
struct WasiResponseDef;
impl JsClassDef<WasiResponse> for WasiResponseDef {
    const CLASS_NAME: &'static str = "WasiResponse\0";
    const CONSTRUCTOR_ARGC: u8 = 0;

    fn constructor(_ctx: &mut Context, _argv: &[JsValue]) -> Result<WasiResponse, JsValue> {
        use super::core::request;
        use super::core::*;
        Ok(WasiResponse(HttpResponse {
            version: Version::V1_0,
            status_code: 200,
            status_text: "OK".to_string(),
            headers: Default::default(),
            body_len: BodyLen::Length(0),
        }))
    }

    fn proto_init(p: &mut JsClassProto<WasiResponse, Self>) {
        struct BodyLength;
        impl JsClassGetterSetter<WasiResponse> for BodyLength {
            const NAME: &'static str = "bodyLength\0";

            fn getter(ctx: &mut Context, this_val: &mut WasiResponse) -> JsValue {
                match this_val.0.body_len {
                    BodyLen::Length(n) => JsValue::Int(n as i32),
                    BodyLen::Chunked => ctx.new_string("chunked").into(),
                }
            }

            fn setter(_ctx: &mut Context, this_val: &mut WasiResponse, val: JsValue) {
                match val {
                    JsValue::UnDefined | JsValue::Null => {
                        this_val.0.body_len = BodyLen::Length(0);
                    }
                    JsValue::Int(n) => {
                        this_val.0.body_len = BodyLen::Length(n as usize);
                    }
                    _ => {}
                }
            }
        }
        p.add_getter_setter::<BodyLength>();

        struct Headers;
        impl JsClassGetterSetter<WasiResponse> for Headers {
            const NAME: &'static str = "headers\0";

            fn getter(ctx: &mut Context, this_val: &mut WasiResponse) -> JsValue {
                let mut headers = ctx.new_object();
                for (k, v) in &this_val.0.headers {
                    headers.set(k.as_str(), ctx.new_string(v.as_str()).into());
                }
                headers.into()
            }

            fn setter(ctx: &mut Context, this_val: &mut WasiResponse, val: JsValue) {
                if let JsValue::Object(headers) = val {
                    if let Ok(h) = headers.to_map() {
                        let mut new_header = HashMap::new();
                        for (k, v) in h {
                            if let JsValue::String(v_str) = ctx.value_to_string(&v) {
                                new_header.insert(k, v_str.to_string());
                            }
                        }
                        this_val.0.headers = new_header;
                    }
                }
            }
        }
        p.add_getter_setter::<Headers>();

        struct Status;
        impl JsClassGetterSetter<WasiResponse> for Status {
            const NAME: &'static str = "status\0";

            fn getter(_ctx: &mut Context, this_val: &mut WasiResponse) -> JsValue {
                JsValue::Int(this_val.0.status_code as i32)
            }

            fn setter(_ctx: &mut Context, this_val: &mut WasiResponse, val: JsValue) {
                if let JsValue::Int(status) = val {
                    this_val.0.status_code = status as u16;
                    this_val.0.status_text = match status {
                        200 => "OK",
                        400 => "Bad Request",
                        404 => "Not Found",
                        500 => "Internal Server Error",
                        _ => "",
                    }
                    .to_string();
                }
            }
        }
        p.add_getter_setter::<Status>();

        struct Version;
        impl JsClassGetterSetter<WasiResponse> for Version {
            const NAME: &'static str = "version\0";

            fn getter(ctx: &mut Context, this_val: &mut WasiResponse) -> JsValue {
                ctx.new_string(&format!("{}", this_val.0.version)).into()
            }

            fn setter(_ctx: &mut Context, this_val: &mut WasiResponse, val: JsValue) {
                if let JsValue::String(version) = val {
                    let version = version.to_string();
                    if let Ok(m) = super::core::Version::from_str(version.as_str()) {
                        this_val.0.version = m;
                    }
                }
            }
        }
        p.add_getter_setter::<Version>();

        struct StatusText;
        impl JsClassGetterSetter<WasiResponse> for StatusText {
            const NAME: &'static str = "statusText\0";

            fn getter(ctx: &mut Context, this_val: &mut WasiResponse) -> JsValue {
                ctx.new_string(this_val.0.status_text.as_str()).into()
            }

            fn setter(_ctx: &mut Context, this_val: &mut WasiResponse, val: JsValue) {
                if let JsValue::String(status_text) = val {
                    let status_text = status_text.to_string();
                    this_val.0.status_text = status_text;
                }
            }
        }
        p.add_getter_setter::<StatusText>();

        struct Encode;
        impl JsMethod<WasiResponse> for Encode {
            const NAME: &'static str = "encode\0";
            const LEN: u8 = 0;

            fn call(ctx: &mut Context, this_val: &mut WasiResponse, argv: &[JsValue]) -> JsValue {
                let body = argv.get(0);
                let body = match body {
                    Some(JsValue::ArrayBuffer(buffer)) => {
                        let body = buffer.as_ref().to_vec();
                        this_val.0.body_len = BodyLen::Length(body.len());
                        Some(body)
                    }
                    Some(JsValue::String(s)) => {
                        let body = Vec::from(s.to_string());
                        this_val.0.body_len = BodyLen::Length(body.len());
                        Some(body)
                    }
                    _ => {
                        if this_val.0.body_len != BodyLen::Chunked {
                            this_val.0.body_len = BodyLen::Length(0);
                        }
                        None
                    }
                };
                let mut buf = Vec::from(format!("{}", this_val.0));

                if let Some(body) = body {
                    if !body.is_empty() {
                        buf.extend_from_slice(body.as_slice());
                    }
                }
                ctx.new_array_buffer(buf.as_slice()).into()
            }
        }
        p.add_function::<Encode>();

        struct Chunk;
        impl JsMethod<WasiResponse> for Chunk {
            const NAME: &'static str = "chunk\0";
            const LEN: u8 = 0;

            fn call(ctx: &mut Context, this_val: &mut WasiResponse, argv: &[JsValue]) -> JsValue {
                if let Some(JsValue::Object(s)) = argv.get(0) {
                    this_val.0.body_len = BodyLen::Chunked;
                    this_val.0.version = V1_1;
                    let resp_header = Encode::call(ctx, this_val, &[]);
                    let mut s = s.clone();
                    s.invoke("write", &[resp_header]);
                    WasiChunkResponseDef::gen_js_obj(ctx, WasiChunkResponse(s))
                } else {
                    JsValue::UnDefined
                }
            }
        }
        p.add_function::<Chunk>();
    }
}

struct WasiChunkResponse(JsObject);
struct WasiChunkResponseDef;
impl JsClassDef<WasiChunkResponse> for WasiChunkResponseDef {
    const CLASS_NAME: &'static str = "ChunkResponse\0";
    const CONSTRUCTOR_ARGC: u8 = 0;
    fn constructor(_ctx: &mut Context, _argv: &[JsValue]) -> Result<WasiChunkResponse, JsValue> {
        Err(JsValue::UnDefined)
    }

    fn proto_init(p: &mut JsClassProto<WasiChunkResponse, Self>) {
        struct ON;
        impl JsMethod<WasiChunkResponse> for ON {
            const NAME: &'static str = "on\0";
            const LEN: u8 = 0;

            fn call(
                _ctx: &mut Context,
                this_val: &mut WasiChunkResponse,
                argv: &[JsValue],
            ) -> JsValue {
                this_val.0.invoke("on", argv)
            }
        }
        p.add_function::<ON>();

        struct WR;
        impl JsMethod<WasiChunkResponse> for WR {
            const NAME: &'static str = "write\0";
            const LEN: u8 = 1;

            fn call(
                ctx: &mut Context,
                this_val: &mut WasiChunkResponse,
                argv: &[JsValue],
            ) -> JsValue {
                let data = argv.get(0);
                match data {
                    Some(JsValue::String(s)) => {
                        let data = s.to_string();
                        let data_len = data.len();
                        this_val.0.invoke(
                            "write",
                            &[ctx
                                .new_string(format!("{:x}\r\n", data_len).as_str())
                                .into()],
                        );
                        this_val.0.invoke("write", &[s.clone().into()]);
                        this_val.0.invoke("write", &[ctx.new_string("\r\n").into()]);
                    }
                    Some(JsValue::ArrayBuffer(buff)) => {
                        let data = buff.as_ref();
                        let data_len = data.len();
                        this_val.0.invoke(
                            "write",
                            &[ctx
                                .new_string(format!("{:x}\r\n", data_len).as_str())
                                .into()],
                        );
                        this_val.0.invoke("write", &[buff.clone().into()]);
                        this_val.0.invoke("write", &[ctx.new_string("\r\n").into()]);
                    }
                    Some(JsValue::Object(o)) => {
                        let data = o.to_string();
                        let data_len = data.len();
                        this_val.0.invoke(
                            "write",
                            &[ctx
                                .new_string(format!("{:x}\r\n", data_len).as_str())
                                .into()],
                        );
                        this_val.0.invoke("write", &[o.clone().into()]);
                        this_val.0.invoke("write", &[ctx.new_string("\r\n").into()]);
                    }
                    _ => {}
                };
                JsValue::Bool(true)
            }
        }
        p.add_function::<WR>();

        struct End;
        impl JsMethod<WasiChunkResponse> for End {
            const NAME: &'static str = "end\0";
            const LEN: u8 = 1;

            fn call(
                ctx: &mut Context,
                this_val: &mut WasiChunkResponse,
                argv: &[JsValue],
            ) -> JsValue {
                let data = argv.get(0);
                match data {
                    Some(JsValue::String(s)) => {
                        let data = s.to_string();
                        let data_len = data.len();
                        this_val.0.invoke(
                            "write",
                            &[ctx
                                .new_string(format!("{:x}\r\n", data_len).as_str())
                                .into()],
                        );
                        this_val.0.invoke("write", &[s.clone().into()]);
                        this_val.0.invoke("write", &[ctx.new_string("\r\n").into()]);
                    }
                    Some(JsValue::ArrayBuffer(buff)) => {
                        let data = buff.as_ref();
                        let data_len = data.len();
                        this_val.0.invoke(
                            "write",
                            &[ctx
                                .new_string(format!("{:x}\r\n", data_len).as_str())
                                .into()],
                        );
                        this_val.0.invoke("write", &[buff.clone().into()]);
                        this_val.0.invoke("write", &[ctx.new_string("\r\n").into()]);
                    }
                    Some(JsValue::Object(o)) => {
                        let data = o.to_string();
                        let data_len = data.len();
                        this_val.0.invoke(
                            "write",
                            &[ctx
                                .new_string(format!("{:x}\r\n", data_len).as_str())
                                .into()],
                        );
                        this_val.0.invoke("write", &[o.clone().into()]);
                        this_val.0.invoke("write", &[ctx.new_string("\r\n").into()]);
                    }
                    _ => {}
                };
                this_val
                    .0
                    .invoke("end", &[ctx.new_string("0\r\n\r\n").into()]);
                // drop socket
                this_val.0 = ctx.new_object();
                JsValue::Bool(true)
            }
        }
        p.add_function::<End>();
    }
}

mod js_url {
    use url::quirks::password;

    use crate::*;

    pub(super) struct URLDef;
    impl JsClassDef<url::Url> for URLDef {
        const CLASS_NAME: &'static str = "URL\0";

        const CONSTRUCTOR_ARGC: u8 = 1;

        fn constructor(ctx: &mut Context, argv: &[JsValue]) -> Result<url::Url, JsValue> {
            let input = argv.get(0);
            if let Some(JsValue::String(url_str)) = input {
                url::Url::parse(url_str.as_str())
                    .map_err(|e| ctx.throw_internal_type_error(e.to_string().as_str()).into())
            } else {
                Err(JsValue::UnDefined)
            }
        }

        fn proto_init(p: &mut JsClassProto<url::Url, Self>) {
            p.add_getter_setter::<Scheme>();
            p.add_getter_setter::<Username>();
            p.add_getter_setter::<Password>();
            p.add_getter_setter::<Host>();
            p.add_getter_setter::<Port>();
            p.add_getter_setter::<Path>();
            p.add_getter_setter::<Query>();
            p.add_function::<ToString>();
        }
    }

    struct ToString;
    impl JsMethod<url::Url> for ToString {
        const NAME: &'static str = "toString\0";

        const LEN: u8 = 0;

        fn call(ctx: &mut Context, this_val: &mut url::Url, _argv: &[JsValue]) -> JsValue {
            ctx.new_string(format!("{}", this_val).as_str()).into()
        }
    }

    struct Scheme;
    impl JsClassGetterSetter<url::Url> for Scheme {
        const NAME: &'static str = "scheme\0";

        fn getter(ctx: &mut Context, this_val: &mut url::Url) -> JsValue {
            ctx.new_string(this_val.scheme()).into()
        }

        fn setter(_ctx: &mut Context, _this_val: &mut url::Url, _val: JsValue) {}
    }

    struct Username;
    impl JsClassGetterSetter<url::Url> for Username {
        const NAME: &'static str = "username\0";

        fn getter(ctx: &mut Context, this_val: &mut url::Url) -> JsValue {
            ctx.new_string(this_val.username()).into()
        }

        fn setter(_ctx: &mut Context, _this_val: &mut url::Url, _val: JsValue) {}
    }

    struct Password;
    impl JsClassGetterSetter<url::Url> for Password {
        const NAME: &'static str = "password\0";

        fn getter(ctx: &mut Context, this_val: &mut url::Url) -> JsValue {
            let password = this_val.password().unwrap_or_default();
            ctx.new_string(password).into()
        }

        fn setter(_ctx: &mut Context, _this_val: &mut url::Url, _val: JsValue) {}
    }

    struct Host;
    impl JsClassGetterSetter<url::Url> for Host {
        const NAME: &'static str = "host\0";

        fn getter(ctx: &mut Context, this_val: &mut url::Url) -> JsValue {
            match this_val.host_str() {
                Some(host) => ctx.new_string(host).into(),
                None => JsValue::UnDefined,
            }
        }

        fn setter(_ctx: &mut Context, _this_val: &mut url::Url, _val: JsValue) {}
    }

    struct Port;
    impl JsClassGetterSetter<url::Url> for Port {
        const NAME: &'static str = "port\0";

        fn getter(_ctx: &mut Context, this_val: &mut url::Url) -> JsValue {
            match this_val.port_or_known_default() {
                Some(port) => JsValue::Int(port as i32),
                None => JsValue::UnDefined,
            }
        }

        fn setter(_ctx: &mut Context, _this_val: &mut url::Url, _val: JsValue) {}
    }

    struct Path;
    impl JsClassGetterSetter<url::Url> for Path {
        const NAME: &'static str = "path\0";

        fn getter(ctx: &mut Context, this_val: &mut url::Url) -> JsValue {
            ctx.new_string(this_val.path()).into()
        }

        fn setter(_ctx: &mut Context, _this_val: &mut url::Url, _val: JsValue) {}
    }

    struct Query;
    impl JsClassGetterSetter<url::Url> for Query {
        const NAME: &'static str = "query\0";

        fn getter(ctx: &mut Context, this_val: &mut url::Url) -> JsValue {
            match this_val.query() {
                Some(query) => ctx.new_string(query).into(),
                None => JsValue::UnDefined,
            }
        }

        fn setter(_ctx: &mut Context, _this_val: &mut url::Url, _val: JsValue) {}
    }
}
use js_url::URLDef;

struct HttpX;

impl ModuleInit for HttpX {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let class_ctor = ctx.register_class(Buffer);
        m.add_export(Buffer::CLASS_NAME, class_ctor);

        let class_ctor = ctx.register_class(WasiRequest);
        m.add_export(WasiRequest::CLASS_NAME, class_ctor);

        let class_ctor = ctx.register_class(WasiResponseDef);
        m.add_export(WasiResponseDef::CLASS_NAME, class_ctor);

        let class_ctor = ctx.register_class(WasiChunkResponseDef);
        m.add_export(WasiChunkResponseDef::CLASS_NAME, class_ctor);

        let class_ctor = ctx.register_class(URLDef);
        m.add_export(URLDef::CLASS_NAME, class_ctor);
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module(
        "wasi_http\0",
        HttpX,
        &[
            Buffer::CLASS_NAME,
            WasiRequest::CLASS_NAME,
            WasiResponseDef::CLASS_NAME,
            WasiChunkResponseDef::CLASS_NAME,
            URLDef::CLASS_NAME,
        ],
    )
}
