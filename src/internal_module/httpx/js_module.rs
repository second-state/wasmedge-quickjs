use super::core::chunk::HttpChunk;
use super::core::request::HttpRequest;
use super::core::ParseError;
use crate::event_loop::AsyncTcpConn;
use crate::internal_module::httpx::core::response::{BodyLen, HttpResponse};
use crate::internal_module::httpx::core::Version::V1_1;
use crate::{
    register_class, AsObject, Context, JsClassDef, JsClassProto, JsClassTool, JsFn, JsModuleDef,
    JsObject, JsValue, ModuleInit,
};
use std::collections::HashMap;
use std::fmt::format;
use std::io::BufReader;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

struct Buffer(Vec<u8>, usize);

impl Deref for Buffer {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        if self.len() > self.1 {
            &self.0[self.1..]
        } else {
            &[]
        }
    }
}

impl Buffer {
    fn js_buffer(&self, ctx: &mut Context) -> JsValue {
        let buf = self.as_ref();
        if buf.len() > 0 {
            ctx.new_array_buffer(buf).into()
        } else {
            JsValue::Null
        }
    }

    fn js_length(&self, _ctx: &mut Context) -> JsValue {
        JsValue::Int(self.as_ref().len() as i32)
    }

    fn js_append(
        &mut self,
        _this_obj: &mut JsObject,
        _ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        match argv.get(0) {
            Some(JsValue::ArrayBuffer(data)) => {
                self.extend_from_slice(data.as_ref());
                JsValue::Bool(true)
            }
            Some(JsValue::Object(obj)) => {
                if let Some(v) = Buffer::opaque(&JsValue::Object(obj.clone())) {
                    self.extend_from_slice(v.as_ref());
                    JsValue::Bool(true)
                } else {
                    JsValue::Bool(false)
                }
            }
            _ => JsValue::Bool(false),
        }
    }

    fn js_parse_request(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        match HttpRequest::parse(self.as_ref()) {
            Ok(req) => HttpRequest::wrap_obj(ctx, req),
            Err(ParseError::Pending) => JsValue::UnDefined,
            Err(e) => {
                let err = ctx.new_error(format!("{:?}", e).as_str());
                ctx.throw_error(err).into()
            }
        }
    }

    fn js_parse_response(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        match HttpResponse::parse(self.as_ref()) {
            Ok((resp, n)) => {
                self.1 += n;
                HttpResponse::wrap_obj(ctx, resp)
            }
            Err(ParseError::Pending) => JsValue::UnDefined,
            Err(e) => ctx.new_error(format!("{:?}", e).as_str()),
        }
    }

    fn js_parse_chunk_data(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        match HttpChunk::parse(self.as_ref()) {
            Ok((buf, n)) => {
                let r = if buf.len() == 0 {
                    JsValue::Null
                } else {
                    let array_buf = ctx.new_array_buffer(buf);
                    array_buf.into()
                };
                self.1 += n;
                r
            }
            Err(ParseError::Pending) => JsValue::UnDefined,
            Err(e) => ctx.new_error(format!("{:?}", e).as_str()),
        }
    }

    fn js_clear(
        &mut self,
        _this_obj: &mut JsObject,
        _ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        self.0.clear();
        self.1 = 0;
        JsValue::UnDefined
    }
}

impl JsClassDef for Buffer {
    type RefType = Buffer;

    const CLASS_NAME: &'static str = "Buffer";
    const CONSTRUCTOR_ARGC: u8 = 0;

    fn constructor_fn(_ctx: &mut Context, argv: &[JsValue]) -> Result<Buffer, JsValue> {
        if let Some(JsValue::ArrayBuffer(s)) = argv.get(0) {
            Ok(Buffer(s.as_ref().to_vec(), 0))
        } else {
            Ok(Buffer(vec![], 0))
        }
    }

    const FIELDS: &'static [crate::JsClassField<Self::RefType>] = &[
        ("length", Self::js_length, None),
        ("buffer", Self::js_buffer, None),
    ];

    const METHODS: &'static [crate::JsClassMethod<Self::RefType>] = &[
        ("append", 1, Self::js_append),
        ("write", 1, Self::js_append),
        ("parseRequest", 0, Self::js_parse_request),
        ("parseResponse", 0, Self::js_parse_response),
        ("parseChunk", 0, Self::js_parse_chunk_data),
        ("clear", 0, Self::js_clear),
    ];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }
}

