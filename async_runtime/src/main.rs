#[macro_use]
extern crate log;
use anyhow::Result;
use std::collections::LinkedList;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net;
use tokio::sync::mpsc;

use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::WasiCtx;

#[derive(Clone)]
enum NetSocket {
    TcpListener(Arc<net::TcpListener>),
    TcpStream(tokio::sync::mpsc::Sender<NetIoTask>),
    None,
}

#[derive(Debug)]
enum NetIoTask {
    Read {
        data_ptr: usize,
        len: usize,
        callback: usize,
    },
    Write {
        data: Vec<u8>,
        callback: usize,
    },
}

impl NetSocket {
    fn is_closed(&self) -> bool {
        match self {
            NetSocket::None => true,
            _ => false,
        }
    }

    fn close(&mut self) {
        *self = NetSocket::None
    }
}

#[derive(Debug)]
enum NetTask {
    Connect(String, usize),
    Listen(u32, usize),
    Accept(usize, usize),
    Close(usize),
    Read {
        s: usize,
        data_ptr: usize,
        len: usize,
        callback: usize,
    },
    Write {
        s: usize,
        data_ptr: usize,
        len: usize,
        callback: usize,
    },
}

enum NetTaskResult {
    Connect(std::io::Result<net::TcpStream>, usize),
    Listen(std::io::Result<net::TcpListener>, usize),
    Accept(std::io::Result<net::TcpStream>, usize),
    Close(usize),
    Read {
        data: std::io::Result<Vec<u8>>,
        data_ptr: usize,
        callback: usize,
    },
    Write {
        len: std::io::Result<usize>,
        callback: usize,
    },
}

struct StoreData {
    ctx: WasiCtx,
    tasks: LinkedList<NetTask>,
}

