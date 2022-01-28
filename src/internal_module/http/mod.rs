use crate::Context;

pub fn init_module(ctx: &mut Context) {
    let code = include_str!("./core.js");
    ctx.eval_module_str(code, "http");
}
