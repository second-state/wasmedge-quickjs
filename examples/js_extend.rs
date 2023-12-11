use wasmedge_quickjs::js_class;
use wasmedge_quickjs::{
    AsObject, Context, ExtendsJsClassDef, JsClassDef, JsClassField, JsClassMethod, JsClassTool,
    JsObject, JsValue, Runtime,
};

#[derive(Debug)]
struct ClassA(i32);

impl ClassA {
    pub fn get_val(&self, _ctx: &mut Context) -> JsValue {
        JsValue::Int(self.0)
    }

    pub fn inc(
        &mut self,
        _this_obj: &mut JsObject,
        _ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        self.0 += 1;
        JsValue::Int(self.0)
    }
}

impl JsClassDef for ClassA {
    type RefType = ClassA;

    const CLASS_NAME: &'static str = "ClassA";

    const CONSTRUCTOR_ARGC: u8 = 1;

    const FIELDS: &'static [JsClassField<Self::RefType>] = &[("val", ClassA::get_val, None)];

    const METHODS: &'static [JsClassMethod<Self::RefType>] = &[("inc", 0, ClassA::inc)];

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

impl ClassB {
    pub fn get_val_b(&self, _ctx: &mut Context) -> JsValue {
        JsValue::Int(self.1)
    }

    pub fn inc_b(
        &mut self,
        _this_obj: &mut JsObject,
        _ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        self.1 += 1;
        JsValue::Int(self.1)
    }

    pub fn display(
        &mut self,
        _this_obj: &mut JsObject,
        _ctx: &mut Context,
        _argv: &[JsValue],
    ) -> JsValue {
        println!("display=> {:?}", self);
        JsValue::UnDefined
    }
}

impl ExtendsJsClassDef for ClassB {
    type RefType = ClassB;

    type BaseDef = ClassA;

    const EXT_CLASS_NAME: &'static str = "ClassB";

    const CONSTRUCTOR_ARGC: u8 = 1;

    const FIELDS: &'static [JsClassField<Self::RefType>] = &[("val_b", ClassB::get_val_b, None)];

    const METHODS: &'static [JsClassMethod<Self::RefType>] =
        &[("inc_b", 0, ClassB::inc_b), ("display", 0, ClassB::display)];

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
}

fn main() {
    let mut rt = Runtime::new();
    rt.run_with_context(|ctx| {
        let a_ctor = js_class::register_class::<ClassA>(ctx);
        let b_ctor = js_class::register_class::<ClassB>(ctx);

        let a_proto = ClassA::proto(ctx);
        let b_proto = ClassB::proto(ctx);

        js_class::class_extends(ctx, b_proto, a_proto);

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
