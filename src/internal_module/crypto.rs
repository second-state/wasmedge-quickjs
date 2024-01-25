use crate::event_loop::wasi_fs::{Errno, Size};
use crate::quickjs_sys::*;
use crate::EventLoop;
use core::arch;
use crypto_wasi::{
    generate_key_pair, hkdf_hmac, pbkdf2, raw, scrypt, Cipheriv, Decipheriv, Hash, Hmac,
    KeyEncodingFormat, PrivateKey, PrivateKeyEncodingType, PublicKey, PublicKeyEncodingType,
};

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
            password.as_ref(),
            salt.as_ref(),
            *iters as usize,
            *key_len as usize,
            alg.as_str(),
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

fn gen_keypair(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let alg = get_arg!(argv, JsValue::String, 0);
    match { generate_key_pair(alg.as_str()) } {
        Ok((pk, sk)) => {
            let js_pk = JsKeyObjectHandle::PubKey(pk);
            let js_sk = JsKeyObjectHandle::PriKey(sk);
            let mut arr = ctx.new_array();
            arr.put(0, JsKeyObjectHandle::wrap_obj(ctx, js_pk));
            arr.put(1, JsKeyObjectHandle::wrap_obj(ctx, js_sk));
            JsValue::Array(arr)
        }
        Err(e) => {
            let err = errno_to_js_object(ctx, e);
            JsValue::Exception(ctx.throw_error(err))
        }
    }
}

struct JsHash {
    handle: Hash,
}

impl JsHash {
    pub fn js_update(
        &mut self,
        _this: &mut JsObject,
        _ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let data = get_arg!(argv, JsValue::ArrayBuffer, 0);
        if let Ok(()) = self.handle.update(data.as_ref()) {
            JsValue::Bool(true)
        } else {
            JsValue::Bool(false)
        }
    }

    pub fn js_digest(
        &mut self,
        _this: &mut JsObject,
        ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        if let Ok(res) = self.handle.digest() {
            ctx.new_array_buffer(&res).into()
        } else {
            JsValue::UnDefined
        }
    }

    fn copy(&self) -> Result<Self, raw::CryptoErrno> {
        self.handle.copy().map(|h| JsHash { handle: h })
    }
}

impl JsClassDef for JsHash {
    type RefType = JsHash;

    const CLASS_NAME: &'static str = "JsHash";

    const CONSTRUCTOR_ARGC: u8 = 1;

    const FIELDS: &'static [JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [JsClassMethod<Self::RefType>] = &[
        ("update", 1, Self::js_update),
        ("digest", 0, Self::js_digest),
    ];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(ctx: &mut Context, argv: &[JsValue]) -> Result<Self::RefType, JsValue> {
        match argv.get(0) {
            Some(JsValue::String(alg)) => Hash::create(alg.as_str())
                .or_else(|e| {
                    let err = errno_to_js_object(ctx, e);
                    Err(JsValue::Exception(ctx.throw_error(err)))
                })
                .map(|h| JsHash { handle: h }),
            Some(obj) => JsHash::opaque(obj).ok_or(JsValue::UnDefined).and_then(|h| {
                h.copy().or_else(|e| {
                    let err = errno_to_js_object(ctx, e);
                    Err(JsValue::Exception(ctx.throw_error(err)))
                })
            }),
            _ => Err(JsValue::UnDefined),
        }
    }
}

struct JsHmac {
    handle: Hmac,
}

impl JsHmac {
    pub fn js_update(
        &mut self,
        _this: &mut JsObject,
        _ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let data = get_arg!(argv, JsValue::ArrayBuffer, 0);
        if let Ok(()) = self.handle.update(data.as_ref()) {
            JsValue::Bool(true)
        } else {
            JsValue::Bool(false)
        }
    }

    pub fn js_digest(
        &mut self,
        _this: &mut JsObject,
        ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        if let Ok(res) = self.handle.digest() {
            ctx.new_array_buffer(&res).into()
        } else {
            JsValue::UnDefined
        }
    }
}

impl JsClassDef for JsHmac {
    type RefType = JsHmac;

    const CLASS_NAME: &'static str = "JsHmac";

