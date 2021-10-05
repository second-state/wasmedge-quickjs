use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir_path = Path::new(&out_dir);
    std::fs::copy("lib/libquickjs.a", out_dir_path.join("libquickjs.a"))
        .expect("Could not copy libquickjs.a to output directory");
    println!("cargo:rustc-link-search={}", &out_dir);
    println!("cargo:rustc-link-lib=quickjs");
}
