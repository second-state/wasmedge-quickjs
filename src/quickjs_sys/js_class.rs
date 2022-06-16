use super::qjs as q;
use crate::quickjs_sys::qjs::{JSContext, JS_GetOpaque, JS_NewCFunction2};
use crate::{Context, EventLoop, JsObject, JsValue};
use std::any::TypeId;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::DerefMut;

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
            return JsValue::Exception(nctx.throw_type_error("Invalid Class")).into_qjs_value();
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
            return JsValue::Exception(nctx.throw_type_error("Invalid Class")).into_qjs_value();
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

        let js_ref = JsValue::from_qjs_value(ctx, this_val);
        let this_obj = q::JS_GetOpaque(js_ref.into_qjs_value(), class_id) as *mut D;

        if this_obj.is_null() {
            return JsValue::Exception(nctx.throw_type_error("Invalid Class")).into_qjs_value();
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
            return JsValue::Exception(nctx.throw_type_error("Invalid Class")).into_qjs_value();
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
            return JsValue::Exception(n_ctx.throw_type_error("Invalid Class")).into_qjs_value();
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
            return JsValue::Exception(n_ctx.throw_type_error("Invalid Class")).into_qjs_value();
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
    pub fn set_proto(&mut self, parent_proto: JsValue) {
        unsafe {
            let parent_proto = parent_proto.get_qjs_value();
            let _ = q::JS_SetPrototype(self.ctx, self.proto_obj, parent_proto);
        }
    }

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
            let name = Vec::from(name);
            let name = Vec::leak(name);

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

    fn proto_init(ctx: &mut Context, p: &mut JsClassProto<C, S>);

    fn finalizer(_data: &mut C, _event_loop: &mut EventLoop) {}

    fn class_value(ctx: &mut Context) -> JsValue {
        unsafe {
            let ctor = JsClassStore::<S, C>::class_ctor(None);
            JsValue::from_qjs_value(ctx.ctx, q::JS_DupValue_real(ctx.ctx, ctor))
        }
    }

    fn proto_obj(ctx: &mut Context) -> JsValue {
        unsafe {
            let ctx = ctx.ctx;
            let class_id = JsClassStore::<S, C>::class_id(None);
            JsValue::from_qjs_value(ctx, q::JS_GetClassProto(ctx, class_id))
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
        new_target: q::JSValue,
        len: ::std::os::raw::c_int,
        argv: *mut q::JSValue,
    ) -> q::JSValue {
        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });
        let n_ctx = n_ctx.deref_mut();

        let new_target = JsValue::from_qjs_value(ctx, new_target);

        let proto = new_target.get("prototype").unwrap_or(JsValue::Null);
        if let JsValue::Exception(_) = &proto {
            return q::JS_Throw(ctx, proto.into_qjs_value());
        }
        let _ = new_target.into_qjs_value();

        let mut arg_vec = vec![];
        for i in 0..len {
            let arg = argv.offset(i as isize);
            let v = *arg;
            let v = JsValue::from_qjs_value(ctx, q::JS_DupValue_real(ctx, v));
            arg_vec.push(v);
        }
        let data = Def::constructor(n_ctx, arg_vec.as_slice());
        match data {
            Ok(data) => {
                let class_id = JsClassStore::<Def, C>::class_id(None);
                let obj = q::JS_NewObjectProtoClass(ctx, proto.get_qjs_value(), class_id);

                if q::JS_IsException_real(obj) > 0 {
                    q::JS_Throw(ctx, obj)
                } else {
                    let ptr_data = Box::leak(Box::new(data));
                    q::JS_SetOpaque(obj, (ptr_data as *mut C).cast());
                    obj
                }
            }
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
        let mut proto_ref = JsClassProto::<D, Def> {
            ctx: ctx.ctx,
            proto_obj: q::JS_NewObject(ctx.ctx),
            _p: Default::default(),
            _def: Default::default(),
        };

        Def::proto_init(ctx, &mut proto_ref);

        let js_ctor = JS_NewCFunction2(
            ctx.ctx,
            Some(JsClassDefTrampoline::<D, Def>::constructor),
            Def::CLASS_NAME.as_ptr().cast(),
            Def::CONSTRUCTOR_ARGC as i32,
            q::JSCFunctionEnum_JS_CFUNC_constructor,
            0,
        );

        q::JS_SetConstructor(ctx.ctx, js_ctor, proto_ref.proto_obj);
        q::JS_SetClassProto(ctx.ctx, class_id, proto_ref.proto_obj);
        JsClassStore::<Def, D>::class_ctor(Some(js_ctor));
        JsValue::from_qjs_value(ctx.ctx, js_ctor)
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
}

pub mod v2 {
    use crate::quickjs_sys::qjs::*;
    use crate::{Context, EventLoop, JsValue};

    use std::collections::HashMap;

    fn parse_c_string(s: &mut String) {
        if !s.ends_with('\0') {
            let len = s.len();
            s.push('\0');
            s.truncate(len);
        }
    }

    unsafe extern "C" fn js_method_magic_trampoline<Def: JsClassDef>(
        ctx: *mut JSContext,
        this_val: JSValue,
        len: i32,
        argv: *mut JSValue,
        magic: i32,
    ) -> JSValue {
        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });

        let class_id = Def::class_id();
        let data = JS_GetOpaque(this_val, class_id) as *mut Def::RefType;
        if data.is_null() {
            return JsValue::Exception(n_ctx.throw_type_error("Invalid Class")).into_qjs_value();
        }

        let mut arg_vec = vec![];
        for i in 0..len {
            let arg = argv.offset(i as isize);
            let v = *arg;
            let v = JsValue::from_qjs_value(ctx, JS_DupValue_real(ctx, v));
            arg_vec.push(v);
        }

        let data = data.as_mut().unwrap();

        let name = Def::method_index(magic as usize);
        let r = Def::method_fn(name, data, &mut n_ctx, &arg_vec);
        r.into_qjs_value()
    }

    unsafe extern "C" fn getter_magic_trampoline<Def: JsClassDef>(
        ctx: *mut JSContext,
        this_val: JSValue,
        magic: i32,
    ) -> JSValue {
        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });

        let class_id = Def::class_id();
        let data = JS_GetOpaque(this_val, class_id) as *mut Def::RefType;
        if data.is_null() {
            return JsValue::Exception(n_ctx.throw_type_error("Invalid Class")).into_qjs_value();
        }

        let data = data.as_mut().unwrap();
        let name = Def::field_index(magic as usize);

        let r = Def::field_get(name, data, &mut n_ctx);
        r.into_qjs_value()
    }

    unsafe extern "C" fn setter_magic_trampoline<Def: JsClassDef>(
        ctx: *mut JSContext,
        this_val: JSValue,
        val: JSValue,
        magic: i32,
    ) -> JSValue {
        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });

        let class_id = Def::class_id();
        let data = JS_GetOpaque(this_val, class_id) as *mut Def::RefType;
        if data.is_null() {
            return JsValue::Exception(n_ctx.throw_type_error("Invalid Class")).into_qjs_value();
        }

        let data = data.as_mut().unwrap();
        let name = Def::field_index(magic as usize);
        let val = JsValue::from_qjs_value(ctx, JS_DupValue_real(ctx, val));

        Def::field_set(name, data, &mut n_ctx, val);

        js_undefined()
    }

    #[derive(Debug, Default)]
    pub struct JsClassProto {
        methods: HashMap<String, (u8, usize)>,
        fields: HashMap<String, usize>,
    }

    fn into_proto_function_list<Def: JsClassDef>(
        p: JsClassProto,
    ) -> &'static [JSCFunctionListEntry] {
        let mut entry_vec = vec![];

        let JsClassProto { methods, fields } = p;

        for (mut field_name, i) in fields {
            parse_c_string(&mut field_name);

            let e = JSCFunctionListEntry {
                name: field_name.as_ptr().cast(),
                prop_flags: JS_PROP_CONFIGURABLE as u8,
                def_type: JS_DEF_CGETSET_MAGIC as u8,
                magic: i as i16,
                u: JSCFunctionListEntry__bindgen_ty_1 {
                    getset: JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_2 {
                        get: JSCFunctionType {
                            getter_magic: Some(getter_magic_trampoline::<Def>),
                        },
                        set: JSCFunctionType {
                            setter_magic: Some(setter_magic_trampoline::<Def>),
                        },
                    },
                },
            };

            entry_vec.push(e);
            std::mem::forget(field_name);
        }

        for (mut method_name, (argc, i)) in methods {
            parse_c_string(&mut method_name);
            let e = JSCFunctionListEntry {
                name: method_name.as_ptr().cast(),
                prop_flags: (JS_PROP_WRITABLE | JS_PROP_CONFIGURABLE) as u8,
                def_type: JS_DEF_CFUNC as u8,
                magic: i as i16,
                u: JSCFunctionListEntry__bindgen_ty_1 {
                    func: JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_1 {
                        length: argc,
                        cproto: JSCFunctionEnum_JS_CFUNC_generic_magic as u8,
                        cfunc: JSCFunctionType {
                            generic_magic: Some(js_method_magic_trampoline::<Def>),
                        },
                    },
                },
            };
            entry_vec.push(e);
            std::mem::forget(method_name);
        }

        Vec::leak(entry_vec)
    }

    pub trait JsClassDefExtends {
        type RefType: Sized
            + AsRef<<Self::BaseDef as JsClassDef>::RefType>
            + AsMut<<Self::BaseDef as JsClassDef>::RefType>;

        type BaseDef: JsClassDef;

        const CLASS_NAME: &'static str;
        const CONSTRUCTOR_ARGC: u8;
        const FIELDS: &'static [&'static str];
        const METHODS: &'static [(&'static str, u8)];

        unsafe fn mut_class_id_ptr() -> &'static mut u32;

        fn constructor_fn(ctx: &mut Context, argv: &[JsValue]) -> Result<Self::RefType, JsValue>;

        fn method_fn(
            name: &str,
            this: &mut Self::RefType,
            ctx: &mut Context,
            argv: &[JsValue],
        ) -> JsValue {
            <Self::BaseDef as JsClassDef>::method_fn(name, this.as_mut(), ctx, argv)
        }
        fn field_get(name: &str, this: &Self::RefType, ctx: &mut Context) -> JsValue {
            <Self::BaseDef as JsClassDef>::field_get(name, this.as_ref(), ctx)
        }
        fn field_set(name: &str, this: &mut Self::RefType, ctx: &mut Context, val: JsValue) {
            <Self::BaseDef as JsClassDef>::field_set(name, this.as_mut(), ctx, val)
        }

        fn finalizer(_data: &mut Self::RefType, _event_loop: Option<&mut EventLoop>) {}

        fn gc_mark(_data: &Self::RefType, _make: &mut dyn Fn(&JsValue)) {}
    }

    impl<S: JsClassDefExtends> JsClassDef for S {
        type RefType = <Self as JsClassDefExtends>::RefType;

        const CLASS_NAME: &'static str = <Self as JsClassDefExtends>::CLASS_NAME;

        const CONSTRUCTOR_ARGC: u8 = <Self as JsClassDefExtends>::CONSTRUCTOR_ARGC;

        const FIELDS: &'static [&'static str] = <Self as JsClassDefExtends>::FIELDS;
        const METHODS: &'static [(&'static str, u8)] = <Self as JsClassDefExtends>::METHODS;

        unsafe fn mut_class_id_ptr() -> &'static mut u32 {
            <Self as JsClassDefExtends>::mut_class_id_ptr()
        }

        fn methods_size() -> usize {
            Self::METHODS.len()
                + <<Self as JsClassDefExtends>::BaseDef as JsClassDef>::methods_size()
        }

        fn method_index(i: usize) -> &'static str {
            let base_methods_len =
                <<Self as JsClassDefExtends>::BaseDef as JsClassDef>::methods_size();
            if i < base_methods_len {
                <<Self as JsClassDefExtends>::BaseDef as JsClassDef>::method_index(i)
            } else {
                if let Some(s) = Self::METHODS.get(i - base_methods_len) {
                    s.0
                } else {
                    ""
                }
            }
        }

        fn field_size() -> usize {
            Self::FIELDS.len() + <<Self as JsClassDefExtends>::BaseDef as JsClassDef>::field_size()
        }

        fn field_index(i: usize) -> &'static str {
            let base_fields_len =
                <<Self as JsClassDefExtends>::BaseDef as JsClassDef>::field_size();
            if i < base_fields_len {
                <<Self as JsClassDefExtends>::BaseDef as JsClassDef>::field_index(i)
            } else {
                if let Some(s) = Self::FIELDS.get(i - base_fields_len) {
                    *s
                } else {
                    ""
                }
            }
        }

        fn constructor_fn(ctx: &mut Context, argv: &[JsValue]) -> Result<Self::RefType, JsValue> {
            <Self as JsClassDefExtends>::constructor_fn(ctx, argv)
        }

        fn finalizer(data: &mut Self::RefType, event_loop: Option<&mut EventLoop>) {
            if let Some(e) = event_loop {
                <Self as JsClassDefExtends>::finalizer(data, Some(e));
                <<Self as JsClassDefExtends>::BaseDef as JsClassDef>::finalizer(
                    data.as_mut(),
                    Some(e),
                );
            } else {
                <Self as JsClassDefExtends>::finalizer(data, None);
                <<Self as JsClassDefExtends>::BaseDef as JsClassDef>::finalizer(
                    data.as_mut(),
                    None,
                );
            }
        }

        fn gc_mark(data: &Self::RefType, make: &mut dyn Fn(&JsValue)) {
            <<Self as JsClassDefExtends>::BaseDef as JsClassDef>::gc_mark(data.as_ref(), make);
            <Self as JsClassDefExtends>::gc_mark(data, make);
        }

        fn property_keys_init(p: &mut JsClassProto) {
            <<Self as JsClassDefExtends>::BaseDef as JsClassDef>::property_keys_init(p);

            let l = <<Self as JsClassDefExtends>::BaseDef as JsClassDef>::methods_size();
            for (i, (name, argc)) in Self::METHODS.iter().enumerate() {
                p.methods.insert(name.to_string(), (*argc, i + l));
            }

            let l = <<Self as JsClassDefExtends>::BaseDef as JsClassDef>::field_size();
            for (i, name) in Self::FIELDS.iter().enumerate() {
                p.fields.insert(name.to_string(), i + l);
            }
        }

        fn method_fn(
            name: &str,
            this: &mut Self::RefType,
            ctx: &mut Context,
            argv: &[JsValue],
        ) -> JsValue {
            <Self as JsClassDefExtends>::method_fn(name, this, ctx, argv)
        }

        fn field_get(name: &str, this: &Self::RefType, ctx: &mut Context) -> JsValue {
            <Self as JsClassDefExtends>::field_get(name, this, ctx)
        }

        fn field_set(name: &str, this: &mut Self::RefType, ctx: &mut Context, val: JsValue) {
            <Self as JsClassDefExtends>::field_set(name, this, ctx, val)
        }
    }

    pub trait JsClassDef {
        type RefType: Sized;

        const CLASS_NAME: &'static str;
        const CONSTRUCTOR_ARGC: u8;

        const FIELDS: &'static [&'static str];
        const METHODS: &'static [(&'static str, u8)];

        unsafe fn mut_class_id_ptr() -> &'static mut u32;

        /// don't modify on impl trait
        fn class_id() -> u32 {
            unsafe { *Self::mut_class_id_ptr() }
        }

        /// don't modify on impl trait
        fn proto(ctx: &mut Context) -> JsValue {
            ctx.get_class_proto(Self::class_id())
        }

        /// don't modify on impl trait
        fn constructor(ctx: &mut Context) -> Option<JsValue> {
            ctx.get_class_constructor(Self::class_id())
        }

        fn constructor_fn(ctx: &mut Context, argv: &[JsValue]) -> Result<Self::RefType, JsValue>;

        /// don't modify on impl trait
        fn property_keys_init(p: &mut JsClassProto) {
            for (i, (name, argc)) in Self::METHODS.iter().enumerate() {
                p.methods.insert(name.to_string(), (*argc, i));
            }
            for (i, name) in Self::FIELDS.iter().enumerate() {
                p.fields.insert(name.to_string(), i);
            }
        }

        /// don't modify on impl trait
        fn methods_size() -> usize {
            Self::METHODS.len()
        }

        /// don't modify on impl trait
        fn method_index(i: usize) -> &'static str {
            if let Some(s) = Self::METHODS.get(i) {
                s.0
            } else {
                ""
            }
        }

        fn method_fn(
            name: &str,
            this: &mut Self::RefType,
            ctx: &mut Context,
            argv: &[JsValue],
        ) -> JsValue;

        /// don't modify on impl trait
        fn field_size() -> usize {
            Self::FIELDS.len()
        }

        /// don't modify on impl trait
        fn field_index(i: usize) -> &'static str {
            if let Some(s) = Self::FIELDS.get(i) {
                *s
            } else {
                ""
            }
        }

        fn field_get(name: &str, this: &Self::RefType, ctx: &mut Context) -> JsValue;
        fn field_set(name: &str, this: &mut Self::RefType, ctx: &mut Context, val: JsValue);

        fn finalizer(_data: &mut Self::RefType, _event_loop: Option<&mut EventLoop>) {}

        fn gc_mark(_data: &Self::RefType, _make: &mut dyn Fn(&JsValue)) {}

        /// don't modify on impl trait
        fn opaque_mut(js_obj: &mut JsValue) -> Option<&mut Self::RefType> {
            unsafe {
                let class_id = Self::class_id();
                let ptr = JS_GetOpaque(js_obj.get_qjs_value(), class_id) as *mut Self::RefType;
                ptr.as_mut()
            }
        }

        /// don't modify on impl trait
        fn opaque(js_obj: &JsValue) -> Option<&Self::RefType> {
            unsafe {
                let class_id = Self::class_id();
                let ptr = JS_GetOpaque(js_obj.get_qjs_value(), class_id) as *mut Self::RefType;
                ptr.as_ref()
            }
        }
    }

    unsafe fn gc_mark_value(
        rt: *mut JSRuntime,
        v: &JsValue,
        mark_func: Option<unsafe extern "C" fn(*mut JSRuntime, *mut JSGCObjectHeader)>,
    ) {
        match v {
            JsValue::BigNum(_) => {}
            JsValue::String(_) => {}
            JsValue::Object(_) => {}
            JsValue::ArrayBuffer(_) => {}
            JsValue::Function(_) => {}
            _ => return,
        }
        JS_MarkValue(rt, v.get_qjs_value(), mark_func);
    }

    unsafe extern "C" fn gc_mark<Def: JsClassDef>(
        rt: *mut JSRuntime,
        val: JSValue,
        mark_func: Option<unsafe extern "C" fn(*mut JSRuntime, *mut JSGCObjectHeader)>,
    ) {
        let ptr = JS_GetOpaque(val, Def::class_id()) as *mut Def::RefType;
        if let Some(ptr) = ptr.as_ref() {
            Def::gc_mark(&ptr, &mut |v| gc_mark_value(rt, v, mark_func));
        }
    }

    unsafe extern "C" fn finalizer<Def: JsClassDef>(rt: *mut JSRuntime, val: JSValue) {
        let class_id = Def::class_id();

        let s = JS_GetOpaque(val, class_id) as *mut Def::RefType;
        if !s.is_null() {
            let mut s = Box::from_raw(s);
            let event_loop_ptr = JS_GetRuntimeOpaque(rt) as *mut crate::EventLoop;
            Def::finalizer(&mut s, event_loop_ptr.as_mut());
        }
    }

    unsafe extern "C" fn constructor<Def: JsClassDef>(
        ctx: *mut JSContext,
        new_target: JSValue,
        len: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        let mut n_ctx = std::mem::ManuallyDrop::new(Context { ctx });

        let new_target = JsValue::from_qjs_value(ctx, JS_DupValue_real(ctx, new_target));

        let proto = new_target.get("prototype").unwrap_or(JsValue::Null);
        if let JsValue::Exception(_) = &proto {
            return JS_Throw(ctx, proto.into_qjs_value());
        }

        let mut arg_vec = vec![];
        for i in 0..len {
            let arg = argv.offset(i as isize);
            let v = *arg;
            let v = JsValue::from_qjs_value(ctx, JS_DupValue_real(ctx, v));
            arg_vec.push(v);
        }
        let data = Def::constructor_fn(&mut n_ctx, arg_vec.as_slice());
        match data {
            Ok(data) => {
                let class_id = Def::class_id();
                let obj = JS_NewObjectProtoClass(ctx, proto.get_qjs_value(), class_id);

                if JS_IsException_real(obj) != 0 {
                    JS_Throw(ctx, obj)
                } else {
                    let ptr_data = Box::leak(Box::new(data));
                    JS_SetOpaque(obj, (ptr_data as *mut Def::RefType).cast());
                    obj
                }
            }
            Err(e) => e.into_qjs_value(),
        }
    }

    pub fn register_class<Def: JsClassDef>(ctx: &mut Context) -> JsValue {
        unsafe {
            let rt = ctx.rt();
            let mut class_id = Def::class_id();
            let mut class_name = Def::CLASS_NAME.to_string();
            parse_c_string(&mut class_name);

            if JS_IsRegisteredClass(rt, class_id) == 0 {
                let class_id_ptr = Def::mut_class_id_ptr();
                JS_NewClassID(class_id_ptr);
                class_id = *class_id_ptr;

                let js_def = JSClassDef {
                    class_name: class_name.as_ptr().cast(),
                    finalizer: Some(finalizer::<Def>),
                    gc_mark: Some(gc_mark::<Def>),
                    call: None,
                    exotic: std::ptr::null_mut(),
                };
                JS_NewClass(rt, class_id, &js_def);
            }

            let mut proto_ref = JsClassProto::default();
            Def::property_keys_init(&mut proto_ref);

            //fixme leak
            let function_list = into_proto_function_list::<Def>(proto_ref);

            let proto = JS_NewObject(ctx.ctx);

            JS_SetPropertyFunctionList(
                ctx.ctx,
                proto,
                function_list.as_ptr(),
                function_list.len() as i32,
            );

            let js_ctor = JS_NewCFunction2(
                ctx.ctx,
                Some(constructor::<Def>),
                class_name.as_ptr().cast(),
                Def::CONSTRUCTOR_ARGC as i32,
                JSCFunctionEnum_JS_CFUNC_constructor,
                0,
            );

            JS_SetConstructor(ctx.ctx, js_ctor, proto);
            JS_SetClassProto(ctx.ctx, class_id, proto);
            JsValue::from_qjs_value(ctx.ctx, js_ctor)
        }
    }

    pub fn class_extends(ctx: &mut Context, proto: JsValue, base_proto: JsValue) -> bool {
        unsafe { JS_SetPrototype(ctx.ctx, proto.get_qjs_value(), base_proto.get_qjs_value()) > 0 }
    }

    impl Context {
        pub fn get_class_proto(&self, class_id: u32) -> JsValue {
            unsafe { JsValue::from_qjs_value(self.ctx, JS_GetClassProto(self.ctx, class_id)) }
        }

        pub fn get_class_constructor(&self, class_id: u32) -> Option<JsValue> {
            let proto = self.get_class_proto(class_id);
            proto.get("constructor")
        }

        pub fn call_class_constructor(&self, constructor_fn: JsValue, args: &[JsValue]) -> JsValue {
            unsafe {
                let argc = args.len();
                let mut argv: Vec<JSValue> = args.iter().map(JsValue::get_qjs_value).collect();

                let v = JS_CallConstructor(
                    self.ctx,
                    constructor_fn.get_qjs_value(),
                    argc as i32,
                    argv.as_mut_ptr(),
                );
                JsValue::from_qjs_value(self.ctx, v)
            }
        }
    }
}
