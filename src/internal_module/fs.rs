use crate::quickjs_sys::*;
use std::convert::TryInto;
use std::fs;
use std::fs::Permissions;
use std::io;
use std::os::wasi::fs::{FileTypeExt, MetadataExt};
use std::os::wasi::prelude::FromRawFd;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

fn to_msec(maybe_time: Result<SystemTime, io::Error>) -> JsValue {
    match maybe_time {
        Ok(time) => {
            let msec = time
                .duration_since(UNIX_EPOCH)
                .map(|t| t.as_millis())
                .unwrap_or_else(|err| err.duration().as_millis());
            JsValue::Float(msec as f64)
        }
        Err(_) => JsValue::Null,
    }
}

impl From<u64> for JsValue {
    fn from(val: u64) -> Self {
        JsValue::Float(val as f64)
    }
}

fn permissions_to_mode(permit: Permissions) -> i32 {
    const F_OK: i32 = 0;
    const R_OK: i32 = 4;
    const W_OK: i32 = 2;
    const X_OK: i32 = 1;
    let p = if permit.readonly() {
        F_OK | R_OK | X_OK
    } else {
        F_OK | R_OK | W_OK | X_OK
    };
    p | p << 3 | p << 6
}

fn stat_to_js_object(ctx: &mut Context, stat: fs::Metadata) -> JsValue {
    let mut res = ctx.new_object();
    res.set("is_file", stat.is_file().into());
    res.set("is_directory", stat.is_dir().into());
    res.set("is_symlink", stat.is_symlink().into());
    res.set("is_block_device", stat.file_type().is_block_device().into());
    res.set("is_char_device", stat.file_type().is_char_device().into());
    res.set("is_socket", stat.file_type().is_socket().into());
    res.set("size", stat.len().into());
    res.set("mtime", to_msec(stat.modified()));
    res.set("atime", to_msec(stat.accessed()));
    res.set("birthtime", to_msec(stat.created()));
    res.set("dev", stat.dev().into());
    res.set("ino", stat.ino().into());
    res.set("mode", permissions_to_mode(stat.permissions()).into());
    res.set("nlink", stat.nlink().into());
    res.set("uid", 0.into());
    res.set("gid", 0.into());
    res.set("rdev", 0.into());
    res.set("blksize", 0.into());
    res.set("blocks", 0.into());
    JsValue::Object(res)
}

fn err_to_js_object(ctx: &mut Context, e: io::Error) -> JsValue {
    let mut res = ctx.new_object();
    if let Some(code) = e.raw_os_error() {
        res.set("code", code.into());
    }
    res.set(
        "kind",
        JsValue::String(ctx.new_string(format!("{:?}", e.kind()).as_str())),
    );
    res.set(
        "message",
        JsValue::String(ctx.new_string(e.to_string().as_str())),
    );
    JsValue::Object(res)
}

fn errno_to_js_object(ctx: &mut Context, e: wasi::Errno) -> JsValue {
    let mut res = ctx.new_object();
    res.set("message", JsValue::String(ctx.new_string(e.message())));
    JsValue::Object(res)
}

fn stat_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let path = arg.get(0);
    if path.is_none() {
        return JsValue::UnDefined;
    }
    if let JsValue::String(s) = path.unwrap() {
        return match fs::metadata(s.as_str()) {
            Ok(stat) => stat_to_js_object(ctx, stat),
            Err(e) => {
                let err = err_to_js_object(ctx, e);
                JsValue::Exception(ctx.throw_error(err))
            }
        };
    } else {
        return JsValue::UnDefined;
    }
}

fn fstat_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let fd = arg.get(0);
    if fd.is_none() {
        return JsValue::UnDefined;
    }
    if let JsValue::Int(f) = fd.unwrap() {
        let f = unsafe { fs::File::from_raw_fd(*f) };
        return match f.metadata() {
            Ok(stat) => stat_to_js_object(ctx, stat),
            Err(e) => {
                let err = err_to_js_object(ctx, e);
                JsValue::Exception(ctx.throw_error(err))
            }
        };
    } else {
        return JsValue::UnDefined;
    }
}

