use std::os::wasi::prelude::RawFd;
pub use wasmedge_wasi_socket::nslookup;
pub use wasmedge_wasi_socket::socket::*;
pub type RawSocket = RawFd;
