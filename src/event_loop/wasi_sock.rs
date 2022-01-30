use std::ffi::CString;
use std::io;
use std::io::{Read, Write};
use std::net::{
    IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs,
};
use std::os::wasi::prelude::{AsRawFd, RawFd};

pub mod socket_types {
    pub const AF_INET4: u8 = 0;
    pub const AF_INET6: u8 = 1;

    pub const IPPROTO_TCP: i32 = 0;
    pub const IPPROTO_UDP: i32 = 1;

    pub const SHUT_RD: i32 = 1;
    pub const SHUT_WR: i32 = 2;
    pub const SHUT_RDWR: i32 = 3;

    pub const SOCK_DGRAM: i32 = 0;
    pub const SOCK_STREAM: i32 = 1;

    pub const SOL_SOCKET: i32 = 0;

    pub const SO_TYPE: i32 = 1;
    pub const SO_ERROR: i32 = 2;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WasiAddress {
    pub buf: *const u8,
    pub size: usize,
}

#[repr(C)]
pub struct IovecRead {
    pub buf: *mut u8,
    pub size: usize,
}

#[repr(C)]
pub struct IovecWrite {
    pub buf: *const u8,
    pub size: usize,
}

pub type RawSocket = RawFd;

#[derive(Debug)]
pub struct Socket(pub RawSocket);

impl Drop for Socket {
    fn drop(&mut self) {
        self.shutdown(Shutdown::Both);
    }
}

impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

macro_rules! syscall {
        ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
            #[allow(unused_unsafe)]
            let res = unsafe { libc::$fn($($arg, )*) };
            if res == -1 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(res)
            }
        }};
    }

fn fcntl_add(fd: RawFd, get_cmd: i32, set_cmd: i32, flag: i32) -> io::Result<()> {
    let previous = syscall!(fcntl(fd, get_cmd))?;
    let new = previous | flag;
    if new != previous {
        syscall!(fcntl(fd, set_cmd, new)).map(|_| ())
    } else {
        // Flag was already set.
        Ok(())
    }
}

/// Remove `flag` to the current set flags of `F_GETFD`.
fn fcntl_remove(fd: RawFd, get_cmd: i32, set_cmd: i32, flag: i32) -> io::Result<()> {
    let previous = syscall!(fcntl(fd, get_cmd))?;
    let new = previous & !flag;
    if new != previous {
        syscall!(fcntl(fd, set_cmd, new)).map(|_| ())
    } else {
        // Flag was already set.
        Ok(())
    }
}

