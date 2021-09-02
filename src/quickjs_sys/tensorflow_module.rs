/// reference https://github.com/second-state/wasmedge_tensorflow_interface
mod wasmedge_tensorflow {
    /// wasmedge_tensorflow host functions.
    #[link(wasm_import_module = "wasmedge_tensorflow")]
    extern "C" {
        pub fn wasmedge_tensorflow_create_session(model_buf: *const u8, model_buf_len: u32) -> u64;
        pub fn wasmedge_tensorflow_delete_session(context: u64);
        pub fn wasmedge_tensorflow_run_session(context: u64) -> u32;
        pub fn wasmedge_tensorflow_get_output_tensor(
            context: u64,
            output_name: *const u8,
            output_name_len: u32,
            index: u32,
        ) -> u64;
        pub fn wasmedge_tensorflow_get_tensor_len(tensor_ptr: u64) -> u32;
        pub fn wasmedge_tensorflow_get_tensor_data(tensor_ptr: u64, buf: *mut u8);
        pub fn wasmedge_tensorflow_append_input(
            context: u64,
            input_name: *const u8,
            input_name_len: u32,
            index: u32,
            dim_vec: *const u8,
            dim_cnt: u32,
            data_type: u32,
            tensor_buf: *const u8,
            tensor_buf_len: u32,
        );
        pub fn wasmedge_tensorflow_append_output(
            context: u64,
            output_name: *const u8,
            output_name_len: u32,
            index: u32,
        );
        pub fn wasmedge_tensorflow_clear_input(context: u64);
        pub fn wasmedge_tensorflow_clear_output(context: u64);
    }

    /// wasmedge_tensorflowlite host functions.
    #[link(wasm_import_module = "wasmedge_tensorflowlite")]
    extern "C" {
        pub fn wasmedge_tensorflowlite_create_session(
            model_buf: *const u8,
            model_buf_len: u32,
        ) -> u64;
        pub fn wasmedge_tensorflowlite_delete_session(context: u64);
        pub fn wasmedge_tensorflowlite_run_session(context: u64) -> u32;
        pub fn wasmedge_tensorflowlite_get_output_tensor(
            context: u64,
            output_name: *const u8,
            output_name_len: u32,
        ) -> u64;
        pub fn wasmedge_tensorflowlite_get_tensor_len(tensor_ptr: u64) -> u32;
        pub fn wasmedge_tensorflowlite_get_tensor_data(tensor_ptr: u64, buf: *mut u8);
        pub fn wasmedge_tensorflowlite_append_input(
            context: u64,
            input_name: *const u8,
            input_name_len: u32,
            tensor_buf: *const u8,
            tensor_buf_len: u32,
        );
    }

    /// wasmedge_image host helper functions.
    #[link(wasm_import_module = "wasmedge_image")]
    extern "C" {
        pub fn wasmedge_image_load_jpg_to_rgb8(
            img_buf: *const u8,
            img_buf_len: u32,
            img_width: u32,
            img_height: u32,
            dst_buf: *mut u8,
        ) -> u32;
        pub fn wasmedge_image_load_jpg_to_bgr8(
            img_buf: *const u8,
            img_buf_len: u32,
            img_width: u32,
            img_height: u32,
            dst_buf: *mut u8,
        ) -> u32;
        pub fn wasmedge_image_load_jpg_to_rgb32f(
            img_buf: *const u8,
            img_buf_len: u32,
            img_width: u32,
            img_height: u32,
            dst_buf: *mut u8,
        ) -> u32;
        pub fn wasmedge_image_load_jpg_to_bgr32f(
            img_buf: *const u8,
            img_buf_len: u32,
            img_width: u32,
            img_height: u32,
            dst_buf: *mut u8,
        ) -> u32;
        pub fn wasmedge_image_load_png_to_rgb8(
            img_buf: *const u8,
            img_buf_len: u32,
            img_width: u32,
            img_height: u32,
            dst_buf: *mut u8,
        ) -> u32;
        pub fn wasmedge_image_load_png_to_bgr8(
            img_buf: *const u8,
            img_buf_len: u32,
            img_width: u32,
            img_height: u32,
            dst_buf: *mut u8,
        ) -> u32;
        pub fn wasmedge_image_load_png_to_rgb32f(
            img_buf: *const u8,
            img_buf_len: u32,
            img_width: u32,
            img_height: u32,
            dst_buf: *mut u8,
        ) -> u32;
        pub fn wasmedge_image_load_png_to_bgr32f(
            img_buf: *const u8,
            img_buf_len: u32,
            img_width: u32,
            img_height: u32,
            dst_buf: *mut u8,
        ) -> u32;
    }
}
//---------------------

