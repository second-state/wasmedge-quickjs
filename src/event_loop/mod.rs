mod poll;
mod wasi_sock;
pub mod wasi_fs;

use crate::event_loop::poll::{Eventtype, Subscription};
use crate::{quickjs_sys as qjs, Context, JsValue};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, LinkedList};
use std::io;
use std::mem::ManuallyDrop;
use std::net::{SocketAddr, SocketAddrV4};
use std::ops::Add;

pub use wasi_sock::nslookup;

pub(crate) enum NetPollEvent {
    Accept,
    Read,
    Connect,
}

pub struct AsyncTcpServer(wasi_sock::Socket);
impl AsyncTcpServer {
    pub fn async_accept(
        &mut self,
        event_loop: &mut EventLoop,
        callback: Box<dyn FnOnce(&mut qjs::Context, PollResult)>,
        timeout: Option<std::time::Duration>,
    ) {
        let s = self.0 .0;
        if let Some(timeout) = timeout {
            let ddl = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .add(timeout)
                .as_nanos();
            event_loop
                .io_selector
                .add_task(PollTask::SocketTimeout(SocketTimeoutTask {
                    s,
                    event: NetPollEvent::Accept,
                    timeout: ddl,
                    callback,
                }));
        } else {
            event_loop
                .io_selector
                .add_task(PollTask::Socket(SocketTask {
                    s,
                    event: NetPollEvent::Accept,
                    callback,
                }));
        }
    }
}

