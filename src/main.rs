pub mod quickjs_sys;

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
    use quickjs_sys as q;
    let code = r#"
    import {host_inc} from 'host_function_demo'
    print('js say => hello js')
    print('js say => host_inc(2)=',host_inc(2))
    "#;
    let mut ctx = q::Context::new();
    ctx.eval_str(code, "<input>");
}
