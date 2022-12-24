mod lib;
mod raw;

use core::arch;

use crate::event_loop::wasi_fs::{Errno, Size};
use crate::quickjs_sys::*;
use crate::EventLoop;

mod wasi_snapshot_preview1 {
    #[link(wasm_import_module = "wasi_snapshot_preview1")]
    extern "C" {
        /// Write high-quality random data into a buffer.
        /// This function blocks when the implementation is unable to immediately
        /// provide sufficient high-quality random data.
        /// This function may execute slowly, so when large mounts of random data are
        /// required, it's advisable to use this function to seed a pseudo-random
        /// number generator, rather than to provide the random data directly.
        pub fn random_get(arg0: i32, arg1: i32) -> i32;
    }
}

/// Write high-quality random data into a buffer.
/// This function blocks when the implementation is unable to immediately
/// provide sufficient high-quality random data.
/// This function may execute slowly, so when large mounts of random data are
/// required, it's advisable to use this function to seed a pseudo-random
/// number generator, rather than to provide the random data directly.
///
/// ## Parameters
///
/// * `buf` - The buffer to fill with random data.
unsafe fn random_get(buf: *mut u8, buf_len: Size) -> Result<(), Errno> {
    let ret = wasi_snapshot_preview1::random_get(buf as i32, buf_len as i32);
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

macro_rules! get_arg {
    ($argv:ident, $m:path, $i:expr) => {
        if let Some($m(val)) = $argv.get($i) {
            val
        } else {
            return JsValue::UnDefined;
        }
    };
}

fn timing_safe_equal(_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let a = get_arg!(argv, JsValue::ArrayBuffer, 0);
    let b = get_arg!(argv, JsValue::ArrayBuffer, 1);
    let buf1 = a.as_ref();
    let buf2 = b.as_ref();
    let mut eq = true;
    for i in 0..buf1.len() {
        eq &= buf1[i] == buf2[i];
    }
    eq.into()
}

fn random_fill(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let buf = get_arg!(argv, JsValue::ArrayBuffer, 0);
    let offset = get_arg!(argv, JsValue::Int, 1);
    let size = get_arg!(argv, JsValue::Int, 2);
    return match unsafe {
        let (ptr, buf_len) = buf.get_mut_ptr();
        random_get(
            ptr.offset(*offset as isize),
            (buf_len - *offset as usize).min(*size as usize),
        )
    } {
        Ok(()) => JsValue::UnDefined,
        Err(e) => {
            let err = super::fs::errno_to_js_object(ctx, e);
            JsValue::Exception(ctx.throw_error(err))
        }
    };
}

use self::lib::{hkdf_hmac, pbkdf2, scrypt};

pub fn errno_to_js_object(ctx: &mut Context, e: raw::CryptoErrno) -> JsValue {
    let mut res = ctx.new_object();
    res.set("message", JsValue::String(ctx.new_string(e.message())));
    res.set("code", JsValue::String(ctx.new_string(e.name())));
    res.set("errno", JsValue::Int(e.raw() as i32));
    JsValue::Object(res)
}

fn pbkdf2_sync(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let password = get_arg!(argv, JsValue::ArrayBuffer, 0);
    let salt = get_arg!(argv, JsValue::ArrayBuffer, 1);
    let iters = get_arg!(argv, JsValue::Int, 2);
    let key_len = get_arg!(argv, JsValue::Int, 3);
    let alg = get_arg!(argv, JsValue::String, 4);
    match {
        pbkdf2(
            alg.as_str(),
            password.as_ref(),
            salt.as_ref(),
            *iters as usize,
            *key_len as usize,
        )
    } {
        Ok(res) => ctx.new_array_buffer(res.as_slice()).into(),
        Err(e) => {
            let err = errno_to_js_object(ctx, e);
            JsValue::Exception(ctx.throw_error(err))
        }
    }
}

fn scrypt_sync(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let password = get_arg!(argv, JsValue::ArrayBuffer, 0);
    let salt = get_arg!(argv, JsValue::ArrayBuffer, 1);
    let n = *get_arg!(argv, JsValue::Int, 2);
    let r = *get_arg!(argv, JsValue::Int, 3);
    let p = *get_arg!(argv, JsValue::Int, 4);
    let key_len = *get_arg!(argv, JsValue::Int, 5);
    if key_len == 0 {
        return ctx.new_array_buffer(&vec![0; 0]).into();
    }
    match {
        scrypt(
            password.as_ref(),
            salt.as_ref(),
            n as usize,
            r as usize,
            p as usize,
            key_len as usize,
        )
    } {
        Ok(res) => ctx.new_array_buffer(res.as_slice()).into(),
        Err(e) => {
            let err = errno_to_js_object(ctx, e);
            JsValue::Exception(ctx.throw_error(err))
        }
    }
}

fn hkdf_sync(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let key = get_arg!(argv, JsValue::ArrayBuffer, 0);
    let salt = get_arg!(argv, JsValue::ArrayBuffer, 1);
    let info = get_arg!(argv, JsValue::ArrayBuffer, 2);
    let key_len = get_arg!(argv, JsValue::Int, 3);
    let alg = get_arg!(argv, JsValue::String, 4);
    match {
        hkdf_hmac(
            alg.as_str(),
            key.as_ref(),
            salt.as_ref(),
            info.as_ref(),
            *key_len as usize,
        )
    } {
        Ok(res) => ctx.new_array_buffer(res.as_slice()).into(),
        Err(e) => {
            let err = errno_to_js_object(ctx, e);
            JsValue::Exception(ctx.throw_error(err))
        }
    }
}

struct Crypto;

impl ModuleInit for Crypto {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        m.add_export(
            "timing_safe_equal\0",
            ctx.wrap_function("timing_safe_equal", timing_safe_equal)
                .into(),
        );
        m.add_export(
            "random_fill\0",
            ctx.wrap_function("random_fill", random_fill).into(),
        );
        m.add_export(
            "pbkdf2_sync\0",
            ctx.wrap_function("pbkdf2_sync", pbkdf2_sync).into(),
        );
        m.add_export(
            "scrypt_sync\0",
            ctx.wrap_function("scrypt_sync", scrypt_sync).into(),
        );
        m.add_export(
            "hkdf_sync\0",
            ctx.wrap_function("hkdf_sync", hkdf_sync).into(),
        );
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module(
        "_node:crypto\0",
        Crypto,
        &[
            "timing_safe_equal\0",
            "random_fill\0",
            "pbkdf2_sync\0",
            "scrypt_sync\0",
            "hkdf_sync\0",
        ],
    )
}
