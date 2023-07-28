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

// #[tokio::main(flavor = "current_thread")]
fn main() {
    use wasmedge_quickjs as q;
    env_logger::init();

    let tokio_rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // let mut rt = q::Runtime::new();

    // let r = rt
    //     .async_run_with_context(Box::new(|ctx| {
    //         let (file_path, mut rest_arg) = args_parse();
    //         let code = std::fs::read_to_string(&file_path);
    //         match code {
    //             Ok(code) => {
    //                 rest_arg.insert(0, file_path.clone());
    //                 ctx.put_args(rest_arg);
    //                 ctx.eval_buf(code.into_bytes(), &file_path, 1)
    //             }
    //             Err(e) => {
    //                 eprintln!("{}", e.to_string());
    //                 JsValue::UnDefined
    //             }
    //         }
    //     }))
    //     .await;
    // log::info!("{r:?}");

    tokio_rt.block_on(async {
        let notify = std::sync::Arc::new(tokio::sync::Notify::new());
        let notify_ = notify.clone();
        let duration = std::time::Duration::from_millis(5000);
        let fut = async move {
            match tokio::time::timeout(duration, notify.notified()).await {
                Ok(_) => {
                    println!("ok");
                }
                Err(_) => {
                    println!("timeout");
                }
            }
        };
        let r = tokio::spawn(async move { fut.await }).await;
        notify_.notify_one();
    });
}