pub mod tensorflow {
    use super::wasmedge_tensorflow::*;
    use crate::quickjs_sys::*;
    use std::path::Path;

    pub enum InputDataType {
        F32 = 1,
        F64 = 2,
        I32 = 3,
        U8 = 4,
        U16 = 17,
        U32 = 22,
        U64 = 23,
        I16 = 5,
        I8 = 6,
        I64 = 9,
        Bool = 10,
    }

    pub struct TensorflowSession {
        context: u64,
        data: Vec<u8>,
    }

    impl Drop for TensorflowSession {
        fn drop(&mut self) {
            unsafe {
                wasmedge_tensorflow_delete_session(self.context);
            }
        }
    }

    impl TensorflowSession {
        pub fn new_from_path<T: AsRef<Path>>(path: T) -> Result<Self, String> {
            let data = std::fs::read(path).map_err(|e| e.to_string())?;
            let context = unsafe {
                wasmedge_tensorflow_create_session(
                    data.as_slice().as_ptr().cast(),
                    data.len() as u32,
                )
            };
            Ok(TensorflowSession { context, data })
        }

        pub unsafe fn add_input(
            &mut self,
            name: &str,
            tensor_buf: *const u8,
            tensor_buf_len: u32,
            data_type: u32,
            shape: &[i64],
        ) {
            let mut idx: u32 = 0;

            let name_pair: Vec<&str> = name.split(":").collect();
            if name_pair.len() > 1 {
                idx = name_pair[1].parse().unwrap();
            }
            let input_name = make_c_string(name_pair[0]);
            wasmedge_tensorflow_append_input(
                self.context,
                input_name.as_ptr() as *const u8,
                input_name.as_bytes().len() as u32,
                idx,
                shape.as_ptr() as *const u8,
                shape.len() as u32,
                data_type,
                tensor_buf,
                tensor_buf_len,
            );
        }

        pub unsafe fn add_output(&mut self, name: &str) {
            let name_pair: Vec<&str> = name.split(":").collect();
            let output_name = make_c_string(name_pair[0]);
            let mut idx = 0;
            if name_pair.len() > 1 {
                idx = name_pair[1].parse().unwrap()
            };
            wasmedge_tensorflow_append_output(
                self.context,
                output_name.as_ptr() as *const u8,
                output_name.as_bytes().len() as u32,
                idx,
            );
        }

        pub unsafe fn run(&mut self) {
            wasmedge_tensorflow_run_session(self.context);
        }

        pub unsafe fn get_output(&self, name: &str) -> Vec<u8> {
            // Parse name and operation index.
            let name_pair: Vec<&str> = name.split(":").collect();
            let output_name = make_c_string(name_pair[0]);
            let mut idx = 0;
            if name_pair.len() > 1 {
                idx = name_pair[1].parse().unwrap()
            };

            // Get tensor data.
            let tensor = wasmedge_tensorflow_get_output_tensor(
                self.context,
                output_name.as_ptr() as *const u8,
                output_name.as_bytes().len() as u32,
                idx,
            );
            let buf_len = wasmedge_tensorflow_get_tensor_len(tensor) as usize;
            if buf_len == 0 {
                return Vec::new();
            }
            let mut data = vec![0u8; buf_len];
            wasmedge_tensorflow_get_tensor_data(tensor, data.as_mut_ptr() as *mut u8);
            return data;
        }

        pub unsafe fn clear_input(&mut self) {
            wasmedge_tensorflow_clear_input(self.context);
        }

