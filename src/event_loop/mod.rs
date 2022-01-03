mod wasi_sock;

use crate::event_loop::poll::Subscription;
use crate::{quickjs_sys as qjs, Context, JsValue};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, LinkedList};
use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use std::ops::Add;

trait AsPollFd {
    fn as_subscription(&self) -> poll::Subscription;
}

#[derive(Clone)]
struct Timeout(u128, qjs::JsFunction);

impl Timeout {
    fn as_subscription(&self, index: usize) -> Subscription {
        let nanoseconds = self.0;
        poll::Subscription {
            userdata: index as u64,
            u: poll::SubscriptionU {
                tag: poll::EVENTTYPE_CLOCK,
                u: poll::SubscriptionUU {
                    clock: poll::SubscriptionClock {
                        id: poll::CLOCKID_REALTIME,
                        timeout: nanoseconds as u64,
                        precision: 0,
                        flags: poll::SUBCLOCKFLAGS_SUBSCRIPTION_CLOCK_ABSTIME,
                    },
                },
            },
        }
    }
}

struct TcpListener(wasi_sock::Socket, qjs::JsObject);

impl AsPollFd for TcpListener {
    fn as_subscription(&self) -> Subscription {
        poll::Subscription {
            userdata: self.0 .0 as u64,
            u: poll::SubscriptionU {
                tag: poll::EVENTTYPE_FD_READ,
                u: poll::SubscriptionUU {
                    fd_read: poll::SubscriptionFdReadwrite {
                        file_descriptor: self.0 .0 as u32,
                    },
                },
            },
        }
    }
}

struct TcpConn(wasi_sock::Socket, qjs::JsObject);

impl AsPollFd for TcpConn {
    fn as_subscription(&self) -> Subscription {
        poll::Subscription {
            userdata: self.0 .0 as u64,
            u: poll::SubscriptionU {
                tag: poll::EVENTTYPE_FD_READ,
                u: poll::SubscriptionUU {
                    fd_read: poll::SubscriptionFdReadwrite {
                        file_descriptor: self.0 .0 as u32,
                    },
                },
            },
        }
    }
}

impl TcpConn {
    pub fn on_connect(&mut self, ctx: &mut qjs::Context) -> io::Result<()> {
        if let qjs::JsValue::Function(on_connect) = self.1.get("on_connect") {
            let mut obj = ctx.new_object();
            obj.set("fd", JsValue::Int(self.0 .0));
            let peer = self.0.get_peer()?;
            obj.set("peer", ctx.new_string(peer.to_string().as_str()).into());
            on_connect.call(&mut [obj.into()]);
        }
        Ok(())
    }
    pub fn on_read(&mut self, ctx: &mut qjs::Context) -> io::Result<usize> {
        let mut buff = [0u8; 1024];
        let mut data = vec![];
        loop {
            let n = self.0.recv(&mut buff)?;
            data.extend_from_slice(&buff[0..n]);
            if n == 0 && data.is_empty() {
                if let qjs::JsValue::Function(on_close) = self.1.get("on_close") {
                    on_close.call(&mut []);
                }
                break Ok(0);
            } else if n < 1024 {
                if let qjs::JsValue::Function(on_read) = self.1.get("on_read") {
                    let array_buf = ctx.new_array_buffer(data.as_slice());
                    on_read.call(&mut [JsValue::Int(self.0 .0), array_buf.into()]);
                }
                break Ok(data.len());
            }
        }
    }
    pub fn on_error(&mut self, ctx: &mut qjs::Context, e: &str) {
        if let qjs::JsValue::Function(on_error) = self.1.get("on_error") {
            let e = ctx.new_error(e);
            on_error.call(&mut [e]);
        }
    }
}

enum EnumFd {
    TcpListener(TcpListener),
    TcpConn(TcpConn),
}

impl AsPollFd for EnumFd {
    fn as_subscription(&self) -> poll::Subscription {
        match self {
            EnumFd::TcpListener(s) => s.as_subscription(),
            EnumFd::TcpConn(s) => s.as_subscription(),
        }
    }
}

mod poll {
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
}

#[derive(Default)]
struct IoSelector {
    fds: HashMap<i32, EnumFd>,
    timeouts: Vec<Option<Timeout>>,
}

impl IoSelector {
    pub fn register(&mut self, s: EnumFd) -> i32 {
        let fd = match &s {
            EnumFd::TcpListener(s) => s.0 .0,
            EnumFd::TcpConn(s) => s.0 .0,
        };
        self.fds.insert(fd, s);
        fd
    }

    pub fn unregister(&mut self, fd: i32) {
        self.fds.remove(&fd);
    }

    pub fn set_timeout(&mut self, timeout: Timeout) -> usize {
        let mut n = 0;
        for t in &mut self.timeouts {
            if t.is_none() {
                t.insert(timeout);
                return n;
            }
            n += 1;
        }
        self.timeouts.push(Some(timeout));
        return n;
    }

    pub fn clear_timeout(&mut self, id: usize) -> Option<Timeout> {
        self.timeouts.get_mut(id)?.take()
    }