impl HttpRequest {
    pub fn js_get_body(&self, ctx: &mut Context) -> JsValue {
        if self.body.len() > 0 {
            ctx.new_array_buffer(self.body.as_slice()).into()
        } else {
            JsValue::Null
        }
    }

    pub fn js_set_body(&mut self, _ctx: &mut Context, val: JsValue) {
        match val {
            JsValue::String(s) => {
                self.body = Vec::from(s.to_string());
            }
            JsValue::Object(obj) => {
                if let Some(v) = Buffer::opaque(&JsValue::Object(obj)) {
                    self.body = v.to_vec();
                }
            }
            JsValue::ArrayBuffer(buf) => {
                self.body = buf.to_vec();
            }
            _ => {}
        }
    }

    pub fn js_get_headers(&self, ctx: &mut Context) -> JsValue {
        let mut headers = ctx.new_object();
        for (k, v) in &self.headers {
            headers.set(k.as_str(), ctx.new_string(v.as_str()).into());
        }
        headers.into()
    }

    pub fn js_set_headers(&mut self, ctx: &mut Context, val: JsValue) {
        if let JsValue::Object(headers) = val {
            if let Ok(h) = headers.to_map() {
                self.headers.clear();
                for (k, v) in h {
                    if let JsValue::String(v_str) = ctx.value_to_string(&v) {
                        self.headers.insert(k.to_lowercase(), v_str.to_string());
                    }
                }
            }
        }
    }

    pub fn js_get_method(&self, ctx: &mut Context) -> JsValue {
        ctx.new_string(&format!("{:?}", self.method)).into()
    }

    pub fn js_set_method(&mut self, _ctx: &mut Context, val: JsValue) {
        if let JsValue::String(method) = val {
            let method = method.to_string().to_uppercase();
            if let Ok(m) = super::core::Method::from_str(method.as_str()) {
                self.method = m;
            }
        }
    }

    pub fn js_get_version(&self, ctx: &mut Context) -> JsValue {
        ctx.new_string(&format!("{}", self.version)).into()
    }

    pub fn js_set_version(&mut self, _ctx: &mut Context, val: JsValue) {
        if let JsValue::String(version) = val {
            let version = version.to_string();
            if let Ok(m) = super::core::Version::from_str(version.as_str()) {
                self.version = m;
            }
        }
    }

    pub fn js_get_uri(&self, ctx: &mut Context) -> JsValue {
        ctx.new_string(format!("{}", self.resource).as_str()).into()
    }

    pub fn js_set_uri(&mut self, _ctx: &mut Context, val: JsValue) {
        if let JsValue::String(uri) = val {
            let uri = uri.to_string();
            let uri = super::core::request::Resource::Path(uri);
            self.resource = uri;
        }
    }

    pub fn js_get_header(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        if let Some(JsValue::String(s)) = argv.first() {
            let key = s.as_str();
            if let Some(v) = self.headers.get(key) {
                ctx.new_string(&v).into()
            } else {
                JsValue::Null
            }
        } else {
            JsValue::Null
        }
    }

    pub fn js_set_header(
        &mut self,
        _this_obj: &mut JsObject,
        _ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        if let (Some(JsValue::String(k)), Some(JsValue::String(v))) = (argv.get(0), argv.get(1)) {
            self.headers
                .insert(k.as_str().to_lowercase(), v.to_string());
        }

        JsValue::UnDefined
    }

    pub fn js_encode(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        let mut buf = Vec::from(format!("{}", self));
        buf.extend_from_slice(self.body.as_slice());
        ctx.new_array_buffer(buf.as_slice()).into()
    }
}

impl JsClassDef for HttpRequest {
    type RefType = HttpRequest;

    const CLASS_NAME: &'static str = "WasiRequest";
    const CONSTRUCTOR_ARGC: u8 = 0;

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(_ctx: &mut Context, _argv: &[JsValue]) -> Result<HttpRequest, JsValue> {
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

    const FIELDS: &'static [crate::JsClassField<Self::RefType>] = &[
        ("body", Self::js_get_body, Some(Self::js_set_body)),
        ("headers", Self::js_get_headers, Some(Self::js_set_headers)),
        ("method", Self::js_get_method, Some(Self::js_set_method)),
        ("version", Self::js_get_version, Some(Self::js_set_version)),
        ("uri", Self::js_get_uri, Some(Self::js_set_uri)),
    ];

