use std::io::Write;

use crate::event_loop::{AsyncTcpConn, AsyncTcpServer, PollResult};
use crate::*;

impl AsyncTcpConn {
    pub fn connect(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        use wasmedge_wasi_socket::ToSocketAddrs;
        let host = argv.get(0);
        let port = argv.get(1);
        let timeout = argv.get(2);
        let (p, ok, error) = ctx.new_promise();
        let event_loop = ctx.event_loop();
        if let (Some(JsValue::String(host)), Some(JsValue::Int(port)), Some(event_loop)) =
            (host, port, event_loop)
        {
            let timeout = if let Some(JsValue::Int(timeout)) = timeout {
                Some(std::time::Duration::from_millis((*timeout) as u64))
            } else {
                None
            };

            let host = host.to_string();

            let addr = (host.as_str(), *port as u16)
                .to_socket_addrs()
                .and_then(|mut it| {
                    it.next().ok_or(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("Unknown domain name {}", host),
                    ))
                });

            match addr {
                Ok(addr) => {
                    if let Err(e) = event_loop.tcp_connect(
                        &addr,
                        Box::new(move |ctx, event| match event {
                            PollResult::Connect(cs) => {
                                if let JsValue::Function(ok) = ok {
                                    let cs = AsyncTcpConn::wrap_obj(ctx, cs);
                                    ok.call(&[cs]);
                                }
                            }
                            PollResult::Error(e) => {
                                let err_msg = e.to_string();
                                let e = ctx.new_error(err_msg.as_str());
                                if let JsValue::Function(error) = error {
                                    error.call(&[e]);
                                }
                            }
                            PollResult::Timeout => {
                                let e = std::io::Error::from(std::io::ErrorKind::TimedOut);
                                let e = ctx.new_error(e.to_string().as_str());
                                if let JsValue::Function(error) = error {
                                    error.call(&[e]);
                                }
                            }
                            _ => {
                                let e = std::io::Error::from(std::io::ErrorKind::Unsupported);
                                let e = ctx.new_error(e.to_string().as_str());
                                if let JsValue::Function(error) = error {
                                    error.call(&[e]);
                                }
                            }
                        }),
                        timeout,
                    ) {
                        let e = ctx.throw_internal_type_error(e.to_string().as_str());
                        return e.into();
                    };
                }
                Err(e) => {
                    let e = ctx.throw_internal_type_error(e.to_string().as_str());
                    return e.into();
                }
            }
            p
        } else {
            JsValue::UnDefined
        }
    }

    pub fn on(
        _this_val: &mut AsyncTcpConn,
        _this_obj: &mut JsObject,
        _ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        JsValue::UnDefined
    }

    pub fn js_read(
        this_val: &mut AsyncTcpConn,
        this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let (p, ok, error) = ctx.new_promise();
        if let Some(event_poll) = ctx.event_loop() {
            let timeout = if let Some(JsValue::Int(timeout)) = argv.get(0) {
                Some(std::time::Duration::from_millis((*timeout) as u64))
            } else {
                None
            };

            let mut this_obj = JsValue::Object(this_obj.clone());
            this_val.async_read(
                event_poll,
                Box::new(move |ctx, event| match event {
                    PollResult::Read(_) => {
                        let this_val = Self::opaque_mut(&mut this_obj).unwrap();
                        let data = this_val.read_all();

                        match data {
                            Ok(data) => {
                                if let JsValue::Function(ok) = ok {
                                    let ret = if data.len() > 0 {
                                        let buff = ctx.new_array_buffer(data.as_slice());
                                        JsValue::ArrayBuffer(buff)
                                    } else {
                                        JsValue::UnDefined
                                    };
                                    ok.call(&[ret]);
                                }
                            }
                            Err(e) => {
                                let err_msg = e.to_string();
                                let e = ctx.new_error(err_msg.as_str());
                                if let JsValue::Function(error) = error {
                                    error.call(&[e]);
                                }
                            }
                        }
                    }
                    PollResult::Error(e) => {
                        let err_msg = e.to_string();
                        let e = ctx.new_error(err_msg.as_str());
                        if let JsValue::Function(error) = error {
                            error.call(&[e]);
                        }
                    }
                    PollResult::Timeout => {
                        let e = std::io::Error::from(std::io::ErrorKind::TimedOut);
                        let e = ctx.new_error(e.to_string().as_str());
                        if let JsValue::Function(error) = error {
                            error.call(&[e]);
                        }
                    }
                    _ => {
                        let e = std::io::Error::from(std::io::ErrorKind::Unsupported);
                        let e = ctx.new_error(e.to_string().as_str());
                        if let JsValue::Function(error) = error {
                            error.call(&[e]);
                        }
                    }
                }),
                timeout,
            );
            p
        } else {
            JsValue::UnDefined
        }
    }

    pub fn js_write(
        this_val: &mut AsyncTcpConn,
        _this_obj: &mut JsObject,
        _ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let data = argv.get(0);
        match data {
            Some(JsValue::String(s)) => {
                this_val.write(s.to_string().as_bytes());
            }
            Some(JsValue::ArrayBuffer(buff)) => {
                this_val.write(buff.as_ref());
            }
            Some(JsValue::Object(o)) => {
                this_val.write(o.to_string().as_bytes());
            }
            Some(JsValue::Symbol(s)) => {
                let data = format!("{:?}", s);
                this_val.write(data.as_bytes());
            }
            _ => {}
        };
        JsValue::Bool(true)
    }

    pub fn js_local(
        this_val: &mut AsyncTcpConn,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        match this_val.local() {
            Ok(addr) => ctx.new_string(addr.to_string().as_str()).into(),
            Err(e) => ctx.throw_internal_type_error(e.to_string().as_str()).into(),
        }
    }

    pub fn js_peer(
        this_val: &mut AsyncTcpConn,
        _this_obj: &mut JsObject,
        ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        match this_val.peer() {
            Ok(addr) => ctx.new_string(addr.to_string().as_str()).into(),
            Err(e) => ctx.throw_internal_type_error(e.to_string().as_str()).into(),
        }
    }
}