fn io_error_to_code(kind: std::io::ErrorKind) -> usize {
    use std::io::ErrorKind::*;
    match kind {
        NotFound => 1,
        ConnectionRefused => 2,
        ConnectionReset => 3,
        ConnectionAborted => 4,
        NotConnected => 5,
        AddrInUse => 6,
        AddrNotAvailable => 7,
        BrokenPipe => 8,
        WouldBlock => 9,
        InvalidInput => 10,
        InvalidData => 11,
        TimedOut => 12,
        WriteZero => 13,
        Interrupted => 14,
        Other => 15,
        UnexpectedEof => 16,
        _ => 15,
    }
}
fn find_empty_v_sockets(v_sockets: &mut Vec<NetSocket>) -> usize {
    let len = v_sockets.len();
    for (i, s) in v_sockets.iter_mut().enumerate() {
        if s.is_closed() {
            return i;
        }
    }
    v_sockets.extend(vec![NetSocket::None; 64]);
    len
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let engine = Engine::default();
    let mut linker = Linker::<StoreData>::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.ctx)?;

    {
        let _ = linker.func_wrap(
            "wasmedge_async_sock",
            "async_sock_tcp_connect",
            |mut caller: Caller<'_, StoreData>, host_ptr: i32, host_len: i32, callback: i32| {
                if let Some(Extern::Memory(memory)) = caller.get_export("memory") {
                    let mut host = vec![0; host_len as usize];
                    let _ = memory.read(&mut caller, host_ptr as usize, &mut host);
                    let host_str = String::from_utf8(host).expect("");
                    trace!("async_sock_tcp_connect: host={}", host_str);
                    let store = caller.data_mut();
                    store
                        .tasks
                        .push_back(NetTask::Connect(host_str, callback as usize));
                } else {
                    panic!("no found memory");
                }
            },
        );

        let _ = linker.func_wrap(
            "wasmedge_async_sock",
            "async_sock_tcp_listen",
            |mut caller: Caller<'_, StoreData>, port: i32, callback: i32| {
                trace!("async_sock_tcp_listen: port={}", port);
                let store = caller.data_mut();
                store
                    .tasks
                    .push_back(NetTask::Listen(port as u32, callback as usize));
            },
        );

        let _ = linker.func_wrap(
            "wasmedge_async_sock",
            "async_sock_accept",
            |mut caller: Caller<'_, StoreData>, sock: i32, callback: i32| {
                trace!("async_sock_accept: sock={}", sock);

                let store = caller.data_mut();
                store
                    .tasks
                    .push_back(NetTask::Accept(sock as usize, callback as usize));
            },
        );

        let _ = linker.func_wrap(
            "wasmedge_async_sock",
            "async_sock_close",
            |mut caller: Caller<'_, StoreData>, sock: i32| {
                trace!("async_sock_close: sock={}", sock);

                let store = caller.data_mut();
                store.tasks.push_back(NetTask::Close(sock as usize));
            },
        );

        let _ = linker.func_wrap(
            "wasmedge_async_sock",
            "async_sock_read",
            |mut caller: Caller<'_, StoreData>,
             sock: i32,
             data_ptr: i32,
             len: i32,
             callback: i32| {
                trace!(
                    "async_sock_read: sock={} data_ptr={} len={}",
                    sock,
                    data_ptr,
                    len
                );

                let store = caller.data_mut();
                store.tasks.push_back(NetTask::Read {
                    s: sock as usize,
                    data_ptr: data_ptr as usize,
                    len: len as usize,
                    callback: callback as usize,
                });
            },
        );

        let _ = linker.func_wrap(
            "wasmedge_async_sock",
            "async_sock_write",
            |mut caller: Caller<'_, StoreData>,
             sock: i32,
             data_ptr: i32,
             len: i32,
             callback: i32| {
                trace!(
                    "async_sock_write: sock={} data_ptr={} len={}",
                    sock,
                    data_ptr,
                    len
                );

                let store = caller.data_mut();
                store.tasks.push_back(NetTask::Write {
                    s: sock as usize,
                    data_ptr: data_ptr as usize,
                    len: len as usize,
                    callback: callback as usize,
                });
            },
        );
    }

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(
        &engine,
        StoreData {
            ctx: wasi,
            tasks: Default::default(),
        },
    );

    let mut v_socks = vec![];

    // Instantiate our module with the imports we've created, and run it.
    let file_name = std::env::args().nth(1).unwrap_or("wasi.wasm".to_string());
    let module = Module::from_file(&engine, file_name.as_str())?;

    let inst = linker.instantiate(&mut store, &module)?;

    let async_main = inst
        .get_func(&mut store, "async_main")
        .ok_or(anyhow::anyhow!("'async_main' not found"))?;

    let wasmedge_async_new_socket_trampoline = inst
        .get_func(&mut store, "wasmedge_async_new_socket_trampoline")
        .ok_or(anyhow::anyhow!(
            "'wasmedge_async_new_socket_trampoline' not found"
        ))?;

    let wasmedge_async_read_callback_trampoline = inst
        .get_func(&mut store, "wasmedge_async_read_callback_trampoline")
        .ok_or(anyhow::anyhow!(
            "'wasmedge_async_read_callback_trampoline' not found"
        ))?;

    let wasmedge_async_write_callback_trampoline = inst
        .get_func(&mut store, "wasmedge_async_write_callback_trampoline")
        .ok_or(anyhow::anyhow!(
            "'wasmedge_async_write_callback_trampoline' not found"
        ))?;

    let _ = async_main.call(&mut store, &[]);

    let (wx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let mut task_total = 0;
    'start_poll: loop {
        while let Some(task) = store.data_mut().tasks.pop_front() {
            trace!("start poll {:?}", task);
            task_total += 1;
            match task {
                NetTask::Connect(host, callback) => {
                    let wx = wx.clone();
                    tokio::spawn(async move {
                        let s = net::TcpStream::connect(host.as_str()).await;
                        trace!("connect {}:{:?}", host, s);
                        let _ = wx.send(NetTaskResult::Connect(s, callback));
                    });
                }
                NetTask::Listen(port, callback) => {
                    let wx = wx.clone();
                    tokio::spawn(async move {
                        let s = net::TcpListener::bind(format!("0.0.0.0:{}", port)).await;
                        let _ = wx.send(NetTaskResult::Listen(s, callback));
                    });
                }
                NetTask::Accept(s, callback) => {
                    let wx = wx.clone();
                    if let Some(NetSocket::TcpListener(s)) = v_socks.get(s).cloned() {
                        tokio::spawn(async move {
                            let s = s.accept().await;
                            let _ = match s {
                                Ok((s, _)) => wx.send(NetTaskResult::Accept(Ok(s), callback)),
                                Err(e) => wx.send(NetTaskResult::Accept(Err(e), callback)),
                            };
                        });
                    }
                }
                NetTask::Close(s) => {
                    if let Some(s) = v_socks.get_mut(s) {
                        if let NetSocket::TcpListener(_) = s {
                            task_total -= 1;
                        }
                        NetSocket::close(s);
                    }
                }
                NetTask::Read {
                    s,
                    data_ptr,
                    len,
                    callback,
                } => {
                    if let Some(NetSocket::TcpStream(sock)) = v_socks.get(s) {
                        let _ = sock
                            .send(NetIoTask::Read {
                                data_ptr,
                                len,
                                callback,
                            })
                            .await;
                    }
                }
                NetTask::Write {
                    s,
                    data_ptr,
                    len,
                    callback,
                } => {
                    if let Some(Extern::Memory(memory)) = inst.get_export(&mut store, "memory") {
                        let mut buff = vec![0u8; len];
                        if let Ok(_) = memory.read(&mut store, data_ptr, buff.as_mut_slice()) {
                            if let Some(NetSocket::TcpStream(sock)) = v_socks.get(s).cloned() {
                                let _ = sock
                                    .send(NetIoTask::Write {
                                        data: buff,
                                        callback,
                                    })
                                    .await;
                            }
                        } else {
                            let _ = wx.send(NetTaskResult::Write {
                                len: Err(std::io::Error::from(std::io::ErrorKind::WriteZero)),
                                callback,
                            });
                        }
                    } else {
                        panic!("not found wasm memory")
                    }
                }
            }
        }
        trace!("handle task result {}", task_total);
        while let Some(event_result) = {
            if task_total <= 0 {
                None
            } else {
                rx.recv().await
            }
        } {
            task_total -= 1;

            match event_result {
                NetTaskResult::Connect(s, callback) => {
                    let mut slot_index = 0;

                    let error_code = match s {
                        Ok(s) => {
                            let (io_wx, io_rx) = tokio::sync::mpsc::channel(128);
                            slot_index = find_empty_v_sockets(&mut v_socks);
                            tokio::spawn(spilt_tcp_stream(wx.clone(), io_rx, s, slot_index));
                            v_socks.insert(slot_index, NetSocket::TcpStream(io_wx));
                            0
                        }
                        Err(e) => io_error_to_code(e.kind()),
                    };
                    let _ = wasmedge_async_new_socket_trampoline.call(
                        &mut store,
                        &[
                            Val::I32(slot_index as i32),
                            Val::I32(error_code as i32),
                            Val::I32(callback as i32),
                        ],
                    );
                }
                NetTaskResult::Listen(s, callback) => {
                    let mut slot_index = 0;

                    let error_code = match s {
                        Ok(s) => {
                            slot_index = find_empty_v_sockets(&mut v_socks);
                            v_socks.insert(slot_index, NetSocket::TcpListener(Arc::new(s)));
                            0
                        }
                        Err(e) => io_error_to_code(e.kind()),
                    };
                    let _ = wasmedge_async_new_socket_trampoline.call(
                        &mut store,
                        &[
                            Val::I32(slot_index as i32),
                            Val::I32(error_code as i32),
                            Val::I32(callback as i32),
                        ],
                    );
                }
                NetTaskResult::Accept(s, callback) => {
                    let mut slot_index = 0;
                    let error_code = match s {
                        Ok(s) => {
                            let (io_wx, io_rx) = tokio::sync::mpsc::channel(128);
                            slot_index = find_empty_v_sockets(&mut v_socks);
                            tokio::spawn(spilt_tcp_stream(wx.clone(), io_rx, s, slot_index));
                            v_socks.insert(slot_index, NetSocket::TcpStream(io_wx));
                            0
                        }
                        Err(e) => io_error_to_code(e.kind()),
                    };
                    let _ = wasmedge_async_new_socket_trampoline.call(
                        &mut store,
                        &[
                            Val::I32(slot_index as i32),
                            Val::I32(error_code as i32),
                            Val::I32(callback as i32),
                        ],
                    );
                }
                NetTaskResult::Close(s) => {
                    trace!("close {}", s);
                }
                NetTaskResult::Read {
                    data,
                    data_ptr,
                    callback,
                } => {
                    let mut len = 0;
                    let error_code = match data {
                        Ok(data) => {
                            if let Some(Extern::Memory(memory)) =
                                inst.get_export(&mut store, "memory")
                            {
                                if let Ok(_) = memory.write(&mut store, data_ptr, data.as_slice()) {
                                    len = data.len();
                                    0
                                } else {
                                    io_error_to_code(std::io::ErrorKind::Other)
                                }
                            } else {
                                panic!("not found wasm memory")
                            }
                        }
                        Err(e) => io_error_to_code(e.kind()),
                    };
                    let _ = wasmedge_async_read_callback_trampoline.call(
                        &mut store,
                        &[
                            Val::I32(len as i32),
                            Val::I32(error_code as i32),
                            Val::I32(callback as i32),
                        ],
                    );
                }
                NetTaskResult::Write { len, callback } => {
                    let mut write_total = 0;
                    let error_code = match len {
                        Ok(len) => {
                            write_total = len;
                            0
                        }
                        Err(e) => io_error_to_code(e.kind()),
                    };
                    let _ = wasmedge_async_write_callback_trampoline.call(
                        &mut store,
                        &[
                            Val::I32(write_total as i32),
                            Val::I32(error_code as i32),
                            Val::I32(callback as i32),
                        ],
                    );
                }
            }
            continue 'start_poll;
        }
        trace!("exit poll");
        break Ok(());
    }
}

