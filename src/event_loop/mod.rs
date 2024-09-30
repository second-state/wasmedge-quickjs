mod certs;
mod poll;
pub mod wasi_fs;
mod wasi_sock;

use crate::{quickjs_sys as qjs, Context, JsClassTool, JsValue};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, LinkedList};
use std::convert::TryInto;
use std::io::{self, Read, Write};
use std::mem::ManuallyDrop;
use std::net::{SocketAddr, SocketAddrV4};
use std::ops::Add;
use std::os::fd::{AsRawFd, FromRawFd};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use rustls::OwnedTrustAnchor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
pub use wasi_sock::nslookup;

pub(crate) enum NetPollEvent {
    Accept,
    Read,
    Connect,
}

pub struct AsyncTcpServer(pub(crate) tokio::net::TcpListener);
impl AsyncTcpServer {
    pub fn bind(port: u16) -> io::Result<Self> {
        use std::os::fd::IntoRawFd;
        let listener = wasmedge_wasi_socket::TcpListener::bind(("0.0.0.0", port), true)?;
        let listener = unsafe { std::net::TcpListener::from_raw_fd(listener.into_raw_fd()) };
        let async_listener = tokio::net::TcpListener::from_std(listener)?;
        Ok(AsyncTcpServer(async_listener))
    }

    pub async fn accept(
        &mut self,
        ctx: &mut Context,
        timeout: Option<std::time::Duration>,
    ) -> Result<JsValue, JsValue> {
        if let Some(duration) = timeout {
            match tokio::time::timeout(duration, self.0.accept()).await {
                Ok(Ok((conn, addr))) => {
                    log::trace!("tcp accept a socket[{addr}]");
                    Ok(AsyncTcpConn::wrap_obj(ctx, AsyncTcpConn(conn)))
                }
                Ok(Err(e)) => {
                    log::trace!("tcp accept error: {e}");
                    Err(ctx.new_error(e.to_string().as_str()))
                }
                Err(e) => {
                    let err = std::io::Error::new(std::io::ErrorKind::TimedOut, e.to_string());
                    Err(ctx.new_error(err.to_string().as_str()).into())
                }
            }
        } else {
            match self.0.accept().await {
                Ok((conn, addr)) => {
                    log::trace!("tcp accept a socket[{addr}]");
                    Ok(AsyncTcpConn::wrap_obj(ctx, AsyncTcpConn(conn)))
                }
                Err(e) => {
                    log::trace!("tcp accept error: {e}");
                    Err(ctx.new_error(e.to_string().as_str()))
                }
            }
        }
    }
}

pub struct AsyncTcpConn(pub(crate) tokio::net::TcpStream);
impl AsyncTcpConn {
    pub async fn async_connect<R: tokio::net::ToSocketAddrs>(addr: R) -> io::Result<Self> {
        tokio::net::TcpStream::connect(addr)
            .await
            .map(|conn| AsyncTcpConn(conn))
    }

    pub async fn async_read_all(&mut self) -> io::Result<Vec<u8>> {
        let mut data = vec![];
        let mut buff = [0u8; 1024 * 4];

        log::trace!("tcp read_all");

        loop {
            match self.0.read(&mut buff).await {
                Ok(0) => {
                    log::trace!("tcp read: 0");
                    return Ok(data);
                }
                Ok(n) => {
                    log::trace!("tcp read: {n}");
                    data.extend_from_slice(&buff[0..n]);
                    if n < buff.len() {
                        return Ok(data);
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    log::trace!("tcp read: WouldBlock");
                    return Ok(data);
                }
                Err(e) => {
                    log::trace!("tcp read: {e}");
                    return Err(e);
                }
            }
        }
    }

    pub async fn async_write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.0.write_all(buf).await
    }

    pub fn local(&self) -> io::Result<SocketAddr> {
        self.0.local_addr()
    }

    pub fn peer(&self) -> io::Result<SocketAddr> {
        self.0.peer_addr()
    }
}

#[cfg(feature = "tls")]
pub struct AsyncTlsConn(pub(crate) tokio_rustls::client::TlsStream<tokio::net::TcpStream>);

#[cfg(feature = "tls")]
impl AsyncTlsConn {
    pub async fn async_connect<R: tokio::net::ToSocketAddrs, S: AsRef<str>>(
        addr: R,
        domain: S,
    ) -> io::Result<Self> {
        use rustls::ClientConfig;
        use tokio_rustls::client::TlsStream;

        let io = tokio::net::TcpStream::connect(addr).await?;
        let mut root_store = rustls::RootCertStore::empty();
        if let Ok(custom_certs) = certs::load_certs_from_env() {
            log::info!("using custom certs");
            for cert in custom_certs {
                root_store.add(&cert).unwrap();
            }
        } else {
            log::info!("falling back to webpki certs");
            root_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
                OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            }));
        }

        let config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let connector = tokio_rustls::TlsConnector::from(Arc::new(config));
        let domain: rustls::ServerName = domain
            .as_ref()
            .try_into()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let conn = connector.connect(domain, io).await?;

        Ok(Self(conn))
    }

    pub async fn async_read_all(&mut self) -> io::Result<Vec<u8>> {
        let mut data = vec![];
        let mut buff = [0u8; 1024 * 4];

        log::trace!("tls read_all");

        loop {
            match self.0.read(&mut buff).await {
                Ok(0) => {
                    log::trace!("tls read: 0");
                    return Ok(data);
                }
                Ok(n) => {
                    log::trace!("tls read: {n}");
                    data.extend_from_slice(&buff[0..n]);
                    if n < buff.len() {
                        return Ok(data);
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    log::trace!("tls read: WouldBlock");
                    return Ok(data);
                }
                Err(e) => {
                    log::trace!("tls read: {e}");
                    return Err(e);
                }
            }
        }
    }

    pub async fn async_write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.0.write_all(buf).await
    }

    pub fn local(&self) -> io::Result<SocketAddr> {
        self.0.get_ref().0.local_addr()
    }

    pub fn peer(&self) -> io::Result<SocketAddr> {
        self.0.get_ref().0.peer_addr()
    }
}

#[derive(Default)]
pub struct EventLoop {
    next_tick_queue: LinkedList<Box<dyn FnOnce()>>,
    immediate_queue: LinkedList<Box<dyn FnOnce()>>,
    pub(crate) waker: Option<std::task::Waker>,
    pub(crate) sub_tasks: LinkedList<tokio::task::JoinHandle<()>>,
}

impl EventLoop {
    pub fn add_immediate_task(&mut self, callback: Box<dyn FnOnce()>) {
        self.immediate_queue.push_back(callback);
    }

    pub fn run_tick_task(&mut self) -> usize {
        let mut i = 0;
        let mut cb_vec = LinkedList::new();
        while let Some(f) = self.next_tick_queue.pop_front() {
            cb_vec.push_back(f);
        }
        while let Some(f) = self.immediate_queue.pop_front() {
            cb_vec.push_back(f);
        }
        while let Some(f) = cb_vec.pop_front() {
            f();
            i += 1;
        }
        i
    }

    pub fn set_next_tick(&mut self, callback: Box<dyn FnOnce()>) {
        self.next_tick_queue.push_back(callback);
    }
}
