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
fn test_fs_access() {
    test_js_file("test/fs/test-fs-access.js");
}

#[test]
fn test_fs_append_file_sync() {
    test_js_file("test/fs/test-fs-append-file-sync.js");
}

#[test]
#[ignore = "https://github.com/second-state/wasmedge-quickjs/pull/84#issuecomment-1278664721"]
fn test_fs_append_file() {
    test_js_file("test/fs/test-fs-append-file.js");
}

#[test]
fn test_fs_assert_encoding_error() {
    test_js_file("test/fs/test-fs-assert-encoding-error.js");
}

#[test]
fn test_fs_buffer() {
    test_js_file("test/fs/test-fs-buffer.js");
}

#[test]
fn test_fs_buffertype_writesync() {
    test_js_file("test/fs/test-fs-buffertype-writesync.js");
}

#[test]
fn test_fs_close_errors() {
    test_js_file("test/fs/test-fs-close-errors.js");
}

#[test]
fn test_fs_close() {
    test_js_file("test/fs/test-fs-close.js");
}

#[test]
fn test_fs_constants() {
    test_js_file("test/fs/test-fs-constants.js");
}

#[test]
fn test_fs_copyfile() {
    test_js_file("test/fs/test-fs-copyfile.js");
}

#[test]
fn test_fs_exists() {
    test_js_file("test/fs/test-fs-exists.js");
}

#[test]
fn test_fs_link() {
    test_js_file("test/fs/test-fs-link.js");
}

#[test]
fn test_fs_open() {
    test_js_file("test/fs/test-fs-open.js");
}

#[test]
fn test_fs_promises_exists() {
    test_js_file("test/fs/test-fs-promises-exists.js");
}

#[test]
#[ignore = "https://github.com/second-state/wasmedge-quickjs/pull/84#issuecomment-1278664721"]
fn test_fs_promises_file_handle_close_errors() {
    test_js_file("test/fs/test-fs-promises-file-handle-close-errors.js");
}

#[test]
fn test_fs_promises_file_handle_close() {
    test_js_file("test/fs/test-fs-promises-file-handle-close.js");
}

#[test]
fn test_fs_promises_file_handle_stat() {
    test_js_file("test/fs/test-fs-promises-file-handle-stat.js");
}

#[test]
#[ignore = "https://github.com/second-state/wasmedge-quickjs/pull/84#issuecomment-1278664721"]
fn test_fs_readfile_empty() {
    test_js_file("test/fs/test-fs-readfile-empty.js");
}

#[test]
#[ignore = "https://github.com/second-state/wasmedge-quickjs/pull/84#issuecomment-1278664721"]
fn test_fs_readfile_fd() {
    test_js_file("test/fs/test-fs-readfile-fd.js");
}

#[test]
#[ignore = "https://github.com/second-state/wasmedge-quickjs/pull/84#issuecomment-1278664721"]
fn test_fs_readfile_flags() {
    test_js_file("test/fs/test-fs-readfile-flags.js");
}

#[test]
#[ignore = "https://github.com/second-state/wasmedge-quickjs/pull/84#issuecomment-1278664721"]
fn test_fs_readfile_unlink() {
    test_js_file("test/fs/test-fs-readfile-unlink.js");
}

#[test]
#[ignore = "https://github.com/second-state/wasmedge-quickjs/pull/84#issuecomment-1278664721"]
fn test_fs_readfile_zero_byte_liar() {
    test_js_file("test/fs/test-fs-readfile-zero-byte-liar.js");
}

#[test]
fn test_fs_readlink_type_check() {
    test_js_file("test/fs/test-fs-readlink-type-check.js");
}

#[test]
fn test_fs_read_sync_optional_params() {
    test_js_file("test/fs/test-fs-readSync-optional-params.js");
}

#[test]
fn test_fs_read_sync_position_validation() {
    test_js_file("test/fs/test-fs-readSync-position-validation.js");
}

#[test]
#[ignore = "https://github.com/second-state/wasmedge-quickjs/pull/84#issuecomment-1278664721"]
fn test_fs_readv_promises() {
    test_js_file("test/fs/test-fs-readv-promises.js");
}

#[test]
#[ignore = "https://github.com/second-state/wasmedge-quickjs/pull/84#issuecomment-1278664721"]
fn test_fs_readv_promisify() {
    test_js_file("test/fs/test-fs-readv-promisify.js");
}

#[test]
fn test_fs_readv_sync() {
    test_js_file("test/fs/test-fs-readv-sync.js");
}

#[test]
#[ignore = "https://github.com/second-state/wasmedge-quickjs/pull/84#issuecomment-1278664721"]
fn test_fs_readv() {
    test_js_file("test/fs/test-fs-readv.js");
}

#[test]
fn test_fs_ready_event_stream() {
    test_js_file("test/fs/test-fs-ready-event-stream.js");
}

#[test]
fn test_fs_stat_date() {
    test_js_file("test/fs/test-fs-stat-date.js");
}

#[test]
fn test_fs_stat() {
    test_js_file("test/fs/test-fs-stat.js");
}
