use crate::*;

/// reference https://github.com/second-state/wasmedge_tensorflow_interface
mod wasmedge_tensorflow {
    // wasmedge_tensorflow host functions.
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

    // wasmedge_tensorflowlite host functions.
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

    // wasmedge_image host helper functions.
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

mod tensorflow {
    use super::wasmedge_tensorflow::*;
    use crate::*;
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

    impl TensorflowSession {
        fn js_add_input_8u(
            &mut self,
            _: &mut JsObject,
            ctx: &mut Context,
            argv: &[JsValue],
        ) -> JsValue {
            let name = if let Some(JsValue::String(s)) = argv.get(0) {
                s.to_string()
            } else {
                return ctx.throw_type_error("'name' must be of type string").into();
            };

            let tensor_buf = if let Some(JsValue::ArrayBuffer(buf)) = argv.get(1) {
                buf.as_ref()
            } else {
                return ctx
                    .throw_type_error("'tensor_buf' must be of type buffer")
                    .into();
            };

            let shape = if let Some(JsValue::Array(arr)) = argv.get(2) {
                match arr.to_vec() {
                    Ok(a) => a,
                    Err(e) => return e.into(),
                }
            } else {
                return ctx.throw_type_error("'shape' must be of type array").into();
            };

            let mut shape_arr = vec![];

            for i in shape {
                let v = match i {
                    JsValue::Int(i) => i as i64,
                    JsValue::Float(i) => i as i64,
                    _ => {
                        return ctx
                            .throw_type_error("'shape' must be of type number array")
                            .into()
                    }
                };
                shape_arr.push(v);
            }

            unsafe {
                self.add_input(
                    name.as_str(),
                    tensor_buf.as_ptr(),
                    tensor_buf.len() as u32,
                    InputDataType::U8 as u32,
                    shape_arr.as_slice(),
                );
            }
            JsValue::UnDefined
        }

        fn js_add_input_32f(
            &mut self,
            _: &mut JsObject,
            ctx: &mut Context,
            argv: &[JsValue],
        ) -> JsValue {
            let name = if let Some(JsValue::String(s)) = argv.get(0) {
                s.to_string()
            } else {
                return ctx.throw_type_error("'name' must be of type string").into();
            };

            let tensor_buf = if let Some(JsValue::ArrayBuffer(buf)) = argv.get(1) {
                buf.as_ref()
            } else {
                return ctx
                    .throw_type_error("'tensor_buf' must be of type buffer")
                    .into();
            };

            let shape = if let Some(JsValue::Array(arr)) = argv.get(2) {
                match arr.to_vec() {
                    Ok(a) => a,
                    Err(e) => return e.into(),
                }
            } else {
                return ctx.throw_type_error("'shape' must be of type array").into();
            };

            let mut shape_arr = vec![];

            for i in shape {
                let v = match i {
                    JsValue::Int(i) => i as i64,
                    JsValue::Float(i) => i as i64,
                    _ => {
                        return ctx
                            .throw_type_error("'shape' must be of type number array")
                            .into()
                    }
                };
                shape_arr.push(v);
            }

            unsafe {
                self.add_input(
                    name.as_str(),
                    tensor_buf.as_ptr(),
                    tensor_buf.len() as u32,
                    InputDataType::F32 as u32,
                    shape_arr.as_slice(),
                );
            }
            JsValue::UnDefined
        }

        fn js_add_output(
            &mut self,
            _: &mut JsObject,
            ctx: &mut Context,
            argv: &[JsValue],
        ) -> JsValue {
            let name = if let Some(JsValue::String(s)) = argv.get(0) {
                s.to_string()
            } else {
                return ctx.throw_type_error("'name' must be of type string").into();
            };

            unsafe {
                self.add_output(name.as_str());
            }
            JsValue::UnDefined
        }

        fn js_run(&mut self, _: &mut JsObject, _ctx: &mut Context, _argv: &[JsValue]) -> JsValue {
            unsafe { self.run() }
            JsValue::UnDefined
        }

        fn js_get_output(
            &mut self,
            _: &mut JsObject,
            ctx: &mut Context,
            argv: &[JsValue],
        ) -> JsValue {
            let name = if let Some(JsValue::String(s)) = argv.get(0) {
                s.to_string()
            } else {
                return ctx.throw_type_error("'name' must be of type string").into();
            };
            let data = unsafe { self.get_output(name.as_str()) };

            ctx.new_array_buffer(data.as_slice()).into()
        }

        fn js_clear_output(
            &mut self,
            _: &mut JsObject,
            _ctx: &mut Context,
            _argv: &[JsValue],
        ) -> JsValue {
            unsafe { self.clear_output() }
            JsValue::UnDefined
        }

        fn js_clear_input(
            &mut self,
            _: &mut JsObject,
            _ctx: &mut Context,
            _argv: &[JsValue],
        ) -> JsValue {
            unsafe { self.clear_input() }
            JsValue::UnDefined
        }
    }

    impl JsClassDef for TensorflowSession {
        type RefType = TensorflowSession;
        const CLASS_NAME: &'static str = "TensorflowSession\0";
        const CONSTRUCTOR_ARGC: u8 = 1;

