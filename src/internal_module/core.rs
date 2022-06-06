use crate::quickjs_sys::*;
use crate::EventLoop;
use std::string::FromUtf8Error;

fn set_timeout(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let callback = argv.get(0);
    let timeout = argv.get(1);
    let rest_args = argv.get(2..).map(|args| args.to_vec());
    if let (Some(JsValue::Function(callback)), Some(JsValue::Int(timeout)), Some(event_loop)) =
        (callback, timeout, ctx.event_loop())
    {
        let n = event_loop.set_timeout(
            callback.clone(),
            std::time::Duration::from_millis((*timeout) as u64),
            rest_args,
        );
        JsValue::Int(n as i32)
    } else {
        JsValue::UnDefined
    }
}

fn set_immediate(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let callback = argv.get(0);
    let args = argv.get(1..).map(|v| v.to_vec());
    if let (Some(JsValue::Function(callback)), Some(event_loop)) = (callback, ctx.event_loop())
    {
        event_loop.set_next_tick(callback.clone(), args);
    }
    JsValue::UnDefined
}

struct ClearTimeout;
impl JsFn for ClearTimeout {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let timeout_id = argv.get(0);
        if let (Some(JsValue::Int(timeout_id)), Some(event_loop)) = (timeout_id, ctx.event_loop()) {
            event_loop.clear_timeout((*timeout_id) as usize);
        }
        JsValue::UnDefined
    }
}

pub fn init_ext_function(_ctx: &mut Context) {}

pub fn init_global_function(ctx: &mut Context) {
    let mut global = ctx.get_global();
    global.set(
        "clearTimeout",
        ctx.new_function::<ClearTimeout>("clearTimeout").into(),
    );
    global.set(
        "setTimeout",
        ctx.wrap_function("setTimeout", set_timeout).into(),
    );
    global.set(
        "setImmediate",
        ctx.wrap_function("setImmediate",set_immediate).into(),
    );
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
