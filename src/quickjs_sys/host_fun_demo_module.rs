use super::*;

mod host_extern {
    #[link(wasm_import_module = "extern")]
    extern "C" {
        pub fn host_inc(v: i32) -> i32;
    }
}

unsafe extern "C" fn bind_host_inc(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    if argv.is_null() || argc < 1 {
        return js_throw_type_error(ctx, "too few arguments to function ‘host_inc’");
    }
    let mut v = 0;
    if JS_ToInt32(ctx, &mut v, *argv.offset(0)) > 0 {
        return js_exception();
    }
    JS_NewInt32_real(ctx, host_extern::host_inc(v))
}

unsafe extern "C" fn js_module_init(
    ctx: *mut JSContext,
    m: *mut JSModuleDef,
) -> ::std::os::raw::c_int {
    JS_SetModuleExport(
        ctx,
        m,
        make_c_string("host_inc").as_ptr(),
        JS_NewCFunction_real(ctx, Some(bind_host_inc), "host_inc\0".as_ptr().cast(), 1),
    );
    0
}

pub unsafe fn init_module(ctx: *mut JSContext) -> *mut JSModuleDef {
    let name = make_c_string("host_function_demo");
    let m = JS_NewCModule(ctx, name.as_ptr(), Some(js_module_init));
    if m.is_null() {
        return m;
    }
    JS_AddModuleExport(ctx, m, make_c_string("host_inc").as_ptr());
    return m;
}
