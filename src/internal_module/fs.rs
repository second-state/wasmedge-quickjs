use crate::quickjs_sys::*;
use std::convert::TryInto;
use std::fs;
use std::fs::Permissions;
use std::io;
use std::os::wasi::fs::{FileTypeExt, MetadataExt};
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
    if permit.readonly() {
        F_OK | R_OK | X_OK
    } else {
        F_OK | R_OK | W_OK | X_OK
    }
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

struct FS;

impl ModuleInit for FS {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let stat_s = ctx.wrap_function("statSync", stat_sync);
        let lstat_s = ctx.wrap_function("lstatSync", lstat_sync);
        m.add_export("statSync", stat_s.into());
        m.add_export("lstatSync", lstat_s.into());
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module("_node:fs\0", FS, &["statSync\0", "lstatSync\0"])
}
