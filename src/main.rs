#![allow(dead_code, unused_imports, unused_must_use)]

use std::borrow::{Borrow, BorrowMut};
use wasmedge_quickjs::*;

fn args_parse() -> (String, Vec<String>) {
    use argparse::ArgumentParser;
    let mut file_path = String::new();
    let mut res_args: Vec<String> = vec![];
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut file_path)
            .add_argument("file", argparse::Store, "js file")
            .required();
        ap.refer(&mut res_args)
            .add_argument("arg", argparse::List, "arg");
        ap.parse_args_or_exit();
    }
    (file_path, res_args)
}

fn main() {
    use wasmedge_quickjs as q;

   test_errors();

    let mut rt = q::Runtime::new();
    rt.run_with_context(|ctx| {
        let (file_path, mut rest_arg) = args_parse();
        let code = std::fs::read_to_string(&file_path);
        match code {
            Ok(code) => {
                rest_arg.insert(0, file_path.clone());
                ctx.put_args(rest_arg);
                ctx.eval_module_str(code, &file_path);
            }
            Err(e) => {
                eprintln!("{}", e.to_string());
            }
        }
        ctx.js_loop().unwrap();
    });
}

fn test_errors() {
    use wasmedge_quickjs as q;

    let mut rt = q::Runtime::new();
    rt.run_with_context(|ctx| {
        let code = String::from("const x = {};x.hello.world;");
        let r = ctx.eval_global_str(code, false);
        match r {
            JsValue::Exception(exception) => {
                println!("Exception value:{:?}", exception.get_message());
            }
            _ => {
                println!("return value:{:?}", r);
            }
        }

        let code = String::from("throw new Error('x.hello');");
        let r = ctx.eval_global_str(code, false);
        match r {
            JsValue::Exception(exception) => {
                println!("Exception value:{:?}", exception.get_message());
            }
            _ => {
                println!("return value:{:?}", r);
            }
        }

        let code = String::from("throw 6;");
        let r = ctx.eval_global_str(code, false);
        match r {
            JsValue::Exception(exception) => {
                println!("Exception value:{:?}", exception.get_message());
            }
            _ => {
                println!("return value:{:?}", r);
            }
        }
    });
}
