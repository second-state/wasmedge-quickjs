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
    let mut ctx = q::Context::new();
    let code = include_str!("../demo.js");
    ctx.eval_str(code, "<input>");
}
