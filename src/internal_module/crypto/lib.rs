use super::raw;

pub type CryptoErrno = raw::CryptoErrno;

const NONE_OPTS: raw::OptOptions = raw::OptOptions {
    tag: raw::OPT_OPTIONS_U_NONE.raw(),
    u: raw::OptOptionsUnion { none: () },
};

const NONE_KEY: raw::OptSymmetricKey = raw::OptSymmetricKey {
    tag: raw::OPT_SYMMETRIC_KEY_U_NONE.raw(),
    u: raw::OptSymmetricKeyUnion { none: () },
};

pub struct Hmac {
    handle: raw::SymmetricState,
}

impl Hmac {
    pub fn create<T>(alg: &str, key: T) -> Result<Self, raw::CryptoErrno>
    where
        T: AsRef<[u8]>,
    {
        let alg = match alg {
            "sha256" | "SHA256" | "HMAC/SHA-256" => "HMAC/SHA-256",
            "sha512" | "SHA512" | "HMAC/SHA-512" => "HMAC/SHA-512",
            _ => return Err(raw::CRYPTO_ERRNO_UNSUPPORTED_ALGORITHM),
        };
        let handle = {
            let key = key.as_ref();
            unsafe {
                let key = raw::symmetric_key_import(alg, key.as_ptr(), key.len())?;
                let opt = raw::OptSymmetricKey {
                    tag: raw::OPT_SYMMETRIC_KEY_U_SOME.raw(),
                    u: raw::OptSymmetricKeyUnion { some: key },
                };
                let state = raw::symmetric_state_open(alg, opt, NONE_OPTS)?;
                raw::symmetric_key_close(key)?;
                state
            }
        };
        Ok(Self { handle })
    }

    pub fn update(&mut self, data: impl AsRef<[u8]>) -> Result<(), raw::CryptoErrno> {
        let data = data.as_ref();
        unsafe { raw::symmetric_state_absorb(self.handle, data.as_ptr(), data.len()) }
    }

    pub fn digest(&mut self) -> Result<Vec<u8>, raw::CryptoErrno> {
        unsafe {
            let tag = raw::symmetric_state_squeeze_tag(self.handle)?;
            let len = raw::symmetric_tag_len(tag)?;
            let mut out = vec![0; len];
            raw::symmetric_tag_pull(tag, out.as_mut_ptr(), out.len())?;
            raw::symmetric_tag_close(tag)?;
            Ok(out)
        }
    }

    pub fn digest_into(&mut self, mut buf: impl AsMut<[u8]>) -> Result<(), raw::CryptoErrno> {
        let buf = buf.as_mut();
        unsafe {
            let tag = raw::symmetric_state_squeeze_tag(self.handle)?;
            raw::symmetric_tag_pull(tag, buf.as_mut_ptr(), buf.len())?;
            raw::symmetric_tag_close(tag)?;
        }
        Ok(())
    }
}

impl Drop for Hmac {
    fn drop(&mut self) {
        unsafe {
            raw::symmetric_state_close(self.handle).unwrap();
        }
    }
}

pub struct Hash {
    handle: raw::SymmetricState,
    hash_len: usize,
}

impl Hash {
    pub fn create(alg: &str) -> Result<Self, raw::CryptoErrno> {
        let (alg, hash_len) = match alg {
            "sha256" | "SHA256" | "SHA-256" => ("SHA-256", 32),
            "sha512" | "SHA512" | "SHA-512" => ("SHA-512", 64),
            _ => return Err(raw::CRYPTO_ERRNO_UNSUPPORTED_ALGORITHM),
        };
        let handle = { unsafe { raw::symmetric_state_open(alg, NONE_KEY, NONE_OPTS)? } };
        Ok(Self { handle, hash_len })
    }

    pub fn update(&mut self, data: impl AsRef<[u8]>) -> Result<(), raw::CryptoErrno> {
        let data = data.as_ref();
        unsafe { raw::symmetric_state_absorb(self.handle, data.as_ptr(), data.len()) }
    }

