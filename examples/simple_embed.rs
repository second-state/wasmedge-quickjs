use quickjs_rs_wasi::*;
fn main() {
    let mut ctx = Context::new();

    let code = r#"
    let point = import('example_js/hello.js')
    point.then((mod_h)=>{
        return mod_h.kk()
    })
    "#;

    let mut args = ctx.new_array();
    args.set(0, ctx.new_string("hello").into());
    ctx.get_global().set("args", args.into());
    let p = ctx.eval_global_str(code);
    println!("before poll:{:?}", p);
    ctx.promise_loop_poll();
    println!("after poll:{:?}", p);
    if let JsValue::Promise(p) = p {
        let v = p.get_result();
        println!("v = {:?}", v);
    }
}