    const CONSTRUCTOR_ARGC: u8 = 2;

    const FIELDS: &'static [JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [JsClassMethod<Self::RefType>] = &[
        ("update", 1, Self::js_update),
        ("digest", 0, Self::js_digest),
    ];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(ctx: &mut Context, argv: &[JsValue]) -> Result<Self::RefType, JsValue> {
        match (argv.get(0), argv.get(1)) {
            (Some(JsValue::String(alg)), Some(JsValue::ArrayBuffer(key))) => {
                Hmac::create(alg.as_str(), key.as_ref())
                    .or_else(|e| {
                        let err = errno_to_js_object(ctx, e);
                        Err(JsValue::Exception(ctx.throw_error(err)))
                    })
                    .map(|h| JsHmac { handle: h })
            }
            _ => Err(JsValue::UnDefined),
        }
    }
}

enum JsCipher {
    Cipher(Cipheriv),
    Decipher(Decipheriv),
}

impl JsCipher {
    pub fn js_update(
        &mut self,
        _this: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        if let Some(JsValue::ArrayBuffer(buf)) = argv.get(0) {
            match self {
                JsCipher::Cipher(c) => c.update(buf.as_ref()),
                JsCipher::Decipher(d) => d.update(buf.as_ref()),
            }
            .map_or(JsValue::UnDefined, |()| ctx.new_array_buffer(&[]).into())
        } else {
            JsValue::UnDefined
        }
    }

    pub fn js_set_aad(
        &mut self,
        _this: &mut JsObject,
        _ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        if let Some(JsValue::ArrayBuffer(buf)) = argv.get(0) {
            match self {
                JsCipher::Cipher(c) => c.set_aad(buf.as_ref()),
                JsCipher::Decipher(d) => d.set_aad(buf.as_ref()),
            }
            .map_or(JsValue::UnDefined, |()| JsValue::Bool(true))
        } else {
            JsValue::UnDefined
        }
    }

    pub fn js_set_auth_tag(
        &mut self,
        _this: &mut JsObject,
        _ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        if let Some(JsValue::ArrayBuffer(buf)) = argv.get(0) {
            match self {
                JsCipher::Cipher(_) => JsValue::UnDefined,
                JsCipher::Decipher(d) => d
                    .set_auth_tag(buf.as_ref())
                    .map_or(JsValue::UnDefined, |()| JsValue::Bool(true)),
            }
        } else {
            JsValue::UnDefined
        }
    }

    pub fn js_get_auth_tag(
        &mut self,
        _this: &mut JsObject,
        ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        match self {
            JsCipher::Cipher(c) => c
                .get_auth_tag()
                .map_or(JsValue::UnDefined, |tag| ctx.new_array_buffer(&tag).into()),
            JsCipher::Decipher(_) => JsValue::UnDefined,
        }
    }

    pub fn js_final(
        &mut self,
        _this: &mut JsObject,
        ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        match self {
            JsCipher::Cipher(c) => c.fin(),
            JsCipher::Decipher(d) => d.fin(),
        }
        .map_or(JsValue::UnDefined, |res| ctx.new_array_buffer(&res).into())
    }

    pub fn js_set_auto_padding(
        &mut self,
        _this: &mut JsObject,
        _ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        true.into()
    }
}

impl JsClassDef for JsCipher {
    type RefType = Self;

    const CLASS_NAME: &'static str = "JsCipher";

    const CONSTRUCTOR_ARGC: u8 = 5;

