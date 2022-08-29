use crate::*;
use std::path::Path;
use crate::quickjs_sys::*;

fn js_check_prime(
    ctx: &mut Context, 
    _this_val: JsValue,
    argv: &[JsValue]
) -> JsValue {
    
    let n = if let Some(JsValue::Int(s)) = argv.get(0) {
        *s
    } else {
        return ctx.throw_type_error("parameter must be of type integer").into();
    };
    if n <= 1 {
        return JsValue::Bool(false);
    }
    let limit = (n as f64).sqrt() as i32;
    for a in 2..limit {
        if n % a == 0 {
            return JsValue::Bool(false);
        }
    }
    JsValue::Bool(true)
}




struct Crypto;
impl ModuleInit for Crypto{
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let f = ctx.wrap_function("check_prime", js_check_prime);
        m.add_export("check_prime\0", f.into());
    }

}
pub fn init_module_crypto(ctx: &mut Context) {
    ctx.register_module(
        "crypto\0",
        Crypto,
        &[
            "check_prime\0"
        ],    
    )
}