    const METHODS: &'static [crate::JsClassMethod<Self::RefType>] = &[
        ("encode", 0, Self::js_encode),
        ("getHeader", 1, Self::js_get_header),
        ("setHeader", 1, Self::js_set_header),
    ];
}

impl HttpResponse {
    fn js_get_body_length(&self, ctx: &mut Context) -> JsValue {
        match self.body_len {
            BodyLen::Length(n) => JsValue::Int(n as i32),
            BodyLen::Chunked => ctx.new_string("chunked").into(),
        }
    }

    fn js_set_body_length(&mut self, _ctx: &mut Context, val: JsValue) {
        match val {
            JsValue::UnDefined | JsValue::Null => {
                self.body_len = BodyLen::Length(0);
            }
            JsValue::Int(n) => {
                self.body_len = BodyLen::Length(n as usize);
            }
            _ => {}
        }
    }

    fn js_get_headers(&self, ctx: &mut Context) -> JsValue {
        let mut headers = ctx.new_object();
        for (k, v) in &self.headers {
            headers.set(k.as_str(), ctx.new_string(v.as_str()).into());
        }
        headers.into()
    }

    fn js_set_headers(&mut self, ctx: &mut Context, val: JsValue) {
        if let JsValue::Object(headers) = val {
            if let Ok(h) = headers.to_map() {
                self.headers.clear();
                for (k, v) in h {
                    if let JsValue::String(v_str) = ctx.value_to_string(&v) {
                        self.headers.insert(k, v_str.to_string());
                    }
                }
            }
        }
    }

    fn js_get_status(&self, _ctx: &mut Context) -> JsValue {
        JsValue::Int(self.status_code as i32)
    }

    fn js_set_status(&mut self, _ctx: &mut Context, val: JsValue) {
        if let JsValue::Int(status) = val {
            self.status_code = status as u16;
            self.status_text = match status {
                200 => "OK",
                400 => "Bad Request",
                404 => "Not Found",
                500 => "Internal Server Error",
                _ => "",
            }
            .to_string();
        }
    }

    fn js_get_version(&self, ctx: &mut Context) -> JsValue {
        ctx.new_string(&format!("{}", self.version)).into()
    }

    fn js_set_version(&mut self, _ctx: &mut Context, val: JsValue) {
        if let JsValue::String(version) = val {
            let version = version.to_string();
            if let Ok(m) = super::core::Version::from_str(version.as_str()) {
                self.version = m;
            }
        }
    }

    fn js_get_status_text(&self, ctx: &mut Context) -> JsValue {
        ctx.new_string(self.status_text.as_str()).into()
    }

    fn js_set_status_text(&mut self, _ctx: &mut Context, val: JsValue) {
        if let JsValue::String(status_text) = val {
            let status_text = status_text.to_string();
            self.status_text = status_text;
        }
    }

    fn js_encode(&mut self, _this: &mut JsObject, ctx: &mut Context, argv: &[JsValue]) -> JsValue {
        let body = argv.get(0);
        let body = match body {
            Some(JsValue::ArrayBuffer(buffer)) => {
                let body = buffer.as_ref().to_vec();
                self.body_len = BodyLen::Length(body.len());
                Some(body)
            }
            Some(JsValue::String(s)) => {
                let body = Vec::from(s.to_string());
                self.body_len = BodyLen::Length(body.len());
                Some(body)
            }
            _ => {
                if self.body_len != BodyLen::Chunked {
                    self.body_len = BodyLen::Length(0);
                }
                None
            }
        };
        let mut buf = Vec::from(format!("{}", self));

        if let Some(body) = body {
            if !body.is_empty() {
                buf.extend_from_slice(body.as_slice());
            }
        }
        ctx.new_array_buffer(buf.as_slice()).into()
    }

    fn js_chunk(&mut self, _this: &mut JsObject, ctx: &mut Context, argv: &[JsValue]) -> JsValue {
        if let Some(JsValue::Object(s)) = argv.get(0) {
            self.body_len = BodyLen::Chunked;
            self.version = V1_1;

            let header_buff = Vec::from(format!("{}", self));
            let resp_header = ctx.new_array_buffer(&header_buff);

            let mut s = s.clone();
            s.invoke("write", &[resp_header.into()]);
            WasiChunkResponse::wrap_obj(ctx, WasiChunkResponse(s.into()))
        } else {
            JsValue::UnDefined
        }
    }
}

impl JsClassDef for HttpResponse {
    type RefType = HttpResponse;

