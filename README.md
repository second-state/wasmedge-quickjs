# A wasi quickjs binding for rust
this branch show how to import a custom host function into quickjs.

# Build
```shell
#build wasm
$ cargo wasi build --release

#build custom webassembly Runtime
$ cd wasmedge_c

#build a custom Runtime
wasmedge_c/$ gcc demo_wasmedge.c -lwasmedge_c -o demo_wasmedge
```

# Run
```shell
wasmedge_c/$ export LD_LIBRARY_PATH=.

wasmedge_c/$ ./demo_wasmedge ../target/wasm32-wasi/release/quickjs-rs-wasi.wasm example_js/hello.js WasmEdge Runtime
js=> Hello WasmEdge Runtime
Runtime(c)=> host_inc call : 3
js=> host_inc(2)= 3

Runtime(c)=> OK
wasmedge_c/$ 
```