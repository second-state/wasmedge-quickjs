# A wasi quickjs binding for rust
this example show how to import a custom host function into quickjs.

# Build

```shell
#build wasm
$ cargo build --target wasm32-wasi --release

#build custom webassembly Runtime
$ cd wasmedge_c

#build a custom Runtime
wasmedge_c/$ gcc demo_wasmedge.c -lwasmedge_c -o demo_wasmedge
```

# Run

```shell
wasmedge_c/$ export LD_LIBRARY_PATH=.

wasmedge_c/$ ./demo_wasmedge ../../../target/wasm32-wasi/release/host_function.wasm
Runtime(c)=> host_inc call : 3
js=> host_inc(2)= 3

Runtime(c)=> OK
wasmedge_c/$ 
```