# Run JavaScript in WebAssembly

Now supporting wasmedge socket for HTTP requests and Tensorflow in JavaScript programs!

## Prerequisites

Install [Rust](https://www.rust-lang.org/tools/install) and [wasmedge CLI tool](https://github.com/WasmEdge/WasmEdge/blob/master/docs/install.md).
Make sure that you use the `-e all` option to install the WasmEdge Tensorflow extensions if you want to try the Tensorflow examples below.

```shell
rustup target add wasm32-wasi
```

## Hello js

### Build

```shell
$ cargo build --target wasm32-wasi --release
```

### Run

```shell
$ cd example_js
$ wasmedge --dir .:. ../target/wasm32-wasi/release/quickjs-rs-wasi.wasm hello.js WasmEdge Runtime
```

## HTTP Request

### Build

```shell
$ cargo build --target wasm32-wasi --release
```

### Run

Bonus: This example also shows how to import another JavaScript file into the current program.

```shell
$ cd example_js
$ wasmedge --dir .:. ../target/wasm32-wasi/release/quickjs-rs-wasi.wasm http_demo.js
```

## TensorFlow

### Build

Note: Build the QuickJS interpreter with the WasmEdge Tensorflow extension.

```shell
$ cargo build --target wasm32-wasi --release --features=tensorflow
```

### Run

```shell
$ cd example_js/tensorflow_lite_demo
$ wasmedge-tensorflow --dir .:. ../../target/wasm32-wasi/release/quickjs-rs-wasi.wasm main.js
```

## Optional: Get static-lib & binding.rs

If you want to build a custom libquickjs.a to export some static c function.

See [[quickjs-wasi]](https://github.com/second-state/quickjs-wasi)

```shell
run quickjs-wasi/lib/build.sh
```
