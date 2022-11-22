use crate::quickjs_sys::*;
use crate::EventLoop;

fn isatty(_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    match argv.get(0) {
        Some(JsValue::Int(fd)) => (unsafe { libc::isatty(*fd) } == 1).into(),
        Some(JsValue::Float(fd)) => (unsafe { libc::isatty(*fd as i32) } == 1).into(),
        _ => JsValue::UnDefined
    }
}

struct TTY;

impl ModuleInit for TTY {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let f = ctx.wrap_function("isatty", isatty);
        m.add_export("isatty\0", f.into());
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module(
        "_node:tty\0", 
        TTY, 
        &[
            "isatty\0"
        ]
    )
}