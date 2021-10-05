
## Build

```
cargo build --target wasm32-wasi --release
```

## Run

```
wasmedge --dir .:. target/wasm32-wasi/release/embed_rust_module.wasm
```
