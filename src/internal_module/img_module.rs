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

struct ImageClassDef;
impl JsClassDef<JsImage> for ImageClassDef {
    const CLASS_NAME: &'static str = "Image\0";
    const CONSTRUCTOR_ARGC: u8 = 1;

    fn constructor(ctx: &mut Context, argv: &[JsValue]) -> Result<JsImage, JsValue> {
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

    fn proto_init(p: &mut JsClassProto<JsImage, Self>) {
        struct SaveToFile;
        impl JsMethod<JsImage> for SaveToFile {
            const NAME: &'static str = "save_to_file\0";
            const LEN: u8 = 1;

            fn call(ctx: &mut Context, this_val: &mut JsImage, argv: &[JsValue]) -> JsValue {
                let path = if let Some(JsValue::String(p)) = argv.get(0) {
                    p.to_string()
                } else {
                    return ctx.throw_type_error("'path' is not string").into();
                };

                let r = this_val.save_to_file(path);
                if let Err(e) = r {
                    ctx.throw_internal_type_error(e.as_str()).into()
                } else {
                    JsValue::UnDefined
                }
            }
        }

        struct SaveToBuf;
        impl JsMethod<JsImage> for SaveToBuf {
            const NAME: &'static str = "save_to_buf\0";
            const LEN: u8 = 1;

            fn call(ctx: &mut Context, this_val: &mut JsImage, argv: &[JsValue]) -> JsValue {
                let fmt = if let Some(JsValue::String(p)) = argv.get(0) {
                    p.to_string()
                } else {
                    return ctx.throw_type_error("'fmt' is not string").into();
                };

                let r = this_val.save_to_buf(fmt.as_str());
                match r {
                    Ok(d) => ctx.new_array_buffer(d.as_slice()).into(),
                    Err(e) => ctx.throw_internal_type_error(e.as_str()).into(),
                }
            }
        }

        struct ResizeFn;
        impl JsMethod<JsImage> for ResizeFn {
            const NAME: &'static str = "resize\0";
            const LEN: u8 = 2;

            fn call(ctx: &mut Context, this_val: &mut JsImage, argv: &[JsValue]) -> JsValue {
                let w = if let Some(JsValue::Int(w)) = argv.get(0) {
                    *w
                } else {
                    return ctx.throw_type_error("'w' is not int").into();
                };

                let h = if let Some(JsValue::Int(h)) = argv.get(1) {
                    *h
                } else {
                    return ctx.throw_type_error("'h' is not int").into();
                };

                let new_img = this_val.resize(w as u32, h as u32);
                ImageClassDef::gen_js_obj(ctx, new_img)
            }
        }

        struct Pixels;
        impl JsMethod<JsImage> for Pixels {
            const NAME: &'static str = "pixels\0";
            const LEN: u8 = 0;

            fn call(ctx: &mut Context, this_val: &mut JsImage, _argv: &[JsValue]) -> JsValue {
                let pixels = this_val.pixels();
                ctx.new_array_buffer(pixels).into()
            }
        }

        struct Pixels32f;
        impl JsMethod<JsImage> for Pixels32f {
            const NAME: &'static str = "pixels_32f\0";
            const LEN: u8 = 0;

            fn call(ctx: &mut Context, this_val: &mut JsImage, _argv: &[JsValue]) -> JsValue {
                let pixels = this_val.pixels();
                let mut pixels_32f = vec![0f32; pixels.len()];
                for (i, p) in pixels.iter().enumerate() {
                    pixels_32f[i] = (*p as f32) / 255.;
                }

                ctx.new_array_buffer_t(pixels_32f.as_slice()).into()
            }
        }

        struct RGB;
        impl JsMethod<JsImage> for RGB {
            const NAME: &'static str = "to_rgb\0";
            const LEN: u8 = 0;

            fn call(ctx: &mut Context, this_val: &mut JsImage, _argv: &[JsValue]) -> JsValue {
                let new_img = this_val.to_rgb();
                ImageClassDef::gen_js_obj(ctx, new_img)
            }
        }

        struct BGR;
        impl JsMethod<JsImage> for BGR {
            const NAME: &'static str = "to_bgr\0";
            const LEN: u8 = 0;

            fn call(ctx: &mut Context, this_val: &mut JsImage, _argv: &[JsValue]) -> JsValue {
                let new_img = this_val.to_bgr();
                ImageClassDef::gen_js_obj(ctx, new_img)
            }
        }

        struct LUMA;
        impl JsMethod<JsImage> for LUMA {
            const NAME: &'static str = "to_luma\0";
            const LEN: u8 = 0;

            fn call(ctx: &mut Context, this_val: &mut JsImage, _argv: &[JsValue]) -> JsValue {
                let new_img = this_val.to_luma();
                ImageClassDef::gen_js_obj(ctx, new_img)
            }
        }

        struct DrawHollowRect;
        impl JsMethod<JsImage> for DrawHollowRect {
            const NAME: &'static str = "draw_hollow_rect\0";
            const LEN: u8 = 5;

            fn call(ctx: &mut Context, this_val: &mut JsImage, argv: &[JsValue]) -> JsValue {
                let top_x = if let Some(JsValue::Int(v)) = argv.get(0) {
                    *v
                } else {
                    return ctx.throw_type_error("'top_x' is not int").into();
                };

                let top_y = if let Some(JsValue::Int(v)) = argv.get(1) {
                    *v
                } else {
                    return ctx.throw_type_error("'top_y' is not int").into();
                };

                let w = if let Some(JsValue::Int(v)) = argv.get(2) {
                    *v as u32
                } else {
                    return ctx.throw_type_error("'w' is not int").into();
                };

                let h = if let Some(JsValue::Int(v)) = argv.get(3) {
                    *v as u32
                } else {
                    return ctx.throw_type_error("'h' is not int").into();
                };

                let color = if let Some(JsValue::Int(v)) = argv.get(4) {
                    *v as u32
                } else {
                    return ctx.throw_type_error("'color' is not int").into();
                };

                let color_arr = [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255u8];

                this_val.draw_hollow_rect((top_x, top_y), (w, h), color_arr);

                JsValue::UnDefined
            }
        }

        struct DrawFilledRect;
        impl JsMethod<JsImage> for DrawFilledRect {
            const NAME: &'static str = "draw_filled_rect\0";
            const LEN: u8 = 5;

            fn call(ctx: &mut Context, this_val: &mut JsImage, argv: &[JsValue]) -> JsValue {
                let top_x = if let Some(JsValue::Int(v)) = argv.get(1) {
                    *v
                } else {
                    return ctx.throw_type_error("'top_x' is not int").into();
                };

                let top_y = if let Some(JsValue::Int(v)) = argv.get(2) {
                    *v
                } else {
                    return ctx.throw_type_error("'top_y' is not int").into();
                };

                let w = if let Some(JsValue::Int(v)) = argv.get(3) {
                    *v as u32
                } else {
                    return ctx.throw_type_error("'w' is not int").into();
                };

                let h = if let Some(JsValue::Int(v)) = argv.get(4) {
                    *v as u32
                } else {
                    return ctx.throw_type_error("'h' is not int").into();
                };

                let color = if let Some(JsValue::Int(v)) = argv.get(0) {
                    *v as u32
                } else {
                    return ctx.throw_type_error("'color' is not int").into();
                };

                let color_arr = [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255u8];

                this_val.draw_filled_rect((top_x, top_y), (w, h), color_arr);

                JsValue::UnDefined
            }
        }

        p.add_function::<SaveToFile>();
        p.add_function::<SaveToBuf>();
        p.add_function::<ResizeFn>();
        p.add_function::<Pixels>();
        p.add_function::<Pixels32f>();
        p.add_function::<RGB>();
        p.add_function::<BGR>();
        p.add_function::<LUMA>();
        p.add_function::<DrawFilledRect>();
        p.add_function::<DrawHollowRect>();
    }
}

struct ImageModule;

impl ModuleInit for ImageModule {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let class_ctor = ctx.register_class(ImageClassDef);
        m.add_export("Image\0", class_ctor);
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module("image\0", ImageModule, &["Image\0"]);
}