    pub fn digest(&mut self) -> Result<Vec<u8>, raw::CryptoErrno> {
        let mut out = vec![0; self.hash_len];
        self.digest_into(&mut out)?;
        Ok(out)
    }

    pub fn digest_into(&mut self, mut buf: impl AsMut<[u8]>) -> Result<(), raw::CryptoErrno> {
        let buf = buf.as_mut();
        unsafe {
            raw::symmetric_state_squeeze(self.handle, buf.as_mut_ptr(), buf.len())?;
        }
        Ok(())
    }

    pub fn copy(&self) -> Result<Self, raw::CryptoErrno> {
        let h = unsafe { raw::symmetric_state_clone(self.handle) }?;
        Ok(Self {
            handle: h,
            hash_len: self.hash_len,
        })
    }
}

impl Drop for Hash {
    fn drop(&mut self) {
        unsafe {
            raw::symmetric_state_close(self.handle).unwrap();
        }
    }
}

/// Behaviour like
///
/// ```js
/// let hmac = createHmac(alg, key);
/// infos.forEach(info => hmac.update(info));
/// let return = hmac.digest();
/// ```
pub fn hmac(
    alg: &str,
    key: impl AsRef<[u8]>,
    infos: &[impl AsRef<[u8]>],
) -> Result<Vec<u8>, raw::CryptoErrno> {
    let mut hash = Hmac::create(alg, key)?;
    for info in infos {
        hash.update(info)?;
    }
    hash.digest()
}

/// Behaviour like
///
/// ```js
/// let hash = createHash(alg);
/// infos.forEach(info => hash.update(info));
/// let return = hash.digest();
/// ```
pub fn hash(alg: &str, infos: &[impl AsRef<[u8]>]) -> Result<Vec<u8>, raw::CryptoErrno> {
    let mut hash = Hash::create(alg)?;
    for info in infos {
        hash.update(info)?;
    }
    hash.digest()
}

fn hkdf_extract(
    alg: &str,
    key: impl AsRef<[u8]>,
    salt: impl AsRef<[u8]>,
) -> Result<raw::SymmetricKey, raw::CryptoErrno> {
    let (extract_alg, expand_alg) = match alg {
        "sha256" | "SHA256" => ("HKDF-EXTRACT/SHA-256", "HKDF-EXPAND/SHA-256"),
        "sha512" | "SHA512" => ("HKDF-EXTRACT/SHA-512", "HKDF-EXPAND/SHA-512"),
        _ => return Err(raw::CRYPTO_ERRNO_UNSUPPORTED_ALGORITHM),
    };
    let key = key.as_ref();
    let salt = salt.as_ref();
    if !key.is_empty() {
        unsafe {
            let extract_key = raw::symmetric_key_import(extract_alg, key.as_ptr(), key.len())?;
            let extract_handle = raw::symmetric_state_open(
                extract_alg,
                raw::OptSymmetricKey {
                    tag: raw::OPT_SYMMETRIC_KEY_U_SOME.raw(),
                    u: raw::OptSymmetricKeyUnion { some: extract_key },
                },
                NONE_OPTS,
            )?;
            raw::symmetric_state_absorb(extract_handle, salt.as_ptr(), salt.len())?;
            let expand_key = raw::symmetric_state_squeeze_key(extract_handle, expand_alg)?;
            raw::symmetric_state_close(extract_handle)?;
            raw::symmetric_key_close(extract_key)?;
            Ok(expand_key)
        }
    } else {
        let res = hmac(alg, salt, &[key])?;
        unsafe { raw::symmetric_key_import(expand_alg, res.as_ptr(), res.len()) }
    }
}

fn hkdf_extract_raw(
    alg: &str,
    key: impl AsRef<[u8]>,
    salt: impl AsRef<[u8]>,
) -> Result<Vec<u8>, raw::CryptoErrno> {
    hmac(alg, salt, &[key])
}