        const FIELDS: &'static [JsClassField<Self::RefType>] = &[];
        const METHODS: &'static [JsClassMethod<Self::RefType>] = &[
            ("add_input_8u", 3, Self::js_add_input_8u),
            ("add_input_32f", 3, Self::js_add_input_32f),
            ("add_output", 1, Self::js_add_output),
            ("run", 0, Self::js_run),
            ("get_output", 1, Self::js_get_output),
            ("clear_output", 0, Self::js_clear_output),
            ("clear_input", 0, Self::js_clear_input),
        ];

        unsafe fn mut_class_id_ptr() -> &'static mut u32 {
            static mut CLASS_ID: u32 = 0;
            &mut CLASS_ID
        }

        fn constructor_fn(
            ctx: &mut Context,
            argv: &[JsValue],
        ) -> Result<TensorflowSession, JsValue> {
            match argv.get(0).ok_or(JsValue::UnDefined)? {
                JsValue::String(path) => {
                    let path = path.to_string();
                    let session = TensorflowSession::new_from_path(path)
                        .map_err(|e| ctx.throw_internal_type_error(e.as_str()))?;
                    Ok(session)
                }
                _ => Err(JsValue::UnDefined),
            }
        }
    }

    struct TensorflowModDef;
    impl ModuleInit for TensorflowModDef {
        fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
            let ctor = register_class::<TensorflowSession>(ctx);
            m.add_export(TensorflowSession::CLASS_NAME, ctor)
        }
    }

    pub fn init_module_tensorflow(ctx: &mut Context) {
        ctx.register_module(
            "tensorflow\0",
            TensorflowModDef,
            &[TensorflowSession::CLASS_NAME],
        )
    }
}

mod tensorflow_lite {
    use super::{tensorflow, wasmedge_tensorflow::*};
    use crate::*;
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

    impl TensorflowLiteSession {
        pub fn js_add_input(
            &mut self,
            _: &mut JsObject,
            ctx: &mut Context,
            argv: &[JsValue],
        ) -> JsValue {
            let name = if let Some(JsValue::String(s)) = argv.get(0) {
                s.to_string()
            } else {
                return ctx.throw_type_error("'name' must be of type string").into();
            };

            let tensor_buf = if let Some(JsValue::ArrayBuffer(buf)) = argv.get(1) {
                buf.as_ref()
            } else {
                return ctx
                    .throw_type_error("'tensor_buf' must be of type buffer")
                    .into();
            };

            unsafe {
                self.add_input(name.as_str(), tensor_buf.as_ptr(), tensor_buf.len() as u32);
            }
            JsValue::UnDefined
        }

        pub fn js_run(
            &mut self,
            _: &mut JsObject,
            _ctx: &mut Context,
            _argv: &[JsValue],
        ) -> JsValue {
            unsafe { self.run() }
            JsValue::UnDefined
        }

        pub fn js_get_output(
            &mut self,
            _: &mut JsObject,
            ctx: &mut Context,
            argv: &[JsValue],
        ) -> JsValue {
            let name = if let Some(JsValue::String(s)) = argv.get(0) {
                s.to_string()
            } else {
                return ctx.throw_type_error("'name' must be of type string").into();
            };
            let data = unsafe { self.get_output(name.as_str()) };

            ctx.new_array_buffer(data.as_slice()).into()
        }
    }

    impl JsClassDef for TensorflowLiteSession {
        type RefType = TensorflowLiteSession;
        const CLASS_NAME: &'static str = "TensorflowLiteSession\0";
        const CONSTRUCTOR_ARGC: u8 = 1;

        const FIELDS: &'static [JsClassField<Self::RefType>] = &[];
        const METHODS: &'static [JsClassMethod<Self::RefType>] = &[
            ("add_input", 2, Self::js_add_input),
            ("run", 0, Self::js_run),
            ("get_output", 1, Self::js_get_output),
        ];

        unsafe fn mut_class_id_ptr() -> &'static mut u32 {
            static mut CLASS_ID: u32 = 0;
            &mut CLASS_ID
        }

        fn constructor_fn(
            ctx: &mut Context,
            argv: &[JsValue],
        ) -> Result<TensorflowLiteSession, JsValue> {
            match argv.get(0).ok_or(JsValue::UnDefined)? {
                JsValue::String(path) => {
                    let path = path.to_string();
                    let session = TensorflowLiteSession::new_from_path(path)
                        .map_err(|e| ctx.throw_internal_type_error(e.as_str()))?;
                    Ok(session)
                }
                _ => Err(JsValue::UnDefined),
            }
        }
    }

    struct TensorflowModDef;
    impl ModuleInit for TensorflowModDef {
        fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
            let ctor = register_class::<TensorflowLiteSession>(ctx);
            m.add_export(TensorflowLiteSession::CLASS_NAME, ctor)
        }
    }

    pub fn init_module_tensorflow_lite(ctx: &mut Context) {
        ctx.register_module(
            "tensorflow_lite\0",
            TensorflowModDef,
            &[TensorflowLiteSession::CLASS_NAME],
        )
    }
}

pub use tensorflow::init_module_tensorflow;
pub use tensorflow_lite::init_module_tensorflow_lite;