fn lstat_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let path = arg.get(0);
    if path.is_none() {
        return JsValue::UnDefined;
    }
    if let JsValue::String(s) = path.unwrap() {
        return match fs::symlink_metadata(s.as_str()) {
            Ok(stat) => stat_to_js_object(ctx, stat),
            Err(e) => {
                let err = err_to_js_object(ctx, e);
                JsValue::Exception(ctx.throw_error(err))
            }
        };
    } else {
        return JsValue::UnDefined;
    }
}

fn mkdir_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let path = arg.get(0);
    let recursive = arg.get(1);
    let mode = arg.get(2);
    if path.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::String(s)) = path {
        if let Some(JsValue::Bool(r)) = recursive {
            if let Some(JsValue::Int(_m)) = mode {
                let res = if *r {
                    fs::create_dir_all(s.as_str())
                } else {
                    fs::create_dir(s.as_str())
                };
                return match res {
                    Ok(()) => JsValue::UnDefined,
                    Err(e) => {
                        let err = err_to_js_object(ctx, e);
                        JsValue::Exception(ctx.throw_error(err))
                    }
                };
            }
        }
    }
    return JsValue::UnDefined;
}

fn rmdir_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let path = arg.get(0);
    let recursive = arg.get(1);
    if path.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::String(s)) = path {
        if let Some(JsValue::Bool(r)) = recursive {
            let res = if *r {
                fs::remove_dir_all(s.as_str())
            } else {
                fs::remove_dir(s.as_str())
            };
            return match res {
                Ok(()) => JsValue::UnDefined,
                Err(e) => {
                    let err = err_to_js_object(ctx, e);
                    JsValue::Exception(ctx.throw_error(err))
                }
            };
        }
    }
    return JsValue::UnDefined;
}

fn rm_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let path = arg.get(0);
    let recursive = arg.get(1);
    let force = arg.get(2);
    if path.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::String(s)) = path {
        if let Some(JsValue::Bool(r)) = recursive {
            if let Some(JsValue::Bool(f)) = force {
                let res = fs::metadata(s.as_str()).and_then(|stat| {
                    if stat.is_file() {
                        fs::remove_file(s.as_str())
                    } else {
                        if *r {
                            fs::remove_dir_all(s.as_str())
                        } else {
                            fs::remove_dir(s.as_str())
                        }
                    }
                });
                return match res {
                    Ok(()) => JsValue::UnDefined,
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::NotFound && *f {
                            JsValue::UnDefined
                        } else {
                            let err = err_to_js_object(ctx, e);
                            JsValue::Exception(ctx.throw_error(err))
                        }
                    }
                };
            }
        }
    }
    return JsValue::UnDefined;
}

fn rename_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let old_path = arg.get(0);
    let new_path = arg.get(1);
    if old_path.is_none() || new_path.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::String(from)) = old_path {
        if let Some(JsValue::String(to)) = new_path {
            return match fs::rename(from.as_str(), to.as_str()) {
                Ok(()) => JsValue::UnDefined,
                Err(e) => {
                    let err = err_to_js_object(ctx, e);
                    JsValue::Exception(ctx.throw_error(err))
                }
            };
        }
    }
    return JsValue::UnDefined;
}

fn truncate_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let path = arg.get(0);
    let len = arg.get(1);
    if path.is_none() || len.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::String(p)) = path {
        if let Some(JsValue::Int(l)) = len {
            let res = fs::File::open(p.as_str()).and_then(|file| file.set_len(*l as u64));
            return match res {
                Ok(()) => JsValue::UnDefined,
                Err(e) => {
                    let err = err_to_js_object(ctx, e);
                    JsValue::Exception(ctx.throw_error(err))
                }
            };
        }
    }
    return JsValue::UnDefined;
}

fn ftruncate_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let fd = arg.get(0);
    let len = arg.get(1);
    if fd.is_none() || len.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::Int(f)) = fd {
        if let Some(JsValue::Int(l)) = len {
            let res = unsafe { wasi::fd_filestat_set_size(*f as u32, *l as u64) };
            return match res {
                Ok(()) => JsValue::UnDefined,
                Err(e) => {
                    let err = errno_to_js_object(ctx, e);
                    JsValue::Exception(ctx.throw_error(err))
                }
            };
        }
    }
    return JsValue::UnDefined;
}