    const CLASS_NAME: &'static str = "WasiResponse";
    const CONSTRUCTOR_ARGC: u8 = 0;

    const FIELDS: &'static [crate::JsClassField<Self::RefType>] = &[
        (
            "bodyLength",
            Self::js_get_body_length,
            Some(Self::js_set_body_length),
        ),
        ("headers", Self::js_get_headers, Some(Self::js_set_headers)),
        ("status", Self::js_get_status, Some(Self::js_set_status)),
        ("version", Self::js_get_version, Some(Self::js_set_version)),
        (
            "statusText",
            Self::js_get_status_text,
            Some(Self::js_set_status_text),
        ),
    ];

    const METHODS: &'static [crate::JsClassMethod<Self::RefType>] =
        &[("encode", 0, Self::js_encode), ("chunk", 1, Self::js_chunk)];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(_ctx: &mut Context, _argv: &[JsValue]) -> Result<HttpResponse, JsValue> {
        use super::core::request;
        use super::core::*;
        Ok(HttpResponse {
            version: Version::V1_0,
            status_code: 200,
            status_text: "OK".to_string(),
            headers: Default::default(),
            body_len: BodyLen::Length(0),
        })
    }
}

struct WasiChunkResponse(JsValue);

impl WasiChunkResponse {
    pub fn js_on(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        if let Some(v) = self.0.invoke("on", argv) {
            v
        } else {
            ctx.throw_internal_type_error("socket is shutdown").into()
        }
    }

    pub fn js_write(
        &mut self,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        if let JsValue::UnDefined = self.0 {
            return ctx.throw_internal_type_error("socket is shutdown").into();
        }
        match argv.get(0) {
            Some(JsValue::String(s)) => {
                let data = s.to_string();
                let data_len = data.len();
                self.0.invoke(
                    "write",
                    &[ctx
                        .new_string(format!("{:x}\r\n", data_len).as_str())
                        .into()],
                );
                self.0.invoke("write", &[s.clone().into()]);
                self.0.invoke("write", &[ctx.new_string("\r\n").into()]);
            }
            Some(JsValue::ArrayBuffer(buff)) => {
                let data = buff.as_ref();
                let data_len = data.len();
                self.0.invoke(
                    "write",
                    &[ctx
                        .new_string(format!("{:x}\r\n", data_len).as_str())
                        .into()],
                );
                self.0.invoke("write", &[buff.clone().into()]);
                self.0.invoke("write", &[ctx.new_string("\r\n").into()]);
            }
            Some(JsValue::Object(o)) => {
                let data = o.to_string();
                let data_len = data.len();
                self.0.invoke(
                    "write",
                    &[ctx
                        .new_string(format!("{:x}\r\n", data_len).as_str())
                        .into()],
                );
                self.0.invoke("write", &[o.clone().into()]);
                self.0.invoke("write", &[ctx.new_string("\r\n").into()]);
            }
            Some(JsValue::Symbol(s)) => {
                let data = format!("{:?}", s);
                let data_len = data.len();
                self.0.invoke(
                    "write",
                    &[ctx
                        .new_string(format!("{:x}\r\n", data_len).as_str())
                        .into()],
                );
                self.0.invoke("write", &[JsValue::Symbol(s.clone())]);
                self.0.invoke("write", &[ctx.new_string("\r\n").into()]);
            }
            _ => {}
        };
        JsValue::Bool(true)
    }

    pub fn js_end(
        &mut self,
        this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        if let JsValue::UnDefined = self.0 {
            return ctx.throw_internal_type_error("socket is shutdown").into();
        }
        let e = this_obj.invoke("write", argv);
        if e.is_exception() {
            return e;
        }

        self.0.invoke("end", &[ctx.new_string("0\r\n\r\n").into()]);
        // drop socket
        self.0 = JsValue::UnDefined;
        JsValue::Bool(true)
    }
}

impl JsClassDef for WasiChunkResponse {
    type RefType = WasiChunkResponse;

    const CLASS_NAME: &'static str = "ChunkResponse";
    const CONSTRUCTOR_ARGC: u8 = 0;

    const FIELDS: &'static [crate::JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [crate::JsClassMethod<Self::RefType>] = &[
        ("on", 2, Self::js_on),
        ("write", 1, Self::js_write),
        ("end", 1, Self::js_end),
    ];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(_ctx: &mut Context, _argv: &[JsValue]) -> Result<WasiChunkResponse, JsValue> {
        Err(JsValue::UnDefined)
    }

    fn gc_mark(data: &Self, make: &mut dyn Fn(&JsValue)) {
        make(&data.0)
    }
}