/// As same as `hkdf`, but use hmac to manual expand
pub fn hkdf_hmac(
    alg: &str,
    key: impl AsRef<[u8]>,
    salt: impl AsRef<[u8]>,
    info: impl AsRef<[u8]>,
    key_len: usize,
) -> Result<Vec<u8>, raw::CryptoErrno> {
    let key = key.as_ref();
    let salt = salt.as_ref();
    let info = info.as_ref();
    let (_, _, hash_len) = match alg {
        "sha256" | "SHA256" => ("HKDF-EXTRACT/SHA-256", "HKDF-EXPAND/SHA-256", 32),
        "sha512" | "SHA512" => ("HKDF-EXTRACT/SHA-512", "HKDF-EXPAND/SHA-512", 64),
        _ => return Err(raw::CRYPTO_ERRNO_UNSUPPORTED_ALGORITHM),
    };
    let expand_key = hkdf_extract_raw(alg, key, salt)?;
    let mut out = vec![0; key_len];
    let mut last = [].as_slice();
    for (idx, chunk) in out.chunks_mut(hash_len).enumerate() {
        let counter = [idx as u8 + 1];
        chunk.clone_from_slice(&hmac(alg, &expand_key, &[last, info, &counter])?[..chunk.len()]);
        last = chunk;
    }
    Ok(out)
}

/// Behaviour like `crypto.hkdfSync`
pub fn hkdf(
    alg: &str,
    key: impl AsRef<[u8]>,
    salt: impl AsRef<[u8]>,
    info: impl AsRef<[u8]>,
    key_len: usize,
) -> Result<Vec<u8>, raw::CryptoErrno> {
    let key = key.as_ref();
    let salt = salt.as_ref();
    let info = info.as_ref();
    let (_, expand_alg) = match alg {
        "sha256" | "SHA256" => ("HKDF-EXTRACT/SHA-256", "HKDF-EXPAND/SHA-256"),
        "sha512" | "SHA512" => ("HKDF-EXTRACT/SHA-512", "HKDF-EXPAND/SHA-512"),
        _ => return Err(raw::CRYPTO_ERRNO_UNSUPPORTED_ALGORITHM),
    };
    let mut out = vec![0; key_len];
    let expand_key = hkdf_extract(alg, key, salt)?;
    unsafe {
        let expand_handle = raw::symmetric_state_open(
            expand_alg,
            raw::OptSymmetricKey {
                tag: raw::OPT_SYMMETRIC_KEY_U_SOME.raw(),
                u: raw::OptSymmetricKeyUnion { some: expand_key },
            },
            NONE_OPTS,
        )?;
        raw::symmetric_state_absorb(expand_handle, info.as_ptr(), info.len())?;
        raw::symmetric_state_squeeze(expand_handle, out.as_mut_ptr(), out.len())?;
        raw::symmetric_state_close(expand_handle)?;
        raw::symmetric_key_close(expand_key)?;
    }
    Ok(out)
}

/// Behaviour like `crypto.pbkdf2Sync`
pub fn pbkdf2(
    alg: &str,
    password: impl AsRef<[u8]>,
    salt: impl AsRef<[u8]>,
    iters: usize,
    key_len: usize,
) -> Result<Vec<u8>, raw::CryptoErrno> {
    let hash_len = match alg {
        "sha256" | "SHA256" | "HMAC/SHA-256" => 32,
        "sha512" | "SHA512" | "HMAC/SHA-512" => 64,
        _ => return Err(raw::CRYPTO_ERRNO_UNSUPPORTED_ALGORITHM),
    };
    let mut out = vec![0; key_len];
    for (idx, chunk) in out.chunks_mut(hash_len).enumerate() {
        let mut salt_2 = salt.as_ref().to_vec();
        let idx = idx + 1;
        salt_2.push(((idx >> 24) & 0xff) as u8);
        salt_2.push(((idx >> 16) & 0xff) as u8);
        salt_2.push(((idx >> 8) & 0xff) as u8);
        salt_2.push(((idx) & 0xff) as u8);
        let mut res_t = hmac(alg, password.as_ref(), &[&salt_2])?;
        let mut res_u = res_t.clone();
        for _ in 0..iters - 1 {
            res_u = hmac(alg, password.as_ref(), &[&res_u])?;
            for k in 0..res_t.len() {
                res_t[k] ^= res_u[k];
            }
        }
        chunk.copy_from_slice(&res_t[..chunk.len()]);
    }
    Ok(out)
}

