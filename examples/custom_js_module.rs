mod point {
    use quickjs_rs_wasi::*;

    #[derive(Debug)]
    struct Point(i32, i32);

    struct PointDef;

    impl JsClassDef<Point> for PointDef {
        const CLASS_NAME: &'static str = "Point\0";
        const CONSTRUCTOR_ARGC: u8 = 2;

        fn constructor(_: &mut Context, argv: &[JsValue]) -> Option<Point> {
            println!("rust-> new Point {:?}", argv);
            let x = argv.get(0);
            let y = argv.get(1);
            if let ((Some(JsValue::Int(ref x)), Some(JsValue::Int(ref y)))) = (x, y) {
                Some(Point(*x, *y))
            } else {
                None
            }
        }

        fn proto_init(p: &mut JsClassProto<Point, PointDef>) {
            struct X;
            impl JsClassGetterSetter<Point> for X {
                const NAME: &'static str = "x\0";

                fn getter(_: &mut Context, this_val: &mut Point) -> JsValue {
                    println!("rust-> get x");
                    this_val.0.into()
                }

                fn setter(_: &mut Context, this_val: &mut Point, val: JsValue) {
                    println!("rust-> set x:{:?}", val);
                    if let JsValue::Int(x) = val {
                        this_val.0 = x
                    }
                }
            }

            struct Y;
            impl JsClassGetterSetter<Point> for Y {
                const NAME: &'static str = "y\0";

                fn getter(_: &mut Context, this_val: &mut Point) -> JsValue {
                    println!("rust-> get y");
                    this_val.1.into()
                }

                fn setter(_: &mut Context, this_val: &mut Point, val: JsValue) {
                    println!("rust-> set y:{:?}", val);
                    if let JsValue::Int(y) = val {
                        this_val.1 = y
                    }
                }
            }

            struct FnPrint;
            impl JsMethod<Point> for FnPrint {
                const NAME: &'static str = "pprint\0";
                const LEN: u8 = 0;

                fn call(_: &mut Context, this_val: &mut Point, _argv: &[JsValue]) -> JsValue {
                    println!("rust-> pprint: {:?}", this_val);
                    JsValue::Int(1)
                }
            }

            p.add_getter_setter(X);
            p.add_getter_setter(Y);
            p.add_function(FnPrint);
        }
    }

    struct PointModule;
    impl ModuleInit for PointModule {
        fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
            m.add_export("Point\0", PointDef::class_value(ctx));
        }
    }

    pub fn init_point_module(ctx: &mut Context) {
        ctx.register_class(PointDef);
        ctx.register_module("point\0", PointModule, &["Point\0"]);
    }
}

use quickjs_rs_wasi::*;
fn main() {
    let mut ctx = Context::new();
    point::init_point_module(&mut ctx);

    let code = r#"
    let point = import('point')
    point.then((mod_p)=>{
        try{
            let p = new mod_p.Point()
            print("js-> p:",p)
            print("js->",p.x,p.y)
            p.x=2
            p.pprint()
        }catch(e){
            print(e)
        }
        
        let p0 = new mod_p.Point(1,2)
        print("js->",p0.x,p0.y)
        p0.pprint()
    })
    "#;

    ctx.eval_global_str(code);
    ctx.promise_loop_poll();
}
