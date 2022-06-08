use super::qjs as q;
use super::Context;
use crate::quickjs_sys::qjs::{JSContext, JSModuleDef, JS_GetOpaque, JS_NewCFunction2};
use crate::{EventLoop, JsFn, JsObject, JsValue};
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
        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });
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
        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });
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

struct JsClassGetterSetter2Trampoline<D: Sized, Getter, Setter, Def: 'static + JsClassDef<D>>
where
    Getter: Fn(&D, &mut Context) -> JsValue,
    Setter: Fn(&mut D, &mut Context, JsValue),
{
    _d: PhantomData<D>,
    _getter: PhantomData<Getter>,
    _setter: PhantomData<Setter>,
    _def: PhantomData<Def>,
}

impl<D: Sized, Getter, Setter, Def: 'static + JsClassDef<D>>
    JsClassGetterSetter2Trampoline<D, Getter, Setter, Def>
where
    Getter: Fn(&D, &mut Context) -> JsValue,
    Setter: Fn(&mut D, &mut Context, JsValue),
{
    unsafe extern "C" fn getter(ctx: *mut JSContext, this_val: q::JSValue) -> q::JSValue {
        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });
        let nctx = n_ctx.deref_mut();

        let class_id = JsClassStore::<Def, D>::class_id(None);
        let this_obj = q::JS_GetOpaque(this_val, class_id) as *mut D;

        if this_obj.is_null() {
            return q::js_exception();
        }
        let getter = std::mem::zeroed::<Getter>();
        let r = getter(this_obj.as_ref().unwrap(), nctx);
        r.into_qjs_value()
    }

    unsafe extern "C" fn setter(
        ctx: *mut JSContext,
        this_val: q::JSValue,
        val: q::JSValue,
    ) -> q::JSValue {
        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });
        let nctx = n_ctx.deref_mut();
        let class_id = JsClassStore::<Def, D>::class_id(None);
        let this_obj = q::JS_GetOpaque(this_val, class_id) as *mut D;
        if this_obj.is_null() {
            return q::js_exception();
        }
        let val = JsValue::from_qjs_value(ctx, q::JS_DupValue_real(ctx, val));
        let setter = std::mem::zeroed::<Setter>();
        setter(this_obj.as_mut().unwrap(), nctx, val);
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

        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });
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

struct JsMethod2Trampoline<D: Sized, F, Def>
where
    F: Fn(&mut D, &mut Context, &[JsValue]) -> JsValue,
    Def: 'static + JsClassDef<D>,
{
    _d: PhantomData<D>,
    _f: PhantomData<F>,
    _def: PhantomData<Def>,
}

impl<D: Sized, F, Def> JsMethod2Trampoline<D, F, Def>
where
    F: Fn(&mut D, &mut Context, &[JsValue]) -> JsValue,
    Def: 'static + JsClassDef<D>,
{
    unsafe extern "C" fn call(
        ctx: *mut JSContext,
        this_val: q::JSValue,
        len: ::std::os::raw::c_int,
        argv: *mut q::JSValue,
    ) -> q::JSValue {
        let class_id = JsClassStore::<Def, D>::class_id(None);

        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });
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
        let f = std::mem::zeroed::<F>();
        let r = f(this_obj, n_ctx, arg_vec.as_slice());
        r.into_qjs_value()
    }
}

pub trait JsCloneMethod<D: Sized> {
    const NAME: &'static str;
    const LEN: u8;
    fn call(ctx: &mut Context, this_val: (&mut D, JsValue), argv: &[JsValue]) -> JsValue;
}

struct JsCloneMethodTrampoline<D: Sized, T: JsCloneMethod<D>, Def: 'static + JsClassDef<D>> {
    _d: PhantomData<D>,
    _m: PhantomData<T>,
    _def: PhantomData<Def>,
}

