#![allow(dead_code, unused_imports, unused_must_use)]

use std::borrow::{Borrow, BorrowMut};
use wasmedge_quickjs::*;

fn test_js_file(file_path: &str) {
    use wasmedge_quickjs as q;
    let mut rt = q::Runtime::new();
    rt.run_with_context(|ctx| {
        let code = std::fs::read_to_string(&file_path);
        match code {
            Ok(code) => {
                ctx.put_args(vec![file_path.clone()]);
                ctx.eval_module_str(code, &file_path);
                if let JsValue::Bool(false) = ctx.get_global().get("assertPass") {
                    panic!("assert fail");
                }
            }
            Err(e) => {
                eprintln!("{}", e.to_string());
                panic!("open js test file fail");
            }
        }
        ctx.js_loop().unwrap();
    });
}

#[test]
fn test_fs_access() {
    test_js_file("test/fs/test-fs-access.js");
}

#[test]
#[ignore] // this test will timeout on github action
fn test_fs_append_file() {
    test_js_file("test/fs/test-fs-append-file.js");
}

#[test]
fn test_fs_append_file_sync() {
    test_js_file("test/fs/test-fs-append-file-sync.js");
}

#[test]
fn test_fs_close_errors() {
    test_js_file("test/fs/test-fs-close-errors.js");
}
