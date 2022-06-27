use std::borrow::Cow;

use crate::quickjs_sys::*;
use encoding::{
    all::{
        whatwg::{ISO_8859_8_I, X_USER_DEFINED},
        *,
    },
    DecoderTrap, EncoderTrap, Encoding,
};

fn text_encode(ctx: &mut Context, _: JsValue, params: &[JsValue]) -> JsValue {
    let s = params.get(0);
    let utf_label = match params.get(1).clone() {
        Some(JsValue::String(s)) => s.as_str(),
        _ => "",
    };
    if s.is_none() {
        return JsValue::UnDefined;
    }
    if let JsValue::String(s) = ctx.value_to_string(s.unwrap()) {
        match utf_label {
            "" | "utf8" | "utf-8" => {
                let b = UTF_8.encode(s.as_str(), EncoderTrap::Replace);
                match b {
                    Ok(ret) => ctx.new_array_buffer(&ret).into(),
                    Err(e) => {
                        ctx.throw_type_error(&e);
                        JsValue::UnDefined
                    }
                }
            }
            _ => JsValue::UnDefined,
        }
    } else {
        JsValue::UnDefined
    }
}

fn text_decode(ctx: &mut Context, _: JsValue, params: &[JsValue]) -> JsValue {
    let s = params.get(0);
    let utf_label = match params.get(1).clone() {
        Some(JsValue::String(s)) => s.as_str(),
        _ => "",
    };
    let fatal = match params.get(2) {
        Some(JsValue::Bool(b)) => *b,
        _ => false,
    };

    if s.is_none() {
        return JsValue::UnDefined;
    }

    let trap = if fatal {
        DecoderTrap::Strict
    } else {
        DecoderTrap::Replace
    };

    fn ret_to_js(ctx: &mut Context, ret: Result<String, Cow<str>>) -> JsValue {
        match &ret {
            Ok(s) => ctx.new_string(s).into(),
            Err(e) => ctx.new_error(e).into(),
        }
    }

    if let JsValue::ArrayBuffer(s) = s.unwrap() {
        match utf_label {
            "" | "utf8" | "utf-8" => {
                let b = UTF_8.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "gbk" => {
                let b = GBK.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "gb18030" => {
                let b = GB18030.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "hz-gb-2312" => {
                let b = HZ.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "big5" => {
                let b = BIG5_2003.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "euc-jp" => {
                let b = EUC_JP.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-2022-jp" => {
                let b = ISO_2022_JP.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "utf-16be" => {
                let b = UTF_16BE.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "utf-16le" => {
                let b = UTF_16LE.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "x-user-defined" => {
                let b = X_USER_DEFINED.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "ibm866" => {
                let b = IBM866.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-8859-2" => {
                let b = ISO_8859_2.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-8859-3" => {
                let b = ISO_8859_3.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-8859-4" => {
                let b = ISO_8859_4.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-8859-5" => {
                let b = ISO_8859_5.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-8859-6" => {
                let b = ISO_8859_6.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-8859-7" => {
                let b = ISO_8859_7.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-8859-8" => {
                let b = ISO_8859_8.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-8859-8i" => {
                let b = ISO_8859_8_I.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-8859-10" => {
                let b = ISO_8859_10.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-8859-13" => {
                let b = ISO_8859_13.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-8859-14" => {
                let b = ISO_8859_14.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-8859-15" => {
                let b = ISO_8859_15.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "iso-8859-16" => {
                let b = ISO_8859_16.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "windows-874" => {
                let b = WINDOWS_874.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "windows-1250" => {
                let b = WINDOWS_1250.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "windows-1251" => {
                let b = WINDOWS_1251.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "windows-1252" => {
                let b = WINDOWS_1252.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "windows-1253" => {
                let b = WINDOWS_1253.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "windows-1254" => {
                let b = WINDOWS_1254.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "windows-1255" => {
                let b = WINDOWS_1255.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "windows-1256" => {
                let b = WINDOWS_1256.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "windows-1257" => {
                let b = WINDOWS_1257.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            "windows-1258" => {
                let b = WINDOWS_1258.decode(s.as_ref(), trap);
                ret_to_js(ctx, b)
            }
            _ => JsValue::UnDefined,
        }
    } else {
        JsValue::UnDefined
    }
}

pub fn init_encoding_module(ctx: &mut Context) {
    ctx.register_fn_module(
        "_encoding",
        &["text_encode", "text_decode"],
        |ctx: &mut Context, m: &mut JsModuleDef| {
            let text_encode = ctx.wrap_function("text_encode", text_encode);
            m.add_export("text_encode", text_encode.into());

            let text_decode = ctx.wrap_function("text_decode", text_decode);
            m.add_export("text_decode", text_decode.into());
        },
    );
}
