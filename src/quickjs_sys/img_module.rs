use super::*;
use image::imageops::FilterType;
use image::{Bgr, DynamicImage, ImageFormat, Rgba};
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut, draw_text_mut, Canvas};
use imageproc::rect::Rect;
use std::path::Path;

pub struct JsImage(pub DynamicImage);

impl JsImage {
    pub fn resize(&self, w: u32, h: u32) -> JsImage {
        JsImage(self.0.resize_exact(w, h, FilterType::Nearest))
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

    pub fn to_rgb(&self) -> Self {
        JsImage(DynamicImage::ImageRgb8(self.0.to_rgb8()))
    }

    pub fn to_bgr(&self) -> Self {
        JsImage(DynamicImage::ImageBgr8(self.0.to_bgr8()))
    }

    pub fn to_luma(&self) -> Self {
        JsImage(DynamicImage::ImageLuma8(self.0.to_luma8()))
    }

    pub fn pixels(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
// bindings
unsafe extern "C" fn bind_save_to_file(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    if argv.is_null() || argc < 1 {
        return js_throw_type_error(ctx, "too few arguments to function ‘save_to_file’");
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
            Err(e) => return js_throw_type_error(ctx, e),
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

unsafe extern "C" fn bind_save_to_buf(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    if argv.is_null() || argc == 0 {
        return js_throw_type_error(ctx, "too few arguments to function ‘save_to_buf’");
    }
    let img_ptr = JS_GetOpaque(this_val, JS_IMG_CLASS_ID) as *mut JsImage;
    if img_ptr.is_null() {
        return js_exception();
    }
    let img = img_ptr.as_mut().unwrap();

    let param = *argv;
    return if JS_IsString_real(param) > 0 {
        let format = match to_string(ctx, param) {
            Ok(path) => path,
            Err(e) => return js_throw_type_error(ctx, e),
        };
        let data = match img.save_to_buf(format.as_str()) {
            Ok(buf) => buf,
            Err(e) => return js_throw_error(ctx, e.to_string()),
        };
        JS_NewArrayBufferCopy(ctx, data.as_ptr(), data.len())
    } else {
        js_exception()
    };
}

unsafe extern "C" fn bind_pixels(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    let img_ptr = JS_GetOpaque(this_val, JS_IMG_CLASS_ID) as *mut JsImage;
    if img_ptr.is_null() {
        return js_exception();
    }
    let img = img_ptr.as_mut().unwrap();

    let pixels = img.pixels();

    JS_NewArrayBufferCopy(ctx, pixels.as_ptr(), pixels.len())
}

unsafe extern "C" fn bind_pixels_32f(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    let img_ptr = JS_GetOpaque(this_val, JS_IMG_CLASS_ID) as *mut JsImage;
    if img_ptr.is_null() {
        return js_exception();
    }
    let img = img_ptr.as_mut().unwrap();

    let pixels = img.pixels();
    let mut pixels_32f = vec![0f32; pixels.len()];
    for (i, p) in pixels.iter().enumerate() {
        pixels_32f[i] = (*p as f32) / 255.;
    }

    JS_NewArrayBufferCopy(
        ctx,
        pixels_32f.as_ptr().cast(),
        pixels_32f.len() * std::mem::size_of::<f32>(),
    )
}

unsafe extern "C" fn bind_resize(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    let mut w = 0;
    let mut h = 0;
    if argv.is_null() || argc < 2 {
        return js_throw_type_error(ctx, "too few arguments to function ‘resize’");
    }
    if JS_ToUint32_real(ctx, &mut w, *argv.offset(0)) > 0 {
        return js_exception();
    }
    if JS_ToUint32_real(ctx, &mut h, *argv.offset(1)) > 0 {
        return js_exception();
    }
    let img_ptr = JS_GetOpaque(this_val, JS_IMG_CLASS_ID) as *mut JsImage;
    if img_ptr.is_null() {
        return js_exception();
    }
    let img = img_ptr.as_mut().unwrap();

    let new_img = img.resize(w, h);
    JsImage_to_JSValue(ctx, Box::new(new_img))
}

unsafe extern "C" fn bind_draw_hollow_rect(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    let mut top_x = 0;
    let mut top_y = 0;
    let mut w = 0;
    let mut h = 0;
    let mut color = 0;
    if argv.is_null() || argc < 5 {
        return js_throw_type_error(ctx, "too few arguments to function ‘hollow_rect’");
    }
    if JS_ToInt32(ctx, &mut top_x, *argv.offset(0)) > 0 {
        return js_exception();
    }
    if JS_ToInt32(ctx, &mut top_y, *argv.offset(1)) > 0 {
        return js_exception();
    }
    if JS_ToUint32_real(ctx, &mut w, *argv.offset(2)) > 0 {
        return js_exception();
    }
    if JS_ToUint32_real(ctx, &mut h, *argv.offset(3)) > 0 {
        return js_exception();
    }
    if JS_ToUint32_real(ctx, &mut color, *argv.offset(4)) > 0 {
        return js_exception();
    }

    let img_ptr = JS_GetOpaque(this_val, JS_IMG_CLASS_ID) as *mut JsImage;
    if img_ptr.is_null() {
        return js_exception();
    }
    let img = img_ptr.as_mut().unwrap();

    let color_arr = [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255u8];
    img.draw_hollow_rect((top_x, top_y), (w, h), color_arr);
    js_undefined()
}

unsafe extern "C" fn bind_draw_filled_rect(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    let mut top_x = 0;
    let mut top_y = 0;
    let mut w = 0;
    let mut h = 0;
    let mut color = 0;
    if argv.is_null() || argc < 5 {
        return js_throw_type_error(ctx, "too few arguments to function ‘filled_rect’");
    }
    if JS_ToInt32(ctx, &mut top_x, *argv.offset(0)) > 0 {
        return js_exception();
    }
    if JS_ToInt32(ctx, &mut top_y, *argv.offset(1)) > 0 {
        return js_exception();
    }
    if JS_ToUint32_real(ctx, &mut w, *argv.offset(2)) > 0 {
        return js_exception();
    }
    if JS_ToUint32_real(ctx, &mut h, *argv.offset(3)) > 0 {
        return js_exception();
    }
    if JS_ToUint32_real(ctx, &mut color, *argv.offset(4)) > 0 {
        return js_exception();
    }

    let img_ptr = JS_GetOpaque(this_val, JS_IMG_CLASS_ID) as *mut JsImage;
    if img_ptr.is_null() {
        return js_exception();
    }
    let img = img_ptr.as_mut().unwrap();

    let color_arr = [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255u8];
    img.draw_filled_rect((top_x, top_y), (w, h), color_arr);
    js_undefined()
}

unsafe extern "C" fn bind_to_rgb(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    let img_ptr = JS_GetOpaque(this_val, JS_IMG_CLASS_ID) as *mut JsImage;
    if img_ptr.is_null() {
        return js_exception();
    }
    let img = img_ptr.as_mut().unwrap();
    let new_img = img.to_rgb();
    JsImage_to_JSValue(ctx, Box::new(new_img))
}

unsafe extern "C" fn bind_to_bgr(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    let img_ptr = JS_GetOpaque(this_val, JS_IMG_CLASS_ID) as *mut JsImage;
    if img_ptr.is_null() {
        return js_exception();
    }
    let img = img_ptr.as_mut().unwrap();
    let new_img = img.to_bgr();
    JsImage_to_JSValue(ctx, Box::new(new_img))
}

unsafe extern "C" fn bind_to_luma(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut JSValue,
) -> JSValue {
    let img_ptr = JS_GetOpaque(this_val, JS_IMG_CLASS_ID) as *mut JsImage;
    if img_ptr.is_null() {
        return js_exception();
    }
    let img = img_ptr.as_mut().unwrap();
    let new_img = img.to_luma();
    JsImage_to_JSValue(ctx, Box::new(new_img))
}

// unsafe rust

unsafe extern "C" fn js_finalizer(rt: *mut JSRuntime, val: JSValue) {
    let s = JS_GetOpaque(val, JS_IMG_CLASS_ID) as *mut JsImage;
    if !s.is_null() {
        Box::from_raw(s);
    }
}

unsafe fn JsImage_to_JSValue(ctx: *mut JSContext, img: Box<JsImage>) -> JSValue {
    let obj = JS_NewObjectClass(ctx, JS_IMG_CLASS_ID as i32);
    if JS_IsException_real(obj) > 0 {
        return obj;
    }
    let ptr_data = Box::leak(img);
    JS_SetOpaque(obj, (ptr_data as *mut JsImage).cast());
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
    let img = if JS_IsString_real(param) > 0 {
        let path = match to_string(ctx, param) {
            Ok(path) => path,
            Err(e) => return js_throw_type_error(ctx, e),
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
            return js_exception();
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

    return JsImage_to_JSValue(ctx, img);
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

static mut JS_IMG_PROTO_FUNCS: [JSCFunctionListEntry; 10] = [
    CFUNC_DEF!("save_to_file\0", bind_save_to_file, 1),
    CFUNC_DEF!("save_to_buf\0", bind_save_to_buf, 1),
    CFUNC_DEF!("resize\0", bind_resize, 2),
    CFUNC_DEF!("pixels\0", bind_pixels, 0),
    CFUNC_DEF!("pixels_32f\0", bind_pixels_32f, 0),
    CFUNC_DEF!("to_rgb\0", bind_to_rgb, 0),
    CFUNC_DEF!("to_bgr\0", bind_to_bgr, 0),
    CFUNC_DEF!("to_luma\0", bind_to_luma, 0),
    CFUNC_DEF!("draw_hollow_rect\0", bind_draw_hollow_rect, 5),
    CFUNC_DEF!("draw_filled_rect\0", bind_draw_filled_rect, 5),
];

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
