use std::collections::HashMap;
use std::str::from_utf8;
include!("../../lib/binding.rs");

pub struct Runtime(*mut JSRuntime);

impl Runtime {
    pub fn new() -> Runtime {
        unsafe { Runtime(JS_NewRuntime()) }
    }

    pub fn new_context(&mut self) -> Context {
        unsafe {
            let ctx = JS_NewContext(self.0);
            JS_AddIntrinsicBigFloat(ctx);
            JS_AddIntrinsicBigDecimal(ctx);
            JS_AddIntrinsicOperators(ctx);
            JS_EnableBignumExt(ctx, 1);
            js_std_add_console(ctx);
            js_init_module_std(ctx, "std\0".as_ptr() as *const i8);
            js_init_module_os(ctx, "os\0".as_ptr() as *const i8);
            Context(ctx)
        }
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            js_std_free_handlers(self.0);
            JS_FreeRuntime(self.0);
        }
    }
}

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

pub struct Context(*mut JSContext);

impl Context {
    pub fn get_global(&mut self) -> Value {
        unsafe {
            Value {
                ctx: self.0,
                v: get_global(self.0),
                tag: JS_TAG_OBJECT,
            }
        }
    }

    pub fn eval_str(&mut self, code: &str, filename: &str) {
        unsafe {
            js_eval_buf(
                self.0,
                make_c_string(code).as_ptr() as *mut std::os::raw::c_void,
                code.len() as i32,
                make_c_string(filename).as_ptr() as *const i8,
                JS_EVAL_TYPE_GLOBAL as i32,
            );
            js_std_loop(self.0);
        }
    }

    pub fn new_function(&mut self, name: &str, func: JSCFunction) -> Value {
        unsafe {
            let v = new_function(self.0, name, func);
            Value {
                ctx: self.0,
                v,
                tag: JS_ValueGetTag_real(v),
            }
        }
    }

    pub fn new_object(&mut self) -> Value {
        unsafe {
            Value {
                ctx: self.0,
                v: JS_NewObject(self.0),
                tag: JS_TAG_OBJECT,
            }
        }
    }

    pub fn new_bool(&mut self, b: bool) -> Value {
        unsafe {
            Value {
                ctx: self.0,
                v: JS_NewBool_real(self.0, if b { 1 } else { 0 }),
                tag: JS_TAG_BOOL,
            }
        }
    }

    pub fn new_array_buff(&mut self, buff: &[u8]) -> Value {
        unsafe {
            let buff = JS_NewArrayBufferCopy(self.0, buff.as_ptr() as *const u8, buff.len() as u32);
            Value {
                ctx: self.0,
                v: buff,
                tag: JS_TAG_OBJECT,
            }
        }
    }

    pub fn new_string(&mut self, s: &str) -> Value {
        unsafe {
            let v = JS_NewStringLen(self.0, s.as_ptr() as *const i8, s.len() as u32);
            Value {
                ctx: self.0,
                v,
                tag: JS_TAG_STRING,
            }
        }
    }

    pub fn free_value(&mut self, v: Value) {
        unsafe {
            JS_FreeValue_real(self.0, v.v);
        }
    }

    pub fn deserialize_array(&mut self, v: JSValue) -> Result<Vec<Value>, String> {
        unsafe { deserialize_array(self.0, v) }
    }

    pub fn deserialize_object(&mut self, obj: JSValue) -> Result<HashMap<String, Value>, String> {
        unsafe { deserialize_object(self.0, obj) }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { JS_FreeContext(self.0) }
    }
}

unsafe fn get_global(context: *mut JSContext) -> JSValue {
    JS_GetGlobalObject(context)
}

unsafe fn new_string(context: *mut JSContext, s: &str) -> JSValue {
    JS_NewStringLen(context, s.as_ptr() as *const i8, s.len() as u32)
}

