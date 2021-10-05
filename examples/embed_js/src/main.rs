use quickjs_rs_wasi::JsValue;
use quickjs_rs_wasi::*;

fn main() {
    let mut ctx = Context::new();
    js_hello(&mut ctx);
    run_js_code(&mut ctx);
    run_js_function(&mut ctx);
    run_rust_function(&mut ctx);
    rust_new_object_and_js_call(&mut ctx);
    js_new_object_and_rust_call(&mut ctx);
    js_promise(&mut ctx);
}

fn js_hello(ctx: &mut Context) {
    println!("\n<----run_simple_js---->");
    let code = r#"print('hello quickjs')"#;
    let r = ctx.eval_global_str(code);
    println!("return value:{:?}", r);
}

fn run_js_code(ctx: &mut Context) {
    println!("\n<----run_js_code---->");
    let code = r#"
    let a = 1+1;
    print('js print: 1+1=',a);
    'hello'; // eval_return
    "#;
    let r = ctx.eval_global_str(code);
    println!("return value:{:?}", r);
}

fn run_js_function(ctx: &mut Context) {
    println!("\n<----run_js_function---->");
    let code = r#"
    (x)=>{
        print("js print: x=",x)
    }
    "#;
    let r = ctx.eval_global_str(code);
    println!("return value:{:?}", r);
    if let JsValue::Function(f) = r {
        let hello_str = ctx.new_string("hello");
        let mut argv = vec![hello_str.into()];
        let r = f.call(&mut argv);
        println!("return value:{:?}", r);
    }

    let code = r#"
    (x)=>{
        print("\nx=",x)
        let old_value = x[0]
        x[0] = 1
        return old_value
    }
    "#;
    let r = ctx.eval_global_str(code);
    if let JsValue::Function(f) = r {
        let mut x = ctx.new_array();
        x.set(0, 0.into());
        x.set(1, 1.into());
        x.set(2, 2.into());

        let mut argv = vec![x.into()];
        println!("argv = {:?}", argv);
        let r = f.call(&mut argv);
        println!("return value:{:?}", r);
    }
}

fn run_rust_function(ctx: &mut Context) {
    println!("\n<----run_rust_function---->");

    struct HelloFn;
    impl JsFn for HelloFn {
        fn call(_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
            println!("hello from rust");
            println!("argv={:?}", argv);
            JsValue::UnDefined
        }
    }
    let f = ctx.new_function::<HelloFn>("hello");
    ctx.get_global().set("hi", f.into());
    let code = r#"hi(1,2,3)"#;
    let r = ctx.eval_global_str(code);
    println!("return value:{:?}", r);
}

fn rust_new_object_and_js_call(ctx: &mut Context) {
    println!("\n<----rust_new_object_and_js_call---->");
    let mut obj = ctx.new_object();
    obj.set("a", 1.into());
    obj.set("b", ctx.new_string("abc").into());

    struct ObjectFn;
    impl JsFn for ObjectFn {
        fn call(_ctx: &mut Context, this_val: JsValue, argv: &[JsValue]) -> JsValue {
            println!("hello from rust");
            println!("argv={:?}", argv);
            if let JsValue::Object(obj) = this_val {
                let obj_map = obj.to_map();
                println!("this={:#?}", obj_map);
            }
            JsValue::UnDefined
        }
    }

    let f = ctx.new_function::<ObjectFn>("anything");
    obj.set("f", f.into());

    ctx.get_global().set("test_obj", obj.into());

    let code = r#"
    print('test_obj keys=',Object.keys(test_obj))
    print('test_obj.a=',test_obj.a)
    print('test_obj.b=',test_obj.b)
    test_obj.f(1,2,3,"hi")
    "#;

    ctx.eval_global_str(code);
}

fn js_new_object_and_rust_call(ctx: &mut Context) {
    println!("\n<----js_new_object_and_rust_call---->");
    let code = r#"
    let obj = {
        a:1,
        b:"abc",
        f(x){
            print('this=',Object.keys(this))
            print('x=',x)
            print('something_from_rust=',this.something_from_rust)
        }
    }
    obj
    "#;
    if let JsValue::Object(mut obj) = ctx.eval_global_str(code) {
        let mut args = vec![ctx.new_string("rust_args_string").into()];

        let obj_map = obj.to_map();
        println!("{:#?}", obj_map);

        if let Ok(o) = obj_map {
            println!("---call function---");
            if let Some(JsValue::Function(f)) = o.get("f") {
                f.call(&mut args);
            }
        }
        obj.set("something_from_rust", 255.into());
        println!("---call function from obj---");
        obj.invoke("f", &mut args);
    }
}

fn js_promise(ctx: &mut Context) {
    println!("\n<----promise---->");
    let code = r#"
    async function f1(){
        print("f1 running")
        return 1
    }
    async function f(){
        print("f running")
        let f1_result = await f1();
        print("await f1")
        return f1_result
    };
    f
    "#;

    let r = ctx.eval_global_str(code);
    println!("{:?}", r);
    if let JsValue::Function(f) = r {
        let mut args = vec![];
        let r = f.call(&mut args);
        println!("{:?}", r);
        if let JsValue::Promise(p) = r {
            let result = p.get_result();
            println!("promise result:{:?}", result);
            println!("poll promise");
            ctx.promise_loop_poll();
            let result = p.get_result();
            println!("promise result:{:?}", result);
        }
    }
}
