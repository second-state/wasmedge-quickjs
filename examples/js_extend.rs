use wasmedge_quickjs::js_class::v2::{self, JsClassDef};
use wasmedge_quickjs::{AsObject, JsValue, Runtime};

#[derive(Debug)]
struct ClassA(i32);
impl v2::JsClassDef for ClassA {
    type RefType = ClassA;

    const CLASS_NAME: &'static str = "A";

    const CONSTRUCTOR_ARGC: u8 = 1;

    const FIELDS: &'static [&'static str] = &["val"];

    const METHODS: &'static [(&'static str, u8)] = &[("inc", 0)];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(
        _ctx: &mut wasmedge_quickjs::Context,
        argv: &[wasmedge_quickjs::JsValue],
    ) -> Result<Self::RefType, wasmedge_quickjs::JsValue> {
        match argv.get(0) {
            Some(JsValue::Int(v)) => Ok(ClassA(*v)),
            _ => Ok(ClassA(0)),
        }
    }

    fn method_fn(
        name: &str,
        this: &mut Self::RefType,
        _ctx: &mut wasmedge_quickjs::Context,
        _argv: &[wasmedge_quickjs::JsValue],
    ) -> wasmedge_quickjs::JsValue {
        if name == "inc" {
            this.0 += 1;
            JsValue::Int(this.0)
        } else {
            JsValue::UnDefined
        }
    }

    fn field_get(
        name: &str,
        this: &Self::RefType,
        _ctx: &mut wasmedge_quickjs::Context,
    ) -> wasmedge_quickjs::JsValue {
        if name == "val" {
            JsValue::Int(this.0)
        } else {
            JsValue::UnDefined
        }
    }

    fn field_set(
        _name: &str,
        _this: &mut Self::RefType,
        _ctx: &mut wasmedge_quickjs::Context,
        _val: wasmedge_quickjs::JsValue,
    ) {
    }
}

#[derive(Debug)]
struct ClassB(ClassA, i32);

impl AsRef<ClassA> for ClassB {
    fn as_ref(&self) -> &ClassA {
        &self.0
    }
}

impl AsMut<ClassA> for ClassB {
    fn as_mut(&mut self) -> &mut ClassA {
        &mut self.0
    }
}

impl v2::JsClassDefExtends for ClassB {
    type RefType = ClassB;

    type BaseDef = ClassA;

    const CLASS_NAME: &'static str = "B";

    const CONSTRUCTOR_ARGC: u8 = 1;

    const FIELDS: &'static [&'static str] = &["val_b"];

    const METHODS: &'static [(&'static str, u8)] = &[("inc_b", 0), ("display", 0)];

    unsafe fn mut_class_id_ptr() -> &'static mut u32 {
        static mut CLASS_ID: u32 = 0;
        &mut CLASS_ID
    }

    fn constructor_fn(
        ctx: &mut wasmedge_quickjs::Context,
        argv: &[JsValue],
    ) -> Result<Self::RefType, JsValue> {
        let a = ClassA::constructor_fn(ctx, argv)?;
        Ok(ClassB(a, 1))
    }

    fn method_fn(
        name: &str,
        this: &mut Self::RefType,
        ctx: &mut wasmedge_quickjs::Context,
        argv: &[wasmedge_quickjs::JsValue],
    ) -> wasmedge_quickjs::JsValue {
        match name {
            "inc" => {
                // Overload

                this.0 .0 += 2;
                JsValue::Int(this.0 .0)
            }
            "inc_b" => {
                this.1 += 1;
                JsValue::Int(this.1)
            }
            "display" => {
                println!("display=> {:?}", this);
                JsValue::UnDefined
            }
            _ => ClassA::method_fn(name, this.as_mut(), ctx, argv), // same as super.call()
        }
    }

    fn field_get(
        name: &str,
        this: &Self::RefType,
        ctx: &mut wasmedge_quickjs::Context,
    ) -> wasmedge_quickjs::JsValue {
        if name == "val_b" {
            JsValue::Int(this.1)
        } else {
            ClassA::field_get(name, this.as_ref(), ctx)
        }
    }
}

fn main() {
    let mut rt = Runtime::new();
    rt.run_with_context(|ctx| {
        let a_ctor = v2::register_class::<ClassA>(ctx);
        let b_ctor = v2::register_class::<ClassB>(ctx);

        let a_proto = ClassA::proto(ctx);
        let b_proto = ClassB::proto(ctx);

        v2::class_extends(ctx, b_proto, a_proto);

        let mut global = ctx.get_global();
        global.set("ClassA", a_ctor);
        global.set("ClassB", b_ctor);

        let code = r#"
        let a = new ClassA(1)
        print('a.val =',a.val)
        print('a.inc() =',a.inc())
        print('a.val =',a.val)
        print()

        let b = new ClassB()
        print('b.val =',b.val)
        print('b.inc() =',b.inc())
        print('b.val =',b.val)
        print()

        print('b.val_b =',b.val_b)
        print('b.inc_b() =',b.inc_b())
        print('b.val_b =',b.val_b)
        print()

        b.display()
        print()

        print('b instanceof ClassA =',b instanceof ClassA)
        "#;
        ctx.eval_global_str(code.to_string());
    })
}
