#[macro_use]
mod macros;
mod host_fun_demo_module;
#[cfg(feature = "http")]
mod http_module;
#[cfg(feature = "img")]
mod img_module;
mod require_module;
#[cfg(feature = "tensorflow")]
mod tensorflow_module;

use std::collections::HashMap;
use std::str::from_utf8;

#[allow(warnings)]
mod qjs {
    include!("../../lib/binding.rs");
}

use qjs::*;

struct DroppableValue<T, F>
where
    F: FnMut(&mut T),
{
    value: T,
    drop_fn: F,
}

impl<T, F> DroppableValue<T, F>
where
    F: FnMut(&mut T),
{
    pub fn new(value: T, drop_fn: F) -> Self {
        Self { value, drop_fn }
    }
}

impl<T, F> Drop for DroppableValue<T, F>
where
    F: FnMut(&mut T),
{
    fn drop(&mut self) {
        (self.drop_fn)(&mut self.value);
    }
}

impl<T, F> std::ops::Deref for DroppableValue<T, F>
where
    F: FnMut(&mut T),
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T, F> std::ops::DerefMut for DroppableValue<T, F>
where
    F: FnMut(&mut T),
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

pub struct Context {
    rt: *mut JSRuntime,
    ctx: *mut JSContext,
}

impl Context {
    pub fn new() -> Context {
        unsafe {
            let rt = JS_NewRuntime();
            JS_SetModuleLoaderFunc(rt, None, Some(js_module_loader), 0 as *mut std::ffi::c_void);
            js_std_init_handlers(rt);
            let ctx = JS_NewContext(rt);
            JS_AddIntrinsicBigFloat(ctx);
            JS_AddIntrinsicBigDecimal(ctx);
            JS_AddIntrinsicOperators(ctx);
            JS_EnableBignumExt(ctx, 1);
            js_std_add_console(ctx);
            js_init_module_std(ctx, "std\0".as_ptr() as *const i8);
            js_init_module_os(ctx, "os\0".as_ptr() as *const i8);
            let mut ctx = Context { rt, ctx };
            require_module::init_module_require(&mut ctx);
            #[cfg(feature = "http")]
            http_module::init_module_http(&mut ctx);
            #[cfg(feature = "img")]
            img_module::init_module_image(ctx.ctx);
            #[cfg(feature = "tensorflow")]
            tensorflow_module::init_module_tensorflow(ctx.ctx);
            #[cfg(feature = "tensorflow")]
            tensorflow_module::init_module_tensorflow_lite(ctx.ctx);

            host_fun_demo_module::init_module(ctx.ctx);
            ctx
        }
    }

    fn get_global(&mut self) -> Value {
        unsafe {
            Value {
                ctx: self.ctx,
                v: get_global(self.ctx),
                tag: JS_TAG_OBJECT,
            }
        }
    }

    pub fn put_args<T: AsRef<[String]>>(&mut self, args: T) {
        unsafe {
            let args_obj = JS_NewArray(self.ctx);
            let args = args.as_ref();
            let mut i = 0;
            for arg in args {
                let arg_js_string = JS_NewStringLen(self.ctx, arg.as_ptr().cast(), arg.len());
                JS_SetPropertyUint32(self.ctx, args_obj, i, arg_js_string);
                i += 1;
            }
            let global = get_global(self.ctx);
            JS_SetPropertyStr(self.ctx, global, "args\0".as_ptr().cast(), args_obj);
            JS_FreeValue_real(self.ctx, global);
        }
    }

    pub fn eval_str(&mut self, code: &str, filename: &str) {
        unsafe {
            js_eval_buf(
                self.ctx,
                make_c_string(code).as_ptr() as *mut std::os::raw::c_void,
                code.len() as i32,
                make_c_string(filename).as_ptr() as *const i8,
                JS_EVAL_TYPE_MODULE as i32,
            );
            js_std_loop(self.ctx);
        }
    }

    fn new_function(&mut self, name: &str, func: JSCFunction) -> Value {
        unsafe {
            let v = new_function(self.ctx, name, func);
            Value {
                ctx: self.ctx,
                v,
                tag: JS_ValueGetTag_real(v),
            }
        }
    }

    fn new_object(&mut self) -> Value {
        unsafe {
            Value {
                ctx: self.ctx,
                v: JS_NewObject(self.ctx),
                tag: JS_TAG_OBJECT,
            }
        }
    }

    fn new_bool(&mut self, b: bool) -> Value {
        unsafe {
            Value {
                ctx: self.ctx,
                v: JS_NewBool_real(self.ctx, if b { 1 } else { 0 }),
                tag: JS_TAG_BOOL,
            }
        }
    }