struct ScryptRom {
    b: Vec<u8>,
    r: usize,
    n: usize,
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
            self.x[4] ^= R(self.x[0].wrapping_add(self.x[12]), 7);
            self.x[8] ^= R(self.x[4].wrapping_add(self.x[0]), 9);
            self.x[12] ^= R(self.x[8].wrapping_add(self.x[4]), 13);
            self.x[0] ^= R(self.x[12].wrapping_add(self.x[8]), 18);
            self.x[9] ^= R(self.x[5].wrapping_add(self.x[1]), 7);
            self.x[13] ^= R(self.x[9].wrapping_add(self.x[5]), 9);
            self.x[1] ^= R(self.x[13].wrapping_add(self.x[9]), 13);
            self.x[5] ^= R(self.x[1].wrapping_add(self.x[13]), 18);
            self.x[14] ^= R(self.x[10].wrapping_add(self.x[6]), 7);
            self.x[2] ^= R(self.x[14].wrapping_add(self.x[10]), 9);
            self.x[6] ^= R(self.x[2].wrapping_add(self.x[14]), 13);
            self.x[10] ^= R(self.x[6].wrapping_add(self.x[2]), 18);
            self.x[3] ^= R(self.x[15].wrapping_add(self.x[11]), 7);
            self.x[7] ^= R(self.x[3].wrapping_add(self.x[15]), 9);
            self.x[11] ^= R(self.x[7].wrapping_add(self.x[3]), 13);
            self.x[15] ^= R(self.x[11].wrapping_add(self.x[7]), 18);
            self.x[1] ^= R(self.x[0].wrapping_add(self.x[3]), 7);
            self.x[2] ^= R(self.x[1].wrapping_add(self.x[0]), 9);
            self.x[3] ^= R(self.x[2].wrapping_add(self.x[1]), 13);
            self.x[0] ^= R(self.x[3].wrapping_add(self.x[2]), 18);
            self.x[6] ^= R(self.x[5].wrapping_add(self.x[4]), 7);
            self.x[7] ^= R(self.x[6].wrapping_add(self.x[5]), 9);
            self.x[4] ^= R(self.x[7].wrapping_add(self.x[6]), 13);
            self.x[5] ^= R(self.x[4].wrapping_add(self.x[7]), 18);
            self.x[11] ^= R(self.x[10].wrapping_add(self.x[9]), 7);
            self.x[8] ^= R(self.x[11].wrapping_add(self.x[10]), 9);
            self.x[9] ^= R(self.x[8].wrapping_add(self.x[11]), 13);
            self.x[10] ^= R(self.x[9].wrapping_add(self.x[8]), 18);
            self.x[12] ^= R(self.x[15].wrapping_add(self.x[14]), 7);
            self.x[13] ^= R(self.x[12].wrapping_add(self.x[15]), 9);
            self.x[14] ^= R(self.x[13].wrapping_add(self.x[12]), 13);
            self.x[15] ^= R(self.x[14].wrapping_add(self.x[13]), 18);
        }

        for i in 0..16 {
            self.b32[i] = self.b32[i].wrapping_add(self.x[i]);
        }

        for i in 0..16 {
            self.xx[i * 4 + 0] = (self.b32[i] >> 0 & 0xff) as u8;
            self.xx[i * 4 + 1] = (self.b32[i] >> 8 & 0xff) as u8;
            self.xx[i * 4 + 2] = (self.b32[i] >> 16 & 0xff) as u8;
            self.xx[i * 4 + 3] = (self.b32[i] >> 24 & 0xff) as u8;
        }
    }
}

