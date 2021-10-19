use std::ffi::c_void;

#[link(wasm_import_module = "wasmedge_async_sock")]
extern "C" {
    pub fn async_sock_tcp_connect(host: *const u8, host_len: usize, callback: *mut c_void);
    pub fn async_sock_tcp_listen(port: u32, callback: *mut c_void);
    pub fn async_sock_accept(sock: i32, callback: *mut c_void);
    pub fn async_sock_close(sock: i32);
    pub fn async_sock_read(sock: i32, buf_ptr: *mut u8, len: usize, callback: *mut c_void);
    pub fn async_sock_write(sock: i32, data: *const u8, len: usize, callback: *mut c_void);
}

fn io_error_to_kind(error_code: i32) -> std::io::ErrorKind {
    use std::io::ErrorKind::*;
    const KINDS: [std::io::ErrorKind; 16] = [
        NotFound,
        ConnectionRefused,
        ConnectionReset,
        ConnectionAborted,
        NotConnected,
        AddrInUse,
        AddrNotAvailable,
        BrokenPipe,
        WouldBlock,
        InvalidInput,
        InvalidData,
        TimedOut,
        WriteZero,
        Interrupted,
        Other,
        UnexpectedEof,
    ];

    KINDS
        .get((error_code - 1) as usize)
        .cloned()
        .unwrap_or(Other)
}

#[no_mangle]
unsafe extern "C" fn wasmedge_async_new_socket_trampoline(
    sock: i32,
    error_code: i32,
    callback: *mut c_void,
) {
    if let Some(g) = &GLOBAL_POLL_FN {
        let s = if error_code == 0 {
            Ok(sock)
        } else {
            Err(io_error_to_kind(error_code))
        };
        let f = g.new_socket_trampoline;
        f(callback, s)
    } else {
        panic!("io_poll is not init")
    }
}

#[no_mangle]
unsafe extern "C" fn wasmedge_async_read_callback_trampoline(
    len: usize,
    error_code: i32,
    callback: *mut c_void,
) {
    if let Some(g) = &GLOBAL_POLL_FN {
        let l = if error_code == 0 {
            Ok(len)
        } else {
            Err(io_error_to_kind(error_code))
        };
        let f = g.read_callback_trampoline;
        f(callback, l)
    } else {
        panic!("io_poll is not init")
    }
}

#[no_mangle]
unsafe extern "C" fn wasmedge_async_write_callback_trampoline(
    len: usize,
    error_code: i32,
    callback: *mut c_void,
) {
    if let Some(g) = &GLOBAL_POLL_FN {
        let l = if error_code == 0 {
            Ok(len)
        } else {
            Err(io_error_to_kind(error_code))
        };
        let f = g.write_callback_trampoline;
        f(callback, l)
    } else {
        panic!("io_poll is not init")
    }
}

pub type IOResult<T> = Result<T, std::io::ErrorKind>;

pub struct IOPollFn {
    pub new_socket_trampoline: fn(callback: *mut c_void, sock: IOResult<i32>),
    pub read_callback_trampoline: fn(callback: *mut c_void, res: IOResult<usize>),
    pub write_callback_trampoline: fn(callback: *mut c_void, res: IOResult<usize>),
}

static mut GLOBAL_POLL_FN: Option<IOPollFn> = None;

pub fn init(os_poll_fn: IOPollFn) {
    unsafe { GLOBAL_POLL_FN = Some(os_poll_fn) };
}
