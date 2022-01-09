use super::qjs as q;
use super::Context;
use crate::quickjs_sys::qjs::{JSContext, JSModuleDef, JS_NewCFunction2};
use crate::{EventLoop, JsFn, JsValue};
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Add, DerefMut};

pub trait JsClassGetterSetter<D: Sized> {
    const NAME: &'static str;
    fn getter(ctx: &mut Context, this_val: &mut D) -> JsValue;
    fn setter(ctx: &mut Context, this_val: &mut D, val: JsValue);
}
struct JsClassGetterSetterTrampoline<
    D: Sized,
    GS: JsClassGetterSetter<D>,
    Def: 'static + JsClassDef<D>,
> {
    _d: PhantomData<D>,
    _gs: PhantomData<GS>,
    _def: PhantomData<Def>,
}

impl<D: Sized, GS: JsClassGetterSetter<D>, Def: 'static + JsClassDef<D>>
    JsClassGetterSetterTrampoline<D, GS, Def>
{
    unsafe extern "C" fn getter(ctx: *mut JSContext, this_val: q::JSValue) -> q::JSValue {
        let mut n_ctx = std::mem::ManuallyDrop::new(Context {
            rt: q::JS_GetRuntime(ctx),
            ctx,
        });
        let nctx = n_ctx.deref_mut();

        let class_id = JsClassStore::<Def, D>::class_id(None);
        let this_obj = q::JS_GetOpaque(this_val, class_id) as *mut D;

        if this_obj.is_null() {
            return q::js_exception();
        }
        let r = GS::getter(nctx, this_obj.as_mut().unwrap());
        r.into_qjs_value()
    }

    unsafe extern "C" fn setter(
        ctx: *mut JSContext,
        this_val: q::JSValue,
        val: q::JSValue,
    ) -> q::JSValue {
        let mut n_ctx = std::mem::ManuallyDrop::new(Context {
            rt: q::JS_GetRuntime(ctx),
            ctx,
        });
        let nctx = n_ctx.deref_mut();
        let class_id = JsClassStore::<Def, D>::class_id(None);
        let this_obj = q::JS_GetOpaque(this_val, class_id) as *mut D;
        if this_obj.is_null() {
            return q::js_exception();
        }
        let val = JsValue::from_qjs_value(ctx, q::JS_DupValue_real(ctx, val));
        GS::setter(nctx, this_obj.as_mut().unwrap(), val);
        q::js_undefined()
    }
}

pub trait JsMethod<D: Sized> {
    const NAME: &'static str;
    const LEN: u8;
    fn call(ctx: &mut Context, this_val: &mut D, argv: &[JsValue]) -> JsValue;
}

struct JsMethodTrampoline<D: Sized, T: JsMethod<D>, Def: 'static + JsClassDef<D>> {
    _d: PhantomData<D>,
    _m: PhantomData<T>,
    _def: PhantomData<Def>,
}

impl<D: Sized, T: JsMethod<D>, Def: 'static + JsClassDef<D>> JsMethodTrampoline<D, T, Def> {
    unsafe extern "C" fn call(
        ctx: *mut JSContext,
        this_val: q::JSValue,
        len: ::std::os::raw::c_int,
        argv: *mut q::JSValue,
    ) -> q::JSValue {
        let class_id = JsClassStore::<Def, D>::class_id(None);

        let mut n_ctx = std::mem::ManuallyDrop::new(Context {
            rt: q::JS_GetRuntime(ctx),
            ctx,
        });
        let n_ctx = n_ctx.deref_mut();

        let this_obj = q::JS_GetOpaque(this_val, class_id) as *mut D;
        if this_obj.is_null() {
            return q::js_undefined();
        }

        let this_obj = this_obj.as_mut().unwrap();

        let mut arg_vec = vec![];
        for i in 0..len {
            let arg = argv.offset(i as isize);
            let v = *arg;
            let v = JsValue::from_qjs_value(ctx, q::JS_DupValue_real(ctx, v));
            arg_vec.push(v);
        }
        let r = T::call(n_ctx, this_obj, arg_vec.as_slice());
        r.into_qjs_value()
    }
}

pub struct JsClassProto<D: Sized, Def: 'static + JsClassDef<D>> {
    ctx: *mut q::JSContext,
    proto_obj: q::JSValue,
    _p: PhantomData<D>,
    _def: PhantomData<Def>,
}

