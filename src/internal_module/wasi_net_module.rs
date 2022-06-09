use crate::event_loop::{AsyncTcpConn, AsyncTcpServer, PollResult};
use crate::*;

pub(crate) struct WasiTcpConn;

impl WasiTcpConn {
    pub fn connect(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let addr = argv.get(0);
        let timeout = argv.get(1);
        let (p, ok, error) = ctx.new_promise();
        let event_loop = ctx.event_loop();
        if let (Some(JsValue::String(addr)), Some(event_loop)) = (addr, event_loop) {
            let timeout = if let Some(JsValue::Int(timeout)) = timeout {
                Some(std::time::Duration::from_millis((*timeout) as u64))
            } else {
                None
            };

            let addr = addr.to_string().parse();
            match addr {
                Ok(addr) => {
                    if let Err(e) = event_loop.tcp_connect(
                        &addr,
                        Box::new(move |ctx, event| match event {
                            PollResult::Connect(cs) => {
                                if let JsValue::Function(ok) = ok {
                                    let cs = WasiTcpConn::gen_js_obj(ctx, cs);
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

    pub fn on(_this_val: &mut AsyncTcpConn, _ctx: &mut Context, _argv: &[JsValue]) -> JsValue {
        JsValue::UnDefined
    }

    pub fn read(this_val: &mut AsyncTcpConn, ctx: &mut Context, argv: &[JsValue]) -> JsValue {
        let (p, ok, error) = ctx.new_promise();
        if let Some(event_poll) = ctx.event_loop() {
            let timeout = if let Some(JsValue::Int(timeout)) = argv.get(0) {
                Some(std::time::Duration::from_millis((*timeout) as u64))
            } else {
                None
            };
            this_val.async_read(
                event_poll,
                Box::new(move |ctx, event| match event {
                    PollResult::Read(data) => {
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

    pub fn write(this_val: &mut AsyncTcpConn, _ctx: &mut Context, argv: &[JsValue]) -> JsValue {
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
            _ => {}
        };
        JsValue::Bool(true)
    }

    pub fn local(this_val: &mut AsyncTcpConn, ctx: &mut Context, _argv: &[JsValue]) -> JsValue {
        match this_val.local() {
            Ok(addr) => ctx.new_string(addr.to_string().as_str()).into(),
            Err(e) => ctx.throw_internal_type_error(e.to_string().as_str()).into(),
        }
    }

    pub fn peer(this_val: &mut AsyncTcpConn, ctx: &mut Context, _argv: &[JsValue]) -> JsValue {
        match this_val.peer() {
            Ok(addr) => ctx.new_string(addr.to_string().as_str()).into(),
            Err(e) => ctx.throw_internal_type_error(e.to_string().as_str()).into(),
        }
    }
}

impl JsClassDef<AsyncTcpConn> for WasiTcpConn {
    const CLASS_NAME: &'static str = "WasiTcpConn";
    const CONSTRUCTOR_ARGC: u8 = 0;

    fn constructor(_ctx: &mut Context, _argv: &[JsValue]) -> Result<AsyncTcpConn, JsValue> {
        Err(JsValue::Null)
    }

    fn proto_init(_ctx: &mut Context, p: &mut JsClassProto<AsyncTcpConn, Self>) {
        p.wrap_method("on".to_string(), 1, Self::on);
        p.wrap_method("read".to_string(), 0, Self::read);
        p.wrap_method("write".to_string(), 1, Self::write);
        p.wrap_method("end".to_string(), 1, Self::write);
        p.wrap_method("local".to_string(), 0, Self::local);
        p.wrap_method("peer".to_string(), 0, Self::peer);
    }
}

pub(crate) struct WasiTcpServer;
impl WasiTcpServer {
    pub fn accept(this_val: &mut AsyncTcpServer, ctx: &mut Context, argv: &[JsValue]) -> JsValue {
        let timeout = if let Some(JsValue::Int(timeout)) = argv.get(0) {
            Some(std::time::Duration::from_millis((*timeout) as u64))
        } else {
            None
        };
        let (p, ok, error) = ctx.new_promise();
        if let Some(event_loop) = ctx.event_loop() {
            this_val.async_accept(
                event_loop,
                Box::new(move |ctx, r| match r {
                    PollResult::Accept(cs) => {
                        let cs = WasiTcpConn::gen_js_obj(ctx, cs);
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

impl JsClassDef<AsyncTcpServer> for WasiTcpServer {
    const CLASS_NAME: &'static str = "WasiTcpServer";
    const CONSTRUCTOR_ARGC: u8 = 1;

    fn constructor(ctx: &mut Context, argv: &[JsValue]) -> Result<AsyncTcpServer, JsValue> {
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

    fn proto_init(_ctx: &mut Context, p: &mut JsClassProto<AsyncTcpServer, Self>) {
        p.wrap_method("accept".to_string(), 0, Self::accept);
    }
}

fn js_nsloopup(ctx: &mut Context, _this: JsValue, param: &[JsValue]) -> JsValue {
    let node = param.get(0);
    let service = param.get(1);
    if let (Some(JsValue::String(node)), Some(JsValue::String(service))) = (node, service) {
        let r = event_loop::nslookup(node.as_str(), service.as_str());
        match r {
            Ok(addr_vec) => {
                let mut array = ctx.new_array();
                for (i, addr) in addr_vec.iter().enumerate() {
                    array.set(i, ctx.new_string(addr.to_string().as_str()).into());
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
            WasiTcpServer::CLASS_NAME,
            WasiTcpConn::CLASS_NAME,
            "connect",
            "nsloopup",
        ],
        |ctx, m| {
            let conn = ctx.wrap_function("connect", WasiTcpConn::connect);
            m.add_export("connect", conn.into());

            let class_ctor = ctx.register_class(WasiTcpServer);
            m.add_export(WasiTcpServer::CLASS_NAME, class_ctor);

            let class_ctor = ctx.register_class(WasiTcpConn);
            m.add_export(WasiTcpConn::CLASS_NAME, class_ctor);

            let f = ctx.wrap_function("nsloopup", js_nsloopup);
            m.add_export("nsloopup\0", f.into());
        },
    )
}
