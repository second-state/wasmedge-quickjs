use crate::quickjs_sys::*;
use crate::EventLoop;
use std::string::FromUtf8Error;

struct SetTimeout;
impl JsFn for SetTimeout {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let callback = argv.get(0);
        let timeout = argv.get(1);
        if let (Some(JsValue::Function(callback)), Some(JsValue::Int(timeout)), Some(event_loop)) =
            (callback, timeout, ctx.event_loop())
        {
            let n = event_loop.set_timeout(
                callback.clone(),
                std::time::Duration::from_millis((*timeout) as u64),
            );
            JsValue::Int(n as i32)
        } else {
            JsValue::UnDefined
        }
    }
}

struct SetImmediate;
impl JsFn for SetImmediate {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let callback = argv.get(0);
        if let (Some(JsValue::Function(callback)), Some(event_loop)) = (callback, ctx.event_loop())
        {
            event_loop.set_next_tick(callback.clone());
        }
        JsValue::UnDefined
    }
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

struct NewStringFromUTF8;
impl JsFn for NewStringFromUTF8 {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let obj = argv.get(0);
        match obj {
            Some(JsValue::ArrayBuffer(data)) => {
                let s = String::from_utf8(data.to_vec());
                match s {
                    Ok(s) => ctx.new_string(&s).into(),
                    Err(e) => ctx.throw_type_error(e.to_string().as_str()).into(),
                }
            }
            Some(obj) => ctx.value_to_string(obj),
            None => JsValue::UnDefined,
        }
    }
}

pub fn init_ext_function(ctx: &mut Context) {
    let mut global = ctx.get_global();
    global.set(
        "newStringFromUTF8",
        ctx.new_function::<NewStringFromUTF8>("newStringFromUTF8")
            .into(),
    );
}

pub fn init_event_function(ctx: &mut Context) {
    let mut global = ctx.get_global();
    global.set(
        "clearTimeout",
        ctx.new_function::<ClearTimeout>("clearTimeout").into(),
    );
    global.set(
        "setTimeout",
        ctx.new_function::<SetTimeout>("setTimeout").into(),
    );
    global.set(
        "setImmediate",
        ctx.new_function::<SetImmediate>("setImmediate").into(),
    );
}

struct NextTick;

impl JsFn for NextTick {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        if let (Some(JsValue::Function(callback)), Some(event_loop)) =
            (argv.get(0), ctx.event_loop())
        {
            event_loop.set_next_tick(callback.clone());
        }
        JsValue::UnDefined
    }
}

fn env_object(ctx: &mut Context) -> JsObject {
    let mut env_obj = ctx.new_object();
    let env = std::env::vars();
    for (k, v) in env {
        env_obj.set(&k, ctx.new_string(&v).into());
    }
    env_obj
}

struct Process;
impl ModuleInit for Process {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let next_tick = ctx.new_function::<NextTick>("nextTick");
        m.add_export("nextTick\0", next_tick.into());
        let default = ctx.new_object();
        m.add_export("default\0", default.into());
        let env_obj = env_object(ctx);
        m.add_export("env\0", env_obj.into());
    }
}

pub fn init_process_module(ctx: &mut Context) {
    ctx.register_module("process\0", Process, &["nextTick\0", "default\0", "env\0"])
}

pub fn init_internal_js_module(ctx: &mut Context) {
    let internal_dir = std::env::var("QJS_LIB").unwrap_or("./internal".to_string());

    if let Ok(dir) = std::fs::read_dir(internal_dir) {
        for f in dir {
            if let Ok(f) = f {

                let filename = f.file_name();
                let filename = filename.to_str();

                let path = f.path();
                let path_extension = path.extension();

                if path.exists() && path_extension.is_some() && path_extension.unwrap() == "js" {

                    let fs = std::fs::read_to_string(&path);
                    if let (Some(filename), Ok(code)) = (filename, fs) {

                        let filename = filename.trim_end_matches(".js");
                        ctx.eval_module_str(code.as_str(), filename);
                    }
                }
            }
        }
    };
}