impl<D: Sized, T: JsCloneMethod<D>, Def: 'static + JsClassDef<D>>
    JsCloneMethodTrampoline<D, T, Def>
{
    unsafe extern "C" fn call(
        ctx: *mut JSContext,
        this_val: q::JSValue,
        len: ::std::os::raw::c_int,
        argv: *mut q::JSValue,
    ) -> q::JSValue {
        let class_id = JsClassStore::<Def, D>::class_id(None);

        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });
        let n_ctx = n_ctx.deref_mut();

        let this_obj = q::JS_GetOpaque(this_val, class_id) as *mut D;
        if this_obj.is_null() {
            return q::js_undefined();
        }

        let this_obj = this_obj.as_mut().unwrap();
        let this_js_obj = JsValue::from_qjs_value(ctx, q::JS_DupValue_real(ctx, this_val));

        let mut arg_vec = vec![];
        for i in 0..len {
            let arg = argv.offset(i as isize);
            let v = *arg;
            let v = JsValue::from_qjs_value(ctx, q::JS_DupValue_real(ctx, v));
            arg_vec.push(v);
        }
        let r = T::call(n_ctx, (this_obj, this_js_obj), arg_vec.as_slice());
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
    pub fn add_getter_setter<T: JsClassGetterSetter<D>>(&mut self) {
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

    pub fn wrap_getter_setter<Getter, Setter>(
        &mut self,
        mut name: String,
        _getter: Getter,
        _setter: Setter,
    ) where
        Getter: Fn(&D, &mut Context) -> JsValue,
        Setter: Fn(&mut D, &mut Context, JsValue),
    {
        assert_size_zero!(D, Getter, Setter);

        unsafe {
            use crate::quickjs_sys::*;
            let ctx = self.ctx;
            let proto = self.proto_obj;
            if !name.ends_with('\0') {
                name.push('\0')
            }

            let g = JsClassGetterSetter2Trampoline::<D, Getter, Setter, Def>::getter;
            let s = JsClassGetterSetter2Trampoline::<D, Getter, Setter, Def>::setter;

            let list = Vec::leak(vec![JS_CGETSET_DEF!(name, g, s)]);
            JS_SetPropertyFunctionList(ctx, proto, list.as_ptr(), 1);
        }
    }

    pub fn add_function<T: JsMethod<D>>(&mut self) {
        unsafe {
            use crate::quickjs_sys::*;
            let ctx = self.ctx;
            let proto = self.proto_obj;

            let f = JsMethodTrampoline::<D, T, Def>::call;
            let list = Vec::leak(vec![CFUNC_DEF!(T::NAME, f, T::LEN)]);
            JS_SetPropertyFunctionList(ctx, proto, list.as_ptr(), 1);
        }
    }

    pub fn wrap_method<F: Fn(&mut D, &mut Context, &[JsValue]) -> JsValue>(
        &mut self,
        mut name: String,
        args_size: u8,
        _: F,
    ) {
        assert_size_zero!(D, F);

        unsafe {
            use crate::quickjs_sys::*;
            let ctx = self.ctx;
            let proto = self.proto_obj;
            if !name.ends_with('\0') {
                name.push('\0');
            }

            let f = JsMethod2Trampoline::<D, F, Def>::call;
            let list = Vec::leak(vec![CFUNC_DEF!(name, f, args_size)]);
            JS_SetPropertyFunctionList(ctx, proto, list.as_ptr(), 1);
        }
    }

    pub fn add_clone_function<T: JsCloneMethod<D>>(&mut self) {
        unsafe {
            use crate::quickjs_sys::*;
            let ctx = self.ctx;
            let proto = self.proto_obj;

            let f = JsCloneMethodTrampoline::<D, T, Def>::call;
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

    fn constructor(ctx: &mut Context, argv: &[JsValue]) -> Result<C, JsValue>;

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

    fn opaque_mut(js_obj: &mut JsObject) -> Option<&mut C> {
        unsafe {
            let class_id = JsClassStore::<S, C>::class_id(None);
            let ptr = JS_GetOpaque(js_obj.0.v, class_id) as *mut C;
            ptr.as_mut()
        }
    }

    fn opaque(js_obj: &JsObject) -> Option<&C> {
        unsafe {
            let class_id = JsClassStore::<S, C>::class_id(None);
            let ptr = JS_GetOpaque(js_obj.0.v, class_id) as *mut C;
            ptr.as_ref()
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
        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });
        let n_ctx = n_ctx.deref_mut();
        let mut arg_vec = vec![];
        for i in 0..len {
            let arg = argv.offset(i as isize);
            let v = *arg;
            let v = JsValue::from_qjs_value(ctx, q::JS_DupValue_real(ctx, v));
            arg_vec.push(v);
        }
        let data = Def::constructor(n_ctx, arg_vec.as_slice());
        match data {
            Ok(data) => Def::gen_js_obj(n_ctx, data).into_qjs_value(),
            Err(e) => e.into_qjs_value(),
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
            if !export_string.ends_with('\0'){
                export_string.push('\0');
            }
            q::JS_AddModuleExport(ctx, m, export_string.as_ptr().cast());
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

    pub fn register_module<T: ModuleInit, S: ToString>(&mut self, name: S, _: T, exports: &[&str]) {
        register_module::<T, S>(self, name, exports)
    }
}
