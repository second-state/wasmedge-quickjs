use quickjs_rs_wasi::*;

fn main() {
    let mut ctx = Context::new();

    let code = r#"
    import('async_demo.js').then((demo)=>{
        return demo.wait_simple_val(1)
    })
    "#;

    let p = ctx.eval_global_str(code);
    println!("before poll:{:?}", p);
    if let JsValue::Promise(ref p) = p {
        let v = p.get_result();
        println!("v = {:?}", v);
    }
    ctx.promise_loop_poll();
    println!("after poll:{:?}", p);
    if let JsValue::Promise(ref p) = p {
        let v = p.get_result();
        println!("v = {:?}", v);
    }
}