        pub unsafe fn clear_output(&mut self) {
            wasmedge_tensorflow_clear_output(self.context);
        }
    }
    // bind function
    unsafe extern "C" fn bind_add_input(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
        magic: ::std::os::raw::c_int,
    ) -> JSValue {
        if argv.is_null() || argc < 3 {
            return js_throw_type_error(ctx, "too few arguments to function ‘add_input’");
        }
        // check name
        let name = match to_string(ctx, *argv.offset(0)) {
            Ok(name) => name,
            Err(e) => return js_throw_type_error(ctx, e),
        };
        // check tensor_buf
        let mut tensor_buf_len = 0;
        let tensor_buf = JS_GetArrayBuffer(ctx, &mut tensor_buf_len, *argv.offset(1));
        if tensor_buf.is_null() || tensor_buf_len == 0 {
            return js_exception();
        }
        // check shape
        let shape = match deserialize_array(ctx, *argv.offset(2)) {
            Ok(s) => s,
            Err(e) => return js_throw_type_error(ctx, e),
        };
        let mut shape_arr = vec![0i64; shape.len()];
        for i in 0..shape.len() {
            let mut v = 0i64;
            if JS_ToInt64(ctx, &mut v, (&shape[i]).v) != 0 {
                return js_exception();
            }
            shape_arr[i] = v;
        }

        let session_ptr = JS_GetOpaque(this_val, JS_CLASS_ID) as *mut TensorflowSession;
        if session_ptr.is_null() {
            return js_exception();
        }
        let session = session_ptr.as_mut().unwrap();
        session.add_input(
            name.as_str(),
            tensor_buf,
            tensor_buf_len as u32,
            magic as u32,
            shape_arr.as_slice(),
        );

        js_undefined()
    }

    unsafe extern "C" fn bind_add_output(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        if argv.is_null() || argc < 1 {
            return js_throw_type_error(ctx, "too few arguments to function ‘add_output’");
        }
        // check name
        let name = match to_string(ctx, *argv.offset(0)) {
            Ok(name) => name,
            Err(e) => return js_throw_type_error(ctx, e),
        };

        let session_ptr = JS_GetOpaque(this_val, JS_CLASS_ID) as *mut TensorflowSession;
        if session_ptr.is_null() {
            return js_exception();
        }
        let session = session_ptr.as_mut().unwrap();
        session.add_output(name.as_str());

        js_undefined()
    }

    unsafe extern "C" fn bind_run(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        let session_ptr = JS_GetOpaque(this_val, JS_CLASS_ID) as *mut TensorflowSession;
        if session_ptr.is_null() {
            return js_exception();
        }
        let session = session_ptr.as_mut().unwrap();
        session.run();
        js_undefined()
    }

    unsafe extern "C" fn bind_get_output(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        if argv.is_null() || argc < 1 {
            return js_throw_type_error(ctx, "too few arguments to function ‘get_output’");
        }
        // check name
        let name = match to_string(ctx, *argv.offset(0)) {
            Ok(name) => name,
            Err(e) => return js_throw_type_error(ctx, e),
        };

        let session_ptr = JS_GetOpaque(this_val, JS_CLASS_ID) as *mut TensorflowSession;
        if session_ptr.is_null() {
            return js_exception();
        }
        let session = session_ptr.as_mut().unwrap();
        let out = session.get_output(name.as_str());
        new_array_buff(ctx, &out)
    }

    unsafe extern "C" fn bind_clear_input(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        let session_ptr = JS_GetOpaque(this_val, JS_CLASS_ID) as *mut TensorflowSession;
        if session_ptr.is_null() {
            return js_exception();
        }
        let session = session_ptr.as_mut().unwrap();
        session.clear_input();
        js_undefined()
    }

    unsafe extern "C" fn bind_clear_output(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        let session_ptr = JS_GetOpaque(this_val, JS_CLASS_ID) as *mut TensorflowSession;
        if session_ptr.is_null() {
            return js_exception();
        }
        let session = session_ptr.as_mut().unwrap();
        session.clear_output();
        js_undefined()
    }

