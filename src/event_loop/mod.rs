mod wasi_sock;

use crate::event_loop::poll::Subscription;
use crate::{quickjs_sys as qjs, Context, JsValue};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, LinkedList};
use std::io;
use std::mem::ManuallyDrop;
use std::net::{SocketAddr, SocketAddrV4};
use std::ops::Add;

trait AsPollFd {
    fn as_subscription(&self) -> Option<poll::Subscription>;
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

pub(crate) enum NetPollEvent {
    Accept,
    Read,
    Connect,
}

pub enum NetPollResult {
    Accept(AsyncTcpConn),
    Read(Vec<u8>),
    Connect,
    Error(io::Error),
}

pub struct AsyncTcpServer(wasi_sock::Socket);
pub struct AsyncTcpConn(wasi_sock::Socket);

impl AsyncTcpConn {
    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.send(buf)
    }

    pub fn read(&mut self) -> io::Result<Vec<u8>> {
        let mut buff = [0u8; 1024];
        let mut data = vec![];
        loop {
            let n = self.0.recv(&mut buff)?;
            data.extend_from_slice(&buff[0..n]);
            if n < 1024 {
                break Ok(data);
            }
        }
    }

    pub fn async_read(
        &mut self,
        event_loop: &mut EventLoop,
        callback: Box<dyn FnOnce(&mut qjs::Context, NetPollResult)>,
    ) {
        let fd = PollFd {
            s: self.0 .0,
            event: NetPollEvent::Read,
            callback,
        };
        event_loop.io_selector.register(fd);
    }

    pub fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct PollFd {
    s: wasi_sock::RawSocket,
    event: NetPollEvent,
    callback: Box<dyn FnOnce(&mut qjs::Context, NetPollResult)>,
}

impl AsPollFd for PollFd {
    fn as_subscription(&self) -> Option<poll::Subscription> {
        match self.event {
            NetPollEvent::Accept | NetPollEvent::Read => Some(poll::Subscription {
                userdata: self.s as u64,
                u: poll::SubscriptionU {
                    tag: poll::EVENTTYPE_FD_READ,
                    u: poll::SubscriptionUU {
                        fd_read: poll::SubscriptionFdReadwrite {
                            file_descriptor: self.s as u32,
                        },
                    },
                },
            }),
            NetPollEvent::Connect => Some(poll::Subscription {
                userdata: self.s as u64,
                u: poll::SubscriptionU {
                    tag: poll::EVENTTYPE_FD_WRITE,
                    u: poll::SubscriptionUU {
                        fd_read: poll::SubscriptionFdReadwrite {
                            file_descriptor: self.s as u32,
                        },
                    },
                },
            }),
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
    fds: HashMap<i32, PollFd>,
    timeouts: Vec<Option<Timeout>>,
}

impl IoSelector {
    pub fn register(&mut self, s: PollFd) -> i32 {
        let fd = s.s;
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
            if let Some(fd) = v.as_subscription() {
                subscription_vec.push(fd);
            }
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
                poll::EVENTTYPE_FD_READ | poll::EVENTTYPE_FD_WRITE => {
                    let fd = event.userdata as u32 as i32;
                    match self.fds.remove(&fd) {
                        None => {}
                        Some(PollFd {
                            s,
                            event: net_event,
                            callback,
                        }) => {
                            if event.fd_readwrite.flags & poll::EVENTRWFLAGS_FD_READWRITE_HANGUP > 0
                            {
                                let e = io::Error::from_raw_os_error(1);
                                callback(ctx, NetPollResult::Error(e));
                                continue;
                            }
                            if event.error > 0 {
                                let e = io::Error::from_raw_os_error(event.error as i32);
                                callback(ctx, NetPollResult::Error(e));
                                continue;
                            }

                            match net_event {
                                NetPollEvent::Accept => {
                                    let s = std::mem::ManuallyDrop::new(wasi_sock::Socket(s));
                                    match s.accept() {
                                        Ok(cs) => {
                                            callback(ctx, NetPollResult::Accept(AsyncTcpConn(cs)))
                                        }
                                        Err(e) => callback(ctx, NetPollResult::Error(e)),
                                    }
                                }
                                NetPollEvent::Read => {
                                    let mut s = std::mem::ManuallyDrop::new(AsyncTcpConn(
                                        wasi_sock::Socket(s),
                                    ));
                                    match s.read() {
                                        Ok(data) => callback(ctx, NetPollResult::Read(data)),
                                        Err(e) => callback(ctx, NetPollResult::Error(e)),
                                    }
                                }
                                NetPollEvent::Connect => callback(ctx, NetPollResult::Connect),
                            };
                        }
                    };
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

    pub fn tcp_listen(&mut self, port: u16) -> io::Result<AsyncTcpServer> {
        let addr = format!("0.0.0.0:{}", port)
            .parse()
            .map_err(|_e| io::Error::from(io::ErrorKind::InvalidInput))?;

        use wasi_sock::socket_types as st;
        let s = wasi_sock::Socket::new(st::AF_INET4 as i32, st::SOCK_STREAM)?;
        s.set_nonblocking(true)?;
        s.bind(&addr)?;
        s.listen(1024)?;
        Ok(AsyncTcpServer(s))
    }
    pub fn tcp_accept(
        &mut self,
        tcp_server: &mut AsyncTcpServer,
        callback: Box<dyn FnOnce(&mut qjs::Context, NetPollResult)>,
    ) {
        let s = tcp_server.0 .0;
        let poll_fd = PollFd {
            s,
            event: NetPollEvent::Accept,
            callback,
        };
        self.io_selector.register(poll_fd);
    }
    pub fn tcp_connect(
        &mut self,
        addr: &SocketAddr,
        _callback: impl FnOnce(&mut qjs::Context, NetPollResult),
    ) -> io::Result<()> {
        use wasi_sock::socket_types as st;
        let s = wasi_sock::Socket::new(st::AF_INET4 as i32, st::SOCK_STREAM)?;
        s.set_nonblocking(true)?;
        s.connect(addr)?;
        //todo
        Ok(())
    }

    pub fn close(&mut self, fd: i32) {
        self.io_selector.fds.remove(&fd);
    }
}
