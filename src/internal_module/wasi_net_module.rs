use crate::event_loop::{AsyncTcpConn, AsyncTcpServer, PollResult};
use crate::*;

pub(crate) struct WasiTcpConn;

impl JsClassDef<AsyncTcpConn> for WasiTcpConn {
    const CLASS_NAME: &'static str = "WasiTcpConn\0";
    const CONSTRUCTOR_ARGC: u8 = 1;

    fn constructor(_ctx: &mut Context, _argv: &[JsValue]) -> Result<AsyncTcpConn, JsValue> {
        Err(JsValue::Null)
    }

    fn proto_init(p: &mut JsClassProto<AsyncTcpConn, Self>) {
        struct ON;
        impl JsMethod<AsyncTcpConn> for ON {
            const NAME: &'static str = "on\0";
            const LEN: u8 = 0;

            fn call(
                _ctx: &mut Context,
                _this_val: &mut AsyncTcpConn,
                _argv: &[JsValue],
            ) -> JsValue {
                JsValue::UnDefined
            }
        }
        p.add_function::<ON>();

        struct RD;
        impl JsMethod<AsyncTcpConn> for RD {
            const NAME: &'static str = "read\0";
            const LEN: u8 = 0;

            fn call(ctx: &mut Context, this_val: &mut AsyncTcpConn, argv: &[JsValue]) -> JsValue {
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
        }
        p.add_function::<RD>();

        struct WR;
        impl JsMethod<AsyncTcpConn> for WR {
            const NAME: &'static str = "write\0";
            const LEN: u8 = 1;

            fn call(_ctx: &mut Context, this_val: &mut AsyncTcpConn, argv: &[JsValue]) -> JsValue {
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
        }
        p.add_function::<WR>();

        struct End;
        impl JsMethod<AsyncTcpConn> for End {
            const NAME: &'static str = "end\0";
            const LEN: u8 = 1;

            fn call(_ctx: &mut Context, this_val: &mut AsyncTcpConn, argv: &[JsValue]) -> JsValue {
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
        }
        p.add_function::<End>();

        struct Local;
        impl JsMethod<AsyncTcpConn> for Local {
            const NAME: &'static str = "local\0";
            const LEN: u8 = 1;

            fn call(ctx: &mut Context, this_val: &mut AsyncTcpConn, _argv: &[JsValue]) -> JsValue {
                match this_val.local() {
                    Ok(addr) => ctx.new_string(addr.to_string().as_str()).into(),
                    Err(e) => ctx.throw_internal_type_error(e.to_string().as_str()).into(),
                }
            }
        }
        p.add_function::<Local>();

        struct Peer;
        impl JsMethod<AsyncTcpConn> for Peer {
            const NAME: &'static str = "peer\0";
            const LEN: u8 = 1;

            fn call(ctx: &mut Context, this_val: &mut AsyncTcpConn, _argv: &[JsValue]) -> JsValue {
                match this_val.peer() {
                    Ok(addr) => ctx.new_string(addr.to_string().as_str()).into(),
                    Err(e) => ctx.throw_internal_type_error(e.to_string().as_str()).into(),
                }
            }
        }
        p.add_function::<Peer>();
    }
}

struct TcpConnect;

impl JsFn for TcpConnect {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let addr = argv.get(0);
        let (p, ok, error) = ctx.new_promise();
        let event_loop = ctx.event_loop();
        if let (Some(JsValue::String(addr)), Some(event_loop)) = (addr, event_loop) {
            let addr = addr.to_string().parse();
            match addr {
                Ok(addr) => {
                    let timeout = if let Some(JsValue::Int(timeout)) = argv.get(1) {
                        Some(std::time::Duration::from_millis((*timeout) as u64))
                    } else {
                        None
                    };

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
                        println!("{:?}", e);
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
}

pub(crate) struct WasiTcpServer;
impl JsClassDef<AsyncTcpServer> for WasiTcpServer {
    const CLASS_NAME: &'static str = "WasiTcpServer\0";
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

    fn proto_init(p: &mut JsClassProto<AsyncTcpServer, Self>) {
        struct Accept;
        impl JsMethod<AsyncTcpServer> for Accept {
            const NAME: &'static str = "accept\0";
            const LEN: u8 = 0;

            fn call(ctx: &mut Context, this_val: &mut AsyncTcpServer, argv: &[JsValue]) -> JsValue {
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
        p.add_function::<Accept>();
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
                for (i,addr) in addr_vec.iter().enumerate(){
                    array.set(i, ctx.new_string(addr.to_string().as_str()).into());
                }
                array.into()
            }
            Err(e) => {
                ctx.throw_internal_type_error(e.to_string().as_str()).into()
            }
        }
    } else {
        JsValue::UnDefined
    }
}

struct WasiNet;
impl ModuleInit for WasiNet {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let conn = ctx.new_function::<TcpConnect>("connect");
        m.add_export("connect\0", conn.into());

        let class_ctor = ctx.register_class(WasiTcpServer);
        m.add_export(WasiTcpServer::CLASS_NAME, class_ctor);

        let class_ctor = ctx.register_class(WasiTcpConn);
        m.add_export(WasiTcpConn::CLASS_NAME, class_ctor);

        let f = ctx.wrap_function("nsloopup", js_nsloopup);
        m.add_export("nsloopup\0", f.into());
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module(
        "wasi_net\0",
        WasiNet,
        &[
            WasiTcpServer::CLASS_NAME,
            WasiTcpConn::CLASS_NAME,
            "connect\0",
            "nsloopup\0"
        ],
    )
}