fn scrypt_rom(b: &[u8], r: usize, n: usize, p: usize) -> Vec<u8> {
    let mut rom = ScryptRom {
        b: b.to_vec(),
        r,
        n,
        xy: vec![0; 256 * r],
        v: vec![0; 128 * r * n],
        b32: vec![0; 16],
        x: vec![0; 16],
        xx: vec![0; 64],
    };
    for i in 0..p {
        rom.romix(i, r);
    }
    rom.b
}

/// Behaviour like `crypto.scryptSync`
pub fn scrypt(
    password: impl AsRef<[u8]>,
    salt: impl AsRef<[u8]>,
    n: usize,
    r: usize,
    p: usize,
    keylen: usize,
) -> Result<Vec<u8>, raw::CryptoErrno> {
    let blen = p * 128 * r;
    let b = pbkdf2("HMAC/SHA-256", &password, salt, 1, blen)?;
    let s = scrypt_rom(&b, r, n, p);
    let f = pbkdf2("HMAC/SHA-256", &password, &s, 1, keylen)?;
    Ok(f)
}

/// Convert u8 array to hex string,
/// behaviour like `Buffer.from(arr).toString("hex")`
///
/// # Examples
///
/// ```
/// use crypto_wasi::u8array_to_hex;
///
/// assert_eq!(u8array_to_hex([01, 23, 45]), "01172d".to_string());
/// ```
pub fn u8array_to_hex(arr: impl AsRef<[u8]>) -> String {
    arr.as_ref()
        .iter()
        .map(|v| format!("{:02x}", v))
        .collect::<Vec<_>>()
        .join("")
}

/// Convert hex string to u8 array
///
/// # Examples
///
/// ```
/// use crypto_wasi::hex_to_u8array;
///
/// assert_eq!(hex_to_u8array("01172d"), Some(vec![01, 23, 45]));
/// ```
pub fn hex_to_u8array(arr: &str) -> Option<Vec<u8>> {
    if arr.len() % 2 != 0 || arr.chars().any(|v| !v.is_ascii_hexdigit()) {
        return None;
    }

    fn hex_byte_to_u8(h: u8) -> u8 {
        match h {
            b'0'..=b'9' => h - b'0',
            b'a'..=b'f' => 10 + h - b'a',
            b'A'..=b'F' => 10 + h - b'A',
            _ => unreachable!(),
        }
    }

    Some(
        arr.as_bytes()
            .chunks(2)
            .map(|v| (hex_byte_to_u8(v[0]) << 4) + hex_byte_to_u8(v[1]))
            .collect(),
    )
}

pub struct Cipher {
    handle: raw::SymmetricState,
    message: Vec<u8>,
    tag: Option<Vec<u8>>,
}

impl Cipher {
    pub fn create(
        alg: &str,
        key: impl AsRef<[u8]>,
        iv: impl AsRef<[u8]>,
    ) -> Result<Self, raw::CryptoErrno> {
        let alg = match alg {
            "aes-128-gcm" | "AES-128-GCM" => "AES-128-GCM",
            "aes-256-gcm" | "AES-256-GCM" => "AES-256-GCM",
            _ => return Err(raw::CRYPTO_ERRNO_UNSUPPORTED_ALGORITHM),
        };
        let handle = {
            let key = key.as_ref();
            let iv = iv.as_ref();
            unsafe {
                let raw_key = raw::symmetric_key_import(alg, key.as_ptr(), key.len())?;
                let key = raw::OptSymmetricKey {
                    tag: raw::OPT_SYMMETRIC_KEY_U_SOME.raw(),
                    u: raw::OptSymmetricKeyUnion { some: raw_key },
                };
                let opt = raw::options_open(raw::ALGORITHM_TYPE_SYMMETRIC)?;
                raw::options_set(opt, "nonce", iv.as_ptr(), iv.len())?;
                let opts = raw::OptOptions {
                    tag: raw::OPT_OPTIONS_U_SOME.raw(),
                    u: raw::OptOptionsUnion { some: opt },
                };
                let state = raw::symmetric_state_open(alg, key, opts).unwrap();
                raw::symmetric_key_close(raw_key).unwrap();
                state
            }
        };
        Ok(Self {
            handle,
            message: vec![],
            tag: None,
        })
    }

