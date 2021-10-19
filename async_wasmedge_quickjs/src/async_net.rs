use crate::wasmedge_async_sock as sock;
use std::os::raw::c_void;
use wasmedge_quickjs::*;

pub fn init() {
    let io_poll = sock::IOPollFn {
        new_socket_trampoline,
        read_callback_trampoline,
        write_callback_trampoline,
    };
    sock::init(io_poll)
}

struct AsyncSockTask {
    resolving_fn: JsFunction,
    reject_fn: JsFunction,
}

struct AsyncReadTask {
    sock: i32,
    buf: Vec<u8>,
    offset: usize,
    resolving_fn: JsFunction,
    reject_fn: JsFunction,
}
impl AsyncReadTask {
    const BUFF_LEN: usize = 1024 * 2;
}

struct AsyncWriteTask {
    data: Vec<u8>,
    resolving_fn: JsFunction,
    reject_fn: JsFunction,
}

fn new_socket_trampoline(callback: *mut c_void, sock: sock::IOResult<i32>) {
    let task = unsafe { Box::from_raw(callback.cast::<AsyncSockTask>()) };
    let mut ctx = super::GLOBAL_CTX.0.borrow_mut();

    match sock {
        Ok(s) => {
            let mut argv = [s.into()];
            task.resolving_fn.call(&mut argv);
        }
        Err(e) => {
            let err_msg = format!("{:?}", e);
            let mut argv = [ctx.new_error(err_msg.as_str())];
            task.reject_fn.call(&mut argv);
        }
    }
    ctx.promise_loop_poll();
}
fn read_callback_trampoline(callback: *mut c_void, res: sock::IOResult<usize>) {
    let mut task = unsafe { Box::from_raw(callback.cast::<AsyncReadTask>()) };
    let mut ctx = super::GLOBAL_CTX.0.borrow_mut();

    match res {
        Ok(l) => {
            if l < AsyncReadTask::BUFF_LEN {
                task.buf.truncate(l);
                let buf = ctx.new_array_buffer(task.buf.as_slice());
                task.resolving_fn.call(&mut [buf.into()]);
            } else {
                let offset = task.offset + l;
                task.offset = offset;
                task.buf.resize(offset + AsyncReadTask::BUFF_LEN, 0);

                let task_ptr = Box::leak(task);

                unsafe {
                    sock::async_sock_read(
                        task_ptr.sock,
                        task_ptr.buf.as_mut_ptr().offset(offset as isize),
                        AsyncReadTask::BUFF_LEN,
                        (task_ptr as *mut AsyncReadTask).cast(),
                    )
                }
            }
        }
        Err(e) => {
            let err_msg = format!("{:?}", e);
            let mut argv = [ctx.new_error(err_msg.as_str())];
            task.reject_fn.call(&mut argv);
        }
    }
    ctx.promise_loop_poll();
}
fn write_callback_trampoline(callback: *mut c_void, res: sock::IOResult<usize>) {
    let task = unsafe { Box::from_raw(callback.cast::<AsyncWriteTask>()) };
    let mut ctx = super::GLOBAL_CTX.0.borrow_mut();

    match res {
        Ok(l) => {
            let mut argv = [(l as i32).into()];
            task.resolving_fn.call(&mut argv);
        }
        Err(e) => {
            let err_msg = format!("{:?}", e);
            let mut argv = [ctx.new_error(err_msg.as_str())];
            task.reject_fn.call(&mut argv);
        }
    }
    ctx.promise_loop_poll();
}

struct AsyncTcpConnect;
impl JsFn for AsyncTcpConnect {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let host = if let Some(JsValue::String(host)) = argv.get(0) {
            host.to_string()
        } else {
            return ctx.throw_type_error("'host' is not string").into();
        };

        let (p, r, e) = ctx.new_promise();
        if let (JsValue::Function(resolving_fn), JsValue::Function(reject_fn)) = (r, e) {
            let task = Box::new(AsyncSockTask {
                resolving_fn,
                reject_fn,
            });
            let task_ptr = Box::leak(task) as *mut AsyncSockTask;

            unsafe { sock::async_sock_tcp_connect(host.as_ptr(), host.len(), task_ptr.cast()) }
            p
        } else {
            ctx.throw_internal_type_error("can't new promise").into()
        }
    }
}

struct AsyncTcpListen;
impl JsFn for AsyncTcpListen {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let port = if let Some(JsValue::Int(port)) = argv.get(0) {
            (*port) as u32
        } else {
            return ctx.throw_type_error("'port' is not number").into();
        };

