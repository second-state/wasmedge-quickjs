// copy from https://github.com/bytecodealliance/wasi/blob/main/src/lib_generated.rs
pub type Fd = u32;
pub type Filesize = u64;
pub type Timestamp = u64;
pub type Errno = u16;

pub type Clockid = u32;

pub const CLOCKID_REALTIME: Clockid = 0;
pub const CLOCKID_MONOTONIC: Clockid = 1;
pub const CLOCKID_PROCESS_CPUTIME_ID: Clockid = 2;
pub const CLOCKID_THREAD_CPUTIME_ID: Clockid = 3;

pub type Userdata = u64;
pub type Eventtype = u8;

pub const EVENTTYPE_CLOCK: Eventtype = 0;
pub const EVENTTYPE_FD_READ: Eventtype = 1;
pub const EVENTTYPE_FD_WRITE: Eventtype = 2;

pub type Eventrwflags = u16;

/// The peer of this socket has closed or disconnected.
pub const EVENTRWFLAGS_FD_READWRITE_HANGUP: Eventrwflags = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EventFdReadwrite {
    pub nbytes: Filesize,
    pub flags: Eventrwflags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Event {
    pub userdata: Userdata,
    pub error: Errno,
    pub type_: Eventtype,
    pub fd_readwrite: EventFdReadwrite,
}

pub type Subclockflags = u16;

pub const SUBCLOCKFLAGS_SUBSCRIPTION_CLOCK_ABSTIME: Subclockflags = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SubscriptionClock {
    pub id: Clockid,
    pub timeout: Timestamp,
    pub precision: Timestamp,
    pub flags: Subclockflags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SubscriptionFdReadwrite {
    pub file_descriptor: Fd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union SubscriptionUU {
    pub clock: SubscriptionClock,
    pub fd_read: SubscriptionFdReadwrite,
    pub fd_write: SubscriptionFdReadwrite,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubscriptionU {
    pub tag: u8,
    pub u: SubscriptionUU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Subscription {
    pub userdata: Userdata,
    pub u: SubscriptionU,
}

mod wasi {
    #[link(wasm_import_module = "wasi_snapshot_preview1")]
    extern "C" {
        pub fn poll_oneoff(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
    }
}

pub unsafe fn poll_oneoff(
    in_: *const Subscription,
    out: *mut Event,
    nsubscriptions: usize,
) -> std::io::Result<usize> {
    let mut rp0 = 0_usize;
    let ret = wasi::poll_oneoff(
        in_ as i32,
        out as i32,
        nsubscriptions as i32,
        (&mut rp0) as *mut usize as i32,
    );
    match ret {
        0 => Ok(rp0),
        _ => Err(std::io::Error::from_raw_os_error(ret)),
    }
}
