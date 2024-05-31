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

### Usage with custom ssl certs
```bash
$  wasmedge --dir .:. --dir /etc/ssl:/etc/ssl:readonly --env SSL_CERT_FILE="/etc/ssl/cert.pem" target/wasm32-wasi/release/wasmedge_quickjs.wasm example_js/wasi_https_fetch.js
```
substitute the value of `/etc/ssl` and `/etc/ssl/cert.pem` with the location of your cert folder and cert file