    const FIELDS: &'static [JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [JsClassMethod<Self::RefType>] = &[
        ("update", 1, Self::js_update),
        ("final", 0, Self::js_final),
        ("setAAD", 0, Self::js_set_aad),
        ("setAuthTag", 0, Self::js_set_auth_tag),
        ("getAuthTag", 0, Self::js_get_auth_tag),
        ("setAutoPadding", 0, Self::js_set_auto_padding),
    ];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(ctx: &mut Context, argv: &[JsValue]) -> Result<Self::RefType, JsValue> {
        if let (
            Some(JsValue::String(alg)),
            Some(JsValue::ArrayBuffer(key)),
            Some(JsValue::ArrayBuffer(iv)),
            Some(JsValue::Bool(is_encrypt)),
        ) = (argv.get(0), argv.get(1), argv.get(2), argv.get(4))
        {
            if *is_encrypt {
                Cipheriv::create(alg.as_str(), key.as_ref(), iv.as_ref())
                    .or_else(|e| {
                        let err = errno_to_js_object(ctx, e);
                        Err(JsValue::Exception(ctx.throw_error(err)))
                    })
                    .map(|c| JsCipher::Cipher(c))
            } else {
                Decipheriv::create(alg.as_str(), key.as_ref(), iv.as_ref())
                    .or_else(|e| {
                        let err = errno_to_js_object(ctx, e);
                        Err(JsValue::Exception(ctx.throw_error(err)))
                    })
                    .map(|c| JsCipher::Decipher(c))
            }
        } else {
            Err(JsValue::UnDefined)
        }
    }
}

enum JsKeyObjectHandle {
    PubKey(PublicKey),
    PriKey(PrivateKey),
}

impl JsKeyObjectHandle {
    pub fn js_export(
        &mut self,
        _this: &mut JsObject,
        ctx: &mut Context,
        argv: &[JsValue],
    ) -> JsValue {
        let skenc_enums = [
            PrivateKeyEncodingType::Pkcs1,
            PrivateKeyEncodingType::Pkcs8,
            PrivateKeyEncodingType::Sec1,
        ];
        let pkenc_enums = [PublicKeyEncodingType::Pkcs1, PublicKeyEncodingType::Spki];
        let format_enums = [
            KeyEncodingFormat::Der,
            KeyEncodingFormat::Pem,
            KeyEncodingFormat::Jwk,
        ];
        let enc = get_arg!(argv, JsValue::Int, 0);
        let format = get_arg!(argv, JsValue::Int, 0);
        match self {
            JsKeyObjectHandle::PriKey(sk) => {
                return match sk.export(skenc_enums[*enc as usize], format_enums[*format as usize]) {
                    Ok(res) => ctx.new_array_buffer(res.as_slice()).into(),
                    Err(e) => {
                        let err = errno_to_js_object(ctx, e);
                        JsValue::Exception(ctx.throw_error(err))
                    }
                }
            }
            JsKeyObjectHandle::PubKey(pk) => {
                return match pk.export(pkenc_enums[*enc as usize], format_enums[*format as usize]) {
                    Ok(res) => ctx.new_array_buffer(res.as_slice()).into(),
                    Err(e) => {
                        let err = errno_to_js_object(ctx, e);
                        JsValue::Exception(ctx.throw_error(err))
                    }
                }
            }
        };
    }
}

impl JsClassDef for JsKeyObjectHandle {
    type RefType = Self;

    const CLASS_NAME: &'static str = "JsKeyObjectHandle";

    const CONSTRUCTOR_ARGC: u8 = 0;

    const FIELDS: &'static [JsClassField<Self::RefType>] = &[];

    const METHODS: &'static [JsClassMethod<Self::RefType>] = &[("export", 2, Self::js_export)];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    // can't construct by user
    fn constructor_fn(_ctx: &mut Context, _argv: &[JsValue]) -> Result<Self::RefType, JsValue> {
        Err(JsValue::UnDefined)
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
        m.add_export(
            "gen_keypair\0",
            ctx.wrap_function("gen_keypair", gen_keypair).into(),
        );
        m.add_export(JsHash::CLASS_NAME, register_class::<JsHash>(ctx));
        m.add_export(JsHmac::CLASS_NAME, register_class::<JsHmac>(ctx));
        m.add_export(JsCipher::CLASS_NAME, register_class::<JsCipher>(ctx));
        m.add_export(
            JsKeyObjectHandle::CLASS_NAME,
            register_class::<JsKeyObjectHandle>(ctx),
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
            "gen_keypair\0",
            JsHash::CLASS_NAME,
            JsHmac::CLASS_NAME,
            JsCipher::CLASS_NAME,
            JsKeyObjectHandle::CLASS_NAME,
        ],
    )
}