impl JsClassDef for AsyncTcpConn {
    type RefType = AsyncTcpConn;
    const CLASS_NAME: &'static str = "WasiTcpConn";
    const CONSTRUCTOR_ARGC: u8 = 0;

    const FIELDS: &'static [JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [JsClassMethod<Self::RefType>] = &[
        ("on", 1, Self::on),
        ("read", 0, Self::js_read),
        ("write", 1, Self::js_write),
        ("end", 1, Self::js_write),
        ("local", 0, Self::js_local),
        ("peer", 0, Self::js_peer),
    ];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(_ctx: &mut Context, _argv: &[JsValue]) -> Result<Self::RefType, JsValue> {
        Err(JsValue::Null)
    }
}

impl AsyncTcpServer {
    pub fn js_accept(
        &mut self,
        _this: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let timeout = if let Some(JsValue::Int(timeout)) = argv.get(0) {
            Some(std::time::Duration::from_millis((*timeout) as u64))
        } else {
            None
        };
        let (p, ok, error) = ctx.new_promise();
        if let Some(event_loop) = ctx.event_loop() {
            self.async_accept(
                event_loop,
                Box::new(move |ctx, r| match r {
                    PollResult::Accept(cs) => {
                        let cs = AsyncTcpConn::wrap_obj(ctx, cs);
                        if let JsValue::Function(ok) = ok {
                            ok.call(&[cs]);
                        }
                    }
                    PollResult::Error(e) => {
                        let err_msg = e.to_string();
                        let e = ctx.new_error(err_msg.as_str());
                        if let JsValue::Function(error) = error {
                            error.call(&[e]);
                        }
                    }
                    PollResult::Timeout => {
                        if let JsValue::Function(error) = error {
                            let e = std::io::Error::from(std::io::ErrorKind::TimedOut);
                            let e = ctx.new_error(e.to_string().as_str());
                            error.call(&[e]);
                        }
                    }
                    _ => {
                        if let JsValue::Function(error) = error {
                            let e = std::io::Error::from(std::io::ErrorKind::Unsupported);
                            let e = ctx.new_error(e.to_string().as_str());
                            error.call(&[e]);
                        }
                    }
                }),
                timeout,
            );
            p
        } else {
            JsValue::UnDefined
        }
    }
}

impl JsClassDef for AsyncTcpServer {
    const CLASS_NAME: &'static str = "WasiTcpServer";
    const CONSTRUCTOR_ARGC: u8 = 1;

    type RefType = AsyncTcpServer;

    const FIELDS: &'static [JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [JsClassMethod<Self::RefType>] = &[("accept", 0, Self::js_accept)];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(ctx: &mut Context, argv: &[JsValue]) -> Result<Self::RefType, JsValue> {
        let port = argv.get(0).ok_or_else(|| JsValue::UnDefined)?;
        if let (JsValue::Int(port), Some(event_loop)) = (port, ctx.event_loop()) {
            match event_loop.tcp_listen(*port as u16) {
                Ok(tcp_server) => Ok(tcp_server),
                Err(e) => Err(ctx.throw_internal_type_error(e.to_string().as_str()).into()),
            }
        } else {
            Err(JsValue::UnDefined)
        }
    }
}

fn js_nsloopup(ctx: &mut Context, _this: JsValue, param: &[JsValue]) -> JsValue {
    use wasmedge_wasi_socket::ToSocketAddrs;
    let node = param.get(0);
    let service = param.get(1);
    if let (Some(JsValue::String(node)), Some(JsValue::String(service))) = (node, service) {
        let r = format!("{}:{}", node.as_str(), service.as_str()).to_socket_addrs();
        match r {
            Ok(addr_vec) => {
                let mut array = ctx.new_array();
                for (i, addr) in addr_vec.enumerate() {
                    array.put(i, ctx.new_string(addr.to_string().as_str()).into());
                }
                array.into()
            }
            Err(e) => ctx.throw_internal_type_error(e.to_string().as_str()).into(),
        }
    } else {
        JsValue::UnDefined
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_fn_module(
        "wasi_net\0",
        &[
            AsyncTcpServer::CLASS_NAME,
            AsyncTcpConn::CLASS_NAME,
            "nsloopup",
        ],
        |ctx, m| {
            let class_ctor = register_class::<AsyncTcpServer>(ctx);
            m.add_export(AsyncTcpServer::CLASS_NAME, class_ctor);

            let mut class_ctor = register_class::<AsyncTcpConn>(ctx);
            if let JsValue::Function(tcp_conn_ctor) = &mut class_ctor {
                let conn = ctx.wrap_function("connect", AsyncTcpConn::connect);
                tcp_conn_ctor.set("connect", conn.into());
            }
            m.add_export(AsyncTcpConn::CLASS_NAME, class_ctor);

            let f = ctx.wrap_function("nsloopup", js_nsloopup);
            m.add_export("nsloopup", f.into());
        },
    )
}
