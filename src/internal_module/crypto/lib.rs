use super::raw;

const NONE_OPTS: raw::OptOptions = raw::OptOptions {
    tag: raw::OPT_OPTIONS_U_NONE.raw(),
    u: raw::OptOptionsUnion { none: () },
};

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
    let key = key.as_ref();
    let hmac_alg = match alg {
        "sha256" | "SHA256" | "HMAC/SHA-256" => "HMAC/SHA-256",
        "sha512" | "SHA512" | "HMAC/SHA-512" => "HMAC/SHA-512",
        _ => unreachable!(),
    };
    unsafe {
        let hmac_key = raw::symmetric_key_import(hmac_alg, key.as_ptr(), key.len())?;
        let hmac_handle = raw::symmetric_state_open(
            hmac_alg,
            raw::OptSymmetricKey {
                tag: raw::OPT_SYMMETRIC_KEY_U_SOME.raw(),
                u: raw::OptSymmetricKeyUnion { some: hmac_key },
            },
            NONE_OPTS,
        )?;
        for info in infos {
            let info = info.as_ref();
            raw::symmetric_state_absorb(hmac_handle, info.as_ptr(), info.len())?;
        }
        let tag = raw::symmetric_state_squeeze_tag(hmac_handle)?;
        raw::symmetric_state_close(hmac_handle)?;
        raw::symmetric_key_close(hmac_key)?;
        let len = raw::symmetric_tag_len(tag)?;
        let mut out = vec![0; len];
        raw::symmetric_tag_pull(tag, out.as_mut_ptr(), out.len())?;
        raw::symmetric_tag_close(tag)?;
        Ok(out)
    }
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