    fn new_array_buff(&mut self, buff: &[u8]) -> Value {
        unsafe {
            let buff = JS_NewArrayBufferCopy(self.ctx, buff.as_ptr() as *const u8, buff.len());
            Value {
                ctx: self.ctx,
                v: buff,
                tag: JS_TAG_OBJECT,
            }
        }
    }

    fn new_string(&mut self, s: &str) -> Value {
        unsafe {
            let v = JS_NewStringLen(self.ctx, s.as_ptr() as *const i8, s.len());
            Value {
                ctx: self.ctx,
                v,
                tag: JS_TAG_STRING,
            }
        }
    }

    fn free_value(&mut self, v: Value) {
        unsafe {
            JS_FreeValue_real(self.ctx, v.v);
        }
    }

    fn deserialize_array(&mut self, v: JSValue) -> Result<Vec<Value>, String> {
        unsafe { deserialize_array(self.ctx, v) }
    }

    fn deserialize_object(&mut self, obj: JSValue) -> Result<HashMap<String, Value>, String> {
        unsafe { deserialize_object(self.ctx, obj) }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            js_std_free_handlers(self.rt);
            JS_FreeContext(self.ctx);
            JS_FreeRuntime(self.rt);
        }
    }
}

unsafe fn get_global(context: *mut JSContext) -> JSValue {
    JS_GetGlobalObject(context)
}

unsafe fn new_string(context: *mut JSContext, s: &str) -> JSValue {
    JS_NewStringLen(context, s.as_ptr() as *const i8, s.len())
}

unsafe fn new_array_buff(context: *mut JSContext, buff: &[u8]) -> JSValue {
    JS_NewArrayBufferCopy(context, buff.as_ptr() as *const u8, buff.len())
}

unsafe fn new_function(context: *mut JSContext, name: &str, func: JSCFunction) -> JSValue {
    let name = std::ffi::CString::new(name).unwrap();
    JS_NewCFunction_real(context, func, name.as_ptr(), 1)
}

unsafe fn deserialize_array(context: *mut JSContext, v: JSValue) -> Result<Vec<Value>, String> {
    if JS_IsArray(context, v) == 0 {
        return Err("value is not an Array".into());
    };

    let length_name = std::ffi::CString::new("length").unwrap();
    let len_raw = JS_GetPropertyStr(context, v, length_name.as_ptr());

    let len = to_u32(context, len_raw)?;
    JS_FreeValue_real(context, len_raw);

    let mut values = Vec::new();
    for index in 0..(len as usize) {
        let value_raw = JS_GetPropertyUint32(context, v, index as u32);
        if JS_ValueGetTag_real(value_raw) == JS_TAG_EXCEPTION {
            js_std_dump_error(context);
            return Err("Could not build array".into());
        }
        JS_DupValue_real(context, value_raw);
        values.push(Value {
            ctx: context,
            v: value_raw,
            tag: JS_ValueGetTag_real(value_raw),
        });
    }
    Ok(values)
}

unsafe fn deserialize_object(
    context: *mut JSContext,
    obj: JSValue,
) -> Result<HashMap<String, Value>, String> {
    if JS_ValueGetTag_real(obj) != JS_TAG_OBJECT {
        return Err("value is not an Object".into());
    };
    let mut properties: *mut JSPropertyEnum = std::ptr::null_mut();
    let mut count: u32 = 0;

    let flags = (JS_GPN_STRING_MASK | JS_GPN_SYMBOL_MASK | JS_GPN_ENUM_ONLY) as i32;
    let ret = JS_GetOwnPropertyNames(context, &mut properties, &mut count, obj, flags);
    if ret != 0 {
        js_std_dump_error(context);
        return Err("Could not get object properties".into());
    }

    let properties = DroppableValue::new(properties, |&mut properties| {
        for index in 0..count {
            let prop = unsafe { properties.offset(index as isize) };
            unsafe {
                JS_FreeAtom(context, (*prop).atom);
            }
        }
        unsafe {
            js_free(context, properties as *mut std::ffi::c_void);
        }
    });

    let mut map = HashMap::new();
    for index in 0..count {
        let prop = (*properties).offset(index as isize);
        let raw_value = JS_GetPropertyInternal(context, obj, (*prop).atom, obj, 0);
        if JS_ValueGetTag_real(raw_value) == JS_TAG_EXCEPTION {
            js_std_dump_error(context);
            return Err("Could not get object property".into());
        }

        let value = Value {
            ctx: context,
            v: raw_value,
            tag: JS_ValueGetTag_real(raw_value),
        };

        let key_value = JS_AtomToString(context, (*prop).atom);
        if JS_ValueGetTag_real(key_value) == JS_TAG_EXCEPTION {
            js_std_dump_error(context);
            return Err("Could not get object property name".into());
        }

        let key_res = to_string(context, key_value);
        JS_FreeValue_real(context, key_value);
        let key = key_res?;
        map.insert(key, value);
    }
    Ok(map)
}

