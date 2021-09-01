# A wasi quickjs binding for rust
embed js

See `src/main.rs` and `demo.js`

# Build
```shell
cargo wasi build --release
```

# Run
```shell
wasmedge --dir .:. target/wasm32-wasi/debug/quickjs-rs-wasi.wasm
```

# Get static-lib & binding.rs
If you want to build a custom libquickjs.a.

See [[quickjs-wasi]](https://github.com/second-state/quickjs-wasi)