    pub fn set_aad(&mut self, data: impl AsRef<[u8]>) -> Result<(), raw::CryptoErrno> {
        let data = data.as_ref();
        unsafe { raw::symmetric_state_absorb(self.handle, data.as_ptr(), data.len()) }
    }

    /// in WasmEdge implement of wasi-crypto, `encrypt` can't be called multiple times,
    /// multiple call `encrypt` is also not equivalent to multiple call `update`.
    /// so we store all message and concat it, then encrypt one-time on `final`
    pub fn update(&mut self, data: impl AsRef<[u8]>) -> Result<(), raw::CryptoErrno> {
        self.message.extend_from_slice(data.as_ref());
        Ok(())
    }

    /// `final` is reserved keyword, `fin` looks better than `r#final`
    pub fn fin(&mut self) -> Result<Vec<u8>, raw::CryptoErrno> {
        let mut out = vec![0; self.message.len()];
        unsafe {
            let tag = raw::symmetric_state_encrypt_detached(
                self.handle,
                out.as_mut_ptr(),
                out.len(),
                self.message.as_ptr(),
                self.message.len(),
            )?;
            let len = raw::symmetric_tag_len(tag)?;
            let mut buf = vec![0; len];
            raw::symmetric_tag_pull(tag, buf.as_mut_ptr(), buf.len())?;
            raw::symmetric_tag_close(tag)?;
            self.tag = Some(buf);
        }
        Ok(out)
    }

    /// equivalent to `update(data)` then `final`
    pub fn encrypt(&mut self, data: impl AsRef<[u8]>) -> Result<Vec<u8>, raw::CryptoErrno> {
        let data = data.as_ref();
        let mut out = vec![0; data.len()];
        unsafe {
            let tag = raw::symmetric_state_encrypt_detached(
                self.handle,
                out.as_mut_ptr(),
                out.len(),
                data.as_ptr(),
                data.len(),
            )?;
            let len = raw::symmetric_tag_len(tag)?;
            let mut buf = vec![0; len];
            raw::symmetric_tag_pull(tag, buf.as_mut_ptr(), buf.len())?;
            raw::symmetric_tag_close(tag)?;
            self.tag = Some(buf);
        }
        Ok(out)
    }

    pub fn get_auth_tag(&self) -> Result<&Vec<u8>, raw::CryptoErrno> {
        self.tag.as_ref().ok_or(raw::CRYPTO_ERRNO_INVALID_OPERATION)
    }

    pub fn take_auth_tag(&mut self) -> Result<Vec<u8>, raw::CryptoErrno> {
        self.tag.take().ok_or(raw::CRYPTO_ERRNO_INVALID_OPERATION)
    }
}

impl Drop for Cipher {
    fn drop(&mut self) {
        unsafe {
            raw::symmetric_state_close(self.handle).unwrap();
        }
    }
}

pub fn encrypt(
    alg: &str,
    key: impl AsRef<[u8]>,
    iv: impl AsRef<[u8]>,
    aad: impl AsRef<[u8]>,
    msg: impl AsRef<[u8]>,
) -> Result<(Vec<u8>, Vec<u8>), raw::CryptoErrno> {
    let mut c = Cipher::create(alg, key, iv)?;
    c.set_aad(aad)?;
    let out = c.encrypt(msg)?;
    let tag = c.take_auth_tag()?;
    Ok((out, tag))
}

pub struct Decipher {
    handle: raw::SymmetricState,
    message: Vec<u8>,
    tag: Option<Vec<u8>>,
}

