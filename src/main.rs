pub mod quickjs_sys;

fn args_parse() -> Vec<String> {
    use argparse::ArgumentParser;
    let mut res_args: Vec<String> = vec![];
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut res_args)
            .add_argument("arg", argparse::List, "arg");
        ap.parse_args_or_exit();
    }
    res_args
}

fn main() {
    use quickjs_sys as q;
    let mut ctx = q::Context::new();
    let code = include_str!("../demo.js");
    let mut res_args = args_parse();
    res_args.insert(0, "<process_name>".to_string());
    ctx.put_args(res_args);
    ctx.eval_str(code, "<input>");
}
