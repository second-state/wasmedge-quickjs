# A wasi quickjs binding for rust
support wasmedge socket

# Prerequisites
* Install [Rust](https://www.rust-lang.org/)
* Install [WasmEdge](https://github.com/WasmEdge/WasmEdge)
* Install [wasmedge-tensorflow](https://github.com/second-state/WasmEdge-tensorflow-tools#run-wasmedge-tensorflow-tools)
# HTTP feature
## Build
```shell
cargo wasi build --release --features=http
```
## Run
```shell
wasmedge --dir=.:. target/wasm32-wasi/release/quickjs-rs-wasi.wasm example_js/http_demo.js
```
# Tensorflow feature
## Build
```shell
cargo wasi build --release --features=tensorflow
```
## Run
```shell
#run tensorflow
wasmedge-tensorflow --dir=.:. target/wasm32-wasi/release/quickjs-rs-wasi.wasm example_js/tensorflow_lite_demo/main.js

#run tensorflow-lite
wasmedge-tensorflow --dir=.:. target/wasm32-wasi/release/quickjs-rs-wasi.wasm example_js/tensorflow_demo/main.js
```

# Get static-lib & binding.rs
If you want to build a custom libquickjs.a

See [[quickjs-wasi]](https://github.com/second-state/quickjs-wasi)