impl<D: Sized, Def: 'static + JsClassDef<D>> JsClassProto<D, Def> {
    pub fn add_getter_setter<T: JsClassGetterSetter<D>>(&mut self, _: T) {
        unsafe {
            use crate::quickjs_sys::*;
            let ctx = self.ctx;
            let proto = self.proto_obj;
            let g = JsClassGetterSetterTrampoline::<D, T, Def>::getter;
            let s = JsClassGetterSetterTrampoline::<D, T, Def>::setter;

            let list = Vec::leak(vec![JS_CGETSET_DEF!(T::NAME, g, s)]);
            JS_SetPropertyFunctionList(ctx, proto, list.as_ptr(), 1);
        }
    }

    pub fn add_function<T: JsMethod<D>>(&mut self, _: T) {
        unsafe {
            use crate::quickjs_sys::*;
            let ctx = self.ctx;
            let proto = self.proto_obj;

            let f = JsMethodTrampoline::<D, T, Def>::call;
            let list = Vec::leak(vec![CFUNC_DEF!(T::NAME, f, T::LEN)]);
            JS_SetPropertyFunctionList(ctx, proto, list.as_ptr(), 1);
        }
    }
}

struct JsClassStore<T: 'static + JsClassDef<C>, C: Sized> {
    _c: PhantomData<C>,
    _t: PhantomData<T>,
}

impl<C: Sized, T: 'static + JsClassDef<C>> JsClassStore<T, C> {
    fn class_id(new_id: Option<u32>) -> u32 {
        static mut ID_MAP: Option<HashMap<TypeId, u32>> = None;
        unsafe {
            if ID_MAP.is_none() {
                ID_MAP = Some(HashMap::new())
            }
            let n_id = new_id.unwrap_or(0);

            let m = ID_MAP.as_mut().unwrap();
            let id = std::any::TypeId::of::<T>();
            let i = m.entry(id).or_insert(n_id);
            *i
        }
    }
    fn class_ctor(new_ctor: Option<u64>) -> u64 {
        static mut CTOR_MAP: Option<HashMap<TypeId, u64>> = None;
        unsafe {
            if CTOR_MAP.is_none() {
                CTOR_MAP = Some(HashMap::new())
            }
            let c = new_ctor.unwrap_or(0);
            let id = std::any::TypeId::of::<T>();
            let m = CTOR_MAP.as_mut().unwrap();
            let i = m.entry(id).or_insert(c);
            *i
        }
    }
}

pub trait JsClassDef<C: Sized, S = Self>
where
    S: 'static + JsClassDef<C>,
{
    const CLASS_NAME: &'static str;
    const CONSTRUCTOR_ARGC: u8;

    fn constructor(ctx: &mut Context, argv: &[JsValue]) -> Option<C>;

    fn proto_init(p: &mut JsClassProto<C, S>);

    fn finalizer(_data: &mut C, _event_loop: &mut EventLoop) {}

    fn class_value(ctx: &mut Context) -> JsValue {
        unsafe {
            let ctor = JsClassStore::<S, C>::class_ctor(None);
            JsValue::from_qjs_value(ctx.ctx, q::JS_DupValue_real(ctx.ctx, ctor))
        }
    }

    fn gen_js_obj(ctx: &mut Context, data: C) -> JsValue {
        unsafe {
            let class_id = JsClassStore::<S, C>::class_id(None);
            let obj = q::JS_NewObjectClass(ctx.ctx, class_id as i32);

            if q::JS_IsException_real(obj) > 0 {
                JsValue::from_qjs_value(ctx.ctx, obj)
            } else {
                let ptr_data = Box::leak(Box::new(data));
                q::JS_SetOpaque(obj, (ptr_data as *mut C).cast());
                JsValue::from_qjs_value(ctx.ctx, obj)
            }
        }
    }
}

struct JsClassDefTrampoline<D: Sized, Def: 'static + JsClassDef<D>> {
    _d: PhantomData<D>,
    _def: PhantomData<Def>,
}