impl Socket {
    pub fn new(_addr_family: i32, sock_kind: i32) -> io::Result<Self> {
        unsafe {
            if sock_kind != socket_types::SOCK_STREAM {
                Err(io::Error::from(io::ErrorKind::Unsupported))?;
            }
            let mut fd = 0;
            let res = sock_open(socket_types::AF_INET4 as u8, sock_kind as u8, &mut fd);
            if res == 0 {
                Ok(Socket(fd as i32))
            } else {
                Err(io::Error::from_raw_os_error(res as i32))
            }
        }
    }

    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            let mut send_len: u32 = 0;
            let vec = IovecWrite {
                buf: buf.as_ptr(),
                size: buf.len(),
            };
            let res = sock_send(self.as_raw_fd() as u32, &vec, 1, 0, &mut send_len);
            if res == 0 {
                Ok(send_len as usize)
            } else {
                Err(io::Error::from_raw_os_error(res as i32))
            }
        }
    }

    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        let flags = 0;
        let mut recv_len: usize = 0;
        let mut oflags: usize = 0;
        let mut vec = IovecRead {
            buf: buf.as_mut_ptr(),
            size: buf.len(),
        };

        unsafe {
            let res = sock_recv(
                self.as_raw_fd() as u32,
                &mut vec,
                1,
                flags,
                &mut recv_len,
                &mut oflags,
            );
            if res == 0 {
                Ok(recv_len)
            } else {
                Err(io::Error::from_raw_os_error(res as i32))
            }
        }
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        let fd = self.as_raw_fd();
        if nonblocking {
            fcntl_add(fd, libc::F_GETFL, libc::F_SETFL, libc::O_NONBLOCK)
        } else {
            fcntl_remove(fd, libc::F_GETFL, libc::F_SETFL, libc::O_NONBLOCK)
        }
    }

    pub fn connect(&self, addrs: &SocketAddr) -> io::Result<()> {
        let fd = self.as_raw_fd();
        let mut vaddr: [u8; 4] = [0; 4];
        let mut port: u16 = 0;
        if let SocketAddr::V4(addrs) = addrs {
            vaddr = addrs.ip().octets();
            port = addrs.port();
        }
        let mut addr = WasiAddress {
            buf: vaddr.as_ptr(),
            size: 4,
        };

        unsafe {
            let res = sock_connect(fd as u32, &mut addr, port as u32);
            if res != 0 {
                Err(io::Error::from_raw_os_error(res as i32))
            } else {
                Ok(())
            }
        }
    }

    pub fn bind(&self, addrs: &SocketAddr) -> io::Result<()> {
        unsafe {
            let fd = self.as_raw_fd();
            let mut vaddr: [u8; 16] = [0; 16];
            let port;
            let size;
            match addrs {
                SocketAddr::V4(addr) => {
                    let ip = addr.ip().octets();
                    (&mut vaddr[0..4]).clone_from_slice(&ip);
                    port = addr.port();
                    size = 4;
                }
                SocketAddr::V6(addr) => {
                    let ip = addr.ip().octets();
                    vaddr.clone_from_slice(&ip);
                    port = addr.port();
                    size = 16;
                }
            }
            let mut addr = WasiAddress {
                buf: vaddr.as_ptr(),
                size,
            };
            let res = sock_bind(fd as u32, &mut addr, port as u32);
            if res != 0 {
                Err(io::Error::from_raw_os_error(res as i32))
            } else {
                Ok(())
            }
        }
    }

    pub fn listen(&self, backlog: i32) -> io::Result<()> {
        unsafe {
            let fd = self.as_raw_fd();
            let res = sock_listen(fd as u32, backlog as u32);
            if res != 0 {
                Err(io::Error::from_raw_os_error(res as i32))
            } else {
                Ok(())
            }
        }
    }

    pub fn accept(&self, nonblocking: bool) -> io::Result<Self> {
        unsafe {
            let mut fd: u32 = 0;
            let res = sock_accept(self.as_raw_fd() as u32, &mut fd);
            if res != 0 {
                Err(io::Error::from_raw_os_error(res as i32))
            } else {
                let s = Socket(fd as i32);
                s.set_nonblocking(nonblocking)?;
                Ok(s)
            }
        }
    }

    pub fn get_local(&self) -> io::Result<SocketAddr> {
        unsafe {
            let fd = self.0;
            let addr_buf = [0u8; 16];
            let mut addr = WasiAddress {
                buf: addr_buf.as_ptr(),
                size: 16,
            };
            let mut addr_type = 0;
            let mut port = 0;
            let res = sock_getlocaladdr(fd as u32, &mut addr, &mut addr_type, &mut port);
            if res != 0 {
                Err(io::Error::from_raw_os_error(res as i32))
            } else {
                if addr_type == 4 {
                    let ip_addr = Ipv4Addr::new(addr_buf[0], addr_buf[1], addr_buf[2], addr_buf[3]);
                    Ok(SocketAddr::V4(SocketAddrV4::new(ip_addr, port as u16)))
                } else if addr_type == 6 {
                    let ip_addr = Ipv6Addr::from(addr_buf);
                    Ok(SocketAddr::V6(SocketAddrV6::new(
                        ip_addr,
                        port as u16,
                        0,
                        0,
                    )))
                } else {
                    Err(io::Error::from(io::ErrorKind::Unsupported))
                }
            }
        }
    }

    pub fn get_peer(&self) -> io::Result<SocketAddr> {
        unsafe {
            let fd = self.0;
            let addr_buf = [0u8; 16];
            let mut addr = WasiAddress {
                buf: addr_buf.as_ptr(),
                size: 16,
            };
            let mut addr_type = 0;
            let mut port = 0;
            let res = sock_getpeeraddr(fd as u32, &mut addr, &mut addr_type, &mut port);
            if res != 0 {
                Err(io::Error::from_raw_os_error(res as i32))
            } else {
                if addr_type == 4 {
                    let ip_addr = Ipv4Addr::new(addr_buf[0], addr_buf[1], addr_buf[2], addr_buf[3]);
                    Ok(SocketAddr::V4(SocketAddrV4::new(ip_addr, port as u16)))
                } else if addr_type == 6 {
                    let ip_addr = Ipv6Addr::from(addr_buf);
                    Ok(SocketAddr::V6(SocketAddrV6::new(
                        ip_addr,
                        port as u16,
                        0,
                        0,
                    )))
                } else {
                    Err(io::Error::from(io::ErrorKind::Unsupported))
                }
            }
        }
    }

    pub fn take_error(&self) -> io::Result<()> {
        use socket_types as s;
        unsafe {
            let fd = self.0;
            let mut error = 0;
            let mut len = std::mem::size_of::<i32>() as u32;
            let res = sock_getsockopt(fd as u32, s::SOL_SOCKET, s::SO_ERROR, &mut error, &mut len);
            if res == 0 && error == 0 {
                Ok(())
            } else if res == 0 && error != 0 {
                Err(io::Error::from_raw_os_error(error))
            } else {
                Err(io::Error::from_raw_os_error(res as i32))
            }
        }
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        use socket_types as s;
        unsafe {
            let flags = match how {
                Shutdown::Read => s::SHUT_RD,
                Shutdown::Write => s::SHUT_WR,
                Shutdown::Both => s::SHUT_RDWR,
            };
            let res = sock_shutdown(self.as_raw_fd() as u32, flags as u8);
            if res == 0 {
                Ok(())
            } else {
                Err(io::Error::from_raw_os_error(res as i32))
            }
        }
    }
}

