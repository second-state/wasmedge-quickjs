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

fn timing_safe_equal(_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    if let Some(JsValue::ArrayBuffer(a)) = argv.get(0) {
        if let Some(JsValue::ArrayBuffer(b)) = argv.get(1) {
            let buf1 = a.as_ref();
            let buf2 = b.as_ref();
            let mut eq = true;
            for i in 0..buf1.len() {
                eq &= buf1[i] == buf2[i];
            }
            return eq.into();
        }
    }
    JsValue::UnDefined
}

fn random_fill(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    println!("{:?}", argv);
    if let Some(JsValue::ArrayBuffer(buf)) = argv.get(0) {
        if let Some(JsValue::Int(offset)) = argv.get(1) {
            if let Some(JsValue::Int(size)) = argv.get(2) {
                return match unsafe {
                    let (ptr, buf_len) = buf.get_mut_ptr();
                    random_get(
                        ptr.offset(*offset as isize),
                        (buf_len - *offset as usize).min(*size as usize),
                    )
                } {
                    Ok(()) => {
                        println!("{:?}", buf.to_vec());
                        JsValue::UnDefined
                    }
                    Err(e) => {
                        let err = super::fs::errno_to_js_object(ctx, e);
                        JsValue::Exception(ctx.throw_error(err))
                    }
                };
            }
        }
    }
    JsValue::UnDefined
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
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module(
        "_node:crypto\0",
        Crypto,
        &["timing_safe_equal\0", "random_fill\0"],
    )
}