        let (p, r, e) = ctx.new_promise();
        if let (JsValue::Function(resolving_fn), JsValue::Function(reject_fn)) = (r, e) {
            let task = Box::new(AsyncSockTask {
                resolving_fn,
                reject_fn,
            });
            let task_ptr = Box::leak(task) as *mut AsyncSockTask;

            unsafe { sock::async_sock_tcp_listen(port, task_ptr.cast()) }
            p
        } else {
            ctx.throw_internal_type_error("can't new promise").into()
        }
    }
}

struct AsyncAcceptConnect;
impl JsFn for AsyncAcceptConnect {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let sock = if let Some(JsValue::Int(sock)) = argv.get(0) {
            *sock
        } else {
            return ctx.throw_type_error("'sock' is illegal").into();
        };

        let (p, r, e) = ctx.new_promise();
        if let (JsValue::Function(resolving_fn), JsValue::Function(reject_fn)) = (r, e) {
            let task = Box::new(AsyncSockTask {
                resolving_fn,
                reject_fn,
            });
            let task_ptr = Box::leak(task) as *mut AsyncSockTask;

            unsafe { sock::async_sock_accept(sock, task_ptr.cast()) }
            p
        } else {
            ctx.throw_internal_type_error("can't new promise").into()
        }
    }
}

struct AsyncCloseConnect;
impl JsFn for AsyncCloseConnect {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let sock = if let Some(JsValue::Int(sock)) = argv.get(0) {
            *sock
        } else {
            return ctx.throw_type_error("'sock' is illegal").into();
        };

        unsafe { sock::async_sock_close(sock) };
        JsValue::UnDefined
    }
}

struct AsyncRead;
impl JsFn for AsyncRead {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let sock = if let Some(JsValue::Int(sock)) = argv.get(0) {
            *sock
        } else {
            return ctx.throw_type_error("'sock' is illegal").into();
        };

        let (p, r, e) = ctx.new_promise();
        if let (JsValue::Function(resolving_fn), JsValue::Function(reject_fn)) = (r, e) {
            let buf = vec![0u8; AsyncReadTask::BUFF_LEN];
            let task = Box::new(AsyncReadTask {
                sock,
                buf,
                offset: 0,
                resolving_fn,
                reject_fn,
            });
            let task_ptr = Box::leak(task);

            unsafe {
                sock::async_sock_read(
                    sock,
                    task_ptr.buf.as_mut_ptr(),
                    AsyncReadTask::BUFF_LEN,
                    (task_ptr as *mut AsyncReadTask).cast(),
                )
            }
            p
        } else {
            ctx.throw_internal_type_error("can't new promise").into()
        }
    }
}

struct AsyncWrite;
impl JsFn for AsyncWrite {
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        let sock = if let Some(JsValue::Int(sock)) = argv.get(0) {
            *sock
        } else {
            return ctx.throw_type_error("'sock' is illegal").into();
        };

        let buf = match argv.get(1) {
            Some(JsValue::ArrayBuffer(buf)) => buf.to_vec(),
            Some(JsValue::String(s)) => Vec::from(s.to_string()),
            _ => {
                return ctx
                    .throw_type_error("'data' is not string or ArrayBuffer")
                    .into();
            }
        };
        let (p, r, e) = ctx.new_promise();
        if let (JsValue::Function(resolving_fn), JsValue::Function(reject_fn)) = (r, e) {
            let task = Box::new(AsyncWriteTask {
                data: buf,
                resolving_fn,
                reject_fn,
            });
            let task_ptr = Box::leak(task);

            unsafe {
                sock::async_sock_write(
                    sock,
                    task_ptr.data.as_ptr(),
                    task_ptr.data.len(),
                    (task_ptr as *mut AsyncWriteTask).cast(),
                )
            }
            p
        } else {
            ctx.throw_internal_type_error("can't new promise").into()
        }
    }
}

struct AsyncNetModule;
impl ModuleInit for AsyncNetModule {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let f = ctx.new_function::<AsyncTcpConnect>("tcp_connect");
        m.add_export("tcp_connect\0", f.into());
        let f = ctx.new_function::<AsyncTcpListen>("tcp_listen");
        m.add_export("tcp_listen\0", f.into());
        let f = ctx.new_function::<AsyncAcceptConnect>("accept");
        m.add_export("accept\0", f.into());
        let f = ctx.new_function::<AsyncCloseConnect>("close");
        m.add_export("close\0", f.into());
        let f = ctx.new_function::<AsyncRead>("read");
        m.add_export("read\0", f.into());
        let f = ctx.new_function::<AsyncWrite>("write");
        m.add_export("write\0", f.into());
    }
}
pub fn init_module(ctx: &mut Context) {
    init();
    ctx.register_module(
        "async_net\0",
        AsyncNetModule,
        &[
            "tcp_connect\0",
            "tcp_listen\0",
            "accept\0",
            "close\0",
            "read\0",
            "write\0",
        ],
    )
}