    // bind to quickjs
    unsafe extern "C" fn js_finalizer(rt: *mut JSRuntime, val: JSValue) {
        let s = JS_GetOpaque(val, JS_CLASS_ID) as *mut TensorflowSession;
        if !s.is_null() {
            Box::from_raw(s);
        }
    }

    unsafe fn Session_to_JSValue(ctx: *mut JSContext, session: Box<TensorflowSession>) -> JSValue {
        let obj = JS_NewObjectClass(ctx, JS_CLASS_ID as i32);
        if JS_IsException_real(obj) > 0 {
            return obj;
        }
        let ptr_data = Box::leak(session);
        JS_SetOpaque(obj, (ptr_data as *mut TensorflowSession).cast());
        obj
    }

    unsafe extern "C" fn js_ctor(
        ctx: *mut JSContext,
        new_target: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        if argv.is_null() || argc == 0 {
            return js_throw_type_error(ctx, "too few arguments");
        }
        let param = *argv;
        let session = if JS_IsString_real(param) > 0 {
            let path = match to_string(ctx, param) {
                Ok(path) => path,
                Err(e) => return js_throw_type_error(ctx, e),
            };
            let session = match TensorflowSession::new_from_path(path) {
                Ok(s) => s,
                Err(e) => return js_throw_type_error(ctx, e),
            };

            Box::new(session)
        } else {
            return js_exception();
        };

        return Session_to_JSValue(ctx, session);
    }

    pub static mut JS_CLASS_ID: JSClassID = 0;
    pub static mut JS_CLASS: JSValue = 0;

    static mut JS_CLASS_DEF: JSClassDef = JSClassDef {
        class_name: "TensorflowSession\0".as_ptr() as *const i8,
        finalizer: Some(js_finalizer),
        gc_mark: None,
        call: None,
        exotic: ::std::ptr::null_mut(),
    };

    static mut JS_PROTO_FUNCS: [JSCFunctionListEntry; 7] = [
        CFUNC_MAGIC_DEF!(
            "add_input_8u\0",
            bind_add_input,
            3,
            InputDataType::U8 as i16
        ),
        CFUNC_MAGIC_DEF!(
            "add_input_32f\0",
            bind_add_input,
            3,
            InputDataType::F32 as i16
        ),
        CFUNC_DEF!("add_output\0", bind_add_output, 1),
        CFUNC_DEF!("run\0", bind_run, 0),
        CFUNC_DEF!("get_output\0", bind_get_output, 1),
        CFUNC_DEF!("clear_input\0", bind_clear_input, 0),
        CFUNC_DEF!("clear_output\0", bind_clear_output, 0),
    ];

    unsafe extern "C" fn js_module_init(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
    ) -> ::std::os::raw::c_int {
        JS_NewClassID(&mut JS_CLASS_ID);
        JS_NewClass(JS_GetRuntime(ctx), JS_CLASS_ID, &JS_CLASS_DEF);

        let proto = JS_NewObject(ctx);
        JS_SetPropertyFunctionList(
            ctx,
            proto,
            JS_PROTO_FUNCS.as_ptr(),
            JS_PROTO_FUNCS.len() as i32,
        );

        JS_CLASS = JS_NewCFunction2(
            ctx,
            Some(js_ctor),
            make_c_string("TensorflowSession").as_ptr(),
            1,
            JSCFunctionEnum_JS_CFUNC_constructor,
            0,
        );

        JS_SetConstructor(ctx, JS_CLASS, proto);
        JS_SetClassProto(ctx, JS_CLASS_ID, proto);

        JS_SetModuleExport(
            ctx,
            m,
            make_c_string("TensorflowSession").as_ptr(),
            JS_CLASS,
        );
        0
    }

    pub unsafe fn init_module_tensorflow(ctx: *mut JSContext) -> *mut JSModuleDef {
        let name = make_c_string("tensorflow");
        let m = JS_NewCModule(ctx, name.as_ptr(), Some(js_module_init));
        if m.is_null() {
            return m;
        }
        JS_AddModuleExport(ctx, m, make_c_string("TensorflowSession").as_ptr());
        return m;
    }
}

