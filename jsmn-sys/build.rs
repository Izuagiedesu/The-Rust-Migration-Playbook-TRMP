// build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    // 1. Compile the Shim (which includes jsmn.h)
    cc::Build::new()
        .file("vendor/shim.c") // Compile our fake C file
        .include("vendor")
        .compile("jsmn");

    // 2. Generate Bindings from the Header
    let bindings = bindgen::Builder::default()
        .header("vendor/jsmn.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // 3. Write bindings
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}