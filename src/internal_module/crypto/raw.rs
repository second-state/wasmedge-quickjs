#![allow(dead_code, unused_variables)]
use core::fmt;
use core::mem::MaybeUninit;
#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct CryptoErrno(u16);
/// Operation succeeded.
pub const CRYPTO_ERRNO_SUCCESS: CryptoErrno = CryptoErrno(0);
/// An error occurred when trying to during a conversion from a host type to a
/// guest type.
///
/// Only an internal bug can throw this error.
pub const CRYPTO_ERRNO_GUEST_ERROR: CryptoErrno = CryptoErrno(1);
/// The requested operation is valid, but not implemented by the host.
pub const CRYPTO_ERRNO_NOT_IMPLEMENTED: CryptoErrno = CryptoErrno(2);
/// The requested feature is not supported by the chosen algorithm.
pub const CRYPTO_ERRNO_UNSUPPORTED_FEATURE: CryptoErrno = CryptoErrno(3);
/// The requested operation is valid, but was administratively prohibited.
pub const CRYPTO_ERRNO_PROHIBITED_OPERATION: CryptoErrno = CryptoErrno(4);
/// Unsupported encoding for an import or export operation.
pub const CRYPTO_ERRNO_UNSUPPORTED_ENCODING: CryptoErrno = CryptoErrno(5);
/// The requested algorithm is not supported by the host.
pub const CRYPTO_ERRNO_UNSUPPORTED_ALGORITHM: CryptoErrno = CryptoErrno(6);
/// The requested option is not supported by the currently selected algorithm.
pub const CRYPTO_ERRNO_UNSUPPORTED_OPTION: CryptoErrno = CryptoErrno(7);
/// An invalid or incompatible key was supplied.
///
/// The key may not be valid, or was generated for a different algorithm or
/// parameters set.
pub const CRYPTO_ERRNO_INVALID_KEY: CryptoErrno = CryptoErrno(8);
/// The currently selected algorithm doesn't support the requested output
/// length.
///
/// This error is thrown by non-extensible hash functions, when requesting an
/// output size larger than they produce out of a single block.
pub const CRYPTO_ERRNO_INVALID_LENGTH: CryptoErrno = CryptoErrno(9);
/// A signature or authentication tag verification failed.
pub const CRYPTO_ERRNO_VERIFICATION_FAILED: CryptoErrno = CryptoErrno(10);
/// A secure random numbers generator is not available.
///
/// The requested operation requires random numbers, but the host cannot
/// securely generate them at the moment.
pub const CRYPTO_ERRNO_RNG_ERROR: CryptoErrno = CryptoErrno(11);
/// An error was returned by the underlying cryptography library.
///
/// The host may be running out of memory, parameters may be incompatible with
/// the chosen implementation of an algorithm or another unexpected error may
/// have happened.
///
/// Ideally, the specification should provide enough details and guidance to
/// make this error impossible to ever be thrown.
///
/// Realistically, the WASI crypto module cannot possibly cover all possible
/// error types implementations can return, especially since some of these may
/// be language-specific. This error can thus be thrown when other error types
/// are not suitable, and when the original error comes from the cryptographic
/// primitives themselves and not from the WASI module.
pub const CRYPTO_ERRNO_ALGORITHM_FAILURE: CryptoErrno = CryptoErrno(12);
/// The supplied signature is invalid, or incompatible with the chosen
/// algorithm.
pub const CRYPTO_ERRNO_INVALID_SIGNATURE: CryptoErrno = CryptoErrno(13);
/// An attempt was made to close a handle that was already closed.
pub const CRYPTO_ERRNO_CLOSED: CryptoErrno = CryptoErrno(14);
/// A function was called with an unassigned handle, a closed handle, or handle
/// of an unexpected type.
pub const CRYPTO_ERRNO_INVALID_HANDLE: CryptoErrno = CryptoErrno(15);
/// The host needs to copy data to a guest-allocated buffer, but that buffer is
/// too small.
pub const CRYPTO_ERRNO_OVERFLOW: CryptoErrno = CryptoErrno(16);
/// An internal error occurred.
///
/// This error is reserved to internal consistency checks, and must only be sent
/// if the internal state of the host remains safe after an inconsistency was
/// detected.
pub const CRYPTO_ERRNO_INTERNAL_ERROR: CryptoErrno = CryptoErrno(17);
/// Too many handles are currently open, and a new one cannot be created.
///
/// Implementations are free to represent handles as they want, and to enforce
/// limits to limit resources usage.
pub const CRYPTO_ERRNO_TOO_MANY_HANDLES: CryptoErrno = CryptoErrno(18);
/// A key was provided, but the chosen algorithm doesn't support keys.
///
/// This is returned by symmetric operations.
///
/// Many hash functions, in particular, do not support keys without being used
/// in particular constructions. Blindly ignoring a key provided by mistake
/// while trying to open a context for such as function could cause serious
/// security vulnerabilities.
///
/// These functions must refuse to create the context and return this error
/// instead.
pub const CRYPTO_ERRNO_KEY_NOT_SUPPORTED: CryptoErrno = CryptoErrno(19);
/// A key is required for the chosen algorithm, but none was given.
pub const CRYPTO_ERRNO_KEY_REQUIRED: CryptoErrno = CryptoErrno(20);
/// The provided authentication tag is invalid or incompatible with the current
/// algorithm.
///
/// This error is returned by decryption functions and tag verification
/// functions.
///
/// Unlike `verification_failed`, this error code is returned when the tag
/// cannot possibly verify for any input.
pub const CRYPTO_ERRNO_INVALID_TAG: CryptoErrno = CryptoErrno(21);
/// The requested operation is incompatible with the current scheme.
///
/// For example, the `symmetric_state_encrypt()` function cannot complete if the
/// selected construction is a key derivation function. This error code will be
/// returned instead.
pub const CRYPTO_ERRNO_INVALID_OPERATION: CryptoErrno = CryptoErrno(22);
/// A nonce is required.
///
/// Most encryption schemes require a nonce.
///
/// In the absence of a nonce, the WASI cryptography module can automatically
/// generate one, if that can be done safely. The nonce can be retrieved later
/// with the `symmetric_state_option_get()` function using the `nonce`
/// parameter. If automatically generating a nonce cannot be done safely, the
/// module never falls back to an insecure option and requests an explicit nonce
/// by throwing that error.
pub const CRYPTO_ERRNO_NONCE_REQUIRED: CryptoErrno = CryptoErrno(23);
/// The provided nonce doesn't have a correct size for the given cipher.
pub const CRYPTO_ERRNO_INVALID_NONCE: CryptoErrno = CryptoErrno(24);
/// The named option was not set.
///
/// The caller tried to read the value of an option that was not set.
/// This error is used to make the distinction between an empty option, and an
/// option that was not set and left to its default value.
pub const CRYPTO_ERRNO_OPTION_NOT_SET: CryptoErrno = CryptoErrno(25);
/// A key or key pair matching the requested identifier cannot be found using
/// the supplied information.
///
/// This error is returned by a secrets manager via the `keypair_from_id()`
/// function.
pub const CRYPTO_ERRNO_NOT_FOUND: CryptoErrno = CryptoErrno(26);
/// The algorithm requires parameters that haven't been set.
///
/// Non-generic options are required and must be given by building an `options`
/// set and giving that object to functions instantiating that algorithm.
pub const CRYPTO_ERRNO_PARAMETERS_MISSING: CryptoErrno = CryptoErrno(27);
/// A requested computation is not done yet, and additional calls to the
/// function are required.
///
/// Some functions, such as functions generating key pairs and password
/// stretching functions, can take a long time to complete.
///
/// In order to avoid a host call to be blocked for too long, these functions
/// can return prematurely, requiring additional calls with the same parameters
/// until they complete.
pub const CRYPTO_ERRNO_IN_PROGRESS: CryptoErrno = CryptoErrno(28);
/// Multiple keys have been provided, but they do not share the same type.
///
/// This error is returned when trying to build a key pair from a public key and
/// a secret key that were created for different and incompatible algorithms.
pub const CRYPTO_ERRNO_INCOMPATIBLE_KEYS: CryptoErrno = CryptoErrno(29);
/// A managed key or secret expired and cannot be used any more.
pub const CRYPTO_ERRNO_EXPIRED: CryptoErrno = CryptoErrno(30);
impl CryptoErrno {
    pub const fn raw(&self) -> u16 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "SUCCESS",
            1 => "GUEST_ERROR",
            2 => "NOT_IMPLEMENTED",
            3 => "UNSUPPORTED_FEATURE",
            4 => "PROHIBITED_OPERATION",
            5 => "UNSUPPORTED_ENCODING",
            6 => "UNSUPPORTED_ALGORITHM",
            7 => "UNSUPPORTED_OPTION",
            8 => "INVALID_KEY",
            9 => "INVALID_LENGTH",
            10 => "VERIFICATION_FAILED",
            11 => "RNG_ERROR",
            12 => "ALGORITHM_FAILURE",
            13 => "INVALID_SIGNATURE",
            14 => "CLOSED",
            15 => "INVALID_HANDLE",
            16 => "OVERFLOW",
            17 => "INTERNAL_ERROR",
            18 => "TOO_MANY_HANDLES",
            19 => "KEY_NOT_SUPPORTED",
            20 => "KEY_REQUIRED",
            21 => "INVALID_TAG",
            22 => "INVALID_OPERATION",
            23 => "NONCE_REQUIRED",
            24 => "INVALID_NONCE",
            25 => "OPTION_NOT_SET",
            26 => "NOT_FOUND",
            27 => "PARAMETERS_MISSING",
            28 => "IN_PROGRESS",
            29 => "INCOMPATIBLE_KEYS",
            30 => "EXPIRED",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "Operation succeeded.",
            1 => {
                "An error occurred when trying to during a conversion from a host type to a guest \
                 type.

Only an internal bug can throw this error."
            }
            2 => "The requested operation is valid, but not implemented by the host.",
            3 => "The requested feature is not supported by the chosen algorithm.",
            4 => "The requested operation is valid, but was administratively prohibited.",
            5 => "Unsupported encoding for an import or export operation.",
            6 => "The requested algorithm is not supported by the host.",
            7 => "The requested option is not supported by the currently selected algorithm.",
            8 => {
                "An invalid or incompatible key was supplied.

The key may not be valid, or was generated for a different algorithm or parameters set."
            }
            9 => {
                "The currently selected algorithm doesn't support the requested output length.

This error is thrown by non-extensible hash functions, when requesting an output size larger than \
                 they produce out of a single block."
            }
            10 => "A signature or authentication tag verification failed.",
            11 => {
                "A secure random numbers generator is not available.

The requested operation requires random numbers, but the host cannot securely generate them at the \
                 moment."
            }
            12 => {
                "An error was returned by the underlying cryptography library.

The host may be running out of memory, parameters may be incompatible with the chosen \
                 implementation of an algorithm or another unexpected error may have happened.

Ideally, the specification should provide enough details and guidance to make this error \
                 impossible to ever be thrown.

Realistically, the WASI crypto module cannot possibly cover all possible error types \
                 implementations can return, especially since some of these may be \
                 language-specific.
This error can thus be thrown when other error types are not suitable, and when the original error \
                 comes from the cryptographic primitives themselves and not from the WASI module."
            }
            13 => "The supplied signature is invalid, or incompatible with the chosen algorithm.",
            14 => "An attempt was made to close a handle that was already closed.",
            15 => {
                "A function was called with an unassigned handle, a closed handle, or handle of an \
                 unexpected type."
            }
            16 => {
                "The host needs to copy data to a guest-allocated buffer, but that buffer is too \
                 small."
            }
            17 => {
                "An internal error occurred.

This error is reserved to internal consistency checks, and must only be sent if the internal state \
                 of the host remains safe after an inconsistency was detected."
            }
            18 => {
                "Too many handles are currently open, and a new one cannot be created.

Implementations are free to represent handles as they want, and to enforce limits to limit \
                 resources usage."
            }
            19 => {
                "A key was provided, but the chosen algorithm doesn't support keys.

This is returned by symmetric operations.

Many hash functions, in particular, do not support keys without being used in particular \
                 constructions.
Blindly ignoring a key provided by mistake while trying to open a context for such as function \
                 could cause serious security vulnerabilities.

These functions must refuse to create the context and return this error instead."
            }
            20 => "A key is required for the chosen algorithm, but none was given.",
            21 => {
                "The provided authentication tag is invalid or incompatible with the current \
                 algorithm.

This error is returned by decryption functions and tag verification functions.

Unlike `verification_failed`, this error code is returned when the tag cannot possibly verify for \
                 any input."
            }
            22 => {
                "The requested operation is incompatible with the current scheme.

For example, the `symmetric_state_encrypt()` function cannot complete if the selected construction \
                 is a key derivation function.
This error code will be returned instead."
            }
            23 => {
                "A nonce is required.

Most encryption schemes require a nonce.

In the absence of a nonce, the WASI cryptography module can automatically generate one, if that \
                 can be done safely. The nonce can be retrieved later with the \
                 `symmetric_state_option_get()` function using the `nonce` parameter.
If automatically generating a nonce cannot be done safely, the module never falls back to an \
                 insecure option and requests an explicit nonce by throwing that error."
            }
            24 => "The provided nonce doesn't have a correct size for the given cipher.",
            25 => {
                "The named option was not set.

The caller tried to read the value of an option that was not set.
This error is used to make the distinction between an empty option, and an option that was not set \
                 and left to its default value."
            }
            26 => {
                "A key or key pair matching the requested identifier cannot be found using the \
                 supplied information.

This error is returned by a secrets manager via the `keypair_from_id()` function."
            }
            27 => {
                "The algorithm requires parameters that haven't been set.

Non-generic options are required and must be given by building an `options` set and giving that \
                 object to functions instantiating that algorithm."
            }
            28 => {
                "A requested computation is not done yet, and additional calls to the function are \
                 required.

Some functions, such as functions generating key pairs and password stretching functions, can take \
                 a long time to complete.

In order to avoid a host call to be blocked for too long, these functions can return prematurely, \
                 requiring additional calls with the same parameters until they complete."
            }
            29 => {
                "Multiple keys have been provided, but they do not share the same type.

This error is returned when trying to build a key pair from a public key and a secret key that \
                 were created for different and incompatible algorithms."
            }
            30 => "A managed key or secret expired and cannot be used any more.",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for CryptoErrno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CryptoErrno")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}
impl fmt::Display for CryptoErrno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (error {})", self.name(), self.0)
    }
}

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
impl std::error::Error for CryptoErrno {}

