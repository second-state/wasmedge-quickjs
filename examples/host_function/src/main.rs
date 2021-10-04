mod host_extern {
    use quickjs_rs_wasi::{Context, JsValue};

    #[link(wasm_import_module = "extern")]
    extern "C" {
        pub fn host_inc(v: i32) -> i32;
    }

    pub struct HostIncFn;
    impl quickjs_rs_wasi::JsFn for HostIncFn {
        fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
            if let Some(JsValue::Int(i)) = argv.get(0) {
                unsafe {
                    let r = host_inc(*i);
                    r.into()
                }
            } else {
                ctx.throw_type_error("'v' is not a int").into()
            }
        }
    }
}

use quickjs_rs_wasi::*;

fn main() {
    let mut ctx = Context::new();
    let f = ctx.new_function::<host_extern::HostIncFn>("host_inc");
    ctx.get_global().set("host_inc", f.into());
    ctx.eval_global_str("print('js=> host_inc(2)=',host_inc(2))");
}