impl Decipher {
    pub fn create(
        alg: &str,
        key: impl AsRef<[u8]>,
        iv: impl AsRef<[u8]>,
    ) -> Result<Self, raw::CryptoErrno> {
        let alg = match alg {
            "aes-128-gcm" | "AES-128-GCM" => "AES-128-GCM",
            "aes-256-gcm" | "AES-256-GCM" => "AES-256-GCM",
            _ => return Err(raw::CRYPTO_ERRNO_UNSUPPORTED_ALGORITHM),
        };
        let handle = {
            let key = key.as_ref();
            let iv = iv.as_ref();
            unsafe {
                let raw_key = raw::symmetric_key_import(alg, key.as_ptr(), key.len())?;
                let key = raw::OptSymmetricKey {
                    tag: raw::OPT_SYMMETRIC_KEY_U_SOME.raw(),
                    u: raw::OptSymmetricKeyUnion { some: raw_key },
                };
                let opt = raw::options_open(raw::ALGORITHM_TYPE_SYMMETRIC)?;
                raw::options_set(opt, "nonce", iv.as_ptr(), iv.len())?;
                let opts = raw::OptOptions {
                    tag: raw::OPT_OPTIONS_U_SOME.raw(),
                    u: raw::OptOptionsUnion { some: opt },
                };
                let state = raw::symmetric_state_open(alg, key, opts).unwrap();
                raw::symmetric_key_close(raw_key).unwrap();
                state
            }
        };
        Ok(Self {
            handle,
            message: vec![],
            tag: None,
        })
    }

    pub fn set_aad(&mut self, data: impl AsRef<[u8]>) -> Result<(), raw::CryptoErrno> {
        let data = data.as_ref();
        unsafe { raw::symmetric_state_absorb(self.handle, data.as_ptr(), data.len()) }
    }

    /// in WasmEdge implement of wasi-crypto, `decrypt` can't be called multiple times,
    /// multiple call `decrypt` is also not equivalent to multiple call `update`.
    /// so we store all message and concat it, then decrypt one-time on `final`
    pub fn update(&mut self, data: impl AsRef<[u8]>) -> Result<(), raw::CryptoErrno> {
        self.message.extend_from_slice(data.as_ref());
        Ok(())
    }

    /// `final` is reserved keyword, `fin` looks better than `r#final`
    pub fn fin(&mut self) -> Result<Vec<u8>, raw::CryptoErrno> {
        if let Some(tag) = &self.tag {
            let mut out = vec![0; self.message.len()];
            unsafe {
                raw::symmetric_state_decrypt_detached(
                    self.handle,
                    out.as_mut_ptr(),
                    out.len(),
                    self.message.as_ptr(),
                    self.message.len(),
                    tag.as_ptr(),
                    tag.len(),
                )?;
            }
            Ok(out)
        } else {
            Err(raw::CRYPTO_ERRNO_INVALID_OPERATION)
        }
    }

    /// equivalent to `update(data)` then `final`
    pub fn decrypt(&mut self, data: impl AsRef<[u8]>) -> Result<Vec<u8>, raw::CryptoErrno> {
        let data = data.as_ref();
        if let Some(tag) = &self.tag {
            let mut out = vec![0; data.len()];
            unsafe {
                raw::symmetric_state_decrypt_detached(
                    self.handle,
                    out.as_mut_ptr(),
                    out.len(),
                    data.as_ptr(),
                    data.len(),
                    tag.as_ptr(),
                    tag.len(),
                )?;
            }
            Ok(out)
        } else {
            Err(raw::CRYPTO_ERRNO_INVALID_OPERATION)
        }
    }

    pub fn set_auth_tag(&mut self, data: impl AsRef<[u8]>) -> Result<(), raw::CryptoErrno> {
        self.tag = Some(data.as_ref().to_vec());
        Ok(())
    }
}

impl Drop for Decipher {
    fn drop(&mut self) {
        unsafe {
            raw::symmetric_state_close(self.handle).unwrap();
        }
    }
}

pub fn decrypt(
    alg: &str,
    key: impl AsRef<[u8]>,
    iv: impl AsRef<[u8]>,
    aad: impl AsRef<[u8]>,
    auth_tag: impl AsRef<[u8]>,
    msg: impl AsRef<[u8]>,
) -> Result<Vec<u8>, raw::CryptoErrno> {
    let mut c = Decipher::create(alg, key, iv)?;
    c.set_aad(aad)?;
    c.set_auth_tag(auth_tag)?;
    c.decrypt(msg)
}
