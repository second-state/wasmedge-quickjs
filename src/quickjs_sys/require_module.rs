use super::*;
pub(super) fn init_module_require(ctx: &mut Context) {
    unsafe {
        let ctx = ctx.ctx;
        let init_js = include_str!("../../js_lib/require.js");
        let mut val = JS_Eval(
            ctx,
            make_c_string(init_js).as_ptr(),
            init_js.len(),
            make_c_string("require").as_ptr() as *const i8,
            JS_EVAL_TYPE_MODULE as i32,
        );
        if JS_IsException_real(val) > 0 {
            js_std_dump_error(ctx);
        }
        JS_FreeValue_real(ctx, val);
    }
}
