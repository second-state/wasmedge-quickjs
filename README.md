# Run JavaScript in WebAssembly

Now supporting wasmedge socket for HTTP requests in JavaScript programs!

## Prerequisites

Install [Rust](https://www.rust-lang.org/tools/install) and [wasmedge CLI tool](https://github.com/WasmEdge/WasmEdge/blob/master/docs/install.md).

```shell
rustup target add wasm32-wasi
```

## Build

```shell
cargo build --target wasm32-wasi --release --features=http
```

## Run

```shell
wasmedge --dir .:. target/wasm32-wasi/release/quickjs-rs-wasi.wasm example_js/http_demo.js
```

## Optional: Get static-lib & binding.rs

If you want to build a custom libquickjs.a

See [[quickjs-wasi]](https://github.com/L-jasmine/quickjs-wasi) (branch:rs-binding)

```shell
run quickjs-wasi/lib/build.sh
```
