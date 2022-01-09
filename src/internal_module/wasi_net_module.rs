use crate::event_loop::{AsyncTcpConn, AsyncTcpServer, NetPollResult};
use crate::*;

struct WasiTcpConn;

impl JsClassDef<AsyncTcpConn> for WasiTcpConn {
    const CLASS_NAME: &'static str = "WasiTcpConn\0";
    const CONSTRUCTOR_ARGC: u8 = 1;

    fn constructor(_ctx: &mut Context, _argv: &[JsValue]) -> Option<AsyncTcpConn> {
        None
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
        p.add_function(ON);

        struct RD;
        impl JsMethod<AsyncTcpConn> for RD {
            const NAME: &'static str = "read\0";
            const LEN: u8 = 0;

            fn call(ctx: &mut Context, this_val: &mut AsyncTcpConn, _argv: &[JsValue]) -> JsValue {
                let (p, ok, error) = ctx.new_promise();
                if let Some(event_poll) = ctx.event_loop() {
                    this_val.async_read(
                        event_poll,
                        Box::new(move |ctx, event| match event {
                            NetPollResult::Read(data) => {
                                let buff = ctx.new_array_buffer(data.as_slice());
                                if let JsValue::Function(ok) = ok {
                                    ok.call(&mut [JsValue::ArrayBuffer(buff)]);
                                }
                            }
                            NetPollResult::Error(e) => {
                                let err_msg = e.to_string();
                                let e = ctx.throw_internal_type_error(err_msg.as_str());
                                if let JsValue::Function(error) = error {
                                    error.call(&mut [JsValue::Exception(e)]);
                                }
                            }
                            _ => {
                                let e = std::io::Error::from(std::io::ErrorKind::Unsupported);
                                let e = ctx.throw_internal_type_error(e.to_string().as_str());
                                if let JsValue::Function(error) = error {
                                    error.call(&mut [JsValue::Exception(e)]);
                                }
                            }
                        }),
                    );
                    p
                } else {
                    JsValue::UnDefined
                }
            }
        }
        p.add_function(RD);

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
        p.add_function(WR);

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
        p.add_function(End);
    }
}

struct WasiTcpServer;
impl JsClassDef<AsyncTcpServer> for WasiTcpServer {
    const CLASS_NAME: &'static str = "WasiTcpServer\0";
    const CONSTRUCTOR_ARGC: u8 = 1;

    fn constructor(ctx: &mut Context, argv: &[JsValue]) -> Option<AsyncTcpServer> {
        let port = argv.get(0)?;
        if let (JsValue::Int(port), Some(event_loop)) = (port, ctx.event_loop()) {
            match event_loop.tcp_listen(*port as u16) {
                Ok(tcp_server) => Some(tcp_server),
                Err(e) => {
                    ctx.throw_internal_type_error(e.to_string().as_str());
                    None
                }
            }
        } else {
            None
        }
    }

    fn proto_init(p: &mut JsClassProto<AsyncTcpServer, Self>) {
        struct Accept;
        impl JsMethod<AsyncTcpServer> for Accept {
            const NAME: &'static str = "accept\0";
            const LEN: u8 = 0;

            fn call(
                ctx: &mut Context,
                this_val: &mut AsyncTcpServer,
                _argv: &[JsValue],
            ) -> JsValue {
                let (p, ok, error) = ctx.new_promise();
                if let Some(event_loop) = ctx.event_loop() {
                    event_loop.tcp_accept(
                        this_val,
                        Box::new(move |ctx, r| match r {
                            NetPollResult::Accept(cs) => {
                                let cs = WasiTcpConn::gen_js_obj(ctx, cs);
                                if let JsValue::Function(ok) = ok {
                                    ok.call(&mut [cs]);
                                }
                            }
                            NetPollResult::Error(e) => {
                                let err_msg = e.to_string();
                                let e = ctx.throw_internal_type_error(err_msg.as_str());
                                if let JsValue::Function(error) = error {
                                    error.call(&mut [JsValue::Exception(e)]);
                                }
                            }
                            _ => {
                                let e = std::io::Error::from(std::io::ErrorKind::Unsupported);
                                let e = ctx.throw_internal_type_error(e.to_string().as_str());
                                if let JsValue::Function(error) = error {
                                    error.call(&mut [JsValue::Exception(e)]);
                                }
                            }
                        }),
                    );
                    p
                } else {
                    JsValue::UnDefined
                }
            }
        }
        p.add_function(Accept);
    }
}

struct WasiNet;
impl ModuleInit for WasiNet {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let class_ctor = ctx.register_class(WasiTcpServer);
        m.add_export(WasiTcpServer::CLASS_NAME, class_ctor);

        let class_ctor = ctx.register_class(WasiTcpConn);
        m.add_export(WasiTcpConn::CLASS_NAME, class_ctor);
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module(
        "wasi_net\0",
        WasiNet,
        &[WasiTcpServer::CLASS_NAME, WasiTcpConn::CLASS_NAME],
    )
}