async fn spilt_tcp_stream(
    root_wx: mpsc::UnboundedSender<NetTaskResult>,
    mut task_rx: mpsc::Receiver<NetIoTask>,
    tcp_stream: net::TcpStream,
    v_fd: usize,
) {
    let (r, w) = tcp_stream.into_split();
    let (r_sender, r_receiver) = mpsc::channel(1);
    tokio::spawn(handler_tcp_reader(root_wx.clone(), r_receiver, r, v_fd));
    let (w_sender, w_receiver) = mpsc::channel(1);
    tokio::spawn(handler_tcp_writer(root_wx.clone(), w_receiver, w, v_fd));
    trace!("start spilt_tcp_stream({}) loop", v_fd);

    while let Some(task) = task_rx.recv().await {
        trace!("spilt_tcp_stream({}) recv_task {:?}", v_fd, task);

        match task {
            NetIoTask::Read {
                data_ptr,
                len,
                callback,
            } => {
                let _ = r_sender
                    .send(NetIoTask::Read {
                        data_ptr,
                        len,
                        callback,
                    })
                    .await;
            }
            NetIoTask::Write { data, callback } => {
                let _ = w_sender.send(NetIoTask::Write { data, callback }).await;
            }
        }
    }
    trace!("spilt_tcp_stream({}) close", v_fd);
    let _ = root_wx.send(NetTaskResult::Close(v_fd));
}

