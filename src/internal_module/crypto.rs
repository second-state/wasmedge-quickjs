use core::arch;

use crate::quickjs_sys::*;
use crate::EventLoop;

struct Crypto;

impl ModuleInit for Crypto {
    fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {}
}

pub fn init_module(ctx: &mut Context) {}