mod js_url {
    use std::ops::{Deref, DerefMut};

    use url::quirks::password;

    use crate::*;

    pub(super) struct URL(pub url::Url);

    impl Deref for URL {
        type Target = url::Url;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for URL {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl URL {
        pub fn js_to_string(
            &mut self,
            _this: &mut JsObject,
            ctx: &mut Context,
            _argv: &[JsValue],
        ) -> JsValue {
            ctx.new_string(format!("{}", self.0).as_str()).into()
        }

        pub fn js_get_href(&self, ctx: &mut Context) -> JsValue {
            ctx.new_string(format!("{}", self.0).as_str()).into()
        }

        pub fn js_get_scheme(&self, ctx: &mut Context) -> JsValue {
            ctx.new_string(self.scheme()).into()
        }

        pub fn js_get_username(&self, ctx: &mut Context) -> JsValue {
            ctx.new_string(self.username()).into()
        }

        pub fn js_get_password(&self, ctx: &mut Context) -> JsValue {
            let password = self.password().unwrap_or_default();
            ctx.new_string(password).into()
        }

        pub fn js_get_host(&self, ctx: &mut Context) -> JsValue {
            match self.host_str() {
                Some(host) => ctx.new_string(host).into(),
                None => JsValue::UnDefined,
            }
        }

        pub fn js_get_port(&self, _ctx: &mut Context) -> JsValue {
            match self.port_or_known_default() {
                Some(port) => JsValue::Int(port as i32),
                None => JsValue::UnDefined,
            }
        }

        pub fn js_get_path(&self, ctx: &mut Context) -> JsValue {
            ctx.new_string(self.path()).into()
        }

        pub fn js_get_query(&self, ctx: &mut Context) -> JsValue {
            match self.query() {
                Some(query) => ctx.new_string(query).into(),
                None => JsValue::UnDefined,
            }
        }
    }

    impl JsClassDef for URL {
        type RefType = Self;
        const CLASS_NAME: &'static str = "URL";
        const CONSTRUCTOR_ARGC: u8 = 1;

        const FIELDS: &'static [JsClassField<Self::RefType>] = &[
            ("href", Self::js_get_href, None),
            ("scheme", Self::js_get_scheme, None),
            ("username", Self::js_get_username, None),
            ("password", Self::js_get_password, None),
            ("host", Self::js_get_host, None),
            ("port", Self::js_get_port, None),
            ("path", Self::js_get_path, None),
            ("query", Self::js_get_query, None),
        ];
        const METHODS: &'static [JsClassMethod<Self::RefType>] =
            &[("toString", 0, Self::js_to_string)];

        unsafe fn mut_class_id_ptr() -> &'static mut u32 {
            static mut CLASS_ID: u32 = 0;
            &mut CLASS_ID
        }

        fn constructor_fn(ctx: &mut Context, argv: &[JsValue]) -> Result<URL, JsValue> {
            let input = argv.get(0);
            if let Some(JsValue::String(url_str)) = input {
                let u = url::Url::parse(url_str.as_str()).map_err(|e| {
                    JsValue::Exception(ctx.throw_internal_type_error(e.to_string().as_str()))
                })?;
                Ok(URL(u))
            } else {
                Err(JsValue::UnDefined)
            }
        }
    }
}
use js_url::URL;

struct HttpX;

impl ModuleInit for HttpX {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let class_ctor = register_class::<Buffer>(ctx);
        m.add_export(Buffer::CLASS_NAME, class_ctor);

        let class_ctor = register_class::<HttpRequest>(ctx);
        m.add_export(HttpRequest::CLASS_NAME, class_ctor);

        let class_ctor = register_class::<HttpResponse>(ctx);
        m.add_export(HttpResponse::CLASS_NAME, class_ctor);

        let class_ctor = register_class::<WasiChunkResponse>(ctx);
        m.add_export(WasiChunkResponse::CLASS_NAME, class_ctor);

        let class_ctor = register_class::<URL>(ctx);
        m.add_export(URL::CLASS_NAME, class_ctor);
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module(
        "wasi_http\0",
        HttpX,
        &[
            Buffer::CLASS_NAME,
            HttpRequest::CLASS_NAME,
            HttpResponse::CLASS_NAME,
            WasiChunkResponse::CLASS_NAME,
            URL::CLASS_NAME,
        ],
    )
}
