use super::qjs as q;
use crate::quickjs_sys::qjs::{JSContext, JSModuleDef};
use crate::{Context, JsValue};
use std::marker::PhantomData;
use std::ops::DerefMut;

pub struct JsModuleDef {
    ctx: *mut q::JSContext,
    m: *mut q::JSModuleDef,
}

impl JsModuleDef {
    pub fn add_export<S: ToString>(&mut self, name: S, val: JsValue) {
        unsafe {
            let mut name = name.to_string();
            if !name.ends_with('\0') {
                name.push('\0')
            }
            let v = val.into_qjs_value();
            q::JS_SetModuleExport(self.ctx, self.m, name.as_ptr().cast(), v);
        }
    }
}

pub trait ModuleInit {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef);
}

struct ModuleInitFnTrampoline<F: ModuleInit> {
    _f: PhantomData<F>,
}

impl<F: ModuleInit> ModuleInitFnTrampoline<F> {
    unsafe extern "C" fn init_module(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
    ) -> ::std::os::raw::c_int {
        let mut m = JsModuleDef { ctx, m };
        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });
        let nctx = n_ctx.deref_mut();
        F::init_module(nctx, &mut m);
        0
    }
}

struct FnModuleInitFnTrampoline<F: Fn(&mut Context, &mut JsModuleDef)> {
    _f: PhantomData<F>,
}

impl<F: Fn(&mut Context, &mut JsModuleDef)> FnModuleInitFnTrampoline<F> {
    unsafe extern "C" fn init_module(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
    ) -> ::std::os::raw::c_int {
        let mut m = JsModuleDef { ctx, m };
        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });
        let nctx = n_ctx.deref_mut();
        let init_module = std::mem::zeroed::<F>();
        init_module(nctx, &mut m);
        0
    }
}

fn register_fn_module<F: Fn(&mut Context, &mut JsModuleDef), S: ToString>(
    ctx: &mut Context,
    name: S,
    exports: &[&str],
    _f: F,
) {
    unsafe {
        let mut name = name.to_string();
        if !name.ends_with('\0') {
            name.push('\0');
        }

        let ctx = ctx.ctx;
        let js_module_init = FnModuleInitFnTrampoline::<F>::init_module;
        let m = q::JS_NewCModule(ctx, name.as_ptr().cast(), Some(js_module_init));

        let mut export_string = String::new();

        for s in exports {
            export_string.clear();
            export_string.push_str(*s);
            if !export_string.ends_with('\0') {
                export_string.push('\0');
            }
            q::JS_AddModuleExport(ctx, m, export_string.as_ptr().cast());
        }
    }
}

fn register_module<F: ModuleInit, S: ToString>(ctx: &mut Context, name: S, exports: &[&str]) {
    unsafe {
        let mut name = name.to_string();
        if !name.ends_with('\0') {
            name.push('\0');
        }

        let ctx = ctx.ctx;
        let js_module_init = ModuleInitFnTrampoline::<F>::init_module;
        let m = q::JS_NewCModule(ctx, name.as_ptr().cast(), Some(js_module_init));

        let mut export_string = String::new();

        for s in exports {
            export_string.clear();
            export_string.push_str(*s);
            if !export_string.ends_with('\0') {
                export_string.push('\0');
            }
            q::JS_AddModuleExport(ctx, m, export_string.as_ptr().cast());
        }
    }
}

impl Context {
    pub fn register_module<T: ModuleInit, S: ToString>(&mut self, name: S, _: T, exports: &[&str]) {
        register_module::<T, S>(self, name, exports)
    }

    pub fn register_fn_module<F: Fn(&mut Context, &mut JsModuleDef), S: ToString>(
        &mut self,
        name: S,
        exports: &[&str],
        f: F,
    ) {
        assert_size_zero!(@module, F);
        register_fn_module::<F, S>(self, name, exports, f)
    }
}
