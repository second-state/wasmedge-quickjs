use crate::quickjs_sys::*;
use crate::EventLoop;
use std::string::FromUtf8Error;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct TimeoutId(Arc<tokio::sync::Notify>);
impl TimeoutId {
    pub fn new() -> Self {
        TimeoutId(Arc::new(tokio::sync::Notify::new()))
    }
}

impl JsClassDef for TimeoutId {
    type RefType = TimeoutId;

    const CLASS_NAME: &'static str = "TimeoutId";

    const CONSTRUCTOR_ARGC: u8 = 0;

    const FIELDS: &'static [JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [JsClassMethod<Self::RefType>] = &[];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(_ctx: &mut Context, _argv: &[JsValue]) -> Result<Self::RefType, JsValue> {
        Err(JsValue::UnDefined)
    }
}

fn set_timeout(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let callback = argv.get(0);
    let timeout = argv.get(1);
    let rest_args = argv.get(2..).map(|args| args.to_vec());
    if let (Some(JsValue::Function(callback)), Some(JsValue::Int(timeout))) = (callback, timeout) {
        let timeout = *timeout as u64;
        let callback = callback.clone();

        if timeout == 0 {
            ctx.event_loop().map(|event_loop| {
                event_loop.add_immediate_task(Box::new(move || {
                    if let Some(rest_args) = rest_args {
                        callback.call(&rest_args);
                    } else {
                        callback.call(&[]);
                    }
                }))
            });
            JsValue::UnDefined
        } else {
            let timeout = std::time::Duration::from_millis(timeout);

            let id = TimeoutId::new();
            let id_ = id.clone();

            ctx.future_to_promise(async move {
                tokio::select! {
                    _v=tokio::time::sleep(timeout) => {
                        if let Some(rest_args) = rest_args {
                            callback.call(&rest_args);
                        } else {
                            callback.call(&[]);
                        };
                        Ok(JsValue::UnDefined)
                    }
                    _cancel=id_.0.notified() => {
                        log::trace!("timer cancel");
                        Err(JsValue::UnDefined)
                    }
                }
            });
            TimeoutId::wrap_obj(ctx, id)
        }
    } else {
        JsValue::UnDefined
    }
}

fn set_immediate(ctx: &mut Context, this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let callback = argv.get(0);
    let rest_args = argv.get(1..).map(|v| v.to_vec());
    if let Some(JsValue::Function(callback)) = callback {
        let callback = callback.clone();
        let mut argv = vec![JsValue::Function(callback), JsValue::Int(0)];
        argv.extend(rest_args.unwrap_or_default());
        set_timeout(ctx, this_val, &argv)
    } else {
        JsValue::UnDefined
    }
}

fn next_tick(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let callback = argv.get(0);
    let args = argv.get(1..).map(|v| v.to_vec());
    if let (Some(JsValue::Function(callback)), Some(event_loop)) = (callback, ctx.event_loop()) {
        let callback = callback.clone();
        event_loop.set_next_tick(Box::new(move || {
            match args {
                Some(args) => callback.call(&args),
                None => callback.call(&[]),
            };
        }));
    }
    JsValue::UnDefined
}

fn sleep(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let callback = argv.get(0).cloned();
    let timeout = argv.get(1);
    let rest_args = argv.get(2..).map(|args| args.to_vec());
    if let (Some(JsValue::Function(callback)), Some(JsValue::Int(timeout))) = (callback, timeout) {
        let timeout = *timeout;
        ctx.future_to_promise(async move {
            tokio::time::sleep(std::time::Duration::from_millis(timeout as u64)).await;
            if let Some(rest_args) = rest_args {
                callback.call(&rest_args);
            } else {
                callback.call(&[]);
            };

            Ok(JsValue::UnDefined)
        })
    } else {
        JsValue::UnDefined
    }
}

fn os_exit(_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let code = if let Some(JsValue::Int(c)) = argv.get(0) {
        *c
    } else {
        0
    };

    std::process::exit(code)
}

fn clear_timeout(_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let timeout_id = argv.get(0);
    if let Some(timeout_id) = timeout_id {
        let id = TimeoutId::opaque(&timeout_id);
        if let Some(id) = id {
            id.0.notify_one()
        }
    }
    JsValue::UnDefined
}

pub fn init_ext_function(_ctx: &mut Context) {}

pub fn init_global_function(ctx: &mut Context) {
    register_class::<TimeoutId>(ctx);

    let mut global = ctx.get_global();
    global.set(
        "clearTimeout",
        ctx.wrap_function("clearTimeout", clear_timeout).into(),
    );
    global.set(
        "setTimeout",
        ctx.wrap_function("setTimeout", set_timeout).into(),
    );
    global.set(
        "setImmediate",
        ctx.wrap_function("setImmediate", set_immediate).into(),
    );
    global.set("sleep", ctx.wrap_function("sleep", sleep).into());
    global.set("nextTick", ctx.wrap_function("nextTick", next_tick).into());
    global.set("exit", ctx.wrap_function("exit", os_exit).into());
    global.set("env", env_object(ctx).into());
}

fn env_object(ctx: &mut Context) -> JsObject {
    let mut env_obj = ctx.new_object();
    let env = std::env::vars();
    for (k, v) in env {
        env_obj.set(&k, ctx.new_string(&v).into());
    }
    env_obj
}
