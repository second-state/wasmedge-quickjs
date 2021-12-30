use crate::quickjs_sys::*;
use crate::EventLoop;

struct SetTimeout;
impl JsFn for SetTimeout {
    fn call(_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let callback = argv.get(0);
        let timeout = argv.get(1);
        if let (Some(JsValue::Function(callback)), Some(JsValue::Int(timeout))) =
            (callback, timeout)
        {
            let event_loop = EventLoop::inst();
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
    fn call(_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let callback = argv.get(0);
        if let Some(JsValue::Function(callback)) = callback {
            let event_loop = EventLoop::inst();
            event_loop.set_next_tick(callback.clone());
        }
        JsValue::UnDefined
    }
}

struct ClearTimeout;
impl JsFn for ClearTimeout {
    fn call(_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let timeout_id = argv.get(0);
        if let Some(JsValue::Int(timeout_id)) = timeout_id {
            let event_loop = EventLoop::inst();
            event_loop.clear_timeout((*timeout_id) as usize);
        }
        JsValue::UnDefined
    }
}

pub fn init_event_loop(ctx: &mut Context) {
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
    fn call(_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        if let Some(JsValue::Function(callback)) = argv.get(0) {
            let event_loop = EventLoop::inst();
            event_loop.set_next_tick(callback.clone());
        }
        JsValue::UnDefined
    }
}

struct Process;
impl ModuleInit for Process {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let next_tick = ctx.new_function::<NextTick>("nextTick");
        m.add_export("nextTick\0", next_tick.into());
        let default = ctx.new_object();
        m.add_export("default\0", default.into());
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module("process\0", Process, &["nextTick\0", "default\0"])
}