pub mod tensorflow_lite {

    use super::wasmedge_tensorflow::*;
    use crate::quickjs_sys::*;
    use std::path::Path;

    struct TensorflowLiteSession {
        context: u64,
        data: Vec<u8>,
    }

    impl Drop for TensorflowLiteSession {
        fn drop(&mut self) {
            unsafe {
                wasmedge_tensorflowlite_delete_session(self.context);
            }
        }
    }

    impl TensorflowLiteSession {
        pub fn new_from_path<T: AsRef<Path>>(path: T) -> Result<Self, String> {
            let data = std::fs::read(path).map_err(|e| e.to_string())?;
            let context = unsafe {
                wasmedge_tensorflowlite_create_session(
                    data.as_slice().as_ptr().cast(),
                    data.len() as u32,
                )
            };
            Ok(TensorflowLiteSession { context, data })
        }

        pub unsafe fn add_input(&mut self, name: &str, tensor_buf: *const u8, tensor_buf_len: u32) {
            let input_name = make_c_string(name);
            wasmedge_tensorflowlite_append_input(
                self.context,
                input_name.as_ptr() as *const u8,
                input_name.as_bytes().len() as u32,
                tensor_buf as *const u8,
                tensor_buf_len,
            );
        }

        pub unsafe fn run(&mut self) {
            wasmedge_tensorflowlite_run_session(self.context);
        }

        pub unsafe fn get_output(&self, name: &str) -> Vec<u8> {
            // Parse name and operation index.
            let output_name = make_c_string(name);

            // Get tensor data.
            let tensor = wasmedge_tensorflowlite_get_output_tensor(
                self.context,
                output_name.as_ptr() as *const u8,
                output_name.as_bytes().len() as u32,
            );
            let buf_len = wasmedge_tensorflowlite_get_tensor_len(tensor) as usize;
            if buf_len == 0 {
                return Vec::new();
            }
            let mut data = vec![0u8; buf_len];
            wasmedge_tensorflowlite_get_tensor_data(tensor, data.as_mut_ptr() as *mut u8);
            return data;
        }
    }

    // bind function
    unsafe extern "C" fn bind_add_input(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        if argv.is_null() || argc < 2 {
            return js_throw_type_error(ctx, "too few arguments to function ‘add_input’");
        }
        // check name
        let name = match to_string(ctx, *argv.offset(0)) {
            Ok(name) => name,
            Err(e) => return js_throw_type_error(ctx, e),
        };
        // check tensor_buf
        let mut tensor_buf_len = 0;
        let tensor_buf = JS_GetArrayBuffer(ctx, &mut tensor_buf_len, *argv.offset(1));
        if tensor_buf.is_null() || tensor_buf_len == 0 {
            return js_exception();
        }

        let session_ptr = JS_GetOpaque(this_val, JS_CLASS_ID) as *mut TensorflowLiteSession;
        if session_ptr.is_null() {
            return js_exception();
        }
        let session = session_ptr.as_mut().unwrap();
        session.add_input(name.as_str(), tensor_buf, tensor_buf_len as u32);

        js_undefined()
    }

    unsafe extern "C" fn bind_run(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        let session_ptr = JS_GetOpaque(this_val, JS_CLASS_ID) as *mut TensorflowLiteSession;
        if session_ptr.is_null() {
            return js_exception();
        }
        let session = session_ptr.as_mut().unwrap();
        session.run();
        js_undefined()
    }

    unsafe extern "C" fn bind_get_output(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        if argv.is_null() || argc < 1 {
            return js_throw_type_error(ctx, "too few arguments to function ‘get_output’");
        }
        // check name
        let name = match to_string(ctx, *argv.offset(0)) {
            Ok(name) => name,
            Err(e) => return js_throw_type_error(ctx, e),
        };

        let session_ptr = JS_GetOpaque(this_val, JS_CLASS_ID) as *mut TensorflowLiteSession;
        if session_ptr.is_null() {
            return js_exception();
        }
        let session = session_ptr.as_mut().unwrap();
        let out = session.get_output(name.as_str());
        new_array_buff(ctx, &out)
    }

