use super::*;
use image::imageops::FilterType;
use image::{Bgr, DynamicImage, ImageBuffer, ImageError, ImageFormat, Rgb, Rgba};
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut, draw_text_mut, Canvas};
use imageproc::rect::Rect;
use std::path::Path;

pub struct JsImage(pub DynamicImage);

impl JsImage {
    pub fn resize(&self, w: u32, h: u32) -> JsImage {
        JsImage(self.0.resize(w, h, FilterType::Nearest))
    }

    pub fn draw_filled_rect(&mut self, (x, y): (i32, i32), (w, h): (u32, u32), color: [u8; 4]) {
        let rect = Rect::at(x, y).of_size(w, h);
        draw_filled_rect_mut(&mut self.0, rect, Rgba::from(color))
    }

    pub fn draw_hollow_rect(&mut self, (x, y): (i32, i32), (w, h): (u32, u32), color: [u8; 4]) {
        let rect = Rect::at(x, y).of_size(w, h);
        draw_hollow_rect_mut(&mut self.0, rect, Rgba::from(color))
    }

    pub fn save_to_file<T: AsRef<Path>>(&self, path: T) -> Result<(), String> {
        self.0.save(path).map_err(|e| e.to_string())
    }

    pub fn save_to_buf(&self, form_str: &str) -> Result<Vec<u8>, String> {
        let mut buf = vec![];
        let form = match form_str {
            "jpg" | "jpeg" => ImageFormat::Jpeg,
            "png" => ImageFormat::Png,
            _ => return Err(format!("no supper image format:{}", form_str)),
        };
        self.0.write_to(&mut buf, form).map_err(|e| e.to_string())?;
        Ok(buf)
    }
}
// bindings
unsafe extern "C" fn save_to_file(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    if argv.is_null() || argc == 0 {
        return js_throw_error(ctx, "param path or data is require");
    }
    let img_ptr = JS_GetOpaque(this_val, JS_IMG_CLASS_ID) as *mut JsImage;
    if img_ptr.is_null() {
        return js_exception();
    }
    let img = img_ptr.as_mut().unwrap();

    let param = *argv;
    return if JS_IsString_real(param) > 0 {
        let path = match to_string(ctx, param) {
            Ok(path) => path,
            Err(e) => return js_throw_error(ctx, e),
        };
        let r = img.save_to_file(path);
        match r {
            Ok(_) => js_undefined(),
            Err(e) => js_throw_error(ctx, e),
        }
    } else {
        js_exception()
    };
}

// unsafe rust

unsafe extern "C" fn js_finalizer(rt: *mut JSRuntime, val: JSValue) {
    let s = JS_GetOpaque(val, JS_IMG_CLASS_ID) as *mut JsImage;
    if !s.is_null() {
        Box::from_raw(s);
    }
}

unsafe extern "C" fn js_ctor(
    ctx: *mut JSContext,
    new_target: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    if argv.is_null() || argc == 0 {
        return js_throw_error(ctx, "param path or data is require");
    }
    let param = *argv;
    let img = if JS_IsString_real(param) > 0 {
        let path = match to_string(ctx, param) {
            Ok(path) => path,
            Err(e) => return js_throw_error(ctx, e),
        };
        let img = match image::open(path) {
            Ok(img) => img,
            Err(e) => return js_throw_error(ctx, e.to_string()),
        };
        Box::new(JsImage(img))
    } else {
        let mut psize = 0;
        let ptr = JS_GetArrayBuffer(ctx, &mut psize, param);
        if ptr.is_null() || psize == 0 {
            return js_throw_error(ctx, "param error");
        }
        let buf = Vec::from_raw_parts(ptr, psize, psize);
        let img = image::load_from_memory(buf.as_slice());
        Vec::leak(buf);
        let img = match img {
            Ok(img) => img,
            Err(e) => return js_throw_error(ctx, e.to_string()),
        };
        Box::new(JsImage(img))
    };

    let proto = JS_GetPropertyStr(ctx, new_target, make_c_string("prototype").as_ptr());
    if JS_IsException_real(proto) > 0 {
        return proto;
    }
    let obj = JS_NewObjectProtoClass(ctx, proto, JS_IMG_CLASS_ID);
    JS_FreeValue_real(ctx, proto);

    let ptr_data = Box::leak(img);

    JS_SetOpaque(obj, (ptr_data as *mut JsImage).cast());

    return obj;
}

pub static mut JS_IMG_CLASS_ID: JSClassID = 0;
pub static mut JS_IMG_CLASS: JSValue = 0;

static mut JS_IMG_CLASS_DEF: JSClassDef = JSClassDef {
    class_name: "Image\0".as_ptr() as *const i8,
    finalizer: Some(js_finalizer),
    gc_mark: None,
    call: None,
    exotic: ::std::ptr::null_mut(),
};

static mut JS_IMG_PROTO_FUNCS: [JSCFunctionListEntry; 1] =
    [CFUNC_DEF!("save_to_file\0", save_to_file, 1)];

unsafe extern "C" fn js_module_init(
    ctx: *mut JSContext,
    m: *mut JSModuleDef,
) -> ::std::os::raw::c_int {
    JS_NewClassID(&mut JS_IMG_CLASS_ID);
    JS_NewClass(JS_GetRuntime(ctx), JS_IMG_CLASS_ID, &JS_IMG_CLASS_DEF);

    let proto = JS_NewObject(ctx);
    JS_SetPropertyFunctionList(
        ctx,
        proto,
        JS_IMG_PROTO_FUNCS.as_ptr(),
        JS_IMG_PROTO_FUNCS.len() as i32,
    );

    JS_IMG_CLASS = JS_NewCFunction2(
        ctx,
        Some(js_ctor),
        make_c_string("Image").as_ptr(),
        1,
        JSCFunctionEnum_JS_CFUNC_constructor,
        0,
    );

    JS_SetConstructor(ctx, JS_IMG_CLASS, proto);
    JS_SetClassProto(ctx, JS_IMG_CLASS_ID, proto);

    JS_SetModuleExport(ctx, m, make_c_string("Image").as_ptr(), JS_IMG_CLASS);
    0
}

pub unsafe fn init_module_image(ctx: *mut JSContext) -> *mut JSModuleDef {
    let name = make_c_string("image");
    let m = JS_NewCModule(ctx, name.as_ptr(), Some(js_module_init));
    if m.is_null() {
        return m;
    }
    JS_AddModuleExport(ctx, m, make_c_string("Image").as_ptr());
    return m;
}
