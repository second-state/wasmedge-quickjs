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
                    assert!(false, "js assert fail");
                }
            }
            Err(e) => {
                eprintln!("{}", e.to_string());
                assert!(false, "run js test file fail");
            }
        }
        ctx.js_loop().unwrap();
    });
}

#[test]
fn test_path_basename() {
    test_js_file("test/path/test-path-basename.js");
}

#[test]
fn test_path_dirname() {
    test_js_file("test/path/test-path-dirname.js");
}

#[test]
fn test_path_extname() {
    test_js_file("test/path/test-path-extname.js");
}

#[test]
fn test_path_isabsolute() {
    test_js_file("test/path/test-path-isabsolute.js");
}

#[test]
fn test_path_join() {
    test_js_file("test/path/test-path-join.js");
}

#[test]
fn test_path_makelong() {
    test_js_file("test/path/test-path-makelong.js");
}

#[test]
fn test_path_normalize() {
    test_js_file("test/path/test-path-normalize.js");
}

#[test]
fn test_path_parse_format() {
    test_js_file("test/path/test-path-parse-format.js");
}

#[test]
fn test_path_relative() {
    test_js_file("test/path/test-path-relative.js");
}

#[test]
fn test_path_resolve() {
    test_js_file("test/path/test-path-resolve.js");
}

#[test]
fn test_path_zero_length_strings() {
    test_js_file("test/path/test-path-zero-length-strings.js");
}

#[test]
fn test_path() {
    test_js_file("test/path/test-path.js");
}