unsafe fn to_string(ctx: *mut JSContext, r: JSValue) -> Result<String, String> {
    if JS_ValueGetTag_real(r) != JS_TAG_STRING {
        return Err("Could not convert string: Tag is Not String".into());
    }
    let ptr = JS_ToCStringLen2(ctx, std::ptr::null_mut(), r, 0);
    if ptr.is_null() {
        return Err("Could not convert string: got a null pointer".into());
    }

    let cstr = std::ffi::CStr::from_ptr(ptr);

    let s = cstr
        .to_str()
        .map_err(|e| format!("{}", e).to_string())?
        .to_string();
    JS_FreeCString(ctx, ptr);
    Ok(s)
}

unsafe fn to_i32(ctx: *mut JSContext, v: JSValue) -> Result<i32, String> {
    if JS_ValueGetTag_real(v) == JS_TAG_INT {
        let mut r = 0i32;
        JS_ToInt32(ctx, &mut r as *mut i32, v);
        Ok(r)
    } else {
        Err("value is Not Int".into())
    }
}

unsafe fn to_u32(ctx: *mut JSContext, v: JSValue) -> Result<u32, String> {
    if JS_ValueGetTag_real(v) == JS_TAG_INT {
        let mut r = 0u32;
        JS_ToUint32_real(ctx, &mut r as *mut u32, v);
        Ok(r)
    } else {
        Err("value is Not Int".into())
    }
}

unsafe fn to_i64(ctx: *mut JSContext, v: JSValue) -> Result<i64, String> {
    if JS_ValueGetTag_real(v) == JS_TAG_INT {
        let mut r = 0i64;
        JS_ToInt64(ctx, &mut r as *mut i64, v);
        Ok(r)
    } else {
        Err("value is Not Int".into())
    }
}

unsafe fn js_throw_error<T: Into<Vec<u8>>>(ctx: *mut JSContext, message: T) -> JSValue {
    JS_ThrowInternalError(ctx, make_c_string(message).as_ptr().cast())
}

unsafe fn js_throw_type_error<T: Into<Vec<u8>>>(ctx: *mut JSContext, message: T) -> JSValue {
    JS_ThrowTypeError(ctx, make_c_string(message).as_ptr().cast())
}

fn make_c_string<T: Into<Vec<u8>>>(s: T) -> std::ffi::CString {
    std::ffi::CString::new(s).unwrap_or(Default::default())
}

unsafe fn set(
    ctx: *mut JSContext,
    name: &str,
    this_obj: JSValue,
    v: JSValue,
) -> Result<(), String> {
    if JS_ValueGetTag_real(this_obj) == JS_TAG_OBJECT {
        JS_SetPropertyStr(ctx, this_obj, make_c_string(name).as_ptr(), v);
        Ok(())
    } else {
        Err("this is Not Object".into())
    }
}

unsafe fn delete(ctx: *mut JSContext, name: &str, this_obj: JSValue) -> Result<(), String> {
    if JS_ValueGetTag_real(this_obj) == JS_TAG_OBJECT {
        let atom = JS_NewAtom(ctx, make_c_string(name).as_ptr());
        JS_DeleteProperty(ctx, this_obj, atom, JS_PROP_THROW as i32);
        JS_FreeAtom(ctx, atom);
        Ok(())
    } else {
        Err("this is Not Object".into())
    }
}

pub struct Value {
    ctx: *mut JSContext,
    v: JSValue,
    tag: i32,
}

impl Value {
    pub fn set(&mut self, name: &str, v: Value) -> Result<(), String> {
        unsafe {
            if JS_ValueGetTag_real(self.v) == JS_TAG_OBJECT {
                JS_DupValue_real(self.ctx, v.v);
                JS_SetPropertyStr(self.ctx, self.v, make_c_string(name).as_ptr(), v.v);
                Ok(())
            } else {
                Err("this is Not Object".into())
            }
        }
    }

    pub fn get_string(&mut self) -> Result<String, String> {
        unsafe { to_string(self.ctx, self.v) }
    }

    pub fn get_i32(&mut self) -> Result<i32, String> {
        unsafe { to_i32(self.ctx, self.v) }
    }

    pub fn get_u32(&mut self) -> Result<u32, String> {
        unsafe { to_u32(self.ctx, self.v) }
    }

    pub fn get_i64(&mut self) -> Result<i64, String> {
        unsafe { to_i64(self.ctx, self.v) }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        match self.tag {
            JS_TAG_STRING
            | JS_TAG_OBJECT
            | JS_TAG_FUNCTION_BYTECODE
            | JS_TAG_BIG_INT
            | JS_TAG_BIG_FLOAT
            | JS_TAG_BIG_DECIMAL
            | JS_TAG_SYMBOL => unsafe { JS_FreeValue_real(self.ctx, self.v) },
            _ => {}
        }
    }
}
