use crate::quickjs_sys::*;
use std::convert::TryInto;
use std::fs;
use std::io;
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

fn stat_to_js_object(ctx: &mut Context, stat: std::fs::Metadata) -> JsValue {
    let mut res = ctx.new_object();
    res.set("is_file", stat.is_file().into());
    res.set("is_directory", stat.is_dir().into());
    res.set("is_symlink", stat.is_symlink().into());
    res.set("size", (stat.len() as i32).into());
    res.set("mtime", to_msec(stat.modified()));
    res.set("atime", to_msec(stat.accessed()));
    res.set("birthtime", to_msec(stat.created()));
    res.set("dev", 0.into());
    res.set("ino", 0.into());
    res.set("mode", 0.into());
    res.set("nlink", 0.into());
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

struct FS;

impl ModuleInit for FS {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
        let f = ctx.wrap_function("statSync", stat_sync);
        m.add_export("statSync", f.into());
    }
}

pub fn init_module(ctx: &mut Context) {
    ctx.register_module("_node:fs\0", FS, &["statSync\0"])
}
