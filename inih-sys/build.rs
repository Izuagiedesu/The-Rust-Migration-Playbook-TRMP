// build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("vendor/ini.c")
        .include("vendor")
        .compile("inih");

    let bindings = bindgen::Builder::default()
        .header("vendor/ini.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}