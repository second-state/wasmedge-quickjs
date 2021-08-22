pub mod quickjs_sys;

fn args_parse() -> String {
    use argparse::ArgumentParser;
    let mut file_path = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut file_path)
            .add_argument("file", argparse::Store, "js file")
            .required();
        ap.parse_args_or_exit();
    }
    file_path
}

fn main() {
    use quickjs_sys as q;
    let mut rt = q::Runtime::new();
    let mut ctx = rt.new_context();

    let file_path = args_parse();
    let code = std::fs::read_to_string(&file_path);
    match code {
        Ok(code) => {
            ctx.eval_str(code.as_str(), &file_path);
        }
        Err(e) => {
            eprintln!("{}", e.to_string());
        }
    }
}