    // bind to quickjs
    unsafe extern "C" fn js_finalizer(rt: *mut JSRuntime, val: JSValue) {
        let s = JS_GetOpaque(val, JS_CLASS_ID) as *mut TensorflowLiteSession;
        if !s.is_null() {
            Box::from_raw(s);
        }
    }

    unsafe fn Session_to_JSValue(
        ctx: *mut JSContext,
        session: Box<TensorflowLiteSession>,
    ) -> JSValue {
        let obj = JS_NewObjectClass(ctx, JS_CLASS_ID as i32);
        if JS_IsException_real(obj) > 0 {
            return obj;
        }
        let ptr_data = Box::leak(session);
        JS_SetOpaque(obj, (ptr_data as *mut TensorflowLiteSession).cast());
        obj
    }

    unsafe extern "C" fn js_ctor(
        ctx: *mut JSContext,
        new_target: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        if argv.is_null() || argc == 0 {
            return js_throw_type_error(ctx, "too few arguments");
        }
        let param = *argv;
        let session = if JS_IsString_real(param) > 0 {
            let path = match to_string(ctx, param) {
                Ok(path) => path,
                Err(e) => return js_throw_type_error(ctx, e),
            };
            let session = match TensorflowLiteSession::new_from_path(path) {
                Ok(s) => s,
                Err(e) => return js_throw_type_error(ctx, e),
            };

            Box::new(session)
        } else {
            return js_exception();
        };

        return Session_to_JSValue(ctx, session);
    }

    pub static mut JS_CLASS_ID: JSClassID = 0;
    pub static mut JS_CLASS: JSValue = 0;

    static mut JS_CLASS_DEF: JSClassDef = JSClassDef {
        class_name: "TensorflowSession\0".as_ptr() as *const i8,
        finalizer: Some(js_finalizer),
        gc_mark: None,
        call: None,
        exotic: ::std::ptr::null_mut(),
    };

    static mut JS_PROTO_FUNCS: [JSCFunctionListEntry; 3] = [
        CFUNC_DEF!("add_input\0", bind_add_input, 2),
        CFUNC_DEF!("run\0", bind_run, 0),
        CFUNC_DEF!("get_output\0", bind_get_output, 1),
    ];

    unsafe extern "C" fn js_module_init(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
    ) -> ::std::os::raw::c_int {
        JS_NewClassID(&mut JS_CLASS_ID);
        JS_NewClass(JS_GetRuntime(ctx), JS_CLASS_ID, &JS_CLASS_DEF);

        let proto = JS_NewObject(ctx);
        JS_SetPropertyFunctionList(
            ctx,
            proto,
            JS_PROTO_FUNCS.as_ptr(),
            JS_PROTO_FUNCS.len() as i32,
        );

        JS_CLASS = JS_NewCFunction2(
            ctx,
            Some(js_ctor),
            make_c_string("TensorflowLiteSession").as_ptr(),
            1,
            JSCFunctionEnum_JS_CFUNC_constructor,
            0,
        );

        JS_SetConstructor(ctx, JS_CLASS, proto);
        JS_SetClassProto(ctx, JS_CLASS_ID, proto);

        JS_SetModuleExport(
            ctx,
            m,
            make_c_string("TensorflowLiteSession").as_ptr(),
            JS_CLASS,
        );
        0
    }

    pub unsafe fn init_module_tensorflow_lite(ctx: *mut JSContext) -> *mut JSModuleDef {
        let name = make_c_string("tensorflow_lite");
        let m = JS_NewCModule(ctx, name.as_ptr(), Some(js_module_init));
        if m.is_null() {
            return m;
        }
        JS_AddModuleExport(ctx, m, make_c_string("TensorflowLiteSession").as_ptr());
        return m;
    }
}

pub use tensorflow::init_module_tensorflow;
pub use tensorflow_lite::init_module_tensorflow_lite;
