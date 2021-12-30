pub mod event_loop_module;
#[cfg(feature = "http")]
pub mod http_module;
#[cfg(feature = "img")]
pub mod img_module;
#[cfg(feature = "tensorflow")]
pub mod tensorflow_module;
pub mod wasi_net_module;
