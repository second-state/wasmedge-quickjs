use crate::quickjs_sys::qjs::*;
use crate::{Context, EventLoop, JsObject, JsRef, JsValue};

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

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
    let mut this_obj = JsValue::from_qjs_value(ctx, JS_DupValue_real(ctx, this_val))
        .to_obj()
        .unwrap();

    let r = Def::invoke_method_index(data, &mut this_obj, magic as usize, &mut n_ctx, &arg_vec);
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
    let r = Def::field_get(data, magic as usize, &mut n_ctx);
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
    let val = JsValue::from_qjs_value(ctx, JS_DupValue_real(ctx, val));

    Def::field_set(data, magic as usize, &mut n_ctx, val);
    js_undefined()
}

#[derive(Debug, Default)]
pub struct JsClassProto {
    methods: HashMap<String, (u8, usize)>,
    fields: HashMap<String, usize>,
}

fn into_proto_function_list<Def: JsClassDef>(p: JsClassProto) -> &'static [JSCFunctionListEntry] {
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
                    cproto: JS_CFUNC_generic_magic as u8,
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

pub trait JsClassTool: JsClassDef {
    fn class_id() -> u32;

    fn proto(ctx: &mut Context) -> JsValue {
        ctx.get_class_proto(Self::class_id())
    }

    fn constructor(ctx: &mut Context) -> Option<JsValue> {
        ctx.get_class_constructor(Self::class_id())
    }

    fn opaque_mut(js_obj: &mut JsValue) -> Option<&mut Self::RefType> {
        unsafe {
            let class_id = Self::class_id();
            let ptr = JS_GetOpaque(js_obj.get_qjs_value(), class_id) as *mut Self::RefType;
            ptr.as_mut()
        }
    }

    fn opaque(js_obj: &JsValue) -> Option<&Self::RefType> {
        unsafe {
            let class_id = Self::class_id();
            let ptr = JS_GetOpaque(js_obj.get_qjs_value(), class_id) as *mut Self::RefType;
            ptr.as_ref()
        }
    }

    fn wrap_obj(ctx: &mut Context, data: Self::RefType) -> JsValue {
        unsafe {
            let class_id = Self::class_id();
            let obj = JS_NewObjectClass(ctx.ctx, class_id as i32);

            if JS_IsException_real(obj) > 0 {
                JsValue::from_qjs_value(ctx.ctx, obj)
            } else {
                let ptr_data = Box::leak(Box::new(data));
                JS_SetOpaque(obj, (ptr_data as *mut Self::RefType).cast());
                JsValue::from_qjs_value(ctx.ctx, obj)
            }
        }
    }
}

impl<T: JsClassDef> JsClassTool for T {
    fn class_id() -> u32 {
        unsafe { *Self::mut_class_id_ptr() }
    }
}

pub trait ExtendsJsClassDef {
    type RefType: Sized
        + AsRef<<Self::BaseDef as JsClassDef>::RefType>
        + AsMut<<Self::BaseDef as JsClassDef>::RefType>
        + 'static;

    type BaseDef: JsClassDef;

    const EXT_CLASS_NAME: &'static str;
    const CONSTRUCTOR_ARGC: u8;
    const FIELDS: &'static [JsClassField<Self::RefType>];
    const METHODS: &'static [JsClassMethod<Self::RefType>];

    unsafe fn mut_class_id_ptr() -> &'static mut u32;

    fn constructor_fn(ctx: &mut Context, argv: &[JsValue]) -> Result<Self::RefType, JsValue>;

    fn finalizer(_data: &mut Self::RefType, _event_loop: Option<&mut EventLoop>) {}

    fn gc_mark(_data: &Self::RefType, _make: &mut dyn Fn(&JsValue)) {}
}

impl<S: ExtendsJsClassDef> JsClassDef for S {
    type RefType = <Self as ExtendsJsClassDef>::RefType;

    const CLASS_NAME: &'static str = <Self as ExtendsJsClassDef>::EXT_CLASS_NAME;

    const CONSTRUCTOR_ARGC: u8 = <Self as ExtendsJsClassDef>::CONSTRUCTOR_ARGC;

    const FIELDS: &'static [JsClassField<Self::RefType>] = <Self as ExtendsJsClassDef>::FIELDS;

