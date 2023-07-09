pub use wasmedge_wasi_socket::wasi_poll::*;

pub fn poll(
    in_: *const Subscription,
    out: *mut Event,
    nsubscriptions: usize,
) -> std::io::Result<usize> {
    let mut rp0 = 0_usize;
    let ret = unsafe {
        wasmedge_wasi_socket::wasi_poll::poll_oneoff(
            in_ as i32,
            out as i32,
            nsubscriptions as i32,
            (&mut rp0) as *mut usize as i32,
        )
    };
    match ret {
        0 => Ok(rp0),
        _ => Err(std::io::Error::from_raw_os_error(ret)),
    }
}