#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct KeypairEncoding(u16);
/// Raw bytes.
pub const KEYPAIR_ENCODING_RAW: KeypairEncoding = KeypairEncoding(0);
/// PCSK8/DER encoding.
pub const KEYPAIR_ENCODING_PKCS8: KeypairEncoding = KeypairEncoding(1);
/// PEM encoding.
pub const KEYPAIR_ENCODING_PEM: KeypairEncoding = KeypairEncoding(2);
/// Implementation-defined encoding.
pub const KEYPAIR_ENCODING_LOCAL: KeypairEncoding = KeypairEncoding(3);
impl KeypairEncoding {
    pub const fn raw(&self) -> u16 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "RAW",
            1 => "PKCS8",
            2 => "PEM",
            3 => "LOCAL",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "Raw bytes.",
            1 => "PCSK8/DER encoding.",
            2 => "PEM encoding.",
            3 => "Implementation-defined encoding.",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for KeypairEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeypairEncoding")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct PublickeyEncoding(u16);
/// Raw bytes.
pub const PUBLICKEY_ENCODING_RAW: PublickeyEncoding = PublickeyEncoding(0);
/// PKCS8/DER encoding.
pub const PUBLICKEY_ENCODING_PKCS8: PublickeyEncoding = PublickeyEncoding(1);
/// PEM encoding.
pub const PUBLICKEY_ENCODING_PEM: PublickeyEncoding = PublickeyEncoding(2);
/// SEC-1 encoding.
pub const PUBLICKEY_ENCODING_SEC: PublickeyEncoding = PublickeyEncoding(3);
/// Implementation-defined encoding.
pub const PUBLICKEY_ENCODING_LOCAL: PublickeyEncoding = PublickeyEncoding(4);
impl PublickeyEncoding {
    pub const fn raw(&self) -> u16 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "RAW",
            1 => "PKCS8",
            2 => "PEM",
            3 => "SEC",
            4 => "LOCAL",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "Raw bytes.",
            1 => "PKCS8/DER encoding.",
            2 => "PEM encoding.",
            3 => "SEC-1 encoding.",
            4 => "Implementation-defined encoding.",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for PublickeyEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PublickeyEncoding")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct SecretkeyEncoding(u16);
/// Raw bytes.
pub const SECRETKEY_ENCODING_RAW: SecretkeyEncoding = SecretkeyEncoding(0);
/// PKCS8/DER encoding.
pub const SECRETKEY_ENCODING_PKCS8: SecretkeyEncoding = SecretkeyEncoding(1);
/// PEM encoding.
pub const SECRETKEY_ENCODING_PEM: SecretkeyEncoding = SecretkeyEncoding(2);
/// SEC-1 encoding.
pub const SECRETKEY_ENCODING_SEC: SecretkeyEncoding = SecretkeyEncoding(3);
/// Implementation-defined encoding.
pub const SECRETKEY_ENCODING_LOCAL: SecretkeyEncoding = SecretkeyEncoding(4);
impl SecretkeyEncoding {
    pub const fn raw(&self) -> u16 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "RAW",
            1 => "PKCS8",
            2 => "PEM",
            3 => "SEC",
            4 => "LOCAL",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "Raw bytes.",
            1 => "PKCS8/DER encoding.",
            2 => "PEM encoding.",
            3 => "SEC-1 encoding.",
            4 => "Implementation-defined encoding.",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for SecretkeyEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecretkeyEncoding")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct SignatureEncoding(u16);
/// Raw bytes.
pub const SIGNATURE_ENCODING_RAW: SignatureEncoding = SignatureEncoding(0);
/// DER encoding.
pub const SIGNATURE_ENCODING_DER: SignatureEncoding = SignatureEncoding(1);
impl SignatureEncoding {
    pub const fn raw(&self) -> u16 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "RAW",
            1 => "DER",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "Raw bytes.",
            1 => "DER encoding.",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for SignatureEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SignatureEncoding")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct AlgorithmType(u16);
pub const ALGORITHM_TYPE_SIGNATURES: AlgorithmType = AlgorithmType(0);
pub const ALGORITHM_TYPE_SYMMETRIC: AlgorithmType = AlgorithmType(1);
pub const ALGORITHM_TYPE_KEY_EXCHANGE: AlgorithmType = AlgorithmType(2);
impl AlgorithmType {
    pub const fn raw(&self) -> u16 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "SIGNATURES",
            1 => "SYMMETRIC",
            2 => "KEY_EXCHANGE",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "",
            1 => "",
            2 => "",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for AlgorithmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AlgorithmType")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

pub type Version = u64;
pub type Size = usize;
pub type Timestamp = u64;
pub type ArrayOutput = u32;
pub type Options = u32;
pub type SecretsManager = u32;
pub type Keypair = u32;
pub type SignatureState = u32;
pub type Signature = u32;
pub type Publickey = u32;
pub type Secretkey = u32;
pub type SignatureVerificationState = u32;
pub type SymmetricState = u32;
pub type SymmetricKey = u32;
pub type SymmetricTag = u32;
#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct OptOptionsU(u8);
pub const OPT_OPTIONS_U_SOME: OptOptionsU = OptOptionsU(0);
pub const OPT_OPTIONS_U_NONE: OptOptionsU = OptOptionsU(1);
impl OptOptionsU {
    pub const fn raw(&self) -> u8 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "SOME",
            1 => "NONE",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "",
            1 => "",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for OptOptionsU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OptOptionsU")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union OptOptionsUnion {
    pub none: (),
    pub some: Options,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OptOptions {
    pub tag: u8,
    pub u: OptOptionsUnion,
}

#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct OptSymmetricKeyU(u8);
pub const OPT_SYMMETRIC_KEY_U_SOME: OptSymmetricKeyU = OptSymmetricKeyU(0);
pub const OPT_SYMMETRIC_KEY_U_NONE: OptSymmetricKeyU = OptSymmetricKeyU(1);
impl OptSymmetricKeyU {
    pub const fn raw(&self) -> u8 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "SOME",
            1 => "NONE",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "",
            1 => "",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for OptSymmetricKeyU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OptSymmetricKeyU")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union OptSymmetricKeyUnion {
    pub none: (),
    pub some: SymmetricKey,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OptSymmetricKey {
    pub tag: u8,
    pub u: OptSymmetricKeyUnion,
}

pub type U64 = u64;
pub type SignatureKeypair = Keypair;
pub type SignaturePublickey = Publickey;
pub type SignatureSecretkey = Secretkey;
pub type KxKeypair = Keypair;
pub type KxPublickey = Publickey;
pub type KxSecretkey = Secretkey;
/// Create a new object to set non-default options.
///
/// Example usage:
///
/// ```rust
/// let options_handle = options_open(AlgorithmType::Symmetric)?;
/// options_set(options_handle, "context", context)?;
/// options_set_u64(options_handle, "threads", 4)?;
/// let state = symmetric_state_open("BLAKE3", None, Some(options_handle))?;
/// options_close(options_handle)?;
/// ```
pub unsafe fn options_open(algorithm_type: AlgorithmType) -> Result<Options, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Options>::uninit();
    let ret = wasi_ephemeral_crypto_common::options_open(
        algorithm_type.0 as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Options)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Destroy an options object.
///
/// Objects are reference counted. It is safe to close an object immediately
/// after the last function needing it is called.
pub unsafe fn options_close(handle: Options) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_common::options_close(handle as i32);
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Set or update an option.
///
/// This is used to set algorithm-specific parameters, but also to provide
/// credentials for the secrets management facilities, if required.
///
/// This function may return `unsupported_option` if an option that doesn't
/// exist for any implemented algorithms is specified.
pub unsafe fn options_set(
    handle: Options,
    name: &str,
    value: *const u8,
    value_len: Size,
) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_common::options_set(
        handle as i32,
        name.as_ptr() as i32,
        name.len() as i32,
        value as i32,
        value_len as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Set or update an integer option.
///
/// This is used to set algorithm-specific parameters.
///
/// This function may return `unsupported_option` if an option that doesn't
/// exist for any implemented algorithms is specified.
pub unsafe fn options_set_u64(handle: Options, name: &str, value: u64) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_common::options_set_u64(
        handle as i32,
        name.as_ptr() as i32,
        name.len() as i32,
        value as i64,
    );
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Set or update a guest-allocated memory that the host can use or return data
/// into.
///
/// This is for example used to set the scratch buffer required by memory-hard
/// functions.
///
/// This function may return `unsupported_option` if an option that doesn't
/// exist for any implemented algorithms is specified.
pub unsafe fn options_set_guest_buffer(
    handle: Options,
    name: &str,
    buffer: *mut u8,
    buffer_len: Size,
) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_common::options_set_guest_buffer(
        handle as i32,
        name.as_ptr() as i32,
        name.len() as i32,
        buffer as i32,
        buffer_len as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Return the length of an `array_output` object.
///
/// This allows a guest to allocate a buffer of the correct size in order to
/// copy the output of a function returning this object type.
pub unsafe fn array_output_len(array_output: ArrayOutput) -> Result<Size, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi_ephemeral_crypto_common::array_output_len(
        array_output as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Copy the content of an `array_output` object into an application-allocated
/// buffer.
///
/// Multiple calls to that function can be made in order to consume the data in
/// a streaming fashion, if necessary.
///
/// The function returns the number of bytes that were actually copied. `0`
/// means that the end of the stream has been reached. The total size always
/// matches the output of `array_output_len()`.
///
/// The handle is automatically closed after all the data has been consumed.
///
/// Example usage:
///
/// ```rust
/// let len = array_output_len(output_handle)?;
/// let mut out = vec![0u8; len];
/// array_output_pull(output_handle, &mut out)?;
/// ```
pub unsafe fn array_output_pull(
    array_output: ArrayOutput,
    buf: *mut u8,
    buf_len: Size,
) -> Result<Size, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi_ephemeral_crypto_common::array_output_pull(
        array_output as i32,
        buf as i32,
        buf_len as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// __(optional)__
/// Create a context to use a secrets manager.
///
/// The set of required and supported options is defined by the host.
///
/// The function returns the `unsupported_feature` error code if secrets
/// management facilities are not supported by the host. This is also an
/// optional import, meaning that the function may not even exist.
pub unsafe fn secrets_manager_open(options: OptOptions) -> Result<SecretsManager, CryptoErrno> {
    let mut rp0 = MaybeUninit::<SecretsManager>::uninit();
    let ret = wasi_ephemeral_crypto_common::secrets_manager_open(
        &options as *const _ as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const SecretsManager
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// __(optional)__
/// Destroy a secrets manager context.
///
/// The function returns the `unsupported_feature` error code if secrets
/// management facilities are not supported by the host. This is also an
/// optional import, meaning that the function may not even exist.
pub unsafe fn secrets_manager_close(secrets_manager: SecretsManager) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_common::secrets_manager_close(secrets_manager as i32);
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// __(optional)__
/// Invalidate a managed key or key pair given an identifier and a version.
///
/// This asks the secrets manager to delete or revoke a stored key, a specific
/// version of a key.
///
/// `key_version` can be set to a version number, to `version.latest` to
/// invalidate the current version, or to `version.all` to invalidate all
/// versions of a key.
///
/// The function returns `unsupported_feature` if this operation is not
/// supported by the host, and `not_found` if the identifier and version don't
/// match any existing key.
///
/// This is an optional import, meaning that the function may not even exist.
pub unsafe fn secrets_manager_invalidate(
    secrets_manager: SecretsManager,
    key_id: *const u8,
    key_id_len: Size,
    key_version: Version,
) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_common::secrets_manager_invalidate(
        secrets_manager as i32,
        key_id as i32,
        key_id_len as i32,
        key_version as i64,
    );
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

pub mod wasi_ephemeral_crypto_common {
    #[link(wasm_import_module = "wasi_ephemeral_crypto_common")]
    extern "C" {
        /// Create a new object to set non-default options.
        ///
        /// Example usage:
        ///
        /// ```rust
        /// let options_handle = options_open(AlgorithmType::Symmetric)?;
        /// options_set(options_handle, "context", context)?;
        /// options_set_u64(options_handle, "threads", 4)?;
        /// let state = symmetric_state_open("BLAKE3", None, Some(options_handle))?;
        /// options_close(options_handle)?;
        /// ```
        pub fn options_open(arg0: i32, arg1: i32) -> i32;
        /// Destroy an options object.
        ///
        /// Objects are reference counted. It is safe to close an object
        /// immediately after the last function needing it is called.
        pub fn options_close(arg0: i32) -> i32;
        /// Set or update an option.
        ///
        /// This is used to set algorithm-specific parameters, but also to
        /// provide credentials for the secrets management facilities, if
        /// required.
        ///
        /// This function may return `unsupported_option` if an option that
        /// doesn't exist for any implemented algorithms is specified.
        pub fn options_set(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32;
        /// Set or update an integer option.
        ///
        /// This is used to set algorithm-specific parameters.
        ///
        /// This function may return `unsupported_option` if an option that
        /// doesn't exist for any implemented algorithms is specified.
        pub fn options_set_u64(arg0: i32, arg1: i32, arg2: i32, arg3: i64) -> i32;
        /// Set or update a guest-allocated memory that the host can use or
        /// return data into.
        ///
        /// This is for example used to set the scratch buffer required by
        /// memory-hard functions.
        ///
        /// This function may return `unsupported_option` if an option that
        /// doesn't exist for any implemented algorithms is specified.
        pub fn options_set_guest_buffer(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
        ) -> i32;
        /// Return the length of an `array_output` object.
        ///
        /// This allows a guest to allocate a buffer of the correct size in
        /// order to copy the output of a function returning this object type.
        pub fn array_output_len(arg0: i32, arg1: i32) -> i32;
        /// Copy the content of an `array_output` object into an
        /// application-allocated buffer.
        ///
        /// Multiple calls to that function can be made in order to consume the
        /// data in a streaming fashion, if necessary.
        ///
        /// The function returns the number of bytes that were actually copied.
        /// `0` means that the end of the stream has been reached. The total
        /// size always matches the output of `array_output_len()`.
        ///
        /// The handle is automatically closed after all the data has been
        /// consumed.
        ///
        /// Example usage:
        ///
        /// ```rust
        /// let len = array_output_len(output_handle)?;
        /// let mut out = vec![0u8; len];
        /// array_output_pull(output_handle, &mut out)?;
        /// ```
        pub fn array_output_pull(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
        /// __(optional)__
        /// Create a context to use a secrets manager.
        ///
        /// The set of required and supported options is defined by the host.
        ///
        /// The function returns the `unsupported_feature` error code if secrets
        /// management facilities are not supported by the host. This is
        /// also an optional import, meaning that the function may not even
        /// exist.
        pub fn secrets_manager_open(arg0: i32, arg1: i32) -> i32;
        /// __(optional)__
        /// Destroy a secrets manager context.
        ///
        /// The function returns the `unsupported_feature` error code if secrets
        /// management facilities are not supported by the host. This is
        /// also an optional import, meaning that the function may not even
        /// exist.
        pub fn secrets_manager_close(arg0: i32) -> i32;
        /// __(optional)__
        /// Invalidate a managed key or key pair given an identifier and a
        /// version.
        ///
        /// This asks the secrets manager to delete or revoke a stored key, a
        /// specific version of a key.
        ///
        /// `key_version` can be set to a version number, to `version.latest` to
        /// invalidate the current version, or to `version.all` to invalidate
        /// all versions of a key.
        ///
        /// The function returns `unsupported_feature` if this operation is not
        /// supported by the host, and `not_found` if the identifier and version
        /// don't match any existing key.
        ///
        /// This is an optional import, meaning that the function may not even
        /// exist.
        pub fn secrets_manager_invalidate(arg0: i32, arg1: i32, arg2: i32, arg3: i64) -> i32;
    }
}
/// Generate a new key pair.
///
/// Internally, a key pair stores the supplied algorithm and optional
/// parameters.
///
/// Trying to use that key pair with different parameters will throw an
/// `invalid_key` error.
///
/// This function may return `$crypto_errno.unsupported_feature` if key
/// generation is not supported by the host for the chosen algorithm.
///
/// The function may also return `unsupported_algorithm` if the algorithm is not
/// supported by the host.
///
/// Finally, if generating that type of key pair is an expensive operation, the
/// function may return `in_progress`. In that case, the guest should retry with
/// the same parameters until the function completes.
///
/// Example usage:
///
/// ```rust
/// let kp_handle =
///     ctx.keypair_generate(AlgorithmType::Signatures, "RSA_PKCS1_2048_SHA256", None)?;
/// ```
pub unsafe fn keypair_generate(
    algorithm_type: AlgorithmType,
    algorithm: &str,
    options: OptOptions,
) -> Result<Keypair, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Keypair>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::keypair_generate(
        algorithm_type.0 as i32,
        algorithm.as_ptr() as i32,
        algorithm.len() as i32,
        &options as *const _ as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Keypair)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Import a key pair.
///
/// This function creates a `keypair` object from existing material.
///
/// It may return `unsupported_algorithm` if the encoding scheme is not
/// supported, or `invalid_key` if the key cannot be decoded.
///
/// The function may also return `unsupported_algorithm` if the algorithm is not
/// supported by the host.
///
/// Example usage:
///
/// ```rust
/// let kp_handle = ctx.keypair_import(
///     AlgorithmType::Signatures,
///     "RSA_PKCS1_2048_SHA256",
///     KeypairEncoding::PKCS8,
/// )?;
/// ```
pub unsafe fn keypair_import(
    algorithm_type: AlgorithmType,
    algorithm: &str,
    encoded: *const u8,
    encoded_len: Size,
    encoding: KeypairEncoding,
) -> Result<Keypair, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Keypair>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::keypair_import(
        algorithm_type.0 as i32,
        algorithm.as_ptr() as i32,
        algorithm.len() as i32,
        encoded as i32,
        encoded_len as i32,
        encoding.0 as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Keypair)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// __(optional)__
/// Generate a new managed key pair.
///
/// The key pair is generated and stored by the secrets management facilities.
///
/// It may be used through its identifier, but the host may not allow it to be
/// exported.
///
/// The function returns the `unsupported_feature` error code if secrets
/// management facilities are not supported by the host,
/// or `unsupported_algorithm` if a key cannot be created for the chosen
/// algorithm.
///
/// The function may also return `unsupported_algorithm` if the algorithm is not
/// supported by the host.
///
/// This is also an optional import, meaning that the function may not even
/// exist.
pub unsafe fn keypair_generate_managed(
    secrets_manager: SecretsManager,
    algorithm_type: AlgorithmType,
    algorithm: &str,
    options: OptOptions,
) -> Result<Keypair, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Keypair>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::keypair_generate_managed(
        secrets_manager as i32,
        algorithm_type.0 as i32,
        algorithm.as_ptr() as i32,
        algorithm.len() as i32,
        &options as *const _ as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Keypair)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// __(optional)__
/// Store a key pair into the secrets manager.
///
/// On success, the function stores the key pair identifier into `$kp_id`,
/// into which up to `$kp_id_max_len` can be written.
///
/// The function returns `overflow` if the supplied buffer is too small.
pub unsafe fn keypair_store_managed(
    secrets_manager: SecretsManager,
    kp: Keypair,
    kp_id: *mut u8,
    kp_id_max_len: Size,
) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_asymmetric_common::keypair_store_managed(
        secrets_manager as i32,
        kp as i32,
        kp_id as i32,
        kp_id_max_len as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// __(optional)__
/// Replace a managed key pair.
///
/// This function crates a new version of a managed key pair, by replacing
/// `$kp_old` with `$kp_new`.
///
/// It does several things:
///
/// - The key identifier for `$kp_new` is set to the one of `$kp_old`.
/// - A new, unique version identifier is assigned to `$kp_new`. This version
///   will be equivalent to using `$version_latest` until the key is replaced.
/// - The `$kp_old` handle is closed.
///
/// Both keys must share the same algorithm and have compatible parameters. If
/// this is not the case, `incompatible_keys` is returned.
///
/// The function may also return the `unsupported_feature` error code if secrets
/// management facilities are not supported by the host, or if keys cannot be
/// rotated.
///
/// Finally, `prohibited_operation` can be returned if `$kp_new` wasn't created
/// by the secrets manager, and the secrets manager prohibits imported keys.
///
/// If the operation succeeded, the new version is returned.
///
/// This is an optional import, meaning that the function may not even exist.
pub unsafe fn keypair_replace_managed(
    secrets_manager: SecretsManager,
    kp_old: Keypair,
    kp_new: Keypair,
) -> Result<Version, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Version>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::keypair_replace_managed(
        secrets_manager as i32,
        kp_old as i32,
        kp_new as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Version)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// __(optional)__
/// Return the key pair identifier and version of a managed key pair.
///
/// If the key pair is not managed, `unsupported_feature` is returned instead.
///
/// This is an optional import, meaning that the function may not even exist.
pub unsafe fn keypair_id(
    kp: Keypair,
    kp_id: *mut u8,
    kp_id_max_len: Size,
) -> Result<(Size, Version), CryptoErrno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let mut rp1 = MaybeUninit::<Version>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::keypair_id(
        kp as i32,
        kp_id as i32,
        kp_id_max_len as i32,
        rp0.as_mut_ptr() as i32,
        rp1.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok((
            core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size),
            core::ptr::read(rp1.as_mut_ptr() as i32 as *const Version),
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// __(optional)__
/// Return a managed key pair from a key identifier.
///
/// `kp_version` can be set to `version_latest` to retrieve the most recent
/// version of a key pair.
///
/// If no key pair matching the provided information is found, `not_found` is
/// returned instead.
///
/// This is an optional import, meaning that the function may not even exist.
/// ```
pub unsafe fn keypair_from_id(
    secrets_manager: SecretsManager,
    kp_id: *const u8,
    kp_id_len: Size,
    kp_version: Version,
) -> Result<Keypair, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Keypair>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::keypair_from_id(
        secrets_manager as i32,
        kp_id as i32,
        kp_id_len as i32,
        kp_version as i64,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Keypair)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Create a key pair from a public key and a secret key.
pub unsafe fn keypair_from_pk_and_sk(
    publickey: Publickey,
    secretkey: Secretkey,
) -> Result<Keypair, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Keypair>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::keypair_from_pk_and_sk(
        publickey as i32,
        secretkey as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Keypair)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Export a key pair as the given encoding format.
///
/// May return `prohibited_operation` if this operation is denied or
/// `unsupported_encoding` if the encoding is not supported.
pub unsafe fn keypair_export(
    kp: Keypair,
    encoding: KeypairEncoding,
) -> Result<ArrayOutput, CryptoErrno> {
    let mut rp0 = MaybeUninit::<ArrayOutput>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::keypair_export(
        kp as i32,
        encoding.0 as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const ArrayOutput
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Get the public key of a key pair.
pub unsafe fn keypair_publickey(kp: Keypair) -> Result<Publickey, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Publickey>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::keypair_publickey(
        kp as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Publickey)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Get the secret key of a key pair.
pub unsafe fn keypair_secretkey(kp: Keypair) -> Result<Secretkey, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Secretkey>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::keypair_secretkey(
        kp as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Secretkey)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Destroy a key pair.
///
/// The host will automatically wipe traces of the secret key from memory.
///
/// If this is a managed key, the key will not be removed from persistent
/// storage, and can be reconstructed later using the key identifier.
pub unsafe fn keypair_close(kp: Keypair) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_asymmetric_common::keypair_close(kp as i32);
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Import a public key.
///
/// The function may return `unsupported_encoding` if importing from the given
/// format is not implemented or incompatible with the key type.
///
/// It may also return `invalid_key` if the key doesn't appear to match the
/// supplied algorithm.
///
/// Finally, the function may return `unsupported_algorithm` if the algorithm is
/// not supported by the host.
///
/// Example usage:
///
/// ```rust
/// let pk_handle =
///     ctx.publickey_import(AlgorithmType::Signatures, encoded, PublicKeyEncoding::Sec)?;
/// ```
pub unsafe fn publickey_import(
    algorithm_type: AlgorithmType,
    algorithm: &str,
    encoded: *const u8,
    encoded_len: Size,
    encoding: PublickeyEncoding,
) -> Result<Publickey, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Publickey>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::publickey_import(
        algorithm_type.0 as i32,
        algorithm.as_ptr() as i32,
        algorithm.len() as i32,
        encoded as i32,
        encoded_len as i32,
        encoding.0 as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Publickey)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Export a public key as the given encoding format.
///
/// May return `unsupported_encoding` if the encoding is not supported.
pub unsafe fn publickey_export(
    pk: Publickey,
    encoding: PublickeyEncoding,
) -> Result<ArrayOutput, CryptoErrno> {
    let mut rp0 = MaybeUninit::<ArrayOutput>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::publickey_export(
        pk as i32,
        encoding.0 as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const ArrayOutput
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Check that a public key is valid and in canonical form.
///
/// This function may perform stricter checks than those made during importation
/// at the expense of additional CPU cycles.
///
/// The function returns `invalid_key` if the public key didn't pass the checks.
pub unsafe fn publickey_verify(pk: Publickey) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_asymmetric_common::publickey_verify(pk as i32);
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Compute the public key for a secret key.
pub unsafe fn publickey_from_secretkey(sk: Secretkey) -> Result<Publickey, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Publickey>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::publickey_from_secretkey(
        sk as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Publickey)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Destroy a public key.
///
/// Objects are reference counted. It is safe to close an object immediately
/// after the last function needing it is called.
pub unsafe fn publickey_close(pk: Publickey) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_asymmetric_common::publickey_close(pk as i32);
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Import a secret key.
///
/// The function may return `unsupported_encoding` if importing from the given
/// format is not implemented or incompatible with the key type.
///
/// It may also return `invalid_key` if the key doesn't appear to match the
/// supplied algorithm.
///
/// Finally, the function may return `unsupported_algorithm` if the algorithm is
/// not supported by the host.
///
/// Example usage:
///
/// ```rust
/// let pk_handle = ctx.secretkey_import(AlgorithmType::KX, encoded, SecretKeyEncoding::Raw)?;
/// ```
pub unsafe fn secretkey_import(
    algorithm_type: AlgorithmType,
    algorithm: &str,
    encoded: *const u8,
    encoded_len: Size,
    encoding: SecretkeyEncoding,
) -> Result<Secretkey, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Secretkey>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::secretkey_import(
        algorithm_type.0 as i32,
        algorithm.as_ptr() as i32,
        algorithm.len() as i32,
        encoded as i32,
        encoded_len as i32,
        encoding.0 as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Secretkey)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Export a secret key as the given encoding format.
///
/// May return `unsupported_encoding` if the encoding is not supported.
pub unsafe fn secretkey_export(
    sk: Secretkey,
    encoding: SecretkeyEncoding,
) -> Result<ArrayOutput, CryptoErrno> {
    let mut rp0 = MaybeUninit::<ArrayOutput>::uninit();
    let ret = wasi_ephemeral_crypto_asymmetric_common::secretkey_export(
        sk as i32,
        encoding.0 as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const ArrayOutput
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Destroy a secret key.
///
/// Objects are reference counted. It is safe to close an object immediately
/// after the last function needing it is called.
pub unsafe fn secretkey_close(sk: Secretkey) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_asymmetric_common::secretkey_close(sk as i32);
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

pub mod wasi_ephemeral_crypto_asymmetric_common {
    #[link(wasm_import_module = "wasi_ephemeral_crypto_asymmetric_common")]
    extern "C" {
        /// Generate a new key pair.
        ///
        /// Internally, a key pair stores the supplied algorithm and optional
        /// parameters.
        ///
        /// Trying to use that key pair with different parameters will throw an
        /// `invalid_key` error.
        ///
        /// This function may return `$crypto_errno.unsupported_feature` if key
        /// generation is not supported by the host for the chosen algorithm.
        ///
        /// The function may also return `unsupported_algorithm` if the
        /// algorithm is not supported by the host.
        ///
        /// Finally, if generating that type of key pair is an expensive
        /// operation, the function may return `in_progress`.
        /// In that case, the guest should retry with the same parameters until
        /// the function completes.
        ///
        /// Example usage:
        ///
        /// ```rust
        /// let kp_handle =
        ///     ctx.keypair_generate(AlgorithmType::Signatures, "RSA_PKCS1_2048_SHA256", None)?;
        /// ```
        pub fn keypair_generate(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32;
        /// Import a key pair.
        ///
        /// This function creates a `keypair` object from existing material.
        ///
        /// It may return `unsupported_algorithm` if the encoding scheme is not
        /// supported, or `invalid_key` if the key cannot be decoded.
        ///
        /// The function may also return `unsupported_algorithm` if the
        /// algorithm is not supported by the host.
        ///
        /// Example usage:
        ///
        /// ```rust
        /// let kp_handle = ctx.keypair_import(
        ///     AlgorithmType::Signatures,
        ///     "RSA_PKCS1_2048_SHA256",
        ///     KeypairEncoding::PKCS8,
        /// )?;
        /// ```
        pub fn keypair_import(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
            arg6: i32,
        ) -> i32;
        /// __(optional)__
        /// Generate a new managed key pair.
        ///
        /// The key pair is generated and stored by the secrets management
        /// facilities.
        ///
        /// It may be used through its identifier, but the host may not allow it
        /// to be exported.
        ///
        /// The function returns the `unsupported_feature` error code if secrets
        /// management facilities are not supported by the host,
        /// or `unsupported_algorithm` if a key cannot be created for the chosen
        /// algorithm.
        ///
        /// The function may also return `unsupported_algorithm` if the
        /// algorithm is not supported by the host.
        ///
        /// This is also an optional import, meaning that the function may not
        /// even exist.
        pub fn keypair_generate_managed(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
        ) -> i32;
        /// __(optional)__
        /// Store a key pair into the secrets manager.
        ///
        /// On success, the function stores the key pair identifier into
        /// `$kp_id`, into which up to `$kp_id_max_len` can be written.
        ///
        /// The function returns `overflow` if the supplied buffer is too small.
        pub fn keypair_store_managed(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
        /// __(optional)__
        /// Replace a managed key pair.
        ///
        /// This function crates a new version of a managed key pair, by
        /// replacing `$kp_old` with `$kp_new`.
        ///
        /// It does several things:
        ///
        /// - The key identifier for `$kp_new` is set to the one of `$kp_old`.
        /// - A new, unique version identifier is assigned to `$kp_new`. This
        ///   version will be equivalent to using `$version_latest` until the
        ///   key is replaced.
        /// - The `$kp_old` handle is closed.
        ///
        /// Both keys must share the same algorithm and have compatible
        /// parameters. If this is not the case, `incompatible_keys` is
        /// returned.
        ///
        /// The function may also return the `unsupported_feature` error code if
        /// secrets management facilities are not supported by the host,
        /// or if keys cannot be rotated.
        ///
        /// Finally, `prohibited_operation` can be returned if `$kp_new` wasn't
        /// created by the secrets manager, and the secrets manager prohibits
        /// imported keys.
        ///
        /// If the operation succeeded, the new version is returned.
        ///
        /// This is an optional import, meaning that the function may not even
        /// exist.
        pub fn keypair_replace_managed(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
        /// __(optional)__
        /// Return the key pair identifier and version of a managed key pair.
        ///
        /// If the key pair is not managed, `unsupported_feature` is returned
        /// instead.
        ///
        /// This is an optional import, meaning that the function may not even
        /// exist.
        pub fn keypair_id(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32;
        /// __(optional)__
        /// Return a managed key pair from a key identifier.
        ///
        /// `kp_version` can be set to `version_latest` to retrieve the most
        /// recent version of a key pair.
        ///
        /// If no key pair matching the provided information is found,
        /// `not_found` is returned instead.
        ///
        /// This is an optional import, meaning that the function may not even
        /// exist. ```
        pub fn keypair_from_id(arg0: i32, arg1: i32, arg2: i32, arg3: i64, arg4: i32) -> i32;
        /// Create a key pair from a public key and a secret key.
        pub fn keypair_from_pk_and_sk(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Export a key pair as the given encoding format.
        ///
        /// May return `prohibited_operation` if this operation is denied or
        /// `unsupported_encoding` if the encoding is not supported.
        pub fn keypair_export(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Get the public key of a key pair.
        pub fn keypair_publickey(arg0: i32, arg1: i32) -> i32;
        /// Get the secret key of a key pair.
        pub fn keypair_secretkey(arg0: i32, arg1: i32) -> i32;
        /// Destroy a key pair.
        ///
        /// The host will automatically wipe traces of the secret key from
        /// memory.
        ///
        /// If this is a managed key, the key will not be removed from
        /// persistent storage, and can be reconstructed later using the key
        /// identifier.
        pub fn keypair_close(arg0: i32) -> i32;
        /// Import a public key.
        ///
        /// The function may return `unsupported_encoding` if importing from the
        /// given format is not implemented or incompatible with the key type.
        ///
        /// It may also return `invalid_key` if the key doesn't appear to match
        /// the supplied algorithm.
        ///
        /// Finally, the function may return `unsupported_algorithm` if the
        /// algorithm is not supported by the host.
        ///
        /// Example usage:
        ///
        /// ```rust
        /// let pk_handle =
        ///     ctx.publickey_import(AlgorithmType::Signatures, encoded, PublicKeyEncoding::Sec)?;
        /// ```
        pub fn publickey_import(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
            arg6: i32,
        ) -> i32;
        /// Export a public key as the given encoding format.
        ///
        /// May return `unsupported_encoding` if the encoding is not supported.
        pub fn publickey_export(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Check that a public key is valid and in canonical form.
        ///
        /// This function may perform stricter checks than those made during
        /// importation at the expense of additional CPU cycles.
        ///
        /// The function returns `invalid_key` if the public key didn't pass the
        /// checks.
        pub fn publickey_verify(arg0: i32) -> i32;
        /// Compute the public key for a secret key.
        pub fn publickey_from_secretkey(arg0: i32, arg1: i32) -> i32;
        /// Destroy a public key.
        ///
        /// Objects are reference counted. It is safe to close an object
        /// immediately after the last function needing it is called.
        pub fn publickey_close(arg0: i32) -> i32;
        /// Import a secret key.
        ///
        /// The function may return `unsupported_encoding` if importing from the
        /// given format is not implemented or incompatible with the key type.
        ///
        /// It may also return `invalid_key` if the key doesn't appear to match
        /// the supplied algorithm.
        ///
        /// Finally, the function may return `unsupported_algorithm` if the
        /// algorithm is not supported by the host.
        ///
        /// Example usage:
        ///
        /// ```rust
        /// let pk_handle = ctx.secretkey_import(AlgorithmType::KX, encoded, SecretKeyEncoding::Raw)?;
        /// ```
        pub fn secretkey_import(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
            arg6: i32,
        ) -> i32;
        /// Export a secret key as the given encoding format.
        ///
        /// May return `unsupported_encoding` if the encoding is not supported.
        pub fn secretkey_export(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Destroy a secret key.
        ///
        /// Objects are reference counted. It is safe to close an object
        /// immediately after the last function needing it is called.
        pub fn secretkey_close(arg0: i32) -> i32;
    }
}
/// Export a signature.
///
/// This function exports a signature object using the specified encoding.
///
/// May return `unsupported_encoding` if the signature cannot be encoded into
/// the given format.
pub unsafe fn signature_export(
    signature: Signature,
    encoding: SignatureEncoding,
) -> Result<ArrayOutput, CryptoErrno> {
    let mut rp0 = MaybeUninit::<ArrayOutput>::uninit();
    let ret = wasi_ephemeral_crypto_signatures::signature_export(
        signature as i32,
        encoding.0 as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const ArrayOutput
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Create a signature object.
///
/// This object can be used along with a public key to verify an existing
/// signature.
///
/// It may return `invalid_signature` if the signature is invalid or
/// incompatible with the specified algorithm, as well as `unsupported_encoding`
/// if the encoding is not compatible with the signature type.
///
/// The function may also return `unsupported_algorithm` if the algorithm is not
/// supported by the host.
///
/// Example usage:
///
/// ```rust
/// let signature_handle =
///     ctx.signature_import("ECDSA_P256_SHA256", SignatureEncoding::DER, encoded)?;
/// ```
pub unsafe fn signature_import(
    algorithm: &str,
    encoded: *const u8,
    encoded_len: Size,
    encoding: SignatureEncoding,
) -> Result<Signature, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Signature>::uninit();
    let ret = wasi_ephemeral_crypto_signatures::signature_import(
        algorithm.as_ptr() as i32,
        algorithm.len() as i32,
        encoded as i32,
        encoded_len as i32,
        encoding.0 as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Signature)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Create a new state to collect data to compute a signature on.
///
/// This function allows data to be signed to be supplied in a streaming
/// fashion.
///
/// The state is not closed and can be used after a signature has been computed,
/// allowing incremental updates by calling `signature_state_update()` again
/// afterwards.
///
/// Example usage - signature creation
///
/// ```rust
/// let kp_handle = ctx.keypair_import(
///     AlgorithmType::Signatures,
///     "Ed25519ph",
///     keypair,
///     KeypairEncoding::Raw,
/// )?;
/// let state_handle = ctx.signature_state_open(kp_handle)?;
/// ctx.signature_state_update(state_handle, b"message part 1")?;
/// ctx.signature_state_update(state_handle, b"message part 2")?;
/// let sig_handle = ctx.signature_state_sign(state_handle)?;
/// let raw_sig = ctx.signature_export(sig_handle, SignatureEncoding::Raw)?;
/// ```
pub unsafe fn signature_state_open(kp: SignatureKeypair) -> Result<SignatureState, CryptoErrno> {
    let mut rp0 = MaybeUninit::<SignatureState>::uninit();
    let ret =
        wasi_ephemeral_crypto_signatures::signature_state_open(kp as i32, rp0.as_mut_ptr() as i32);
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const SignatureState
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Absorb data into the signature state.
///
/// This function may return `unsupported_feature` is the selected algorithm
/// doesn't support incremental updates.
pub unsafe fn signature_state_update(
    state: SignatureState,
    input: *const u8,
    input_len: Size,
) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_signatures::signature_state_update(
        state as i32,
        input as i32,
        input_len as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Compute a signature for all the data collected up to that point.
///
/// The function can be called multiple times for incremental signing.
pub unsafe fn signature_state_sign(state: SignatureState) -> Result<ArrayOutput, CryptoErrno> {
    let mut rp0 = MaybeUninit::<ArrayOutput>::uninit();
    let ret = wasi_ephemeral_crypto_signatures::signature_state_sign(
        state as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const ArrayOutput
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Destroy a signature state.
///
/// Objects are reference counted. It is safe to close an object immediately
/// after the last function needing it is called.
///
/// Note that closing a signature state doesn't close or invalidate the key pair
/// object, that be reused for further signatures.
pub unsafe fn signature_state_close(state: SignatureState) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_signatures::signature_state_close(state as i32);
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Create a new state to collect data to verify a signature on.
///
/// This is the verification counterpart of `signature_state`.
///
/// Data can be injected using `signature_verification_state_update()`, and the
/// state is not closed after a verification, allowing incremental verification.
///
/// Example usage - signature verification:
///
/// ```rust
/// let pk_handle = ctx.publickey_import(
///     AlgorithmType::Signatures,
///     "ECDSA_P256_SHA256",
///     encoded_pk,
///     PublicKeyEncoding::Sec,
/// )?;
/// let signature_handle = ctx.signature_import(
///     AlgorithmType::Signatures,
///     "ECDSA_P256_SHA256",
///     encoded_sig,
///     SignatureEncoding::Der,
/// )?;
/// let state_handle = ctx.signature_verification_state_open(pk_handle)?;
/// ctx.signature_verification_state_update(state_handle, "message")?;
/// ctx.signature_verification_state_verify(signature_handle)?;
/// ```
pub unsafe fn signature_verification_state_open(
    kp: SignaturePublickey,
) -> Result<SignatureVerificationState, CryptoErrno> {
    let mut rp0 = MaybeUninit::<SignatureVerificationState>::uninit();
    let ret = wasi_ephemeral_crypto_signatures::signature_verification_state_open(
        kp as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const SignatureVerificationState
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Absorb data into the signature verification state.
///
/// This function may return `unsupported_feature` is the selected algorithm
/// doesn't support incremental updates.
pub unsafe fn signature_verification_state_update(
    state: SignatureVerificationState,
    input: *const u8,
    input_len: Size,
) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_signatures::signature_verification_state_update(
        state as i32,
        input as i32,
        input_len as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Check that the given signature is verifies for the data collected up to that
/// point point.
///
/// The state is not closed and can absorb more data to allow for incremental
/// verification.
///
/// The function returns `invalid_signature` if the signature doesn't appear to
/// be valid.
pub unsafe fn signature_verification_state_verify(
    state: SignatureVerificationState,
    signature: Signature,
) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_signatures::signature_verification_state_verify(
        state as i32,
        signature as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Destroy a signature verification state.
///
/// Objects are reference counted. It is safe to close an object immediately
/// after the last function needing it is called.
///
/// Note that closing a signature state doesn't close or invalidate the public
/// key object, that be reused for further verifications.
pub unsafe fn signature_verification_state_close(
    state: SignatureVerificationState,
) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_signatures::signature_verification_state_close(state as i32);
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Destroy a signature.
///
/// Objects are reference counted. It is safe to close an object immediately
/// after the last function needing it is called.
pub unsafe fn signature_close(signature: Signature) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_signatures::signature_close(signature as i32);
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

pub mod wasi_ephemeral_crypto_signatures {
    #[link(wasm_import_module = "wasi_ephemeral_crypto_signatures")]
    extern "C" {
        /// Export a signature.
        ///
        /// This function exports a signature object using the specified
        /// encoding.
        ///
        /// May return `unsupported_encoding` if the signature cannot be encoded
        /// into the given format.
        pub fn signature_export(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Create a signature object.
        ///
        /// This object can be used along with a public key to verify an
        /// existing signature.
        ///
        /// It may return `invalid_signature` if the signature is invalid or
        /// incompatible with the specified algorithm, as well as
        /// `unsupported_encoding` if the encoding is not compatible with the
        /// signature type.
        ///
        /// The function may also return `unsupported_algorithm` if the
        /// algorithm is not supported by the host.
        ///
        /// Example usage:
        ///
        /// ```rust
        /// let signature_handle =
        ///     ctx.signature_import("ECDSA_P256_SHA256", SignatureEncoding::DER, encoded)?;
        /// ```
        pub fn signature_import(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
        ) -> i32;
        /// Create a new state to collect data to compute a signature on.
        ///
        /// This function allows data to be signed to be supplied in a streaming
        /// fashion.
        ///
        /// The state is not closed and can be used after a signature has been
        /// computed, allowing incremental updates by calling
        /// `signature_state_update()` again afterwards.
        ///
        /// Example usage - signature creation
        ///
        /// ```rust
        /// let kp_handle = ctx.keypair_import(
        ///     AlgorithmType::Signatures,
        ///     "Ed25519ph",
        ///     keypair,
        ///     KeypairEncoding::Raw,
        /// )?;
        /// let state_handle = ctx.signature_state_open(kp_handle)?;
        /// ctx.signature_state_update(state_handle, b"message part 1")?;
        /// ctx.signature_state_update(state_handle, b"message part 2")?;
        /// let sig_handle = ctx.signature_state_sign(state_handle)?;
        /// let raw_sig = ctx.signature_export(sig_handle, SignatureEncoding::Raw)?;
        /// ```
        pub fn signature_state_open(arg0: i32, arg1: i32) -> i32;
        /// Absorb data into the signature state.
        ///
        /// This function may return `unsupported_feature` is the selected
        /// algorithm doesn't support incremental updates.
        pub fn signature_state_update(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Compute a signature for all the data collected up to that point.
        ///
        /// The function can be called multiple times for incremental signing.
        pub fn signature_state_sign(arg0: i32, arg1: i32) -> i32;
        /// Destroy a signature state.
        ///
        /// Objects are reference counted. It is safe to close an object
        /// immediately after the last function needing it is called.
        ///
        /// Note that closing a signature state doesn't close or invalidate the
        /// key pair object, that be reused for further signatures.
        pub fn signature_state_close(arg0: i32) -> i32;
        /// Create a new state to collect data to verify a signature on.
        ///
        /// This is the verification counterpart of `signature_state`.
        ///
        /// Data can be injected using `signature_verification_state_update()`,
        /// and the state is not closed after a verification, allowing
        /// incremental verification.
        ///
        /// Example usage - signature verification:
        ///
        /// ```rust
        /// let pk_handle = ctx.publickey_import(
        ///     AlgorithmType::Signatures,
        ///     "ECDSA_P256_SHA256",
        ///     encoded_pk,
        ///     PublicKeyEncoding::Sec,
        /// )?;
        /// let signature_handle = ctx.signature_import(
        ///     AlgorithmType::Signatures,
        ///     "ECDSA_P256_SHA256",
        ///     encoded_sig,
        ///     SignatureEncoding::Der,
        /// )?;
        /// let state_handle = ctx.signature_verification_state_open(pk_handle)?;
        /// ctx.signature_verification_state_update(state_handle, "message")?;
        /// ctx.signature_verification_state_verify(signature_handle)?;
        /// ```
        pub fn signature_verification_state_open(arg0: i32, arg1: i32) -> i32;
        /// Absorb data into the signature verification state.
        ///
        /// This function may return `unsupported_feature` is the selected
        /// algorithm doesn't support incremental updates.
        pub fn signature_verification_state_update(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Check that the given signature is verifies for the data collected up
        /// to that point point.
        ///
        /// The state is not closed and can absorb more data to allow for
        /// incremental verification.
        ///
        /// The function returns `invalid_signature` if the signature doesn't
        /// appear to be valid.
        pub fn signature_verification_state_verify(arg0: i32, arg1: i32) -> i32;
        /// Destroy a signature verification state.
        ///
        /// Objects are reference counted. It is safe to close an object
        /// immediately after the last function needing it is called.
        ///
        /// Note that closing a signature state doesn't close or invalidate the
        /// public key object, that be reused for further verifications.
        pub fn signature_verification_state_close(arg0: i32) -> i32;
        /// Destroy a signature.
        ///
        /// Objects are reference counted. It is safe to close an object
        /// immediately after the last function needing it is called.
        pub fn signature_close(arg0: i32) -> i32;
    }
}
/// Generate a new symmetric key for a given algorithm.
///
/// `options` can be `None` to use the default parameters, or an
/// algoritm-specific set of parameters to override.
///
/// This function may return `unsupported_feature` if key generation is not
/// supported by the host for the chosen algorithm, or `unsupported_algorithm`
/// if the algorithm is not supported by the host.
pub unsafe fn symmetric_key_generate(
    algorithm: &str,
    options: OptOptions,
) -> Result<SymmetricKey, CryptoErrno> {
    let mut rp0 = MaybeUninit::<SymmetricKey>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_key_generate(
        algorithm.as_ptr() as i32,
        algorithm.len() as i32,
        &options as *const _ as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const SymmetricKey
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Create a symmetric key from raw material.
///
/// The algorithm is internally stored along with the key, and trying to use the
/// key with an operation expecting a different algorithm will return
/// `invalid_key`.
///
/// The function may also return `unsupported_algorithm` if the algorithm is not
/// supported by the host.
pub unsafe fn symmetric_key_import(
    algorithm: &str,
    raw: *const u8,
    raw_len: Size,
) -> Result<SymmetricKey, CryptoErrno> {
    let mut rp0 = MaybeUninit::<SymmetricKey>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_key_import(
        algorithm.as_ptr() as i32,
        algorithm.len() as i32,
        raw as i32,
        raw_len as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const SymmetricKey
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Export a symmetric key as raw material.
///
/// This is mainly useful to export a managed key.
///
/// May return `prohibited_operation` if this operation is denied.
pub unsafe fn symmetric_key_export(
    symmetric_key: SymmetricKey,
) -> Result<ArrayOutput, CryptoErrno> {
    let mut rp0 = MaybeUninit::<ArrayOutput>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_key_export(
        symmetric_key as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const ArrayOutput
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Destroy a symmetric key.
///
/// Objects are reference counted. It is safe to close an object immediately
/// after the last function needing it is called.
pub unsafe fn symmetric_key_close(symmetric_key: SymmetricKey) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_key_close(symmetric_key as i32);
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// __(optional)__
/// Generate a new managed symmetric key.
///
/// The key is generated and stored by the secrets management facilities.
///
/// It may be used through its identifier, but the host may not allow it to be
/// exported.
///
/// The function returns the `unsupported_feature` error code if secrets
/// management facilities are not supported by the host,
/// or `unsupported_algorithm` if a key cannot be created for the chosen
/// algorithm.
///
/// The function may also return `unsupported_algorithm` if the algorithm is not
/// supported by the host.
///
/// This is also an optional import, meaning that the function may not even
/// exist.
pub unsafe fn symmetric_key_generate_managed(
    secrets_manager: SecretsManager,
    algorithm: &str,
    options: OptOptions,
) -> Result<SymmetricKey, CryptoErrno> {
    let mut rp0 = MaybeUninit::<SymmetricKey>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_key_generate_managed(
        secrets_manager as i32,
        algorithm.as_ptr() as i32,
        algorithm.len() as i32,
        &options as *const _ as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const SymmetricKey
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// __(optional)__
/// Store a symmetric key into the secrets manager.
///
/// On success, the function stores the key identifier into `$symmetric_key_id`,
/// into which up to `$symmetric_key_id_max_len` can be written.
///
/// The function returns `overflow` if the supplied buffer is too small.
pub unsafe fn symmetric_key_store_managed(
    secrets_manager: SecretsManager,
    symmetric_key: SymmetricKey,
    symmetric_key_id: *mut u8,
    symmetric_key_id_max_len: Size,
) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_key_store_managed(
        secrets_manager as i32,
        symmetric_key as i32,
        symmetric_key_id as i32,
        symmetric_key_id_max_len as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// __(optional)__
/// Replace a managed symmetric key.
///
/// This function crates a new version of a managed symmetric key, by replacing
/// `$kp_old` with `$kp_new`.
///
/// It does several things:
///
/// - The key identifier for `$symmetric_key_new` is set to the one of
///   `$symmetric_key_old`.
/// - A new, unique version identifier is assigned to `$kp_new`. This version
///   will be equivalent to using `$version_latest` until the key is replaced.
/// - The `$symmetric_key_old` handle is closed.
///
/// Both keys must share the same algorithm and have compatible parameters. If
/// this is not the case, `incompatible_keys` is returned.
///
/// The function may also return the `unsupported_feature` error code if secrets
/// management facilities are not supported by the host, or if keys cannot be
/// rotated.
///
/// Finally, `prohibited_operation` can be returned if `$symmetric_key_new`
/// wasn't created by the secrets manager, and the secrets manager prohibits
/// imported keys.
///
/// If the operation succeeded, the new version is returned.
///
/// This is an optional import, meaning that the function may not even exist.
pub unsafe fn symmetric_key_replace_managed(
    secrets_manager: SecretsManager,
    symmetric_key_old: SymmetricKey,
    symmetric_key_new: SymmetricKey,
) -> Result<Version, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Version>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_key_replace_managed(
        secrets_manager as i32,
        symmetric_key_old as i32,
        symmetric_key_new as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Version)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// __(optional)__
/// Return the key identifier and version of a managed symmetric key.
///
/// If the key is not managed, `unsupported_feature` is returned instead.
///
/// This is an optional import, meaning that the function may not even exist.
pub unsafe fn symmetric_key_id(
    symmetric_key: SymmetricKey,
    symmetric_key_id: *mut u8,
    symmetric_key_id_max_len: Size,
) -> Result<(Size, Version), CryptoErrno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let mut rp1 = MaybeUninit::<Version>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_key_id(
        symmetric_key as i32,
        symmetric_key_id as i32,
        symmetric_key_id_max_len as i32,
        rp0.as_mut_ptr() as i32,
        rp1.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok((
            core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size),
            core::ptr::read(rp1.as_mut_ptr() as i32 as *const Version),
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// __(optional)__
/// Return a managed symmetric key from a key identifier.
///
/// `kp_version` can be set to `version_latest` to retrieve the most recent
/// version of a symmetric key.
///
/// If no key matching the provided information is found, `not_found` is
/// returned instead.
///
/// This is an optional import, meaning that the function may not even exist.
pub unsafe fn symmetric_key_from_id(
    secrets_manager: SecretsManager,
    symmetric_key_id: *const u8,
    symmetric_key_id_len: Size,
    symmetric_key_version: Version,
) -> Result<SymmetricKey, CryptoErrno> {
    let mut rp0 = MaybeUninit::<SymmetricKey>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_key_from_id(
        secrets_manager as i32,
        symmetric_key_id as i32,
        symmetric_key_id_len as i32,
        symmetric_key_version as i64,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const SymmetricKey
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Create a new state to aborb and produce data using symmetric operations.
///
/// The state remains valid after every operation in order to support
/// incremental updates.
///
/// The function has two optional parameters: a key and an options set.
///
/// It will fail with a `key_not_supported` error code if a key was provided but
/// the chosen algorithm doesn't natively support keying.
///
/// On the other hand, if a key is required, but was not provided, a
/// `key_required` error will be thrown.
///
/// Some algorithms may require additional parameters. They have to be supplied
/// as an options set:
///
/// ```rust
/// let options_handle = ctx.options_open()?;
/// ctx.options_set("context", b"My application")?;
/// ctx.options_set_u64("fanout", 16)?;
/// let state_handle = ctx.symmetric_state_open("BLAKE2b-512", None, Some(options_handle))?;
/// ```
///
/// If some parameters are mandatory but were not set, the `parameters_missing`
/// error code will be returned.
///
/// A notable exception is the `nonce` parameter, that is common to most AEAD
/// constructions.
///
/// If a nonce is required but was not supplied:
///
/// - If it is safe to do so, the host will automatically generate a nonce. This
///   is true for nonces that are large enough to be randomly generated, or if
///   the host is able to maintain a global counter.
/// - If not, the function will fail and return the dedicated `nonce_required`
///   error code.
///
/// A nonce that was automatically generated can be retrieved after the function
/// returns with `symmetric_state_get(state_handle, "nonce")`.
///
/// **Sample usage patterns:**
///
/// - **Hashing**
///
/// ```rust
/// let mut out = [0u8; 64];
/// let state_handle = ctx.symmetric_state_open("SHAKE-128", None, None)?;
/// ctx.symmetric_state_absorb(state_handle, b"data")?;
/// ctx.symmetric_state_absorb(state_handle, b"more_data")?;
/// ctx.symmetric_state_squeeze(state_handle, &mut out)?;
/// ```
///
/// - **MAC**
///
/// ```rust
/// let mut raw_tag = [0u8; 64];
/// let key_handle = ctx.symmetric_key_import("HMAC/SHA-512", b"key")?;
/// let state_handle = ctx.symmetric_state_open("HMAC/SHA-512", Some(key_handle), None)?;
/// ctx.symmetric_state_absorb(state_handle, b"data")?;
/// ctx.symmetric_state_absorb(state_handle, b"more_data")?;
/// let computed_tag_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
/// ctx.symmetric_tag_pull(computed_tag_handle, &mut raw_tag)?;
/// ```
///
/// Verification:
///
/// ```rust
/// let state_handle = ctx.symmetric_state_open("HMAC/SHA-512", Some(key_handle), None)?;
/// ctx.symmetric_state_absorb(state_handle, b"data")?;
/// ctx.symmetric_state_absorb(state_handle, b"more_data")?;
/// let computed_tag_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
/// ctx.symmetric_tag_verify(computed_tag_handle, expected_raw_tag)?;
/// ```
///
/// - **Tuple hashing**
///
/// ```rust
/// let mut out = [0u8; 64];
/// let state_handle = ctx.symmetric_state_open("TupleHashXOF256", None, None)?;
/// ctx.symmetric_state_absorb(state_handle, b"value 1")?;
/// ctx.symmetric_state_absorb(state_handle, b"value 2")?;
/// ctx.symmetric_state_absorb(state_handle, b"value 3")?;
/// ctx.symmetric_state_squeeze(state_handle, &mut out)?;
/// ```
/// Unlike MACs and regular hash functions, inputs are domain separated instead
/// of being concatenated.
///
/// - **Key derivation using extract-and-expand**
///
/// Extract:
///
/// ```rust
/// let mut prk = vec![0u8; 64];
/// let key_handle = ctx.symmetric_key_import("HKDF-EXTRACT/SHA-512", b"key")?;
/// let state_handle = ctx.symmetric_state_open("HKDF-EXTRACT/SHA-512", Some(key_handle), None)?;
/// ctx.symmetric_state_absorb(state_handle, b"salt")?;
/// let prk_handle = ctx.symmetric_state_squeeze_key(state_handle, "HKDF-EXPAND/SHA-512")?;
/// ```
///
/// Expand:
///
/// ```rust
/// let mut subkey = vec![0u8; 32];
/// let state_handle = ctx.symmetric_state_open("HKDF-EXPAND/SHA-512", Some(prk_handle), None)?;
/// ctx.symmetric_state_absorb(state_handle, b"info")?;
/// ctx.symmetric_state_squeeze(state_handle, &mut subkey)?;
/// ```
///
/// - **Key derivation using a XOF**
///
/// ```rust
/// let mut subkey1 = vec![0u8; 32];
/// let mut subkey2 = vec![0u8; 32];
/// let key_handle = ctx.symmetric_key_import("BLAKE3", b"key")?;
/// let state_handle = ctx.symmetric_state_open("BLAKE3", Some(key_handle), None)?;
/// ctx.symmetric_absorb(state_handle, b"context")?;
/// ctx.squeeze(state_handle, &mut subkey1)?;
/// ctx.squeeze(state_handle, &mut subkey2)?;
/// ```
///
/// - **Password hashing**
///
/// ```rust
/// let mut memory = vec![0u8; 1_000_000_000];
/// let options_handle = ctx.symmetric_options_open()?;
/// ctx.symmetric_options_set_guest_buffer(options_handle, "memory", &mut memory)?;
/// ctx.symmetric_options_set_u64(options_handle, "opslimit", 5)?;
/// ctx.symmetric_options_set_u64(options_handle, "parallelism", 8)?;
///
/// let state_handle = ctx.symmetric_state_open("ARGON2-ID-13", None, Some(options))?;
/// ctx.symmtric_state_absorb(state_handle, b"password")?;
///
/// let pw_str_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
/// let mut pw_str = vec![0u8; ctx.symmetric_tag_len(pw_str_handle)?];
/// ctx.symmetric_tag_pull(pw_str_handle, &mut pw_str)?;
/// ```
///
/// - **AEAD encryption with an explicit nonce**
///
/// ```rust
/// let key_handle = ctx.symmetric_key_generate("AES-256-GCM", None)?;
/// let message = b"test";
///
/// let options_handle = ctx.symmetric_options_open()?;
/// ctx.symmetric_options_set(options_handle, "nonce", nonce)?;
///
/// let state_handle =
///     ctx.symmetric_state_open("AES-256-GCM", Some(key_handle), Some(options_handle))?;
/// let mut ciphertext = vec![0u8; message.len() + ctx.symmetric_state_max_tag_len(state_handle)?];
/// ctx.symmetric_state_absorb(state_handle, "additional data")?;
/// ctx.symmetric_state_encrypt(state_handle, &mut ciphertext, message)?;
/// ```
///
/// - **AEAD encryption with automatic nonce generation**
///
/// ```rust
/// let key_handle = ctx.symmetric_key_generate("AES-256-GCM-SIV", None)?;
/// let message = b"test";
/// let mut nonce = [0u8; 24];
///
/// let state_handle = ctx.symmetric_state_open("AES-256-GCM-SIV", Some(key_handle), None)?;
///
/// let nonce = ctx.symmetric_state_options_get(state_handle, "nonce")?;
///
/// let mut ciphertext = vec![0u8; message.len() + ctx.symmetric_state_max_tag_len(state_handle)?];
/// ctx.symmetric_state_absorb(state_handle, "additional data")?;
/// ctx.symmetric_state_encrypt(state_handle, &mut ciphertext, message)?;
/// ```
///
/// - **Session authenticated modes**
///
/// ```rust
/// let mut out = [0u8; 16];
/// let mut out2 = [0u8; 16];
/// let mut ciphertext = [0u8; 20];
/// let key_handle = ctx.symmetric_key_generate("Xoodyak-128", None)?;
/// let state_handle = ctx.symmetric_state_open("Xoodyak-128", Some(key_handle), None)?;
/// ctx.symmetric_state_absorb(state_handle, b"data")?;
/// ctx.symmetric_state_encrypt(state_handle, &mut ciphertext, b"abcd")?;
/// ctx.symmetric_state_absorb(state_handle, b"more data")?;
/// ctx.symmetric_state_squeeze(state_handle, &mut out)?;
/// ctx.symmetric_state_squeeze(state_handle, &mut out2)?;
/// ctx.symmetric_state_ratchet(state_handle)?;
/// ctx.symmetric_state_absorb(state_handle, b"more data")?;
/// let next_key_handle = ctx.symmetric_state_squeeze_key(state_handle, "Xoodyak-128")?;
/// // ...
/// ```
pub unsafe fn symmetric_state_open(
    algorithm: &str,
    key: OptSymmetricKey,
    options: OptOptions,
) -> Result<SymmetricState, CryptoErrno> {
    let mut rp0 = MaybeUninit::<SymmetricState>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_open(
        algorithm.as_ptr() as i32,
        algorithm.len() as i32,
        &key as *const _ as i32,
        &options as *const _ as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const SymmetricState
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Retrieve a parameter from the current state.
///
/// In particular, `symmetric_state_options_get("nonce")` can be used to get a
/// nonce that as automatically generated.
///
/// The function may return `options_not_set` if an option was not set, which is
/// different from an empty value.
///
/// It may also return `unsupported_option` if the option doesn't exist for the
/// chosen algorithm.
pub unsafe fn symmetric_state_options_get(
    handle: SymmetricState,
    name: &str,
    value: *mut u8,
    value_max_len: Size,
) -> Result<Size, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_options_get(
        handle as i32,
        name.as_ptr() as i32,
        name.len() as i32,
        value as i32,
        value_max_len as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Retrieve an integer parameter from the current state.
///
/// The function may return `options_not_set` if an option was not set.
///
/// It may also return `unsupported_option` if the option doesn't exist for the
/// chosen algorithm.
pub unsafe fn symmetric_state_options_get_u64(
    handle: SymmetricState,
    name: &str,
) -> Result<U64, CryptoErrno> {
    let mut rp0 = MaybeUninit::<U64>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_options_get_u64(
        handle as i32,
        name.as_ptr() as i32,
        name.len() as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const U64)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Clone a symmetric state.
///
/// The function clones the internal state, assigns a new handle to it and
/// returns the new handle.
pub unsafe fn symmetric_state_clone(handle: SymmetricState) -> Result<SymmetricState, CryptoErrno> {
    let mut rp0 = MaybeUninit::<SymmetricState>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_clone(
        handle as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const SymmetricState
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Destroy a symmetric state.
///
/// Objects are reference counted. It is safe to close an object immediately
/// after the last function needing it is called.
pub unsafe fn symmetric_state_close(handle: SymmetricState) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_close(handle as i32);
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Absorb data into the state.
///
/// - **Hash functions:** adds data to be hashed.
/// - **MAC functions:** adds data to be authenticated.
/// - **Tuplehash-like constructions:** adds a new tuple to the state.
/// - **Key derivation functions:** adds to the IKM or to the subkey
///   information.
/// - **AEAD constructions:** adds additional data to be authenticated.
/// - **Stateful hash objects, permutation-based constructions:** absorbs.
///
/// If the chosen algorithm doesn't accept input data, the `invalid_operation`
/// error code is returned.
///
/// If too much data has been fed for the algorithm, `overflow` may be thrown.
pub unsafe fn symmetric_state_absorb(
    handle: SymmetricState,
    data: *const u8,
    data_len: Size,
) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_absorb(
        handle as i32,
        data as i32,
        data_len as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Squeeze bytes from the state.
///
/// - **Hash functions:** this tries to output an `out_len` bytes digest from
///   the absorbed data. The hash function output will be truncated if
///   necessary. If the requested size is too large, the `invalid_len` error
///   code is returned.
/// - **Key derivation functions:** : outputs an arbitrary-long derived key.
/// - **RNGs, DRBGs, stream ciphers:**: outputs arbitrary-long data.
/// - **Stateful hash objects, permutation-based constructions:** squeeze.
///
/// Other kinds of algorithms may return `invalid_operation` instead.
///
/// For password-stretching functions, the function may return `in_progress`.
/// In that case, the guest should retry with the same parameters until the
/// function completes.
pub unsafe fn symmetric_state_squeeze(
    handle: SymmetricState,
    out: *mut u8,
    out_len: Size,
) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_squeeze(
        handle as i32,
        out as i32,
        out_len as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Compute and return a tag for all the data injected into the state so far.
///
/// - **MAC functions**: returns a tag authenticating the absorbed data.
/// - **Tuplehash-like constructions:** returns a tag authenticating all the
///   absorbed tuples.
/// - **Password-hashing functions:** returns a standard string containing all
///   the required parameters for password verification.
///
/// Other kinds of algorithms may return `invalid_operation` instead.
///
/// For password-stretching functions, the function may return `in_progress`.
/// In that case, the guest should retry with the same parameters until the
/// function completes.
pub unsafe fn symmetric_state_squeeze_tag(
    handle: SymmetricState,
) -> Result<SymmetricTag, CryptoErrno> {
    let mut rp0 = MaybeUninit::<SymmetricTag>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_squeeze_tag(
        handle as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const SymmetricTag
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Use the current state to produce a key for a target algorithm.
///
/// For extract-then-expand constructions, this returns the PRK.
/// For session-base authentication encryption, this returns a key that can be
/// used to resume a session without storing a nonce.
///
/// `invalid_operation` is returned for algorithms not supporting this
/// operation.
pub unsafe fn symmetric_state_squeeze_key(
    handle: SymmetricState,
    alg_str: &str,
) -> Result<SymmetricKey, CryptoErrno> {
    let mut rp0 = MaybeUninit::<SymmetricKey>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_squeeze_key(
        handle as i32,
        alg_str.as_ptr() as i32,
        alg_str.len() as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const SymmetricKey
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Return the maximum length of an authentication tag for the current
/// algorithm.
///
/// This allows guests to compute the size required to store a ciphertext along
/// with its authentication tag.
///
/// The returned length may include the encryption mode's padding requirements
/// in addition to the actual tag.
///
/// For an encryption operation, the size of the output buffer should be
/// `input_len + symmetric_state_max_tag_len()`.
///
/// For a decryption operation, the size of the buffer that will store the
/// decrypted data must be `ciphertext_len - symmetric_state_max_tag_len()`.
pub unsafe fn symmetric_state_max_tag_len(handle: SymmetricState) -> Result<Size, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_max_tag_len(
        handle as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Encrypt data with an attached tag.
///
/// - **Stream cipher:** adds the input to the stream cipher output. `out_len`
///   and `data_len` can be equal, as no authentication tags will be added.
/// - **AEAD:** encrypts `data` into `out`, including the authentication tag to
///   the output. Additional data must have been previously absorbed using
///   `symmetric_state_absorb()`. The `symmetric_state_max_tag_len()` function
///   can be used to retrieve the overhead of adding the tag, as well as padding
///   if necessary.
/// - **SHOE, Xoodyak, Strobe:** encrypts data, squeezes a tag and appends it to
///   the output.
///
/// If `out` and `data` are the same address, encryption may happen in-place.
///
/// The function returns the actual size of the ciphertext along with the tag.
///
/// `invalid_operation` is returned for algorithms not supporting encryption.
pub unsafe fn symmetric_state_encrypt(
    handle: SymmetricState,
    out: *mut u8,
    out_len: Size,
    data: *const u8,
    data_len: Size,
) -> Result<Size, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_encrypt(
        handle as i32,
        out as i32,
        out_len as i32,
        data as i32,
        data_len as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Encrypt data, with a detached tag.
///
/// - **Stream cipher:** returns `invalid_operation` since stream ciphers do not
///   include authentication tags.
/// - **AEAD:** encrypts `data` into `out` and returns the tag separately.
///   Additional data must have been previously absorbed using
///   `symmetric_state_absorb()`. The output and input buffers must be of the
///   same length.
/// - **SHOE, Xoodyak, Strobe:** encrypts data and squeezes a tag.
///
/// If `out` and `data` are the same address, encryption may happen in-place.
///
/// The function returns the tag.
///
/// `invalid_operation` is returned for algorithms not supporting encryption.
pub unsafe fn symmetric_state_encrypt_detached(
    handle: SymmetricState,
    out: *mut u8,
    out_len: Size,
    data: *const u8,
    data_len: Size,
) -> Result<SymmetricTag, CryptoErrno> {
    let mut rp0 = MaybeUninit::<SymmetricTag>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_encrypt_detached(
        handle as i32,
        out as i32,
        out_len as i32,
        data as i32,
        data_len as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const SymmetricTag
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// - **Stream cipher:** adds the input to the stream cipher output. `out_len`
///   and `data_len` can be equal, as no authentication tags will be added.
/// - **AEAD:** decrypts `data` into `out`. Additional data must have been
///   previously absorbed using `symmetric_state_absorb()`.
/// - **SHOE, Xoodyak, Strobe:** decrypts data, squeezes a tag and verify that
///   it matches the one that was appended to the ciphertext.
///
/// If `out` and `data` are the same address, decryption may happen in-place.
///
/// `out_len` must be exactly `data_len` + `max_tag_len` bytes.
///
/// The function returns the actual size of the decrypted message, which can be
/// smaller than `out_len` for modes that requires padding.
///
/// `invalid_tag` is returned if the tag didn't verify.
///
/// `invalid_operation` is returned for algorithms not supporting encryption.
pub unsafe fn symmetric_state_decrypt(
    handle: SymmetricState,
    out: *mut u8,
    out_len: Size,
    data: *const u8,
    data_len: Size,
) -> Result<Size, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_decrypt(
        handle as i32,
        out as i32,
        out_len as i32,
        data as i32,
        data_len as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// - **Stream cipher:** returns `invalid_operation` since stream ciphers do not
///   include authentication tags.
/// - **AEAD:** decrypts `data` into `out`. Additional data must have been
///   previously absorbed using `symmetric_state_absorb()`.
/// - **SHOE, Xoodyak, Strobe:** decrypts data, squeezes a tag and verify that
///   it matches the expected one.
///
/// `raw_tag` is the expected tag, as raw bytes.
///
/// `out` and `data` be must have the same length.
/// If they also share the same address, decryption may happen in-place.
///
/// The function returns the actual size of the decrypted message.
///
/// `invalid_tag` is returned if the tag verification failed.
///
/// `invalid_operation` is returned for algorithms not supporting encryption.
pub unsafe fn symmetric_state_decrypt_detached(
    handle: SymmetricState,
    out: *mut u8,
    out_len: Size,
    data: *const u8,
    data_len: Size,
    raw_tag: *const u8,
    raw_tag_len: Size,
) -> Result<Size, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_decrypt_detached(
        handle as i32,
        out as i32,
        out_len as i32,
        data as i32,
        data_len as i32,
        raw_tag as i32,
        raw_tag_len as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Make it impossible to recover the previous state.
///
/// This operation is supported by some systems keeping a rolling state over an
/// entire session, for forward security.
///
/// `invalid_operation` is returned for algorithms not supporting ratcheting.
pub unsafe fn symmetric_state_ratchet(handle: SymmetricState) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_state_ratchet(handle as i32);
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Return the length of an authentication tag.
///
/// This function can be used by a guest to allocate the correct buffer size to
/// copy a computed authentication tag.
pub unsafe fn symmetric_tag_len(symmetric_tag: SymmetricTag) -> Result<Size, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_tag_len(
        symmetric_tag as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Copy an authentication tag into a guest-allocated buffer.
///
/// The handle automatically becomes invalid after this operation. Manually
/// closing it is not required.
///
/// Example usage:
///
/// ```rust
/// let mut raw_tag = [0u8; 16];
/// ctx.symmetric_tag_pull(raw_tag_handle, &mut raw_tag)?;
/// ```
///
/// The function returns `overflow` if the supplied buffer is too small to copy
/// the tag.
///
/// Otherwise, it returns the number of bytes that have been copied.
pub unsafe fn symmetric_tag_pull(
    symmetric_tag: SymmetricTag,
    buf: *mut u8,
    buf_len: Size,
) -> Result<Size, CryptoErrno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_tag_pull(
        symmetric_tag as i32,
        buf as i32,
        buf_len as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Verify that a computed authentication tag matches the expected value, in
/// constant-time.
///
/// The expected tag must be provided as a raw byte string.
///
/// The function returns `invalid_tag` if the tags don't match.
///
/// Example usage:
///
/// ```rust
/// let key_handle = ctx.symmetric_key_import("HMAC/SHA-256", b"key")?;
/// let state_handle = ctx.symmetric_state_open("HMAC/SHA-256", Some(key_handle), None)?;
/// ctx.symmetric_state_absorb(state_handle, b"data")?;
/// let computed_tag_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
/// ctx.symmetric_tag_verify(computed_tag_handle, expected_raw_tag)?;
/// ```
pub unsafe fn symmetric_tag_verify(
    symmetric_tag: SymmetricTag,
    expected_raw_tag_ptr: *const u8,
    expected_raw_tag_len: Size,
) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_tag_verify(
        symmetric_tag as i32,
        expected_raw_tag_ptr as i32,
        expected_raw_tag_len as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Explicitly destroy an unused authentication tag.
///
/// This is usually not necessary, as `symmetric_tag_pull()` automatically
/// closes a tag after it has been copied.
///
/// Objects are reference counted. It is safe to close an object immediately
/// after the last function needing it is called.
pub unsafe fn symmetric_tag_close(symmetric_tag: SymmetricTag) -> Result<(), CryptoErrno> {
    let ret = wasi_ephemeral_crypto_symmetric::symmetric_tag_close(symmetric_tag as i32);
    match ret {
        0 => Ok(()),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

pub mod wasi_ephemeral_crypto_symmetric {
    #[link(wasm_import_module = "wasi_ephemeral_crypto_symmetric")]
    extern "C" {
        /// Generate a new symmetric key for a given algorithm.
        ///
        /// `options` can be `None` to use the default parameters, or an
        /// algoritm-specific set of parameters to override.
        ///
        /// This function may return `unsupported_feature` if key generation is
        /// not supported by the host for the chosen algorithm, or
        /// `unsupported_algorithm` if the algorithm is not supported by the
        /// host.
        pub fn symmetric_key_generate(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
        /// Create a symmetric key from raw material.
        ///
        /// The algorithm is internally stored along with the key, and trying to
        /// use the key with an operation expecting a different algorithm will
        /// return `invalid_key`.
        ///
        /// The function may also return `unsupported_algorithm` if the
        /// algorithm is not supported by the host.
        pub fn symmetric_key_import(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32;
        /// Export a symmetric key as raw material.
        ///
        /// This is mainly useful to export a managed key.
        ///
        /// May return `prohibited_operation` if this operation is denied.
        pub fn symmetric_key_export(arg0: i32, arg1: i32) -> i32;
        /// Destroy a symmetric key.
        ///
        /// Objects are reference counted. It is safe to close an object
        /// immediately after the last function needing it is called.
        pub fn symmetric_key_close(arg0: i32) -> i32;
        /// __(optional)__
        /// Generate a new managed symmetric key.
        ///
        /// The key is generated and stored by the secrets management
        /// facilities.
        ///
        /// It may be used through its identifier, but the host may not allow it
        /// to be exported.
        ///
        /// The function returns the `unsupported_feature` error code if secrets
        /// management facilities are not supported by the host,
        /// or `unsupported_algorithm` if a key cannot be created for the chosen
        /// algorithm.
        ///
        /// The function may also return `unsupported_algorithm` if the
        /// algorithm is not supported by the host.
        ///
        /// This is also an optional import, meaning that the function may not
        /// even exist.
        pub fn symmetric_key_generate_managed(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
        ) -> i32;
        /// __(optional)__
        /// Store a symmetric key into the secrets manager.
        ///
        /// On success, the function stores the key identifier into
        /// `$symmetric_key_id`, into which up to
        /// `$symmetric_key_id_max_len` can be written.
        ///
        /// The function returns `overflow` if the supplied buffer is too small.
        pub fn symmetric_key_store_managed(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
        /// __(optional)__
        /// Replace a managed symmetric key.
        ///
        /// This function crates a new version of a managed symmetric key, by
        /// replacing `$kp_old` with `$kp_new`.
        ///
        /// It does several things:
        ///
        /// - The key identifier for `$symmetric_key_new` is set to the one of
        ///   `$symmetric_key_old`.
        /// - A new, unique version identifier is assigned to `$kp_new`. This
        ///   version will be equivalent to using `$version_latest` until the
        ///   key is replaced.
        /// - The `$symmetric_key_old` handle is closed.
        ///
        /// Both keys must share the same algorithm and have compatible
        /// parameters. If this is not the case, `incompatible_keys` is
        /// returned.
        ///
        /// The function may also return the `unsupported_feature` error code if
        /// secrets management facilities are not supported by the host,
        /// or if keys cannot be rotated.
        ///
        /// Finally, `prohibited_operation` can be returned if
        /// `$symmetric_key_new` wasn't created by the secrets manager, and the
        /// secrets manager prohibits imported keys.
        ///
        /// If the operation succeeded, the new version is returned.
        ///
        /// This is an optional import, meaning that the function may not even
        /// exist.
        pub fn symmetric_key_replace_managed(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
        /// __(optional)__
        /// Return the key identifier and version of a managed symmetric key.
        ///
        /// If the key is not managed, `unsupported_feature` is returned
        /// instead.
        ///
        /// This is an optional import, meaning that the function may not even
        /// exist.
        pub fn symmetric_key_id(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32;
        /// __(optional)__
        /// Return a managed symmetric key from a key identifier.
        ///
        /// `kp_version` can be set to `version_latest` to retrieve the most
        /// recent version of a symmetric key.
        ///
        /// If no key matching the provided information is found, `not_found` is
        /// returned instead.
        ///
        /// This is an optional import, meaning that the function may not even
        /// exist.
        pub fn symmetric_key_from_id(arg0: i32, arg1: i32, arg2: i32, arg3: i64, arg4: i32) -> i32;
        /// Create a new state to aborb and produce data using symmetric
        /// operations.
        ///
        /// The state remains valid after every operation in order to support
        /// incremental updates.
        ///
        /// The function has two optional parameters: a key and an options set.
        ///
        /// It will fail with a `key_not_supported` error code if a key was
        /// provided but the chosen algorithm doesn't natively support keying.
        ///
        /// On the other hand, if a key is required, but was not provided, a
        /// `key_required` error will be thrown.
        ///
        /// Some algorithms may require additional parameters. They have to be
        /// supplied as an options set:
        ///
        /// ```rust
        /// let options_handle = ctx.options_open()?;
        /// ctx.options_set("context", b"My application")?;
        /// ctx.options_set_u64("fanout", 16)?;
        /// let state_handle = ctx.symmetric_state_open("BLAKE2b-512", None, Some(options_handle))?;
        /// ```
        ///
        /// If some parameters are mandatory but were not set, the
        /// `parameters_missing` error code will be returned.
        ///
        /// A notable exception is the `nonce` parameter, that is common to most
        /// AEAD constructions.
        ///
        /// If a nonce is required but was not supplied:
        ///
        /// - If it is safe to do so, the host will automatically generate a
        ///   nonce. This is true for nonces that are large enough to be
        ///   randomly generated, or if the host is able to maintain a global
        ///   counter.
        /// - If not, the function will fail and return the dedicated
        ///   `nonce_required` error code.
        ///
        /// A nonce that was automatically generated can be retrieved after the
        /// function returns with `symmetric_state_get(state_handle, "nonce")`.
        ///
        /// **Sample usage patterns:**
        ///
        /// - **Hashing**
        ///
        /// ```rust
        /// let mut out = [0u8; 64];
        /// let state_handle = ctx.symmetric_state_open("SHAKE-128", None, None)?;
        /// ctx.symmetric_state_absorb(state_handle, b"data")?;
        /// ctx.symmetric_state_absorb(state_handle, b"more_data")?;
        /// ctx.symmetric_state_squeeze(state_handle, &mut out)?;
        /// ```
        ///
        /// - **MAC**
        ///
        /// ```rust
        /// let mut raw_tag = [0u8; 64];
        /// let key_handle = ctx.symmetric_key_import("HMAC/SHA-512", b"key")?;
        /// let state_handle = ctx.symmetric_state_open("HMAC/SHA-512", Some(key_handle), None)?;
        /// ctx.symmetric_state_absorb(state_handle, b"data")?;
        /// ctx.symmetric_state_absorb(state_handle, b"more_data")?;
        /// let computed_tag_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
        /// ctx.symmetric_tag_pull(computed_tag_handle, &mut raw_tag)?;
        /// ```
        ///
        /// Verification:
        ///
        /// ```rust
        /// let state_handle = ctx.symmetric_state_open("HMAC/SHA-512", Some(key_handle), None)?;
        /// ctx.symmetric_state_absorb(state_handle, b"data")?;
        /// ctx.symmetric_state_absorb(state_handle, b"more_data")?;
        /// let computed_tag_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
        /// ctx.symmetric_tag_verify(computed_tag_handle, expected_raw_tag)?;
        /// ```
        ///
        /// - **Tuple hashing**
        ///
        /// ```rust
        /// let mut out = [0u8; 64];
        /// let state_handle = ctx.symmetric_state_open("TupleHashXOF256", None, None)?;
        /// ctx.symmetric_state_absorb(state_handle, b"value 1")?;
        /// ctx.symmetric_state_absorb(state_handle, b"value 2")?;
        /// ctx.symmetric_state_absorb(state_handle, b"value 3")?;
        /// ctx.symmetric_state_squeeze(state_handle, &mut out)?;
        /// ```
        /// Unlike MACs and regular hash functions, inputs are domain separated
        /// instead of being concatenated.
        ///
        /// - **Key derivation using extract-and-expand**
        ///
        /// Extract:
        ///
        /// ```rust
        /// let mut prk = vec![0u8; 64];
        /// let key_handle = ctx.symmetric_key_import("HKDF-EXTRACT/SHA-512", b"key")?;
        /// let state_handle = ctx.symmetric_state_open("HKDF-EXTRACT/SHA-512", Some(key_handle), None)?;
        /// ctx.symmetric_state_absorb(state_handle, b"salt")?;
        /// let prk_handle = ctx.symmetric_state_squeeze_key(state_handle, "HKDF-EXPAND/SHA-512")?;
        /// ```
        ///
        /// Expand:
        ///
        /// ```rust
        /// let mut subkey = vec![0u8; 32];
        /// let state_handle = ctx.symmetric_state_open("HKDF-EXPAND/SHA-512", Some(prk_handle), None)?;
        /// ctx.symmetric_state_absorb(state_handle, b"info")?;
        /// ctx.symmetric_state_squeeze(state_handle, &mut subkey)?;
        /// ```
        ///
        /// - **Key derivation using a XOF**
        ///
        /// ```rust
        /// let mut subkey1 = vec![0u8; 32];
        /// let mut subkey2 = vec![0u8; 32];
        /// let key_handle = ctx.symmetric_key_import("BLAKE3", b"key")?;
        /// let state_handle = ctx.symmetric_state_open("BLAKE3", Some(key_handle), None)?;
        /// ctx.symmetric_absorb(state_handle, b"context")?;
        /// ctx.squeeze(state_handle, &mut subkey1)?;
        /// ctx.squeeze(state_handle, &mut subkey2)?;
        /// ```
        ///
        /// - **Password hashing**
        ///
        /// ```rust
        /// let mut memory = vec![0u8; 1_000_000_000];
        /// let options_handle = ctx.symmetric_options_open()?;
        /// ctx.symmetric_options_set_guest_buffer(options_handle, "memory", &mut memory)?;
        /// ctx.symmetric_options_set_u64(options_handle, "opslimit", 5)?;
        /// ctx.symmetric_options_set_u64(options_handle, "parallelism", 8)?;
        ///
        /// let state_handle = ctx.symmetric_state_open("ARGON2-ID-13", None, Some(options))?;
        /// ctx.symmtric_state_absorb(state_handle, b"password")?;
        ///
        /// let pw_str_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
        /// let mut pw_str = vec![0u8; ctx.symmetric_tag_len(pw_str_handle)?];
        /// ctx.symmetric_tag_pull(pw_str_handle, &mut pw_str)?;
        /// ```
        ///
        /// - **AEAD encryption with an explicit nonce**
        ///
        /// ```rust
        /// let key_handle = ctx.symmetric_key_generate("AES-256-GCM", None)?;
        /// let message = b"test";
        ///
        /// let options_handle = ctx.symmetric_options_open()?;
        /// ctx.symmetric_options_set(options_handle, "nonce", nonce)?;
        ///
        /// let state_handle =
        ///     ctx.symmetric_state_open("AES-256-GCM", Some(key_handle), Some(options_handle))?;
        /// let mut ciphertext = vec![0u8; message.len() + ctx.symmetric_state_max_tag_len(state_handle)?];
        /// ctx.symmetric_state_absorb(state_handle, "additional data")?;
        /// ctx.symmetric_state_encrypt(state_handle, &mut ciphertext, message)?;
        /// ```
        ///
        /// - **AEAD encryption with automatic nonce generation**
        ///
        /// ```rust
        /// let key_handle = ctx.symmetric_key_generate("AES-256-GCM-SIV", None)?;
        /// let message = b"test";
        /// let mut nonce = [0u8; 24];
        ///
        /// let state_handle = ctx.symmetric_state_open("AES-256-GCM-SIV", Some(key_handle), None)?;
        ///
        /// let nonce = ctx.symmetric_state_options_get(state_handle, "nonce")?;
        ///
        /// let mut ciphertext = vec![0u8; message.len() + ctx.symmetric_state_max_tag_len(state_handle)?];
        /// ctx.symmetric_state_absorb(state_handle, "additional data")?;
        /// ctx.symmetric_state_encrypt(state_handle, &mut ciphertext, message)?;
        /// ```
        ///
        /// - **Session authenticated modes**
        ///
        /// ```rust
        /// let mut out = [0u8; 16];
        /// let mut out2 = [0u8; 16];
        /// let mut ciphertext = [0u8; 20];
        /// let key_handle = ctx.symmetric_key_generate("Xoodyak-128", None)?;
        /// let state_handle = ctx.symmetric_state_open("Xoodyak-128", Some(key_handle), None)?;
        /// ctx.symmetric_state_absorb(state_handle, b"data")?;
        /// ctx.symmetric_state_encrypt(state_handle, &mut ciphertext, b"abcd")?;
        /// ctx.symmetric_state_absorb(state_handle, b"more data")?;
        /// ctx.symmetric_state_squeeze(state_handle, &mut out)?;
        /// ctx.symmetric_state_squeeze(state_handle, &mut out2)?;
        /// ctx.symmetric_state_ratchet(state_handle)?;
        /// ctx.symmetric_state_absorb(state_handle, b"more data")?;
        /// let next_key_handle = ctx.symmetric_state_squeeze_key(state_handle, "Xoodyak-128")?;
        /// // ...
        /// ```
        pub fn symmetric_state_open(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32;
        /// Retrieve a parameter from the current state.
        ///
        /// In particular, `symmetric_state_options_get("nonce")` can be used to
        /// get a nonce that as automatically generated.
        ///
        /// The function may return `options_not_set` if an option was not set,
        /// which is different from an empty value.
        ///
        /// It may also return `unsupported_option` if the option doesn't exist
        /// for the chosen algorithm.
        pub fn symmetric_state_options_get(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
        ) -> i32;
        /// Retrieve an integer parameter from the current state.
        ///
        /// The function may return `options_not_set` if an option was not set.
        ///
        /// It may also return `unsupported_option` if the option doesn't exist
        /// for the chosen algorithm.
        pub fn symmetric_state_options_get_u64(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
        /// Clone a symmetric state.
        ///
        /// The function clones the internal state, assigns a new handle to it
        /// and returns the new handle.
        pub fn symmetric_state_clone(arg0: i32, arg1: i32) -> i32;
        /// Destroy a symmetric state.
        ///
        /// Objects are reference counted. It is safe to close an object
        /// immediately after the last function needing it is called.
        pub fn symmetric_state_close(arg0: i32) -> i32;
        /// Absorb data into the state.
        ///
        /// - **Hash functions:** adds data to be hashed.
        /// - **MAC functions:** adds data to be authenticated.
        /// - **Tuplehash-like constructions:** adds a new tuple to the state.
        /// - **Key derivation functions:** adds to the IKM or to the subkey
        ///   information.
        /// - **AEAD constructions:** adds additional data to be authenticated.
        /// - **Stateful hash objects, permutation-based constructions:**
        ///   absorbs.
        ///
        /// If the chosen algorithm doesn't accept input data, the
        /// `invalid_operation` error code is returned.
        ///
        /// If too much data has been fed for the algorithm, `overflow` may be
        /// thrown.
        pub fn symmetric_state_absorb(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Squeeze bytes from the state.
        ///
        /// - **Hash functions:** this tries to output an `out_len` bytes digest
        ///   from the absorbed data. The hash function output will be truncated
        ///   if necessary. If the requested size is too large, the
        ///   `invalid_len` error code is returned.
        /// - **Key derivation functions:** : outputs an arbitrary-long derived
        ///   key.
        /// - **RNGs, DRBGs, stream ciphers:**: outputs arbitrary-long data.
        /// - **Stateful hash objects, permutation-based constructions:**
        ///   squeeze.
        ///
        /// Other kinds of algorithms may return `invalid_operation` instead.
        ///
        /// For password-stretching functions, the function may return
        /// `in_progress`. In that case, the guest should retry with the
        /// same parameters until the function completes.
        pub fn symmetric_state_squeeze(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Compute and return a tag for all the data injected into the state so
        /// far.
        ///
        /// - **MAC functions**: returns a tag authenticating the absorbed data.
        /// - **Tuplehash-like constructions:** returns a tag authenticating all
        ///   the absorbed tuples.
        /// - **Password-hashing functions:** returns a standard string
        ///   containing all the required parameters for password verification.
        ///
        /// Other kinds of algorithms may return `invalid_operation` instead.
        ///
        /// For password-stretching functions, the function may return
        /// `in_progress`. In that case, the guest should retry with the
        /// same parameters until the function completes.
        pub fn symmetric_state_squeeze_tag(arg0: i32, arg1: i32) -> i32;
        /// Use the current state to produce a key for a target algorithm.
        ///
        /// For extract-then-expand constructions, this returns the PRK.
        /// For session-base authentication encryption, this returns a key that
        /// can be used to resume a session without storing a nonce.
        ///
        /// `invalid_operation` is returned for algorithms not supporting this
        /// operation.
        pub fn symmetric_state_squeeze_key(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
        /// Return the maximum length of an authentication tag for the current
        /// algorithm.
        ///
        /// This allows guests to compute the size required to store a
        /// ciphertext along with its authentication tag.
        ///
        /// The returned length may include the encryption mode's padding
        /// requirements in addition to the actual tag.
        ///
        /// For an encryption operation, the size of the output buffer should be
        /// `input_len + symmetric_state_max_tag_len()`.
        ///
        /// For a decryption operation, the size of the buffer that will store
        /// the decrypted data must be `ciphertext_len -
        /// symmetric_state_max_tag_len()`.
        pub fn symmetric_state_max_tag_len(arg0: i32, arg1: i32) -> i32;
        /// Encrypt data with an attached tag.
        ///
        /// - **Stream cipher:** adds the input to the stream cipher output.
        ///   `out_len` and `data_len` can be equal, as no authentication tags
        ///   will be added.
        /// - **AEAD:** encrypts `data` into `out`, including the authentication
        ///   tag to the output. Additional data must have been previously
        ///   absorbed using `symmetric_state_absorb()`. The
        ///   `symmetric_state_max_tag_len()` function can be used to retrieve
        ///   the overhead of adding the tag, as well as padding if necessary.
        /// - **SHOE, Xoodyak, Strobe:** encrypts data, squeezes a tag and
        ///   appends it to the output.
        ///
        /// If `out` and `data` are the same address, encryption may happen
        /// in-place.
        ///
        /// The function returns the actual size of the ciphertext along with
        /// the tag.
        ///
        /// `invalid_operation` is returned for algorithms not supporting
        /// encryption.
        pub fn symmetric_state_encrypt(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
        ) -> i32;
        /// Encrypt data, with a detached tag.
        ///
        /// - **Stream cipher:** returns `invalid_operation` since stream
        ///   ciphers do not include authentication tags.
        /// - **AEAD:** encrypts `data` into `out` and returns the tag
        ///   separately. Additional data must have been previously absorbed
        ///   using `symmetric_state_absorb()`. The output and input buffers
        ///   must be of the same length.
        /// - **SHOE, Xoodyak, Strobe:** encrypts data and squeezes a tag.
        ///
        /// If `out` and `data` are the same address, encryption may happen
        /// in-place.
        ///
        /// The function returns the tag.
        ///
        /// `invalid_operation` is returned for algorithms not supporting
        /// encryption.
        pub fn symmetric_state_encrypt_detached(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
        ) -> i32;
        /// - **Stream cipher:** adds the input to the stream cipher output.
        ///   `out_len` and `data_len` can be equal, as no authentication tags
        ///   will be added.
        /// - **AEAD:** decrypts `data` into `out`. Additional data must have
        ///   been previously absorbed using `symmetric_state_absorb()`.
        /// - **SHOE, Xoodyak, Strobe:** decrypts data, squeezes a tag and
        ///   verify that it matches the one that was appended to the
        ///   ciphertext.
        ///
        /// If `out` and `data` are the same address, decryption may happen
        /// in-place.
        ///
        /// `out_len` must be exactly `data_len` + `max_tag_len` bytes.
        ///
        /// The function returns the actual size of the decrypted message, which
        /// can be smaller than `out_len` for modes that requires padding.
        ///
        /// `invalid_tag` is returned if the tag didn't verify.
        ///
        /// `invalid_operation` is returned for algorithms not supporting
        /// encryption.
        pub fn symmetric_state_decrypt(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
        ) -> i32;
        /// - **Stream cipher:** returns `invalid_operation` since stream
        ///   ciphers do not include authentication tags.
        /// - **AEAD:** decrypts `data` into `out`. Additional data must have
        ///   been previously absorbed using `symmetric_state_absorb()`.
        /// - **SHOE, Xoodyak, Strobe:** decrypts data, squeezes a tag and
        ///   verify that it matches the expected one.
        ///
        /// `raw_tag` is the expected tag, as raw bytes.
        ///
        /// `out` and `data` be must have the same length.
        /// If they also share the same address, decryption may happen in-place.
        ///
        /// The function returns the actual size of the decrypted message.
        ///
        /// `invalid_tag` is returned if the tag verification failed.
        ///
        /// `invalid_operation` is returned for algorithms not supporting
        /// encryption.
        pub fn symmetric_state_decrypt_detached(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
            arg6: i32,
            arg7: i32,
        ) -> i32;
        /// Make it impossible to recover the previous state.
        ///
        /// This operation is supported by some systems keeping a rolling state
        /// over an entire session, for forward security.
        ///
        /// `invalid_operation` is returned for algorithms not supporting
        /// ratcheting.
        pub fn symmetric_state_ratchet(arg0: i32) -> i32;
        /// Return the length of an authentication tag.
        ///
        /// This function can be used by a guest to allocate the correct buffer
        /// size to copy a computed authentication tag.
        pub fn symmetric_tag_len(arg0: i32, arg1: i32) -> i32;
        /// Copy an authentication tag into a guest-allocated buffer.
        ///
        /// The handle automatically becomes invalid after this operation.
        /// Manually closing it is not required.
        ///
        /// Example usage:
        ///
        /// ```rust
        /// let mut raw_tag = [0u8; 16];
        /// ctx.symmetric_tag_pull(raw_tag_handle, &mut raw_tag)?;
        /// ```
        ///
        /// The function returns `overflow` if the supplied buffer is too small
        /// to copy the tag.
        ///
        /// Otherwise, it returns the number of bytes that have been copied.
        pub fn symmetric_tag_pull(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
        /// Verify that a computed authentication tag matches the expected
        /// value, in constant-time.
        ///
        /// The expected tag must be provided as a raw byte string.
        ///
        /// The function returns `invalid_tag` if the tags don't match.
        ///
        /// Example usage:
        ///
        /// ```rust
        /// let key_handle = ctx.symmetric_key_import("HMAC/SHA-256", b"key")?;
        /// let state_handle = ctx.symmetric_state_open("HMAC/SHA-256", Some(key_handle), None)?;
        /// ctx.symmetric_state_absorb(state_handle, b"data")?;
        /// let computed_tag_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
        /// ctx.symmetric_tag_verify(computed_tag_handle, expected_raw_tag)?;
        /// ```
        pub fn symmetric_tag_verify(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Explicitly destroy an unused authentication tag.
        ///
        /// This is usually not necessary, as `symmetric_tag_pull()`
        /// automatically closes a tag after it has been copied.
        ///
        /// Objects are reference counted. It is safe to close an object
        /// immediately after the last function needing it is called.
        pub fn symmetric_tag_close(arg0: i32) -> i32;
    }
}
/// Perform a simple Diffie-Hellman key exchange.
///
/// Both keys must be of the same type, or else the
/// `$crypto_errno.incompatible_keys` error is returned. The algorithm also has
/// to support this kind of key exchange. If this is not the case, the
/// `$crypto_errno.invalid_operation` error is returned.
///
/// Otherwide, a raw shared key is returned, and can be imported as a symmetric
/// key. ```
pub unsafe fn kx_dh(pk: Publickey, sk: Secretkey) -> Result<ArrayOutput, CryptoErrno> {
    let mut rp0 = MaybeUninit::<ArrayOutput>::uninit();
    let ret = wasi_ephemeral_crypto_kx::kx_dh(pk as i32, sk as i32, rp0.as_mut_ptr() as i32);
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const ArrayOutput
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Create a shared secret and encrypt it for the given public key.
///
/// This operation is only compatible with specific algorithms.
/// If a selected algorithm doesn't support it,
/// `$crypto_errno.invalid_operation` is returned.
///
/// On success, both the shared secret and its encrypted version are returned.
pub unsafe fn kx_encapsulate(pk: Publickey) -> Result<(ArrayOutput, ArrayOutput), CryptoErrno> {
    let mut rp0 = MaybeUninit::<ArrayOutput>::uninit();
    let mut rp1 = MaybeUninit::<ArrayOutput>::uninit();
    let ret = wasi_ephemeral_crypto_kx::kx_encapsulate(
        pk as i32,
        rp0.as_mut_ptr() as i32,
        rp1.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok((
            core::ptr::read(rp0.as_mut_ptr() as i32 as *const ArrayOutput),
            core::ptr::read(rp1.as_mut_ptr() as i32 as *const ArrayOutput),
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

/// Decapsulate an encapsulated secret crated with `kx_encapsulate`
///
/// Return the secret, or `$crypto_errno.verification_failed` on error.
pub unsafe fn kx_decapsulate(
    sk: Secretkey,
    encapsulated_secret: *const u8,
    encapsulated_secret_len: Size,
) -> Result<ArrayOutput, CryptoErrno> {
    let mut rp0 = MaybeUninit::<ArrayOutput>::uninit();
    let ret = wasi_ephemeral_crypto_kx::kx_decapsulate(
        sk as i32,
        encapsulated_secret as i32,
        encapsulated_secret_len as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const ArrayOutput
        )),
        _ => Err(CryptoErrno(ret as u16)),
    }
}

pub mod wasi_ephemeral_crypto_kx {
    #[link(wasm_import_module = "wasi_ephemeral_crypto_kx")]
    extern "C" {
        /// Perform a simple Diffie-Hellman key exchange.
        ///
        /// Both keys must be of the same type, or else the
        /// `$crypto_errno.incompatible_keys` error is returned.
        /// The algorithm also has to support this kind of key exchange. If this
        /// is not the case, the `$crypto_errno.invalid_operation` error is
        /// returned.
        ///
        /// Otherwide, a raw shared key is returned, and can be imported as a
        /// symmetric key. ```
        pub fn kx_dh(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Create a shared secret and encrypt it for the given public key.
        ///
        /// This operation is only compatible with specific algorithms.
        /// If a selected algorithm doesn't support it,
        /// `$crypto_errno.invalid_operation` is returned.
        ///
        /// On success, both the shared secret and its encrypted version are
        /// returned.
        pub fn kx_encapsulate(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Decapsulate an encapsulated secret crated with `kx_encapsulate`
        ///
        /// Return the secret, or `$crypto_errno.verification_failed` on error.
        pub fn kx_decapsulate(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
    }
}
pub const VERSION_UNSPECIFIED: Version = 18374686479671623680;
pub const VERSION_LATEST: Version = 18374686479671623681;
pub const VERSION_ALL: Version = 18374686479671623682;
