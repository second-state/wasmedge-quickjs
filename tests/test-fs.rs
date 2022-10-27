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
            }
            Err(e) => {
                eprintln!("{}", e.to_string());
                assert!(false, "run js test file fail");
            }
        }
        ctx.js_loop().unwrap();
        println!("{:?}", ctx.get_global().get("commonExitCheck"));
        if let JsValue::Function(func) = ctx.get_global().get("commonExitCheck") {
            func.call(&[]);
        }
        ctx.js_loop().unwrap();
        println!("{:?}", ctx.get_global().get("assertPass"));
        if let JsValue::Bool(false) = ctx.get_global().get("assertPass") {
            assert!(false, "js assert fail");
        }
    });
    std::fs::remove_dir_all("./test/.tmp.0");
}

#[test]
fn test_fs_access() {
    test_js_file("test/fs/test-fs-access.js");
}

#[test]
fn test_fs_append_file() {
    test_js_file("test/fs/test-fs-append-file.js");
}

#[test]
fn test_fs_append_file_sync() {
    test_js_file("test/fs/test-fs-append-file-sync.js");
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
#[ignore = "unsupported, chmod"]
fn test_fs_chmod() {
    test_js_file("test/fs/test-fs-chmod.js");
}

#[test]
#[ignore = "unsupported, chmod"]
fn test_fs_chmod_mask() {
    test_js_file("test/fs/test-fs-chmod-mask.js");
}

#[test]
#[ignore = "unsupported, chown"]
fn test_fs_chown_type_check() {
    test_js_file("test/fs/test-fs-chown-type-check.js");
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
#[ignore = "unsupported, chmod"]
fn test_fs_copyfile_respect_permissions() {
    test_js_file("test/fs/test-fs-copyfile-respect-permissions.js");
}

#[test]
fn test_fs_cp() {
    test_js_file("test/fs/test-fs-cp.js");
}

#[test]
fn test_fs_empty_read_stream() {
    test_js_file("test/fs/test-fs-empty-readStream.js");
}

#[test]
fn test_fs_error_messages() {
    test_js_file("test/fs/test-fs-error-messages.js");
}

#[test]
fn test_fs_exists() {
    test_js_file("test/fs/test-fs-exists.js");
}

#[test]
#[ignore = "unsupported, too long path"]
fn test_fs_existssync_false() {
    test_js_file("test/fs/test-fs-existssync-false.js");
}

#[test]
#[ignore = "unsupported, chmod"]
fn test_fs_fchmod() {
    test_js_file("test/fs/test-fs-fchmod.js");
}

#[test]
#[ignore = "unsupported, chown"]
fn test_fs_fchown() {
    test_js_file("test/fs/test-fs-fchown.js");
}

#[test]
#[ignore = "v8 specific"]
fn test_fs_filehandle() {
    test_js_file("test/fs/test-fs-filehandle.js");
}

#[test]
fn test_fs_filehandle_use_after_close() {
    test_js_file("test/fs/test-fs-filehandle-use-after-close.js");
}

#[test]
fn test_fs_fmap() {
    test_js_file("test/fs/test-fs-fmap.js");
}

#[test]
fn test_fs_fsync() {
    test_js_file("test/fs/test-fs-fsync.js");
}

#[test]
#[ignore = "unsupported, chmod"]
fn test_fs_lchmod() {
    test_js_file("test/fs/test-fs-lchmod.js");
}

#[test]
#[ignore = "unsupported, chown"]
fn test_fs_lchown() {
    test_js_file("test/fs/test-fs-lchown.js");
}

#[test]
fn test_fs_link() {
    test_js_file("test/fs/test-fs-link.js");
}

#[test]
#[ignore = "windows specific"]
fn test_fs_long_path() {
    test_js_file("test/fs/test-fs-long-path.js");
}

#[test]
fn test_fs_make_callback() {
    test_js_file("test/fs/test-fs-make-callback.js");
}

#[test]
fn test_fs_make_stats_callback() {
    test_js_file("test/fs/test-fs-makeStatsCallback.js");
}

#[test]
fn test_fs_mkdir() {
    test_js_file("test/fs/test-fs-mkdir.js");
}

#[test]
#[ignore = "unsupported, chmod"]
fn test_fs_mkdir_mode_mask() {
    test_js_file("test/fs/test-fs-mkdir-mode-mask.js");
}

#[test]
#[ignore = "unsupported, child_process"]
fn test_fs_mkdir_recursive_eaccess() {
    test_js_file("test/fs/test-fs-mkdir-recursive-eaccess.js");
}

#[test]
fn test_fs_mkdir_rmdir() {
    test_js_file("test/fs/test-fs-mkdir-rmdir.js");
}

#[test]
fn test_fs_mkdtemp() {
    test_js_file("test/fs/test-fs-mkdtemp.js");
}

#[test]
fn test_fs_mkdtemp_prefix_check() {
    test_js_file("test/fs/test-fs-mkdtemp-prefix-check.js");
}

#[test]
#[ignore = "working"]
fn test_fs_non_number_arguments_throw() {
    test_js_file("test/fs/test-fs-non-number-arguments-throw.js");
}

#[test]
#[ignore = "working"]
fn test_fs_null_bytes() {
    test_js_file("test/fs/test-fs-null-bytes.js");
}

#[test]
#[ignore = "working"]
fn test_fs_opendir() {
    test_js_file("test/fs/test-fs-opendir.js");
}

#[test]
fn test_fs_open_flags() {
    test_js_file("test/fs/test-fs-open-flags.js");
}

#[test]
fn test_fs_open() {
    test_js_file("test/fs/test-fs-open.js");
}

#[test]
#[ignore = "working"]
fn test_fs_open_mode_mask() {
    test_js_file("test/fs/test-fs-open-mode-mask.js");
}

#[test]
#[ignore = "working"]
fn test_fs_open_no_close() {
    test_js_file("test/fs/test-fs-open-no-close.js");
}

#[test]
#[ignore = "working"]
fn test_fs_open_numeric_flags() {
    test_js_file("test/fs/test-fs-open-numeric-flags.js");
}

#[test]
#[ignore = "working"]
fn test_fs_options_immutable() {
    test_js_file("test/fs/test-fs-options-immutable.js");
}

#[test]
fn test_fs_promises_exists() {
    test_js_file("test/fs/test-fs-promises-exists.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_file_handle_aggregate_errors() {
    test_js_file("test/fs/test-fs-promises-file-handle-aggregate-errors.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_file_handle_append_file() {
    test_js_file("test/fs/test-fs-promises-file-handle-append-file.js");
}

#[test]
#[ignore = "unsupported, chomd"]
fn test_fs_promises_file_handle_chmod() {
    test_js_file("test/fs/test-fs-promises-file-handle-chmod.js");
}

#[test]
fn test_fs_promises_file_handle_close_errors() {
    test_js_file("test/fs/test-fs-promises-file-handle-close-errors.js");
}

#[test]
fn test_fs_promises_file_handle_close() {
    test_js_file("test/fs/test-fs-promises-file-handle-close.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_file_handle_op_errors() {
    test_js_file("test/fs/test-fs-promises-file-handle-op-errors.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_file_handle_read_file() {
    test_js_file("test/fs/test-fs-promises-file-handle-readFile.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_file_handle_read() {
    test_js_file("test/fs/test-fs-promises-file-handle-read.js");
}

#[test]
#[ignore = "unsupported, worker_threads"]
fn test_fs_promises_file_handle_read_worker() {
    test_js_file("test/fs/test-fs-promises-file-handle-read-worker.js");
}

#[test]
fn test_fs_promises_file_handle_stat() {
    test_js_file("test/fs/test-fs-promises-file-handle-stat.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_file_handle_stream() {
    test_js_file("test/fs/test-fs-promises-file-handle-stream.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_file_handle_sync() {
    test_js_file("test/fs/test-fs-promises-file-handle-sync.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_file_handle_truncate() {
    test_js_file("test/fs/test-fs-promises-file-handle-truncate.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_file_handle_write_file() {
    test_js_file("test/fs/test-fs-promises-file-handle-writeFile.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_file_handle_write() {
    test_js_file("test/fs/test-fs-promises-file-handle-write.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises() {
    test_js_file("test/fs/test-fs-promises.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_readfile_empty() {
    test_js_file("test/fs/test-fs-promises-readfile-empty.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_readfile() {
    test_js_file("test/fs/test-fs-promises-readfile.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_readfile_with_fd() {
    test_js_file("test/fs/test-fs-promises-readfile-with-fd.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_promises_watch() {
    test_js_file("test/fs/test-fs-promises-watch.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_writefile() {
    test_js_file("test/fs/test-fs-promises-writefile.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_writefile_typedarray() {
    test_js_file("test/fs/test-fs-promises-writefile-typedarray.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_writefile_with_fd() {
    test_js_file("test/fs/test-fs-promises-writefile-with-fd.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promises_write_optional_params() {
    test_js_file("test/fs/test-fs-promises-write-optional-params.js");
}

#[test]
#[ignore = "working"]
fn test_fs_promisified() {
    test_js_file("test/fs/test-fs-promisified.js");
}

#[test]
#[ignore = "working"]
fn test_fs_readdir_buffer() {
    test_js_file("test/fs/test-fs-readdir-buffer.js");
}

#[test]
#[ignore = "working"]
fn test_fs_readdir() {
    test_js_file("test/fs/test-fs-readdir.js");
}

#[test]
#[ignore = "working"]
fn test_fs_readdir_stack_overflow() {
    test_js_file("test/fs/test-fs-readdir-stack-overflow.js");
}

#[test]
#[ignore = "working"]
fn test_fs_readdir_types() {
    test_js_file("test/fs/test-fs-readdir-types.js");
}

#[test]
#[ignore = "working"]
fn test_fs_readdir_ucs2() {
    test_js_file("test/fs/test-fs-readdir-ucs2.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_empty_buffer() {
    test_js_file("test/fs/test-fs-read-empty-buffer.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_file_assert_encoding() {
    test_js_file("test/fs/test-fs-read-file-assert-encoding.js");
}

#[test]
fn test_fs_readfile_empty() {
    test_js_file("test/fs/test-fs-readfile-empty.js");
}

#[test]
#[ignore = "working"]
fn test_fs_readfile_error() {
    test_js_file("test/fs/test-fs-readfile-error.js");
}

#[test]
fn test_fs_readfile_fd() {
    test_js_file("test/fs/test-fs-readfile-fd.js");
}

#[test]
fn test_fs_readfile_flags() {
    test_js_file("test/fs/test-fs-readfile-flags.js");
}

#[test]
#[ignore = "working"]
fn test_fs_readfile() {
    test_js_file("test/fs/test-fs-readfile.js");
}

#[test]
#[ignore = "unsupported, child_process"]
fn test_fs_readfile_pipe() {
    test_js_file("test/fs/test-fs-readfile-pipe.js");
}

#[test]
#[ignore = "unsupported, child_process"]
fn test_fs_readfile_pipe_large() {
    test_js_file("test/fs/test-fs-readfile-pipe-large.js");
}

#[test]
#[ignore = "working"]
fn test_fs_readfilesync_enoent() {
    test_js_file("test/fs/test-fs-readfilesync-enoent.js");
}

#[test]
#[ignore = "linux specific"]
fn test_fs_read_file_sync_hostname() {
    test_js_file("test/fs/test-fs-read-file-sync-hostname.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_file_sync() {
    test_js_file("test/fs/test-fs-read-file-sync.js");
}

#[test]
#[ignore = "unsupported, child_process"]
fn test_fs_readfilesync_pipe_large() {
    test_js_file("test/fs/test-fs-readfilesync-pipe-large.js");
}

#[test]
fn test_fs_readfile_unlink() {
    test_js_file("test/fs/test-fs-readfile-unlink.js");
}

#[test]
fn test_fs_readfile_zero_byte_liar() {
    test_js_file("test/fs/test-fs-readfile-zero-byte-liar.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read() {
    test_js_file("test/fs/test-fs-read.js");
}

#[test]
fn test_fs_readlink_type_check() {
    test_js_file("test/fs/test-fs-readlink-type-check.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_offset_null() {
    test_js_file("test/fs/test-fs-read-offset-null.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_optional_params() {
    test_js_file("test/fs/test-fs-read-optional-params.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_position_validation() {
    test_js_file("test/fs/test-fs-read-position-validation.mjs");
}

#[test]
#[ignore = "working"]
fn test_fs_read_promises_optional_params() {
    test_js_file("test/fs/test-fs-read-promises-optional-params.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream_auto_close() {
    test_js_file("test/fs/test-fs-read-stream-autoClose.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream_concurrent_reads() {
    test_js_file("test/fs/test-fs-read-stream-concurrent-reads.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream_double_close() {
    test_js_file("test/fs/test-fs-read-stream-double-close.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream_encoding() {
    test_js_file("test/fs/test-fs-read-stream-encoding.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream_err() {
    test_js_file("test/fs/test-fs-read-stream-err.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream_fd() {
    test_js_file("test/fs/test-fs-read-stream-fd.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream_fd_leak() {
    test_js_file("test/fs/test-fs-read-stream-fd-leak.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream_file_handle() {
    test_js_file("test/fs/test-fs-read-stream-file-handle.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream_inherit() {
    test_js_file("test/fs/test-fs-read-stream-inherit.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream() {
    test_js_file("test/fs/test-fs-read-stream.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream_patch_open() {
    test_js_file("test/fs/test-fs-read-stream-patch-open.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream_pos() {
    test_js_file("test/fs/test-fs-read-stream-pos.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream_resume() {
    test_js_file("test/fs/test-fs-read-stream-resume.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_stream_throw_type_error() {
    test_js_file("test/fs/test-fs-read-stream-throw-type-error.js");
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
#[ignore = "working"]
fn test_fs_read_type() {
    test_js_file("test/fs/test-fs-read-type.js");
}

#[test]
fn test_fs_readv() {
    test_js_file("test/fs/test-fs-readv.js");
}

#[test]
fn test_fs_readv_promises() {
    test_js_file("test/fs/test-fs-readv-promises.js");
}

#[test]
fn test_fs_readv_promisify() {
    test_js_file("test/fs/test-fs-readv-promisify.js");
}

#[test]
fn test_fs_readv_sync() {
    test_js_file("test/fs/test-fs-readv-sync.js");
}

#[test]
fn test_fs_ready_event_stream() {
    test_js_file("test/fs/test-fs-ready-event-stream.js");
}

#[test]
#[ignore = "working"]
fn test_fs_read_zero_length() {
    test_js_file("test/fs/test-fs-read-zero-length.js");
}

#[test]
#[ignore = "unsupported, realpath"]
fn test_fs_realpath_buffer_encoding() {
    test_js_file("test/fs/test-fs-realpath-buffer-encoding.js");
}

#[test]
#[ignore = "unsupported, realpath"]
fn test_fs_realpath() {
    test_js_file("test/fs/test-fs-realpath.js");
}

#[test]
#[ignore = "unsupported, realpath"]
fn test_fs_realpath_native() {
    test_js_file("test/fs/test-fs-realpath-native.js");
}

#[test]
#[ignore = "unsupported, realpath"]
fn test_fs_realpath_on_substed_drive() {
    test_js_file("test/fs/test-fs-realpath-on-substed-drive.js");
}

#[test]
#[ignore = "unsupported, realpath"]
fn test_fs_realpath_pipe() {
    test_js_file("test/fs/test-fs-realpath-pipe.js");
}

#[test]
#[ignore = "working"]
fn test_fs_rename_type_check() {
    test_js_file("test/fs/test-fs-rename-type-check.js");
}

#[test]
#[ignore = "working"]
fn test_fs_rmdir_recursive() {
    test_js_file("test/fs/test-fs-rmdir-recursive.js");
}

#[test]
#[ignore = "working"]
fn test_fs_rmdir_recursive_sync_warns_not_found() {
    test_js_file("test/fs/test-fs-rmdir-recursive-sync-warns-not-found.js");
}

#[test]
#[ignore = "working"]
fn test_fs_rmdir_recursive_sync_warns_on_file() {
    test_js_file("test/fs/test-fs-rmdir-recursive-sync-warns-on-file.js");
}

#[test]
#[ignore = "working"]
fn test_fs_rmdir_recursive_throws_not_found() {
    test_js_file("test/fs/test-fs-rmdir-recursive-throws-not-found.js");
}

#[test]
#[ignore = "working"]
fn test_fs_rmdir_recursive_throws_on_file() {
    test_js_file("test/fs/test-fs-rmdir-recursive-throws-on-file.js");
}

#[test]
#[ignore = "working"]
fn test_fs_rmdir_recursive_warns_not_found() {
    test_js_file("test/fs/test-fs-rmdir-recursive-warns-not-found.js");
}

#[test]
#[ignore = "working"]
fn test_fs_rmdir_recursive_warns_on_file() {
    test_js_file("test/fs/test-fs-rmdir-recursive-warns-on-file.js");
}

#[test]
#[ignore = "working"]
fn test_fs_rmdir_type_check() {
    test_js_file("test/fs/test-fs-rmdir-type-check.js");
}

#[test]
#[ignore = "working"]
fn test_fs_rm() {
    test_js_file("test/fs/test-fs-rm.js");
}

#[test]
#[ignore = "working"]
fn test_fs_sir_writes_alot() {
    test_js_file("test/fs/test-fs-sir-writes-alot.js");
}

#[test]
fn test_fs_stat_bigint() {
    test_js_file("test/fs/test-fs-stat-bigint.js");
}

#[test]
fn test_fs_stat_date() {
    test_js_file("test/fs/test-fs-stat-date.js");
}

#[test]
fn test_fs_stat() {
    test_js_file("test/fs/test-fs-stat.js");
}

#[test]
#[ignore = "working"]
fn test_fs_stream_construct_compat_error_read() {
    test_js_file("test/fs/test-fs-stream-construct-compat-error-read.js");
}

#[test]
#[ignore = "working"]
fn test_fs_stream_construct_compat_error_write() {
    test_js_file("test/fs/test-fs-stream-construct-compat-error-write.js");
}

#[test]
#[ignore = "working"]
fn test_fs_stream_construct_compat_graceful_fs() {
    test_js_file("test/fs/test-fs-stream-construct-compat-graceful-fs.js");
}

#[test]
#[ignore = "working"]
fn test_fs_stream_construct_compat_old_node() {
    test_js_file("test/fs/test-fs-stream-construct-compat-old-node.js");
}

#[test]
#[ignore = "working"]
fn test_fs_stream_destroy_emit_error() {
    test_js_file("test/fs/test-fs-stream-destroy-emit-error.js");
}

#[test]
#[ignore = "working"]
fn test_fs_stream_double_close() {
    test_js_file("test/fs/test-fs-stream-double-close.js");
}

#[test]
#[ignore = "working"]
fn test_fs_stream_fs_options() {
    test_js_file("test/fs/test-fs-stream-fs-options.js");
}

#[test]
#[ignore = "working"]
fn test_fs_stream_options() {
    test_js_file("test/fs/test-fs-stream-options.js");
}

#[test]
#[ignore = "working"]
fn test_fs_symlink_buffer_path() {
    test_js_file("test/fs/test-fs-symlink-buffer-path.js");
}

#[test]
#[ignore = "working"]
fn test_fs_symlink_dir() {
    test_js_file("test/fs/test-fs-symlink-dir.js");
}

#[test]
#[ignore = "working"]
fn test_fs_symlink_dir_junction() {
    test_js_file("test/fs/test-fs-symlink-dir-junction.js");
}

#[test]
#[ignore = "working"]
fn test_fs_symlink_dir_junction_relative() {
    test_js_file("test/fs/test-fs-symlink-dir-junction-relative.js");
}

#[test]
#[ignore = "working"]
fn test_fs_symlink() {
    test_js_file("test/fs/test-fs-symlink.js");
}

#[test]
#[ignore = "working"]
fn test_fs_symlink_longpath() {
    test_js_file("test/fs/test-fs-symlink-longpath.js");
}

#[test]
#[ignore = "working"]
fn test_fs_sync_fd_leak() {
    test_js_file("test/fs/test-fs-sync-fd-leak.js");
}

#[test]
#[ignore = "working"]
fn test_fs_syncwritestream() {
    test_js_file("test/fs/test-fs-syncwritestream.js");
}

#[test]
#[ignore = "working"]
fn test_fs_timestamp_parsing_error() {
    test_js_file("test/fs/test-fs-timestamp-parsing-error.js");
}

#[test]
#[ignore = "working"]
fn test_fs_truncate_clear_file_zero() {
    test_js_file("test/fs/test-fs-truncate-clear-file-zero.js");
}

#[test]
#[ignore = "working"]
fn test_fs_truncate_fd() {
    test_js_file("test/fs/test-fs-truncate-fd.js");
}

#[test]
#[ignore = "working"]
fn test_fs_truncate() {
    test_js_file("test/fs/test-fs-truncate.js");
}

#[test]
#[ignore = "working"]
fn test_fs_truncate_sync() {
    test_js_file("test/fs/test-fs-truncate-sync.js");
}

#[test]
#[ignore = "working"]
fn test_fs_unlink_type_check() {
    test_js_file("test/fs/test-fs-unlink-type-check.js");
}

#[test]
#[ignore = "working"]
fn test_fs_utils_get_dirents() {
    test_js_file("test/fs/test-fs-utils-get-dirents.js");
}

#[test]
#[ignore = "working"]
fn test_fs_util_validateoffsetlength() {
    test_js_file("test/fs/test-fs-util-validateoffsetlength.js");
}

#[test]
#[ignore = "working"]
fn test_fs_utimes() {
    test_js_file("test/fs/test-fs-utimes.js");
}

#[test]
#[ignore = "working"]
fn test_fs_utimes_y2_k38() {
    test_js_file("test/fs/test-fs-utimes-y2K38.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_watch_abort_signal() {
    test_js_file("test/fs/test-fs-watch-abort-signal.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_watch_close_when_destroyed() {
    test_js_file("test/fs/test-fs-watch-close-when-destroyed.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_watch_encoding() {
    test_js_file("test/fs/test-fs-watch-encoding.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_watch_enoent() {
    test_js_file("test/fs/test-fs-watch-enoent.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_watchfile_bigint() {
    test_js_file("test/fs/test-fs-watchfile-bigint.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_watch_file_enoent_after_deletion() {
    test_js_file("test/fs/test-fs-watch-file-enoent-after-deletion.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_watchfile() {
    test_js_file("test/fs/test-fs-watchfile.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_watchfile_ref_unref() {
    test_js_file("test/fs/test-fs-watchfile-ref-unref.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_watch() {
    test_js_file("test/fs/test-fs-watch.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_watch_recursive() {
    test_js_file("test/fs/test-fs-watch-recursive.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_watch_ref_unref() {
    test_js_file("test/fs/test-fs-watch-ref-unref.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_watch_stop_async() {
    test_js_file("test/fs/test-fs-watch-stop-async.js");
}

#[test]
#[ignore = "unsupported, watch"]
fn test_fs_watch_stop_sync() {
    test_js_file("test/fs/test-fs-watch-stop-sync.js");
}

#[test]
#[ignore = "working"]
fn test_fs_whatwg_url() {
    test_js_file("test/fs/test-fs-whatwg-url.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_buffer() {
    test_js_file("test/fs/test-fs-write-buffer.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_buffer_large() {
    test_js_file("test/fs/test-fs-write-buffer-large.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_file_buffer() {
    test_js_file("test/fs/test-fs-write-file-buffer.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_file_invalid_path() {
    test_js_file("test/fs/test-fs-write-file-invalid-path.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_file() {
    test_js_file("test/fs/test-fs-write-file.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_file_sync() {
    test_js_file("test/fs/test-fs-write-file-sync.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_file_typedarrays() {
    test_js_file("test/fs/test-fs-write-file-typedarrays.js");
}

#[test]
#[ignore = "working"]
fn test_fs_writefile_with_fd() {
    test_js_file("test/fs/test-fs-writefile-with-fd.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write() {
    test_js_file("test/fs/test-fs-write.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_negativeoffset() {
    test_js_file("test/fs/test-fs-write-negativeoffset.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_no_fd() {
    test_js_file("test/fs/test-fs-write-no-fd.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_optional_params() {
    test_js_file("test/fs/test-fs-write-optional-params.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_reuse_callback() {
    test_js_file("test/fs/test-fs-write-reuse-callback.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_sigxfsz() {
    test_js_file("test/fs/test-fs-write-sigxfsz.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_stream_autoclose_option() {
    test_js_file("test/fs/test-fs-write-stream-autoclose-option.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_stream_change_open() {
    test_js_file("test/fs/test-fs-write-stream-change-open.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_stream_close_without_callback() {
    test_js_file("test/fs/test-fs-write-stream-close-without-callback.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_stream_double_close() {
    test_js_file("test/fs/test-fs-write-stream-double-close.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_stream_encoding() {
    test_js_file("test/fs/test-fs-write-stream-encoding.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_stream_end() {
    test_js_file("test/fs/test-fs-write-stream-end.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_stream_err() {
    test_js_file("test/fs/test-fs-write-stream-err.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_stream_file_handle_2() {
    test_js_file("test/fs/test-fs-write-stream-file-handle-2.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_stream_file_handle() {
    test_js_file("test/fs/test-fs-write-stream-file-handle.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_stream_fs() {
    test_js_file("test/fs/test-fs-write-stream-fs.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_stream() {
    test_js_file("test/fs/test-fs-write-stream.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_stream_patch_open() {
    test_js_file("test/fs/test-fs-write-stream-patch-open.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_stream_throw_type_error() {
    test_js_file("test/fs/test-fs-write-stream-throw-type-error.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_sync() {
    test_js_file("test/fs/test-fs-write-sync.js");
}

#[test]
#[ignore = "working"]
fn test_fs_write_sync_optional_params() {
    test_js_file("test/fs/test-fs-write-sync-optional-params.js");
}

#[test]
#[ignore = "working"]
fn test_fs_writev() {
    test_js_file("test/fs/test-fs-writev.js");
}

#[test]
#[ignore = "working"]
fn test_fs_writev_promises() {
    test_js_file("test/fs/test-fs-writev-promises.js");
}

#[test]
#[ignore = "working"]
fn test_fs_writev_sync() {
    test_js_file("test/fs/test-fs-writev-sync.js");
}