impl<'a> Read for &'a Socket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.recv(buf)
    }
}

impl<'a> Write for &'a Socket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.send(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern "C" {
    pub fn sock_open(addr_family: u8, sock_type: u8, fd: *mut u32) -> u32;
    pub fn sock_bind(fd: u32, addr: *mut WasiAddress, port: u32) -> u32;
    pub fn sock_listen(fd: u32, backlog: u32) -> u32;
    pub fn sock_accept(fd: u32, new_fd: *mut u32) -> u32;
    pub fn sock_connect(fd: u32, addr: *mut WasiAddress, port: u32) -> u32;
    pub fn sock_recv(
        fd: u32,
        buf: *const IovecRead,
        buf_len: usize,
        flags: u16,
        recv_len: *mut usize,
        oflags: *mut usize,
    ) -> u32;
    pub fn sock_send(
        fd: u32,
        buf: *const IovecWrite,
        buf_len: u32,
        flags: u16,
        send_len: *mut u32,
    ) -> u32;
    pub fn sock_shutdown(fd: u32, flags: u8) -> u32;
    pub fn sock_getsockopt(
        fd: u32,
        level: i32,
        name: i32,
        flag: *mut i32,
        flag_size: *mut u32,
    ) -> u32;
    pub fn sock_setsockopt(fd: u32, level: i32, name: i32, flag: *const i32, flag_size: u32)
        -> u32;
    pub fn sock_getlocaladdr(
        fd: u32,
        addr: *mut WasiAddress,
        addr_type: *mut u32,
        port: *mut u32,
    ) -> u32;
    pub fn sock_getpeeraddr(
        fd: u32,
        addr: *mut WasiAddress,
        addr_type: *mut u32,
        port: *mut u32,
    ) -> u32;
}