    pub fn poll(&mut self, ctx: &mut qjs::Context) -> io::Result<usize> {
        let mut subscription_vec = Vec::with_capacity(self.fds.len());
        for (i, timeout) in self.timeouts.iter().enumerate() {
            if let Some(timeout) = timeout {
                subscription_vec.push(timeout.as_subscription(i));
            }
        }
        for (_, v) in &self.fds {
            subscription_vec.push(v.as_subscription());
        }
        if subscription_vec.is_empty() {
            return Ok(0);
        }
        let mut revent = vec![
            poll::Event {
                userdata: 0,
                error: 0,
                type_: 0,
                fd_readwrite: poll::EventFdReadwrite {
                    nbytes: 0,
                    flags: 0,
                },
            };
            subscription_vec.len()
        ];

        let n = unsafe {
            poll::poll_oneoff(
                subscription_vec.as_ptr(),
                revent.as_mut_ptr(),
                subscription_vec.len(),
            )
        }?;

        for i in 0..n {
            let event = revent[i];
            match event.type_ {
                poll::EVENTTYPE_CLOCK => {
                    let i = event.userdata as usize;
                    if let Some(timeout) = self.clear_timeout(i) {
                        timeout.1.call(&mut []);
                    };
                }
                poll::EVENTTYPE_FD_READ => {
                    let fd = event.userdata as u32 as i32;
                    let mut need_remove = false;
                    let mut e = None;
                    let cs = match self.fds.get_mut(&fd) {
                        None => None,
                        Some(EnumFd::TcpConn(s)) => {
                            if event.fd_readwrite.flags & poll::EVENTRWFLAGS_FD_READWRITE_HANGUP > 0
                            {
                                s.on_error(ctx, "pipe hang up");
                                need_remove = true;
                            }
                            if event.error > 0 {
                                let e = io::Error::from_raw_os_error(event.error as i32);
                                s.on_error(ctx, e.to_string().as_str());
                                need_remove = true;
                            }
                            if !need_remove {
                                if let Ok(n) = s.on_read(ctx) {
                                    if n == 0 {
                                        need_remove = true;
                                    }
                                };
                            }
                            None
                        }
                        Some(EnumFd::TcpListener(s)) => {
                            if event.error > 0 {
                                e = Some(io::Error::from_raw_os_error(event.error as i32));
                                need_remove = true;
                            }
                            if !need_remove {
                                let cs = s.0.accept();
                                let cs = cs?;
                                cs.set_nonblocking(true);
                                Some((cs, s.1.clone()))
                            } else {
                                None
                            }
                        }
                    };
                    if let Some((cs, callback)) = cs {
                        let mut conn = TcpConn(cs, callback);
                        let fd = self.register(EnumFd::TcpConn(conn));
                        if let Some(EnumFd::TcpConn(ref mut conn)) = self.fds.get_mut(&fd) {
                            conn.on_connect(ctx);
                        };
                    }
                    if need_remove {
                        let _ = self.fds.remove(&fd);
                    }
                    if let Some(e) = e {
                        return Err(e);
                    }
                }
                _ => {}
            };
        }
        Ok(n)
    }
}

#[derive(Default)]
pub struct EventLoop {
    next_tick_queue: LinkedList<qjs::JsFunction>,
    io_selector: IoSelector,
}

impl EventLoop {
    pub fn inst() -> &'static mut EventLoop {
        loop {
            unsafe {
                if let Some(ref mut p) = EVENT_INST {
                    return p;
                }
                EVENT_INST.insert(EventLoop::default());
            }
        }
    }

    pub fn run_once(&mut self, ctx: &mut qjs::Context) -> io::Result<usize> {
        while let Some(f) = self.next_tick_queue.pop_front() {
            f.call(&mut []);
        }
        self.io_selector.poll(ctx)
    }

    pub fn set_timeout(
        &mut self,
        callback: qjs::JsFunction,
        timeout: std::time::Duration,
    ) -> usize {
        let ddl = std::time::SystemTime::now().add(timeout);
        let ddl = ddl
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let timeout_task = Timeout(ddl, callback);
        self.io_selector.set_timeout(timeout_task)
    }

    pub fn clear_timeout(&mut self, timeout_id: usize) {
        self.io_selector.clear_timeout(timeout_id);
    }

    pub fn set_next_tick(&mut self, callback: qjs::JsFunction) {
        self.next_tick_queue.push_back(callback);
    }

    pub fn tcp_listen(&mut self, port: u16, callback: qjs::JsObject) -> io::Result<()> {
        let addr = format!("0.0.0.0:{}", port)
            .parse()
            .map_err(|e| io::Error::from(io::ErrorKind::InvalidInput))?;

        use wasi_sock::socket_types as st;
        let s = wasi_sock::Socket::new(st::AF_INET4 as i32, st::SOCK_STREAM)?;
        s.set_nonblocking(true)?;
        s.bind(&addr)?;
        s.listen(1024)?;

        let s = EnumFd::TcpListener(TcpListener(s, callback));
        self.io_selector.register(s);
        Ok(())
    }

    pub fn write(&mut self, fd: i32, data: &[u8]) -> Option<usize> {
        let s = self.io_selector.fds.get_mut(&fd)?;
        if let EnumFd::TcpConn(conn) = s {
            conn.0.send(data).ok()
        } else {
            None
        }
    }
}

static mut EVENT_INST: Option<EventLoop> = None;
