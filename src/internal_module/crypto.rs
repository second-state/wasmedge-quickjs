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
                    Ok(()) => JsValue::UnDefined,
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

use wasi_crypto_guest::error::Error;
use wasi_crypto_guest::symmetric::*;

fn pbkdf2(
    alg: &'static str,
    password: &[u8],
    salt: &[u8],
    iters: usize,
    key_len: usize,
) -> Result<Vec<u8>, Error> {
    let tag_len = match alg {
        "HMAC/SHA-256" => 32,
        "HMAC/SHA-512" => 64,
        _ => unreachable!(),
    };
    fn pass(alg: &'static str, key: &[u8], salt: &[u8]) -> Result<Vec<u8>, Error> {
        let mut h = SymmetricState::new(alg, Some(&SymmetricKey::from_raw(alg, key)?), None)?;
        h.absorb(salt)?;
        h.squeeze_tag()
    }
    let res = (0..(key_len + tag_len - 1) / tag_len)
        .map(|idx| -> Result<Vec<u8>, Error> {
            let mut salt_2 = salt.to_vec();
            let idx = idx + 1;
            salt_2.push(((idx >> 24) & 0xff) as u8);
            salt_2.push(((idx >> 16) & 0xff) as u8);
            salt_2.push(((idx >> 8) & 0xff) as u8);
            salt_2.push(((idx) & 0xff) as u8);
            let mut res_t = pass(alg, password, &salt_2)?;
            let mut res_u = res_t.clone();
            for _ in 0..iters - 1 {
                res_u = pass(alg, password, &res_u)?;
                for k in 0..res_t.len() {
                    res_t[k] ^= res_u[k];
                }
            }
            Ok(res_t)
        })
        .filter_map(|v| v.ok())
        .flatten()
        .take(key_len)
        .collect::<Vec<_>>();
    Ok(res)
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

fn pbkdf2_sync(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let password = get_arg!(argv, JsValue::ArrayBuffer, 0);
    let salt = get_arg!(argv, JsValue::ArrayBuffer, 1);
    let iters = get_arg!(argv, JsValue::Int, 2);
    let key_len = get_arg!(argv, JsValue::Int, 3);
    let alg = get_arg!(argv, JsValue::String, 4);
    match {
        pbkdf2(
            match alg.as_str() {
                "SHA256" => "HMAC/SHA-256",
                "SHA512" => "HMAC/SHA-512",
                _ => unreachable!(),
            },
            password.as_ref(),
            salt.as_ref(),
            *iters as usize,
            *key_len as usize,
        )
    } {
        Ok(res) => ctx.new_array_buffer(res.as_slice()).into(),
        Err(_e) => {
            // TODO
            JsValue::UnDefined
        }
    }
}

struct ScryptRom {
    b: Vec<u8>,
    r: usize,
    n: usize,
    p: usize,
    xy: Vec<u8>,
    v: Vec<u8>,
    b32: Vec<i32>,
    x: Vec<i32>,
    xx: Vec<u8>,
}

fn blockxor(a: &[u8], b: &mut [u8]) {
    for i in 0..a.len() {
        b[i] ^= a[i];
    }
}

impl ScryptRom {
    fn romix(&mut self, i: usize, r: usize) {
        let block_start = i * 128 * r;
        let offset = (2 * r - 1) * 64;
        let block_len = 128 * r;
        self.xy[0..block_len].copy_from_slice(&self.b[block_start..(block_start + block_len)]);
        for i1 in 0..self.n {
            self.v[i1 * block_len..(i1 + 1) * block_len].copy_from_slice(&self.xy[0..block_len]);
            self.blockmix(block_len);
        }

        fn read_u32le(p: &[u8]) -> u32 {
            (p[0] as u32) + ((p[1] as u32) << 8) + ((p[2] as u32) << 16) + ((p[3] as u32) << 24)
        }

        for _ in 0..self.n {
            let j = read_u32le(&self.xy[offset..]) as usize & (self.n - 1);
            blockxor(
                &self.v[j * block_len..(j + 1) * block_len],
                &mut self.xy[0..block_len],
            );
            self.blockmix(block_len);
        }
        self.b[block_start..block_start + block_len].copy_from_slice(&self.xy[0..block_len]);
    }

    fn blockmix(&mut self, block_len: usize) {
        self.xx[0..64].copy_from_slice(&self.xy[(2 * self.r - 1) * 64..(2 * self.r) * 64]);
        for i in 0..2 * self.r {
            blockxor(&self.xy[i * 64..(i + 1) * 64], &mut self.xx[0..64]);
            self.salsa20_8();
            self.xy[block_len + (i * 64)..block_len + (i * 64) + 64]
                .copy_from_slice(&self.xx[0..64]);
        }
        for i in 0..self.r {
            self.xy.copy_within(
                block_len + (i * 2) * 64..block_len + (i * 2) * 64 + 64,
                i * 64,
            );
            self.xy.copy_within(
                block_len + (i * 2 + 1) * 64..block_len + (i * 2 + 1) * 64 + 64,
                (i + self.r) * 64,
            );
        }
    }

    fn salsa20_8(&mut self) {
        #[inline(always)]
        #[allow(non_snake_case)]
        fn R(i: i32, r: i32) -> i32 {
            i.rotate_left(r as u32)
        }

        for i in 0..16 {
            self.b32[i] = ((self.xx[i * 4 + 0] & 0xff) as i32) << 0;
            self.b32[i] |= ((self.xx[i * 4 + 1] & 0xff) as i32) << 8;
            self.b32[i] |= ((self.xx[i * 4 + 2] & 0xff) as i32) << 16;
            self.b32[i] |= ((self.xx[i * 4 + 3] & 0xff) as i32) << 24;
        }

        self.x.copy_from_slice(&self.b32);

        for _ in 0..4 {
            self.x[4] ^= R(self.x[0] + self.x[12], 7);
            self.x[8] ^= R(self.x[4] + self.x[0], 9);
            self.x[12] ^= R(self.x[8] + self.x[4], 13);
            self.x[0] ^= R(self.x[12] + self.x[8], 18);
            self.x[9] ^= R(self.x[5] + self.x[1], 7);
            self.x[13] ^= R(self.x[9] + self.x[5], 9);
            self.x[1] ^= R(self.x[13] + self.x[9], 13);
            self.x[5] ^= R(self.x[1] + self.x[13], 18);
            self.x[14] ^= R(self.x[10] + self.x[6], 7);
            self.x[2] ^= R(self.x[14] + self.x[10], 9);
            self.x[6] ^= R(self.x[2] + self.x[14], 13);
            self.x[10] ^= R(self.x[6] + self.x[2], 18);
            self.x[3] ^= R(self.x[15] + self.x[11], 7);
            self.x[7] ^= R(self.x[3] + self.x[15], 9);
            self.x[11] ^= R(self.x[7] + self.x[3], 13);
            self.x[15] ^= R(self.x[11] + self.x[7], 18);
            self.x[1] ^= R(self.x[0] + self.x[3], 7);
            self.x[2] ^= R(self.x[1] + self.x[0], 9);
            self.x[3] ^= R(self.x[2] + self.x[1], 13);
            self.x[0] ^= R(self.x[3] + self.x[2], 18);
            self.x[6] ^= R(self.x[5] + self.x[4], 7);
            self.x[7] ^= R(self.x[6] + self.x[5], 9);
            self.x[4] ^= R(self.x[7] + self.x[6], 13);
            self.x[5] ^= R(self.x[4] + self.x[7], 18);
            self.x[11] ^= R(self.x[10] + self.x[9], 7);
            self.x[8] ^= R(self.x[11] + self.x[10], 9);
            self.x[9] ^= R(self.x[8] + self.x[11], 13);
            self.x[10] ^= R(self.x[9] + self.x[8], 18);
            self.x[12] ^= R(self.x[15] + self.x[14], 7);
            self.x[13] ^= R(self.x[12] + self.x[15], 9);
            self.x[14] ^= R(self.x[13] + self.x[12], 13);
            self.x[15] ^= R(self.x[14] + self.x[13], 18);
        }

        for i in 0..16 {
            self.b32[i] += self.x[i];
        }

        for i in 0..16 {
            self.xx[i * 4 + 0] = (self.b32[i] >> 0 & 0xff) as u8;
            self.xx[i * 4 + 1] = (self.b32[i] >> 8 & 0xff) as u8;
            self.xx[i * 4 + 2] = (self.b32[i] >> 16 & 0xff) as u8;
            self.xx[i * 4 + 3] = (self.b32[i] >> 24 & 0xff) as u8;
        }
    }
}

fn scrypt_rom(b: &[u8], r: usize, n: usize, p: usize) -> Result<Vec<u8>, Error> {
    let mut rom = ScryptRom {
        b: b.to_vec(),
        r,
        n,
        p,
        xy: vec![0; 256 * r],
        v: vec![0; 128 * r * n],
        b32: vec![0; 16],
        x: vec![0; 16],
        xx: vec![0; 64],
    };
    for i in 0..p {
        rom.romix(i, r);
    }
    Ok(rom.b)
}

fn scrypt(
    password: &[u8],
    salt: &[u8],
    n: usize,
    r: usize,
    p: usize,
    keylen: usize,
) -> Result<Vec<u8>, Error> {
    let blen = p * 128 * r;
    let b = pbkdf2("HMAC/SHA-256", password, salt, 1, blen)?;
    let s = scrypt_rom(&b, r, n, p)?;
    let f = pbkdf2("HMAC/SHA-256", password, &s, 1, keylen)?;
    Ok(f)
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
        Err(_e) => {
            // TODO
            JsValue::UnDefined
        }
    }
}

fn hkdf(key: &[u8], salt: &[u8], info: &[u8], key_len: usize) -> Result<Vec<u8>, Error> {
    let mut h = SymmetricState::new(
        "HKDF-EXTRACT/SHA-256",
        Some(&SymmetricKey::from_raw("HKDF-EXTRACT/SHA-256", key)?),
        None,
    )?;
    h.absorb(salt)?;
    let pk = h.squeeze_key("HKDF-EXPAND/SHA-256")?;
    let mut p = SymmetricState::new("HKDF-EXPAND/SHA-256", Some(&pk), None)?;
    p.absorb(info)?;
    p.squeeze(key_len)
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
        ],
    )
}
