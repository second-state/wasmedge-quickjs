# Run JavaScript in WebAssembly

Now supporting wasmedge socket for HTTP requests and Tensorflow in JavaScript programs!

## Prerequisites

Make sure that you will have GCC installed on Ubuntu 20.04.

```shell
$ sudo apt update
$ sudo apt install build-essential
```

Install [Rust](https://www.rust-lang.org/tools/install) and use the following command to install the `wasm32-wasi` target.

```shell
$ rustup target add wasm32-wasi
```

Install [wasmedge CLI tool](https://github.com/WasmEdge/WasmEdge/blob/master/docs/install.md). Make sure that you use the `-e all` option to install the WasmEdge Tensorflow extensions if you want to try the Tensorflow examples.

## Hello world

### Build

```shell
$ cargo build --target wasm32-wasi --release
```

### Run

```shell
$ cd example_js
$ wasmedge --dir .:. ../target/wasm32-wasi/release/wasmedge_quickjs.wasm hello.js WasmEdge Runtime
```

## HTTP Request

### Build

```shell
$ cargo build --target wasm32-wasi --release
```

### Run

HTTP client applications.

```shell
$ cd example_js
$ wasmedge --dir .:. ../target/wasm32-wasi/release/wasmedge_quickjs.wasm http_demo.js
```

Run and POST to a HTTP server.

```
# Start the server
$ cd example_js
$ nohup wasmedge --dir .:. ../target/wasm32-wasi/release/wasmedge_quickjs.wasm http_server_demo.js &

# Access the server
$ curl -d "WasmEdge" -X POST http://localhost:8000
echo:WasmEdge
```

> These examples also show how to import another JavaScript file into the current program.

## TensorFlow

### Build

Note: Build the QuickJS interpreter with the WasmEdge Tensorflow extension.

```shell
$ cargo build --target wasm32-wasi --release --features=tensorflow
```

### Run

```shell
$ cd example_js/tensorflow_lite_demo
$ wasmedge-tensorflow --dir .:. ../../target/wasm32-wasi/release/wasmedge_quickjs.wasm main.js
```

## Optional: Get static-lib & binding.rs

If you want to build a custom libquickjs.a to export some static c function.

See [[quickjs-wasi]](https://github.com/second-state/quickjs-wasi)

```shell
run quickjs-wasi/lib/build.sh
```
