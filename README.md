# A wasi quickjs binding for rust
and support wasmedge socket!

# Build
```shell
cargo wasi build --release
```

# Run
```shell
wasmedge --dir .:. target/wasm32-wasi/debug/quickjs-rs-wasi.wasm example_js/http_demo.js
```

# Get static-lib & binding.rs
If you want to build a custom libquickjs.a

See [[quickjs-wasi]](https://github.com/L-jasmine/quickjs-wasi) (branch:rs-binding)
```shell
run quickjs-wasi/lib/build.sh
```