fn realpath_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let path = arg.get(0);
    if path.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::String(p)) = path {
        let res = fs::canonicalize(p.as_str());
        return match res {
            Ok(realpath) => ctx.new_string(realpath.to_str().unwrap()).into(),
            Err(e) => {
                let err = err_to_js_object(ctx, e);
                JsValue::Exception(ctx.throw_error(err))
            }
        };
    }
    return JsValue::UnDefined;
}

fn copy_file_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let from_path = arg.get(0);
    let to_path = arg.get(1);
    if from_path.is_none() || to_path.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::String(from)) = from_path {
        if let Some(JsValue::String(to)) = to_path {
            let res = fs::copy(from.as_str(), to.as_str());
            return match res {
                Ok(_) => JsValue::UnDefined,
                Err(e) => {
                    let err = err_to_js_object(ctx, e);
                    JsValue::Exception(ctx.throw_error(err))
                }
            };
        }
    }
    return JsValue::UnDefined;
}

fn link_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let from_path = arg.get(0);
    let to_path = arg.get(1);
    if from_path.is_none() || to_path.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::String(from)) = from_path {
        if let Some(JsValue::String(to)) = to_path {
            let res = fs::hard_link(from.as_str(), to.as_str());
            return match res {
                Ok(_) => JsValue::UnDefined,
                Err(e) => {
                    let err = err_to_js_object(ctx, e);
                    JsValue::Exception(ctx.throw_error(err))
                }
            };
        }
    }
    return JsValue::UnDefined;
}

fn symlink_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let from_path = arg.get(0);
    let to_path = arg.get(1);
    if from_path.is_none() || to_path.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::String(from)) = from_path {
        if let Some(JsValue::String(to)) = to_path {
            let res = std::os::wasi::fs::symlink_path(from.as_str(), to.as_str());
            return match res {
                Ok(_) => JsValue::UnDefined,
                Err(e) => {
                    let err = err_to_js_object(ctx, e);
                    JsValue::Exception(ctx.throw_error(err))
                }
            };
        }
    }
    return JsValue::UnDefined;
}

fn utime_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let path = arg.get(0);
    let atime = arg.get(1);
    let mtime = arg.get(2);
    if path.is_none() || atime.is_none() || mtime.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::String(p)) = path {
        if let Some(JsValue::Float(a)) = atime {
            if let Some(JsValue::Float(m)) = mtime {
                let res = unsafe {
                    wasi::path_filestat_set_times(
                        3,
                        wasi::LOOKUPFLAGS_SYMLINK_FOLLOW,
                        p.as_str(),
                        *a as u64,
                        *m as u64,
                        wasi::FSTFLAGS_ATIM | wasi::FSTFLAGS_MTIM,
                    )
                };
                return match res {
                    Ok(_) => JsValue::UnDefined,
                    Err(e) => {
                        let err = errno_to_js_object(ctx, e);
                        JsValue::Exception(ctx.throw_error(err))
                    }
                };
            }
        }
    }
    return JsValue::UnDefined;
}

fn futime_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let fd = arg.get(0);
    let atime = arg.get(1);
    let mtime = arg.get(2);
    if fd.is_none() || atime.is_none() || mtime.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::Int(f)) = fd {
        if let Some(JsValue::Float(a)) = atime {
            if let Some(JsValue::Float(m)) = mtime {
                let res = unsafe {
                    wasi::fd_filestat_set_times(
                        *f as u32,
                        *a as u64,
                        *m as u64,
                        wasi::FSTFLAGS_ATIM | wasi::FSTFLAGS_MTIM,
                    )
                };
                return match res {
                    Ok(_) => JsValue::UnDefined,
                    Err(e) => {
                        let err = errno_to_js_object(ctx, e);
                        JsValue::Exception(ctx.throw_error(err))
                    }
                };
            }
        }
    }
    return JsValue::UnDefined;
}

fn fclose_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let fd = arg.get(0);
    if fd.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::Int(f)) = fd {
        let res = unsafe { wasi::fd_close(*f as u32) };
        return match res {
            Ok(_) => JsValue::UnDefined,
            Err(e) => {
                let err = errno_to_js_object(ctx, e);
                JsValue::Exception(ctx.throw_error(err))
            }
        };
    }
    return JsValue::UnDefined;
}