unsafe fn new_array_buff(context: *mut JSContext, buff: &[u8]) -> JSValue {
    JS_NewArrayBufferCopy(context, buff.as_ptr() as *const u8, buff.len() as u32)
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

fn make_c_string<T: Into<Vec<u8>>>(s: T) -> std::ffi::CString {
    std::ffi::CString::new(s).unwrap()
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

pub mod http {
    use super::*;
    use std::convert::TryFrom;

    pub fn add_http(ctx: &mut Context) {
        let mut g = ctx.get_global();
        let mut http_obj = ctx.new_object();
        http_obj.set("get", ctx.new_function("http_get", Some(get)));
        http_obj.set("post", ctx.new_function("http_post", Some(post)));
        http_obj.set("put", ctx.new_function("http_put", Some(put)));
        http_obj.set("patch", ctx.new_function("http_patch", Some(patch)));
        http_obj.set("delete", ctx.new_function("http_delete", Some(delete)));

        g.set("http", http_obj);
    }

    unsafe fn parse_response(
        ctx: *mut JSContext,
        r: Result<http_req::response::Response, http_req::error::Error>,
        body: &[u8],
    ) -> JSValue {
        match r {
            Ok(o) => {
                let obj = JS_NewObject(ctx);
                {
                    let status = JS_NewInt32_real(ctx, u16::from(o.status_code()) as i32);
                    set(ctx, "status", obj, status);
                }
                {
                    let header_obj = JS_NewObject(ctx);
                    for h in o.headers().iter() {
                        set(ctx, h.0.as_ref(), header_obj, new_string(ctx, h.1.as_str()));
                    }
                    set(ctx, "headers", obj, header_obj);
                }
                {
                    let body = new_array_buff(ctx, body);
                    set(ctx, "body", obj, body);
                }
                obj
            }
            Err(e) => JS_ThrowInternalError(ctx, make_c_string(e.to_string()).as_ptr()),
        }
    }

    unsafe fn parse_headers(
        ctx: *mut JSContext,
        req: &mut http_req::request::Request,
        header: JSValue,
    ) -> Result<(), JSValue> {
        let tag = JS_ValueGetTag_real(header);
        if tag != JS_TAG_NULL && tag != JS_TAG_UNDEFINED {
            let obj = deserialize_object(ctx, header);
            if let Err(e) = obj {
                return Err(JS_ThrowInternalError(
                    ctx,
                    make_c_string(e.as_str()).as_ptr(),
                ));
            }
            let obj = obj.unwrap();

            for (k, v) in obj {
                req.header(
                    k.as_str(),
                    to_string(ctx, JS_ToString(ctx, v.v)).unwrap().as_str(),
                );
            }
        }
        Ok(())
    }

    unsafe fn parse_body(ctx: *mut JSContext, body: JSValue) -> Vec<u8> {
        if JS_IsString_real(body) >= 1 {
            return Vec::from(to_string(ctx, body).unwrap());
        }
        let mut len = 0u32;
        let ptr = JS_GetArrayBuffer(ctx, &mut len as *mut u32, body);
        if !ptr.is_null() && len > 0 {
            return Vec::from_raw_parts(ptr, len as usize, (len + 1) as usize).to_vec();
        }
        return Vec::from(to_string(ctx, JS_ToString(ctx, body)).unwrap());
    }

    pub extern "C" fn get(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        unsafe {
            if argc < 1 {
                return JS_ThrowInternalError(ctx, "url is undefined\0".as_ptr() as *const i8);
            }
            let url_value = *argv.offset(0);
            let url = to_string(ctx, url_value).unwrap();

            let addr = http_req::uri::Uri::try_from(url.as_str()).unwrap();
            let mut req = http_req::request::Request::new(&addr);
            req.header("Connection", "Close");

            if argc >= 2 {
                let header = *argv.offset(1);
                if let Err(e) = parse_headers(ctx, &mut req, header) {
                    return e;
                };
            }

            let mut write = Vec::new();
            let r = req.method(http_req::request::Method::GET).send(&mut write);

            parse_response(ctx, r, write.as_slice())
        }
    }

    pub extern "C" fn post(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        unsafe {
            if argc < 1 {
                return JS_ThrowInternalError(ctx, "url is undefined\0".as_ptr() as *const i8);
            }
            let url_value = *argv.offset(0);
            let url = to_string(ctx, url_value).unwrap();

            let addr = http_req::uri::Uri::try_from(url.as_str()).unwrap();
            let mut req = http_req::request::Request::new(&addr);
            req.header("Connection", "Close");

            let body = {
                if argc >= 2 {
                    let body = *argv.offset(1);
                    Some(parse_body(ctx, body))
                } else {
                    None
                }
            };
            if let Some(ref body) = body {
                req.header("Content-Length", &body.len());
                req.body(body);
            }

            if argc >= 3 {
                let header = *argv.offset(2);
                if let Err(e) = parse_headers(ctx, &mut req, header) {
                    return e;
                };
            }

            let mut write = Vec::new();
            let r = req.method(http_req::request::Method::POST).send(&mut write);
            parse_response(ctx, r, write.as_slice())
        }
    }

    pub extern "C" fn put(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        unsafe {
            if argc < 1 {
                return JS_ThrowInternalError(ctx, "url is undefined\0".as_ptr() as *const i8);
            }
            let url_value = *argv.offset(0);
            let url = to_string(ctx, url_value).unwrap();

            let addr = http_req::uri::Uri::try_from(url.as_str()).unwrap();
            let mut req = http_req::request::Request::new(&addr);
            req.header("Connection", "Close");

            let body = {
                if argc >= 2 {
                    let body = *argv.offset(1);
                    Some(parse_body(ctx, body))
                } else {
                    None
                }
            };
            if let Some(ref body) = body {
                req.header("Content-Length", &body.len());
                req.body(body);
            }

            if argc >= 3 {
                let header = *argv.offset(2);
                if let Err(e) = parse_headers(ctx, &mut req, header) {
                    return e;
                };
            }

            let mut write = Vec::new();
            let r = req.method(http_req::request::Method::PUT).send(&mut write);
            parse_response(ctx, r, write.as_slice())
        }
    }

    pub extern "C" fn patch(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        unsafe {
            if argc < 1 {
                return JS_ThrowInternalError(ctx, "url is undefined\0".as_ptr() as *const i8);
            }
            let url_value = *argv.offset(0);
            let url = to_string(ctx, url_value).unwrap();

            let addr = http_req::uri::Uri::try_from(url.as_str()).unwrap();
            let mut req = http_req::request::Request::new(&addr);
            req.header("Connection", "Close");

            let body = {
                if argc >= 2 {
                    let body = *argv.offset(1);
                    Some(parse_body(ctx, body))
                } else {
                    None
                }
            };
            if let Some(ref body) = body {
                req.header("Content-Length", &body.len());
                req.body(body);
            }

            if argc >= 3 {
                let header = *argv.offset(2);
                if let Err(e) = parse_headers(ctx, &mut req, header) {
                    return e;
                };
            }

            let mut write = Vec::new();
            let r = req
                .method(http_req::request::Method::PATCH)
                .send(&mut write);
            parse_response(ctx, r, write.as_slice())
        }
    }

    pub extern "C" fn delete(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        unsafe {
            if argc < 1 {
                return JS_ThrowInternalError(ctx, "url is undefined\0".as_ptr() as *const i8);
            }
            let url_value = *argv.offset(0);
            let url = to_string(ctx, url_value).unwrap();

            let addr = http_req::uri::Uri::try_from(url.as_str()).unwrap();
            let mut req = http_req::request::Request::new(&addr);
            req.header("Connection", "Close");

            let body = {
                if argc >= 2 {
                    let body = *argv.offset(1);
                    Some(parse_body(ctx, body))
                } else {
                    None
                }
            };
            if let Some(ref body) = body {
                req.header("Content-Length", &body.len());
                req.body(body);
            }

            if argc >= 3 {
                let header = *argv.offset(2);
                if let Err(e) = parse_headers(ctx, &mut req, header) {
                    return e;
                };
            }

            let mut write = Vec::new();
            let r = req
                .method(http_req::request::Method::DELETE)
                .send(&mut write);
            parse_response(ctx, r, write.as_slice())
        }
    }
}
