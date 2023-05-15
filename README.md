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

Install [wasmedge CLI tool](https://wasmedge.org/docs/develop/build-and-run/install). Make sure that you [install the WasmEdge Tensorflow Lite plugin](https://wasmedge.org/docs/develop/build-and-run/install#wasi-nn-plugin-with-tensorflow-lite) if you want to try the Tensorflow examples.

## Hello world

### Build

```shell
$ cargo build --target wasm32-wasi --release
```

### Run

```shell
$ wasmedge --dir .:. target/wasm32-wasi/release/wasmedge_quickjs.wasm example_js/hello.js WasmEdge Runtime
```

## Add Core Module With JavaScript(ES)

### Build

```shell
cargo build --target wasm32-wasi --release
```

### Run

```shell
$ wasmedge --dir .:. target/wasm32-wasi/release/wasmedge_quickjs.wasm example_js/module_demo/demo.js 

ReferenceError: could not load module filename 'my_mod_1'

$ ls example_js/module_demo/modules/

my_mod_1.js  my_mod_2.js

# copy `my_mod_1.js` & `my_mod_2.js` into modules/
# and wasmedge_quickjs will load it as a core module 
$ cp example_js/module_demo/modules/* modules/
$ wasmedge --dir .:. target/wasm32-wasi/release/wasmedge_quickjs.wasm example_js/module_demo/demo.js

hello from "my_mod_1.js"
hello from "my_mod_2.js"
```

## Async HTTP Request

### Build

```shell
$ cargo build --target wasm32-wasi --release
```

### Run

HTTP client applications.

```shell
$ wasmedge --dir .:. target/wasm32-wasi/release/wasmedge_quickjs.wasm example_js/wasi_http_client.js
```

Start an HTTP server.

```
$ nohup wasmedge --dir .:. target/wasm32-wasi/release/wasmedge_quickjs.wasm example_js/wasi_http_server.js &
```

Access the server.

```shell
$ curl -d "WasmEdge" -X POST http://localhost:8000
echo:WasmEdge
```

> These examples also show how to import another JavaScript file into the current program.

## React SSR

### Build

```shell
$ cargo build --target wasm32-wasi --release
```

Then use [rollup.js](https://rollupjs.org/) to bundle the React application into a combined JS file. It turns the CommonJS modules in the application into ES6 modules, which [we support](#es6-module-support).

```shell
$ cd example_js/react_ssr
$ npm install
$ npm run build
```

### Run

```shell
$ wasmedge --dir .:. ../../target/wasm32-wasi/release/wasmedge_quickjs.wasm dist/main.js

<div data-reactroot=""><div>This is home</div><div><div>This is page</div></div></div>
```

## React Stream SSR

### Build

```shell
$ cargo build --target wasm32-wasi --release
```

Then use [rollup.js](https://rollupjs.org/) to bundle the React application into a combined JS file. It turns the CommonJS modules in the application into ES6 modules, which [we support](#es6-module-support).

```shell
$ cd example_js/react_ssr_stream
$ npm install
$ npm run build
```

### Run

Start the server.

```shell
$ nohup wasmedge --dir .:. ../../target/wasm32-wasi/release/wasmedge_quickjs.wasm dist/main.mjs &
```

Access the server.

```shell
$ curl http://localhost:8001
```

The results show the response comes in chuncks and the client closes the connection once all chuncks are received.

```shell
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   211    0   211    0     0   1029      0 --:--:-- --:--:-- --:--:--  1024
100   275    0   275    0     0    221      0 --:--:--  0:00:01 --:--:--   220
100   547    0   547    0     0    245      0 --:--:--  0:00:02 --:--:--   245
100  1020    0  1020    0     0    413      0 --:--:--  0:00:02 --:--:--   413

<!DOCTYPE html><html lang="en"><head><meta charSet="utf-8"/><title>Title</title></head><body><div><div> This is LazyHome </div><!--$?--><template id="B:0"></template><div> loading... </div><!--/$--></div></body></html><div hidden id="S:0"><template id="P:1"></template></div><div hidden id="S:1"><div><div>This is lazy page</div></div></div><script>function $RS(a,b){a=document.getElementById(a);b=document.getElementById(b);for(a.parentNode.removeChild(a);a.firstChild;)b.parentNode.insertBefore(a.firstChild,b);b.parentNode.removeChild(b)};$RS("S:1","P:1")</script><script>function $RC(a,b){a=document.getElementById(a);b=document.getElementById(b);b.parentNode.removeChild(b);if(a){a=a.previousSibling;var f=a.parentNode,c=a.nextSibling,e=0;do{if(c&&8===c.nodeType){var d=c.data;if("/$"===d)if(0===e)break;else e--;else"$"!==d&&"$?"!==d&&"$!"!==d||e++}d=c.nextSibling;f.removeChild(c);c=d}while(c);for(;b.firstChild;)f.insertBefore(b.firstChild,c);a.data="$";a._reactRetry&&a._reactRetry()}};$RC("B:0","S:0")</script>
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
$ wasmedge-tensorflow-lite --dir .:. ../../target/wasm32-wasi/release/wasmedge_quickjs.wasm main.js
```

## ES6 module support

### Build

```shell
$ cargo build --target wasm32-wasi --release
```

### Run

```shell
$ cd example_js/es6_module_demo
$ wasmedge --dir .:. ../../target/wasm32-wasi/release/wasmedge_quickjs.wasm demo.js
```

## Optional: Get static-lib & binding.rs

If you want to build a custom libquickjs.a to export some static c function.

See [[quickjs-wasi]](https://github.com/second-state/quickjs-wasi)

```shell
run quickjs-wasi/lib/build.sh
```
