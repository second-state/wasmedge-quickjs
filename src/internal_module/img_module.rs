use crate::*;
use image::imageops::FilterType;
use image::{Bgr, DynamicImage, ImageFormat, Rgba};
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut, draw_text_mut, Canvas};
use imageproc::rect::Rect;
use std::path::Path;

struct JsImage(pub DynamicImage);

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

impl JsImage {
    fn js_save_to_file(
        &mut self,
        _: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let path = if let Some(JsValue::String(p)) = argv.get(0) {
            p.to_string()
        } else {
            return ctx.throw_type_error("'path' must be of type string").into();
        };

        let r = self.save_to_file(path);
        if let Err(e) = r {
            ctx.throw_internal_type_error(e.as_str()).into()
        } else {
            JsValue::UnDefined
        }
    }

    fn js_save_to_buf(&mut self, _: &mut JsObject, ctx: &mut Context, argv: &[JsValue]) -> JsValue {
        let fmt = if let Some(JsValue::String(p)) = argv.get(0) {
            p.to_string()
        } else {
            return ctx.throw_type_error("'fmt' must be of type string").into();
        };

        let r = self.save_to_buf(fmt.as_str());
        match r {
            Ok(d) => ctx.new_array_buffer(d.as_slice()).into(),
            Err(e) => ctx.throw_internal_type_error(e.as_str()).into(),
        }
    }

    fn js_resize(&mut self, _: &mut JsObject, ctx: &mut Context, argv: &[JsValue]) -> JsValue {
        let w = if let Some(JsValue::Int(w)) = argv.get(0) {
            *w
        } else {
            return ctx.throw_type_error("'w' must be of type int").into();
        };

        let h = if let Some(JsValue::Int(h)) = argv.get(1) {
            *h
        } else {
            return ctx.throw_type_error("'h' must be of type int").into();
        };

        let new_img = self.resize(w as u32, h as u32);
        Self::wrap_obj(ctx, new_img)
    }

    fn js_pixels(&mut self, _: &mut JsObject, ctx: &mut Context, _argv: &[JsValue]) -> JsValue {
        ctx.new_array_buffer(self.pixels()).into()
    }

    fn js_pixels_32f(&mut self, _: &mut JsObject, ctx: &mut Context, _argv: &[JsValue]) -> JsValue {
        let pixels = self.pixels();
        let mut pixels_32f = vec![0f32; pixels.len()];
        for (i, p) in pixels.iter().enumerate() {
            pixels_32f[i] = (*p as f32) / 255.;
        }

        ctx.new_array_buffer_t(pixels_32f.as_slice()).into()
    }

    fn js_to_rgb(&mut self, _: &mut JsObject, ctx: &mut Context, _argv: &[JsValue]) -> JsValue {
        let new_img = self.to_rgb();
        Self::wrap_obj(ctx, new_img)
    }

    fn js_to_bgr(&mut self, _: &mut JsObject, ctx: &mut Context, _argv: &[JsValue]) -> JsValue {
        let new_img = self.to_bgr();
        Self::wrap_obj(ctx, new_img)
    }

    fn js_to_luma(&mut self, _: &mut JsObject, ctx: &mut Context, _argv: &[JsValue]) -> JsValue {
        let new_img = self.to_luma();
        Self::wrap_obj(ctx, new_img)
    }

    fn js_draw_hollow_rect(
        &mut self,
        _: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let top_x = if let Some(JsValue::Int(v)) = argv.get(0) {
            *v
        } else {
            return ctx.throw_type_error("'top_x' must be of type int").into();
        };

        let top_y = if let Some(JsValue::Int(v)) = argv.get(1) {
            *v
        } else {
            return ctx.throw_type_error("'top_y' must be of type int").into();
        };

        let w = if let Some(JsValue::Int(v)) = argv.get(2) {
            *v as u32
        } else {
            return ctx.throw_type_error("'w' must be of type int").into();
        };

        let h = if let Some(JsValue::Int(v)) = argv.get(3) {
            *v as u32
        } else {
            return ctx.throw_type_error("'h' must be of type int").into();
        };

        let color = if let Some(JsValue::Int(v)) = argv.get(4) {
            *v as u32
        } else {
            return ctx.throw_type_error("'color' must be of type int").into();
        };

        let color_arr = [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255u8];

        self.draw_hollow_rect((top_x, top_y), (w, h), color_arr);

        JsValue::UnDefined
    }

    fn js_draw_filled_rect(
        &mut self,
        _: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let top_x = if let Some(JsValue::Int(v)) = argv.get(0) {
            *v
        } else {
            return ctx.throw_type_error("'top_x' must be of type int").into();
        };

        let top_y = if let Some(JsValue::Int(v)) = argv.get(1) {
            *v
        } else {
            return ctx.throw_type_error("'top_y' must be of type int").into();
        };

        let w = if let Some(JsValue::Int(v)) = argv.get(2) {
            *v as u32
        } else {
            return ctx.throw_type_error("'w' must be of type int").into();
        };

        let h = if let Some(JsValue::Int(v)) = argv.get(3) {
            *v as u32
        } else {
            return ctx.throw_type_error("'h' must be of type int").into();
        };

        let color = if let Some(JsValue::Int(v)) = argv.get(4) {
            *v as u32
        } else {
            return ctx.throw_type_error("'color' must be of type int").into();
        };

        let color_arr = [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255u8];

        self.draw_filled_rect((top_x, top_y), (w, h), color_arr);

        JsValue::UnDefined
    }
}

impl JsClassDef for JsImage {
    type RefType = Self;
    const CLASS_NAME: &'static str = "Image";
    const CONSTRUCTOR_ARGC: u8 = 1;

    fn constructor_fn(ctx: &mut Context, argv: &[JsValue]) -> Result<JsImage, JsValue> {
        let param = argv.get(0).ok_or(JsValue::UnDefined)?;
        match param {
            JsValue::String(path) => {
                let path = path.to_string();
                let img = image::open(path)
                    .map_err(|e| ctx.throw_internal_type_error(format!("{}", e).as_str()))?;
                Ok(JsImage(img))
            }
            JsValue::ArrayBuffer(data) => {
                let img = image::load_from_memory(data.as_ref())
                    .map_err(|e| ctx.throw_internal_type_error(format!("{}", e).as_str()))?;
                Ok(JsImage(img))
            }
            _ => Err(JsValue::UnDefined),
        }
    }

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    const FIELDS: &'static [JsClassField<Self::RefType>] = &[];
    const METHODS: &'static [JsClassMethod<Self::RefType>] = &[
        ("save_to_file", 1, Self::js_save_to_file),
        ("save_to_buf", 1, Self::js_save_to_buf),
        ("resize", 2, Self::js_resize),
        ("pixels", 0, Self::js_pixels),
        ("pixels_32f", 0, Self::js_pixels_32f),
        ("to_rgb", 0, Self::js_to_rgb),
        ("to_bgr", 0, Self::js_to_bgr),
        ("to_luma", 0, Self::js_to_luma),
        ("draw_hollow_rect", 5, Self::js_draw_hollow_rect),
        ("draw_filled_rect", 5, Self::js_draw_filled_rect),
    ];
}

struct ImageModule;

impl ModuleInit for ImageModule {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let class_ctor = register_class::<JsImage>(ctx);
        m.add_export("Image\0", class_ctor);
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module("image\0", ImageModule, &["Image\0"]);
}
