#![allow(dead_code, unused_imports, unused_must_use)]
use quickjs_rs_wasi::*;

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
    use quickjs_rs_wasi as q;
    let mut ctx = q::Context::new();

    let (file_path, mut rest_arg) = args_parse();
    let code = std::fs::read_to_string(&file_path);
    match code {
        Ok(code) => {
            rest_arg.insert(0, file_path.clone());
            ctx.put_args(rest_arg);
            ctx.eval_module_str(code.as_str(), &file_path);
        }
        Err(e) => {
            eprintln!("{}", e.to_string());
        }
    }
}