impl<C: Sized, Def: 'static + JsClassDef<C>> JsClassDefTrampoline<C, Def> {
    unsafe extern "C" fn constructor(
        ctx: *mut JSContext,
        _: q::JSValue,
        len: ::std::os::raw::c_int,
        argv: *mut q::JSValue,
    ) -> q::JSValue {
        let mut n_ctx = std::mem::ManuallyDrop::new(Context {
            rt: q::JS_GetRuntime(ctx),
            ctx,
        });
        let n_ctx = n_ctx.deref_mut();
        let mut arg_vec = vec![];
        for i in 0..len {
            let arg = argv.offset(i as isize);
            let v = *arg;
            let v = JsValue::from_qjs_value(ctx, q::JS_DupValue_real(ctx, v));
            arg_vec.push(v);
        }
        let data = Def::constructor(n_ctx, arg_vec.as_slice());
        if let Some(data) = data {
            Def::gen_js_obj(n_ctx, data).into_qjs_value()
        } else {
            q::js_undefined()
        }
    }

    unsafe extern "C" fn finalizer(rt: *mut q::JSRuntime, val: q::JSValue) {
        let class_id = JsClassStore::<Def, C>::class_id(None);

        let s = q::JS_GetOpaque(val, class_id) as *mut C;
        if !s.is_null() {
            let mut s = Box::from_raw(s);
            let event_loop_ptr = q::JS_GetRuntimeOpaque(rt) as *mut crate::EventLoop;
            if let Some(event_loop) = event_loop_ptr.as_mut() {
                Def::finalizer(s.as_mut(), event_loop);
            }
        }
    }
}

fn register_class<D, Def>(ctx: &mut Context, _: Def) -> JsValue
where
    D: Sized,
    Def: 'static + JsClassDef<D>,
{
    unsafe {
        let mut class_id = 0;
        q::JS_NewClassID(&mut class_id);

        JsClassStore::<Def, D>::class_id(Some(class_id));

        let js_def = q::JSClassDef {
            class_name: Def::CLASS_NAME.as_ptr().cast(),
            finalizer: Some(JsClassDefTrampoline::<D, Def>::finalizer),
            gc_mark: None,
            call: None,
            exotic: std::ptr::null_mut(),
        };

        q::JS_NewClass(q::JS_GetRuntime(ctx.ctx), class_id, &js_def);
        let proto = q::JS_NewObject(ctx.ctx);
        let mut proto_ref = JsClassProto::<D, Def> {
            ctx: ctx.ctx,
            proto_obj: proto,
            _p: Default::default(),
            _def: Default::default(),
        };

        Def::proto_init(&mut proto_ref);

        let js_ctor = JS_NewCFunction2(
            ctx.ctx,
            Some(JsClassDefTrampoline::<D, Def>::constructor),
            Def::CLASS_NAME.as_ptr().cast(),
            Def::CONSTRUCTOR_ARGC as i32,
            q::JSCFunctionEnum_JS_CFUNC_constructor,
            0,
        );

        q::JS_SetConstructor(ctx.ctx, js_ctor, proto);
        let class_id = JsClassStore::<Def, D>::class_id(None);
        q::JS_SetClassProto(ctx.ctx, class_id, proto);
        JsClassStore::<Def, D>::class_ctor(Some(js_ctor));
        JsValue::from_qjs_value(ctx.ctx, js_ctor)
    }
}

pub struct JsModuleDef {
    ctx: *mut q::JSContext,
    m: *mut q::JSModuleDef,
}

impl JsModuleDef {
    pub fn add_export(&mut self, name: &'static str, val: JsValue) {
        unsafe {
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
        let mut n_ctx = std::mem::ManuallyDrop::new(Context {
            rt: q::JS_GetRuntime(ctx),
            ctx,
        });
        let nctx = n_ctx.deref_mut();
        F::init_module(nctx, &mut m);
        0
    }
}

fn register_module<F: ModuleInit>(ctx: &mut Context, name: &'static str, exports: &[&str]) {
    unsafe {
        let ctx = ctx.ctx;
        let js_module_init = ModuleInitFnTrampoline::<F>::init_module;
        let m = q::JS_NewCModule(ctx, name.as_ptr().cast(), Some(js_module_init));
        for s in exports {
            q::JS_AddModuleExport(ctx, m, (*s).as_ptr().cast());
        }
    }
}

impl Context {
    pub fn register_class<D, Def>(&mut self, d: Def) -> JsValue
    where
        D: Sized,
        Def: 'static + JsClassDef<D>,
    {
        register_class::<D, Def>(self, d)
    }

    pub fn register_module<T: ModuleInit>(&mut self, name: &'static str, _: T, exports: &[&str]) {
        register_module::<T>(self, name, exports)
    }
}