    const METHODS: &'static [JsClassMethod<Self::RefType>] = <Self as ExtendsJsClassDef>::METHODS;

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        <Self as ExtendsJsClassDef>::mut_class_id_ptr()
    }

    #[inline(always)]
    fn methods_size() -> PropEntrySize {
        let l = Self::METHODS.len()
            + *<<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::methods_size();
        PropEntrySize(l)
    }

    #[inline(always)]
    fn invoke_method_index(
        this: &mut Self::RefType,
        this_obj: &mut JsObject,
        i: usize,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let base_methods_len =
            *<<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::methods_size();
        if i < base_methods_len {
            <<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::invoke_method_index(
                this.as_mut(),
                this_obj,
                i,
                ctx,
                argv,
            )
        } else {
            if let Some((_, _, f)) = Self::METHODS.get(i - base_methods_len) {
                f(this, this_obj, ctx, argv)
            } else {
                JsValue::UnDefined
            }
        }
    }

    #[inline(always)]
    fn field_size() -> PropEntrySize {
        let s = Self::FIELDS.len()
            + *<<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::field_size();
        PropEntrySize(s)
    }

    fn field_get(this: &Self::RefType, i: usize, ctx: &mut Context) -> JsValue {
        let base_fields_len = *<<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::field_size();
        if i < base_fields_len {
            <<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::field_get(this.as_ref(), i, ctx)
        } else {
            if let Some((_, getter, _)) = Self::FIELDS.get(i) {
                getter(this, ctx)
            } else {
                JsValue::UnDefined
            }
        }
    }
    fn field_set(this: &mut Self::RefType, i: usize, ctx: &mut Context, val: JsValue) {
        let base_fields_len = *<<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::field_size();
        if i < base_fields_len {
            <<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::field_set(
                this.as_mut(),
                i,
                ctx,
                val,
            )
        } else {
            if let Some((_, _, Some(setter))) = Self::FIELDS.get(i) {
                setter(this, ctx, val)
            }
        }
    }

    fn constructor_fn(ctx: &mut Context, argv: &[JsValue]) -> Result<Self::RefType, JsValue> {
        <Self as ExtendsJsClassDef>::constructor_fn(ctx, argv)
    }

    fn finalizer(data: &mut Self::RefType, event_loop: Option<&mut EventLoop>) {
        if let Some(e) = event_loop {
            <Self as ExtendsJsClassDef>::finalizer(data, Some(e));
            <<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::finalizer(data.as_mut(), Some(e));
        } else {
            <Self as ExtendsJsClassDef>::finalizer(data, None);
            <<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::finalizer(data.as_mut(), None);
        }
    }

    fn gc_mark(data: &Self::RefType, make: &mut dyn Fn(&JsValue)) {
        <<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::gc_mark(data.as_ref(), make);
        <Self as ExtendsJsClassDef>::gc_mark(data, make);
    }

    fn property_keys_init(p: &mut JsClassProto) -> PropInitResult {
        <<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::property_keys_init(p);

        let l = *<<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::methods_size();
        for (i, (name, argc, ..)) in Self::METHODS.iter().enumerate() {
            p.methods.insert(name.to_string(), (*argc, i + l));
        }

        let l = *<<Self as ExtendsJsClassDef>::BaseDef as JsClassDef>::field_size();
        for (i, (name, ..)) in Self::FIELDS.iter().enumerate() {
            p.fields.insert(name.to_string(), i + l);
        }

        PropInitResult(())
    }
}

// only make user can't impl JsClassDef::property_keys_init
pub struct PropInitResult(());

pub struct PropEntrySize(usize);
impl Deref for PropEntrySize {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for PropEntrySize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct PropEntryName(&'static str);
impl Deref for PropEntryName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type JsClassField<T> = (
    &'static str,
    fn(&T, &mut Context) -> JsValue,
    Option<fn(&mut T, &mut Context, JsValue)>,
);

pub type JsClassMethod<T> = (
    &'static str,
    u8,
    fn(&mut T, &mut JsObject, &mut Context, &[JsValue]) -> JsValue,
);

pub trait JsClassDef {
    type RefType: Sized + 'static;

    const CLASS_NAME: &'static str;
    const CONSTRUCTOR_ARGC: u8;

    const FIELDS: &'static [JsClassField<Self::RefType>];

    const METHODS: &'static [JsClassMethod<Self::RefType>];

    unsafe fn mut_class_id_ptr() -> &'static mut u32;

    fn constructor_fn(ctx: &mut Context, argv: &[JsValue]) -> Result<Self::RefType, JsValue>;

    /// don't modify on impl trait
    fn property_keys_init(p: &mut JsClassProto) -> PropInitResult {
        for (i, (name, argc, ..)) in Self::METHODS.iter().enumerate() {
            p.methods.insert(name.to_string(), (*argc, i));
        }
        for (i, (name, ..)) in Self::FIELDS.iter().enumerate() {
            p.fields.insert(name.to_string(), i);
        }

        PropInitResult(())
    }

    /// don't modify on impl trait
    fn methods_size() -> PropEntrySize {
        PropEntrySize(Self::METHODS.len())
    }

    /// don't modify on impl trait
    fn invoke_method_index(
        this: &mut Self::RefType,
        this_obj: &mut JsObject,
        i: usize,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        if let Some((_, _, f)) = Self::METHODS.get(i) {
            f(this, this_obj, ctx, argv)
        } else {
            JsValue::UnDefined
        }
    }

    /// don't modify on impl trait
    fn field_size() -> PropEntrySize {
        PropEntrySize(Self::FIELDS.len())
    }

    /// don't modify on impl trait
    fn field_get(this: &Self::RefType, i: usize, ctx: &mut Context) -> JsValue {
        if let Some((_, getter, _)) = Self::FIELDS.get(i) {
            getter(this, ctx)
        } else {
            JsValue::UnDefined
        }
    }

    /// don't modify on impl trait
    fn field_set(this: &mut Self::RefType, i: usize, ctx: &mut Context, val: JsValue) {
        if let Some((_, _, Some(setter))) = Self::FIELDS.get(i) {
            setter(this, ctx, val)
        }
    }

    fn finalizer(_data: &mut Self::RefType, _event_loop: Option<&mut EventLoop>) {}

    fn gc_mark(_data: &Self::RefType, _make: &mut dyn Fn(&JsValue)) {}
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
            JS_CFUNC_constructor,
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
