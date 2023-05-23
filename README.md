# Run JavaScript in WebAssembly

Checkout the [documentation](https://wasmedge.org/docs/category/develop-wasm-apps-in-javascript)

## Quick start

```
git clone https://github.com/second-state/wasmedge-quickjs
cd wasmedge-quickjs

cargo build --target wasm32-wasi --release

wasmedge --dir .:. target/wasm32-wasi/release/wasmedge_quickjs.wasm example_js/hello.js WasmEdge Runtime
Hello WasmEdge Runtime
```
