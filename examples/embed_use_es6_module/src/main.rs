use wasmedge_quickjs::*;

fn main() {
    let mut ctx = Context::new();

    let code = r#"
    let m = import('es6_module_demo.js')
    m
    "#;

    let p = ctx.eval_global_str(code, true);
    println!("before poll:{:?}", p);
    ctx.promise_loop_poll();
    println!("after poll:{:?}", p);
    if let JsValue::Promise(ref p) = p {
        let m = p.get_result();
        println!("m = {:?}", m);
        if let JsValue::Object(mod_obj) = m {
            let f = mod_obj.get("do_something");
            println!("do_something = {:?}", f);
            if let JsValue::Function(f) = f {
                f.call(&mut [ctx.new_string("hello").into()]);
            }
        }
    }
}
