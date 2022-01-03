use crate::*;

struct TcpListenFn;

impl JsFn for TcpListenFn {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let port = argv.get(0);
        let callback_obj = argv.get(1);
        if let (Some(JsValue::Int(port)), Some(JsValue::Object(callback)), Some(event_loop)) =
            (port, callback_obj, ctx.event_loop())
        {
            if let Err(e) = event_loop.tcp_listen(*port as u16, callback.clone()) {
                ctx.throw_internal_type_error(e.to_string().as_str());
            };
        }
        JsValue::UnDefined
    }
}

struct WasiSock(i32);

impl JsClassDef<WasiSock> for WasiSock {
    const CLASS_NAME: &'static str = "WasiSock\0";
    const CONSTRUCTOR_ARGC: u8 = 1;

    fn constructor(_ctx: &mut Context, argv: &[JsValue]) -> Option<WasiSock> {
        let fd = argv.get(0)?;
        if let JsValue::Int(fd) = fd {
            Some(WasiSock(*fd))
        } else {
            None
        }
    }

    fn proto_init(p: &mut JsClassProto<WasiSock, Self>) {
        struct ON;
        impl JsMethod<WasiSock> for ON {
            const NAME: &'static str = "on\0";
            const LEN: u8 = 0;

            fn call(_ctx: &mut Context, _this_val: &mut WasiSock, _argv: &[JsValue]) -> JsValue {
                JsValue::UnDefined
            }
        }
        p.add_function(ON);

        struct WR;
        impl JsMethod<WasiSock> for WR {
            const NAME: &'static str = "write\0";
            const LEN: u8 = 1;

            fn call(ctx: &mut Context, this_val: &mut WasiSock, argv: &[JsValue]) -> JsValue {
                let data = argv.get(0);
                let fd = this_val.0;
                match (data, ctx.event_loop()) {
                    (Some(JsValue::String(s)), Some(event_loop)) => {
                        event_loop.write(fd, s.to_string().as_bytes())
                    }
                    (Some(JsValue::ArrayBuffer(buff)), Some(event_loop)) => {
                        event_loop.write(fd, buff.as_ref())
                    }
                    (Some(JsValue::Object(o)), Some(event_loop)) => {
                        event_loop.write(fd, o.to_string().as_bytes())
                    }
                    _ => None,
                };
                JsValue::Bool(true)
            }
        }
        p.add_function(WR);

        struct End;
        impl JsMethod<WasiSock> for End {
            const NAME: &'static str = "end\0";
            const LEN: u8 = 1;

            fn call(ctx: &mut Context, this_val: &mut WasiSock, argv: &[JsValue]) -> JsValue {
                let data = argv.get(0);
                let fd = this_val.0;
                match (data, ctx.event_loop()) {
                    (Some(JsValue::String(s)), Some(event_loop)) => {
                        event_loop.write(fd, s.to_string().as_bytes());
                        event_loop.close(fd);
                    }
                    (Some(JsValue::ArrayBuffer(buff)), Some(event_loop)) => {
                        event_loop.write(fd, buff.as_ref());
                        event_loop.close(fd);
                    }
                    (Some(JsValue::Object(o)), Some(event_loop)) => {
                        event_loop.write(fd, o.to_string().as_bytes());
                        event_loop.close(fd);
                    }
                    (_, Some(event_loop)) => {
                        event_loop.close(fd);
                    }
                    _ => {}
                };
                JsValue::UnDefined
            }
        }
        p.add_function(End);
    }
}

struct WasiNet;
impl ModuleInit for WasiNet {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        m.add_export(
            "tcp_listen\0",
            ctx.new_function::<TcpListenFn>("tcp_listen").into(),
        );

        let class_ctor = ctx.register_class(WasiSock(0));
        m.add_export(WasiSock::CLASS_NAME, class_ctor);
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module(
        "wasi_net\0",
        WasiNet,
        &["tcp_listen\0", WasiSock::CLASS_NAME],
    )
}
