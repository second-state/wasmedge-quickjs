use std::io::Write;

use crate::event_loop::{AsyncTcpConn, AsyncTcpServer};
use crate::*;

impl AsyncTcpConn {
    pub fn js_connect(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        use wasmedge_wasi_socket::ToSocketAddrs;
        let host = argv.get(0);
        let port = argv.get(1);
        let timeout = argv.get(2);

        let nctx = ctx.clone();

        if let (Some(JsValue::String(host)), Some(JsValue::Int(port))) = (host, port) {
            let timeout = if let Some(JsValue::Int(timeout)) = timeout {
                Some(std::time::Duration::from_millis((*timeout) as u64))
            } else {
                None
            };

            let host = host.to_string();
            let port = *port as u16;

            let pp = if let Some(duration) = timeout {
                ctx.future_to_promise(async move {
                    let mut ctx = nctx;
                    match tokio::time::timeout(duration, AsyncTcpConn::async_connect((host, port)))
                        .await
                    {
                        Ok(Ok(conn)) => Ok(Self::wrap_obj(&mut ctx, conn)),
                        Ok(Err(e)) => Err(ctx.new_error(e.to_string().as_str())),
                        Err(e) => {
                            let err =
                                std::io::Error::new(std::io::ErrorKind::TimedOut, e.to_string());
                            Err(ctx.new_error(err.to_string().as_str()).into())
                        }
                    }
                })
            } else {
                ctx.future_to_promise(async move {
                    let mut ctx = nctx;
                    match AsyncTcpConn::async_connect((host, port)).await {
                        Ok(conn) => Ok(Self::wrap_obj(&mut ctx, conn)),
                        Err(e) => Err(ctx.new_error(e.to_string().as_str())),
                    }
                })
            };
            pp
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
        _this_val: &mut AsyncTcpConn,
        this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let mut js_obj = this_obj.clone().into();
        let n_ctx = ctx.clone();
        if let Some(JsValue::Int(timeout)) = argv.get(0) {
            let duration = std::time::Duration::from_millis((*timeout) as u64);
            ctx.future_to_promise(async move {
                let mut ctx = n_ctx;
                let this_val = Self::opaque_mut(&mut js_obj).unwrap();
                match tokio::time::timeout(duration, this_val.async_read_all()).await {
                    Ok(Ok(data)) => {
                        if data.len() > 0 {
                            let buff = ctx.new_array_buffer(data.as_slice());
                            Ok(JsValue::ArrayBuffer(buff))
                        } else {
                            Ok(JsValue::UnDefined)
                        }
                    }
                    Ok(Err(err)) => Err(ctx.new_error(err.to_string().as_str()).into()),
                    Err(e) => {
                        let err = std::io::Error::new(std::io::ErrorKind::TimedOut, e.to_string());
                        Err(ctx.new_error(err.to_string().as_str()).into())
                    }
                }
            })
        } else {
            ctx.future_to_promise(async move {
                let mut ctx = n_ctx;
                let this_val = Self::opaque_mut(&mut js_obj).unwrap();
                match this_val.async_read_all().await {
                    Ok(data) => {
                        if data.len() > 0 {
                            let buff = ctx.new_array_buffer(data.as_slice());
                            log::trace!("async_read_all return ArrayBuffer");
                            Ok(JsValue::ArrayBuffer(buff))
                        } else {
                            Ok(JsValue::UnDefined)
                        }
                    }
                    Err(err) => Err(ctx.new_error(err.to_string().as_str()).into()),
                }
            })
        }
    }

    pub fn js_write(
        _this_val: &mut AsyncTcpConn,
        this_obj: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let mut js_obj = JsValue::Object(this_obj.clone());
        match argv.get(0) {
            Some(JsValue::String(s)) => {
                let data = s.to_string();
                ctx.future_to_promise(async move {
                    let this_val = AsyncTcpConn::opaque_mut(&mut js_obj).unwrap();
                    this_val.async_write_all(data.as_bytes()).await;
                    Ok(JsValue::UnDefined)
                });
            }
            Some(JsValue::ArrayBuffer(buff)) => {
                let data = buff.to_vec();
                ctx.future_to_promise(async move {
                    let this_val = AsyncTcpConn::opaque_mut(&mut js_obj).unwrap();
                    this_val.async_write_all(&data).await;
                    Ok(JsValue::UnDefined)
                });
            }
            Some(JsValue::Object(o)) => {
                let data = o.to_string();
                ctx.future_to_promise(async move {
                    let this_val = AsyncTcpConn::opaque_mut(&mut js_obj).unwrap();
                    this_val.async_write_all(data.as_bytes()).await;
                    Ok(JsValue::UnDefined)
                });
            }
            Some(JsValue::Symbol(s)) => {
                let data = format!("{:?}", s);
                ctx.future_to_promise(async move {
                    let this_val = AsyncTcpConn::opaque_mut(&mut js_obj).unwrap();
                    this_val.async_write_all(data.as_bytes()).await;
                    Ok(JsValue::UnDefined)
                });
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
        this: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let timeout = if let Some(JsValue::Int(timeout)) = argv.get(0) {
            Some(std::time::Duration::from_millis((*timeout) as u64))
        } else {
            None
        };
        let n_ctx = ctx.clone();
        let mut js_obj = this.clone().into();
        ctx.future_to_promise(async move {
            let this = Self::opaque_mut(&mut js_obj).unwrap();
            let mut ctx = n_ctx;
            this.accept(&mut ctx, timeout).await
        })
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
        if let JsValue::Int(port) = port {
            match Self::bind(*port as u16) {
                Ok(tcp_server) => Ok(tcp_server),
                Err(e) => {
                    log::trace!("tcp_listen err: {e}");
                    Err(ctx.throw_internal_type_error(e.to_string().as_str()).into())
                }
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
                let conn = ctx.wrap_function("connect", AsyncTcpConn::js_connect);
                tcp_conn_ctor.set("connect", conn.into());
            }
            m.add_export(AsyncTcpConn::CLASS_NAME, class_ctor);

            let f = ctx.wrap_function("nsloopup", js_nsloopup);
            m.add_export("nsloopup", f.into());
        },
    )
}
