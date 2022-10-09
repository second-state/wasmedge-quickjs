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
                unreachable!();
            }
        }
        ctx.js_loop().unwrap();
    });
}

#[test]
fn test_fs_access() {
    test_js_file("test/fs/test-fs-access.js");
}
