use core::arch;

use crate::quickjs_sys::*;
use crate::EventLoop;

fn memory_size(_ctx: &mut Context, _this_val: JsValue, _argv: &[JsValue]) -> JsValue {
    JsValue::Int(arch::wasm32::memory_size::<0>() as i32)
}

struct OS;

impl ModuleInit for OS {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let f = ctx.wrap_function("_memorySize", memory_size);
        m.add_export("_memorySize\0", f.into());
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module(
        "_node:os\0", 
        OS, 
        &[
            "_memorySize\0"
        ]
    )
}