fn fdatasync_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let fd = arg.get(0);
    if fd.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::Int(f)) = fd {
        let res = unsafe { wasi::fd_datasync(*f as u32) };
        return match res {
            Ok(_) => JsValue::UnDefined,
            Err(e) => {
                let err = errno_to_js_object(ctx, e);
                JsValue::Exception(ctx.throw_error(err))
            }
        };
    }
    return JsValue::UnDefined;
}

fn fsync_sync(ctx: &mut Context, _this_val: JsValue, arg: &[JsValue]) -> JsValue {
    let fd = arg.get(0);
    if fd.is_none() {
        return JsValue::UnDefined;
    }
    if let Some(JsValue::Int(f)) = fd {
        let res = unsafe { wasi::fd_sync(*f as u32) };
        return match res {
            Ok(_) => JsValue::UnDefined,
            Err(e) => {
                let err = errno_to_js_object(ctx, e);
                JsValue::Exception(ctx.throw_error(err))
            }
        };
    }
    return JsValue::UnDefined;
}

struct FS;

impl ModuleInit for FS {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let stat_s = ctx.wrap_function("statSync", stat_sync);
        let lstat_s = ctx.wrap_function("lstatSync", lstat_sync);
        let fstat_s = ctx.wrap_function("fstatSync", fstat_sync);
        let mkdir_s = ctx.wrap_function("mkdirSync", mkdir_sync);
        let rmdir_s = ctx.wrap_function("rmdirSync", rmdir_sync);
        let rm_s = ctx.wrap_function("rmSync", rm_sync);
        let rename_s = ctx.wrap_function("renameSync", rename_sync);
        let truncate_s = ctx.wrap_function("truncateSync", truncate_sync);
        let ftruncate_s = ctx.wrap_function("ftruncateSync", ftruncate_sync);
        let realpath_s = ctx.wrap_function("realpathSync", realpath_sync);
        let copy_file_s = ctx.wrap_function("copyFileSync", copy_file_sync);
        let link_s = ctx.wrap_function("linkSync", link_sync);
        let symlink_s = ctx.wrap_function("symlinkSync", symlink_sync);
        let utime_s = ctx.wrap_function("utimeSync", utime_sync);
        let futime_s = ctx.wrap_function("futimeSync", futime_sync);
        let fclose_s = ctx.wrap_function("fcloseSync", fclose_sync);
        let fsync_s = ctx.wrap_function("fsyncSync", fsync_sync);
        let fdatasync_s = ctx.wrap_function("fdatasyncSync", fdatasync_sync);
        m.add_export("statSync", stat_s.into());
        m.add_export("lstatSync", lstat_s.into());
        m.add_export("fstatSync", fstat_s.into());
        m.add_export("mkdirSync", mkdir_s.into());
        m.add_export("rmdirSync", rmdir_s.into());
        m.add_export("rmSync", rm_s.into());
        m.add_export("renameSync", rename_s.into());
        m.add_export("truncateSync", truncate_s.into());
        m.add_export("ftruncateSync", ftruncate_s.into());
        m.add_export("realpathSync", realpath_s.into());
        m.add_export("copyFileSync", copy_file_s.into());
        m.add_export("linkSync", link_s.into());
        m.add_export("symlinkSync", symlink_s.into());
        m.add_export("utimeSync", utime_s.into());
        m.add_export("futimeSync", futime_s.into());
        m.add_export("fcloseSync", fclose_s.into());
        m.add_export("fsyncSync", fsync_s.into());
        m.add_export("fdatasyncSync", fdatasync_s.into());
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module(
        "_node:fs\0",
        FS,
        &[
            "statSync\0",
            "lstatSync\0",
            "fstatSync\0",
            "mkdirSync\0",
            "rmdirSync\0",
            "rmSync\0",
            "renameSync\0",
            "truncateSync\0",
            "realpathSync\0",
            "copyFileSync\0",
            "linkSync\0",
            "symlinkSync\0",
            "utimeSync\0",
            "futimeSync\0",
        ],
    )
}