async fn handler_tcp_reader(
    root_wx: mpsc::UnboundedSender<NetTaskResult>,
    mut task_rx: mpsc::Receiver<NetIoTask>,
    mut reader: net::tcp::OwnedReadHalf,
    v_fd: usize,
) {
    trace!("handler_tcp_reader({}) start", v_fd);
    while let Some(task) = task_rx.recv().await {
        if let NetIoTask::Read {
            data_ptr,
            len,
            callback,
        } = task
        {
            let mut buff = vec![0u8; len];
            let data = match reader.read(buff.as_mut_slice()).await {
                Ok(len) => {
                    buff.truncate(len);
                    trace!("handler_tcp_reader({}) Ok({:?})", v_fd, len);
                    Ok(buff)
                }
                Err(e) => {
                    trace!("handler_tcp_reader({}) Err({:?})", v_fd, e);
                    Err(e)
                }
            };

            let _ = root_wx.send(NetTaskResult::Read {
                data,
                data_ptr,
                callback,
            });
        }
    }
    trace!("handler_tcp_reader({}) end", v_fd);
}

async fn handler_tcp_writer(
    root_wx: mpsc::UnboundedSender<NetTaskResult>,
    mut task_rx: mpsc::Receiver<NetIoTask>,
    mut writer: net::tcp::OwnedWriteHalf,
    v_fd: usize,
) {
    trace!("handler_tcp_writer({}) start", v_fd);
    while let Some(task) = task_rx.recv().await {
        if let NetIoTask::Write { data, callback } = task {
            let len = writer.write(data.as_slice()).await;
            trace!("handler_tcp_writer({}) {:?}", v_fd, len);
            let _ = root_wx.send(NetTaskResult::Write { len, callback });
        }
    }
    trace!("handler_tcp_writer({}) end", v_fd);
}