pub struct AsyncTcpConn(wasi_sock::Socket);
impl AsyncTcpConn {
    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.send(buf)
    }

    pub fn read(&mut self) -> io::Result<Vec<u8>> {
        let mut buff = [0u8; 1024];
        let mut data = vec![];
        loop {
            match self.0.recv(&mut buff) {
                Ok(0) => {
                    return Ok(data);
                }
                Ok(n) => {
                    data.extend_from_slice(&buff[0..n]);
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    return Ok(data);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    pub fn async_read(
        &mut self,
        event_loop: &mut EventLoop,
        callback: Box<dyn FnOnce(&mut qjs::Context, PollResult)>,
        timeout: Option<std::time::Duration>,
    ) {
        if let Some(timeout) = timeout {
            let ddl = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .add(timeout)
                .as_nanos();

            event_loop
                .io_selector
                .add_task(PollTask::SocketTimeout(SocketTimeoutTask {
                    s: self.0 .0,
                    event: NetPollEvent::Read,
                    timeout: ddl,
                    callback,
                }));
        } else {
            event_loop
                .io_selector
                .add_task(PollTask::Socket(SocketTask {
                    s: self.0 .0,
                    event: NetPollEvent::Read,
                    callback,
                }));
        }
    }

    pub fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    pub fn local(&self) -> io::Result<SocketAddr> {
        self.0.get_local()
    }

    pub fn peer(&self) -> io::Result<SocketAddr> {
        self.0.get_peer()
    }
}

pub enum PollResult {
    Timeout,
    Accept(AsyncTcpConn),
    Read(Vec<u8>),
    Connect(AsyncTcpConn),
    Error(io::Error),
    Write(usize)
}

struct TimeoutTask {
    timeout: u128,
    callback: Box<dyn FnOnce(&mut qjs::Context, PollResult)>,
}

impl TimeoutTask {
    fn as_subscription(&self, index: usize) -> Subscription {
        let nanoseconds = self.timeout;
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

struct SocketTask {
    s: wasi_sock::RawSocket,
    event: NetPollEvent,
    callback: Box<dyn FnOnce(&mut qjs::Context, PollResult)>,
}

impl SocketTask {
    fn as_subscription(&self, index: usize) -> Subscription {
        match self.event {
            NetPollEvent::Accept | NetPollEvent::Read => poll::Subscription {
                userdata: index as u64,
                u: poll::SubscriptionU {
                    tag: poll::EVENTTYPE_FD_READ,
                    u: poll::SubscriptionUU {
                        fd_read: poll::SubscriptionFdReadwrite {
                            file_descriptor: self.s as u32,
                        },
                    },
                },
            },
            NetPollEvent::Connect => poll::Subscription {
                userdata: index as u64,
                u: poll::SubscriptionU {
                    tag: poll::EVENTTYPE_FD_WRITE,
                    u: poll::SubscriptionUU {
                        fd_read: poll::SubscriptionFdReadwrite {
                            file_descriptor: self.s as u32,
                        },
                    },
                },
            },
        }
    }
}

struct SocketTimeoutTask {
    s: wasi_sock::RawSocket,
    event: NetPollEvent,
    timeout: u128,
    callback: Box<dyn FnOnce(&mut qjs::Context, PollResult)>,
}

impl SocketTimeoutTask {
    fn as_subscription(&self, index: usize) -> (Subscription, Subscription) {
        let socket_task = match self.event {
            NetPollEvent::Accept | NetPollEvent::Read => poll::Subscription {
                userdata: index as u64,
                u: poll::SubscriptionU {
                    tag: poll::EVENTTYPE_FD_READ,
                    u: poll::SubscriptionUU {
                        fd_read: poll::SubscriptionFdReadwrite {
                            file_descriptor: self.s as u32,
                        },
                    },
                },
            },
            NetPollEvent::Connect => poll::Subscription {
                userdata: index as u64,
                u: poll::SubscriptionU {
                    tag: poll::EVENTTYPE_FD_WRITE,
                    u: poll::SubscriptionUU {
                        fd_read: poll::SubscriptionFdReadwrite {
                            file_descriptor: self.s as u32,
                        },
                    },
                },
            },
        };
        let timeout_task = {
            let nanoseconds = self.timeout;
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
        };
        (socket_task, timeout_task)
    }
}

struct FdReadTask {
    fd: std::os::wasi::io::RawFd,
    len: u64,
    callback: Box<dyn FnOnce(&mut qjs::Context, PollResult)>,
}

impl FdReadTask {
    fn as_subscription(&self, index: usize) -> Subscription {
        poll::Subscription {
            userdata: index as u64,
            u: poll::SubscriptionU {
                tag: poll::EVENTTYPE_FD_READ,
                u: poll::SubscriptionUU {
                    fd_read: poll::SubscriptionFdReadwrite {
                        file_descriptor: self.fd as u32,
                    },
                },
            },
        }
    }
}

struct FdWriteTask {
    fd: std::os::wasi::io::RawFd,
    buf: Vec<u8>,
    callback: Box<dyn FnOnce(&mut qjs::Context, PollResult)>,
}

impl FdWriteTask {
    fn as_subscription(&self, index: usize) -> Subscription {
        poll::Subscription {
            userdata: index as u64,
            u: poll::SubscriptionU {
                tag: poll::EVENTTYPE_FD_WRITE,
                u: poll::SubscriptionUU {
                    fd_write: poll::SubscriptionFdReadwrite {
                        file_descriptor: self.fd as u32,
                    },
                },
            },
        }
    }
}

enum PollTask {
    Timeout(TimeoutTask),
    Socket(SocketTask),
    SocketTimeout(SocketTimeoutTask),
    FdRead(FdReadTask),
    FdWrite(FdWriteTask),
}

#[derive(Default)]
struct IoSelector {
    tasks: Vec<Option<PollTask>>,
}

impl IoSelector {
    pub fn add_task(&mut self, task: PollTask) -> usize {
        let mut n = 0;
        for t in &mut self.tasks {
            if t.is_none() {
                t.insert(task);
                return n;
            }
            n += 1;
        }
        self.tasks.push(Some(task));
        n
    }

    pub fn delete_task(&mut self, id: usize) -> Option<PollTask> {
        self.tasks.get_mut(id)?.take()
    }

    pub fn poll(&mut self, ctx: &mut qjs::Context) -> io::Result<usize> {
        let mut subscription_vec = Vec::with_capacity(self.tasks.len());
        for (i, timeout) in self.tasks.iter().enumerate() {
            if let Some(task) = timeout {
                match task {
                    PollTask::Timeout(task) => {
                        subscription_vec.push(task.as_subscription(i));
                    }
                    PollTask::Socket(task) => {
                        subscription_vec.push(task.as_subscription(i));
                    }
                    PollTask::SocketTimeout(task) => {
                        let (task1, task2) = task.as_subscription(i);
                        subscription_vec.push(task1);
                        subscription_vec.push(task2);
                    }
                    PollTask::FdRead(task) => {
                        subscription_vec.push(task.as_subscription(i));
                    }
                    PollTask::FdWrite(task) => {
                        subscription_vec.push(task.as_subscription(i));
                    }
                }
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
            let index = event.userdata as usize;
            if let Some(task) = self.delete_task(index) {
                match (task, event.type_) {
                    (PollTask::Timeout(TimeoutTask { callback, .. }), poll::EVENTTYPE_CLOCK) => {
                        callback(ctx, PollResult::Timeout);
                    }
                    (
                        PollTask::SocketTimeout(SocketTimeoutTask { callback, .. }),
                        poll::EVENTTYPE_CLOCK,
                    ) => {
                        callback(ctx, PollResult::Timeout);
                    }
                    (
                        PollTask::SocketTimeout(SocketTimeoutTask {
                            s,
                            event: net_event,
                            callback,
                            ..
                        })
                        | PollTask::Socket(SocketTask {
                            s,
                            event: net_event,
                            callback,
                            ..
                        }),
                        poll::EVENTTYPE_FD_READ | poll::EVENTTYPE_FD_WRITE,
                    ) => {
                        if event.error > 0 {
                            let e = io::Error::from_raw_os_error(event.error as i32);
                            callback(ctx, PollResult::Error(e));
                            continue;
                        }

                        match net_event {
                            NetPollEvent::Accept => {
                                let s = std::mem::ManuallyDrop::new(wasi_sock::Socket(s));
                                match s.accept(true) {
                                    Ok(cs) => callback(ctx, PollResult::Accept(AsyncTcpConn(cs))),
                                    Err(e) => callback(ctx, PollResult::Error(e)),
                                }
                            }
                            NetPollEvent::Read => {
                                let mut s =
                                    std::mem::ManuallyDrop::new(AsyncTcpConn(wasi_sock::Socket(s)));
                                match s.read() {
                                    Ok(data) => callback(ctx, PollResult::Read(data)),
                                    Err(e) => callback(ctx, PollResult::Error(e)),
                                }
                            }
                            NetPollEvent::Connect => {
                                if event.fd_readwrite.flags & poll::EVENTRWFLAGS_FD_READWRITE_HANGUP
                                    > 0
                                {
                                    let e = io::Error::from(io::ErrorKind::ConnectionAborted);
                                    callback(ctx, PollResult::Error(e));
                                } else {
                                    let s = AsyncTcpConn(wasi_sock::Socket(s));
                                    callback(ctx, PollResult::Connect(s));
                                }
                            }
                        };
                    }
                    (
                        PollTask::FdRead(FdReadTask { fd, len, callback }),
                        poll::EVENTTYPE_FD_READ,
                    ) => {
                        if event.error > 0 {
                            let e = io::Error::from_raw_os_error(event.error as i32);
                            callback(ctx, PollResult::Error(e));
                            continue;
                        }
                        let len = len as usize; // len.min(event.fd_readwrite.nbytes) as usize;
                        let mut buf = vec![0u8; len];
                        let res = unsafe {
                            wasi_fs::fd_read(
                                fd as u32,
                                &[wasi_fs::Iovec {
                                    buf: buf.as_mut_ptr(),
                                    buf_len: len,
                                }],
                            )
                        };
                        callback(
                            ctx,
                            match res {
                                Ok(_) => PollResult::Read(buf),
                                Err(e) => {
                                    PollResult::Error(io::Error::from_raw_os_error(e.raw() as i32))
                                }
                            },
                        );
                    }
                    (
                        PollTask::FdWrite(FdWriteTask { fd, buf, callback }),
                        poll::EVENTTYPE_FD_WRITE,
                    ) => {
                        if event.error > 0 {
                            let e = io::Error::from_raw_os_error(event.error as i32);
                            callback(ctx, PollResult::Error(e));
                            continue;
                        }
                        let res = unsafe {
                            wasi_fs::fd_write(
                                fd as u32,
                                &[wasi_fs::Ciovec {
                                    buf: buf.as_ptr(),
                                    buf_len: buf.len(),
                                }],
                            )
                        };
                        callback(
                            ctx,
                            match res {
                                Ok(len) => PollResult::Write(len),
                                Err(e) => {
                                    PollResult::Error(io::Error::from_raw_os_error(e.raw() as i32))
                                }
                            },
                        );
                    }
                    (_, _) => {}
                }
            }
        }
        Ok(n)
    }
}

#[derive(Default)]
pub struct EventLoop {
    next_tick_queue: LinkedList<Box<dyn FnOnce(&mut qjs::Context)>>,
    io_selector: IoSelector,
}

impl EventLoop {
    pub fn run_once(&mut self, ctx: &mut qjs::Context) -> io::Result<usize> {
        let n = self.run_tick_task(ctx);
        if n > 0 {
            Ok(n)
        } else {
            self.io_selector.poll(ctx)
        }
    }

    fn run_tick_task(&mut self, ctx: &mut qjs::Context) -> usize {
        let mut i = 0;
        while let Some(f) = self.next_tick_queue.pop_front() {
            f(ctx);
            i += 1;
        }
        i
    }

    pub fn set_timeout(
        &mut self,
        callback: qjs::JsFunction,
        timeout: std::time::Duration,
        args: Option<Vec<JsValue>>,
    ) -> usize {
        let ddl = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .add(timeout)
            .as_nanos();

        let timeout_task = PollTask::Timeout(TimeoutTask {
            timeout: ddl,
            callback: Box::new(move |_ctx, _res| {
                match args {
                    Some(argv) => callback.call(&argv),
                    None => callback.call(&[]),
                };
            }),
        });
        self.io_selector.add_task(timeout_task)
    }

    pub fn clear_timeout(&mut self, timeout_id: usize) {
        if let Some(t) = self.io_selector.tasks.get_mut(timeout_id) {
            if let Some(PollTask::Timeout(_)) = t {
                t.take();
            };
        };
    }

    pub fn set_next_tick(&mut self, callback: Box<dyn FnOnce(&mut qjs::Context)>) {
        self.next_tick_queue.push_back(callback);
    }

    pub fn tcp_listen(&mut self, port: u16) -> io::Result<AsyncTcpServer> {
        let addr = format!("0.0.0.0:{}", port)
            .parse()
            .map_err(|_e| io::Error::from(io::ErrorKind::InvalidInput))?;

        let s = wasi_sock::Socket::new(
            wasi_sock::AddressFamily::Inet4,
            wasi_sock::SocketType::Stream,
        )?;
        s.set_nonblocking(true)?;
        s.bind(&addr)?;
        s.listen(1024)?;
        Ok(AsyncTcpServer(s))
    }

    pub fn tcp_connect(
        &mut self,
        addr: &SocketAddr,
        callback: Box<dyn FnOnce(&mut qjs::Context, PollResult)>,
        timeout: Option<std::time::Duration>,
    ) -> io::Result<()> {
        let s = wasi_sock::Socket::new(
            wasi_sock::AddressFamily::Inet4,
            wasi_sock::SocketType::Stream,
        )?;
        s.set_nonblocking(true)?;
        if let Err(e) = s.connect(addr) {
            // Operation in progress
            if e.raw_os_error() != Some(26) {
                return Err(e);
            }
        }

        if let Some(timeout) = timeout {
            let ddl = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .add(timeout)
                .as_nanos();

            self.io_selector
                .add_task(PollTask::SocketTimeout(SocketTimeoutTask {
                    s: s.0,
                    event: NetPollEvent::Connect,
                    timeout: ddl,
                    callback,
                }));
        } else {
            self.io_selector.add_task(PollTask::Socket(SocketTask {
                s: s.0,
                event: NetPollEvent::Connect,
                callback,
            }));
        }
        std::mem::forget(s);
        Ok(())
    }

    pub fn fd_read(
        &mut self,
        fd: std::os::wasi::io::RawFd,
        len: u64,
        callback: Box<dyn FnOnce(&mut qjs::Context, PollResult)>,
    ) {
        self.io_selector
            .add_task(PollTask::FdRead(FdReadTask { fd, len, callback }));
    }

    pub fn fd_write(
        &mut self,
        fd: std::os::wasi::io::RawFd,
        buf: Vec<u8>,
        callback: Box<dyn FnOnce(&mut qjs::Context, PollResult)>,
    ) {
        self.io_selector
            .add_task(PollTask::FdWrite(FdWriteTask { fd, buf, callback }));
    }
}
