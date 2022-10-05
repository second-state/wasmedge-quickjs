#![allow(dead_code, unused_imports, unused_must_use)]
#[macro_use]
extern crate lazy_static;

mod event_loop;
mod internal_module;
mod quickjs_sys;

pub use event_loop::EventLoop;

pub use quickjs_sys::*;
