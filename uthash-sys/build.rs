// build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    // 1. Compile our C Shim (which includes uthash.h)
    cc::Build::new()
        .file("vendor/shim.c")
        .include("vendor")
        .compile("uthash_demo");

    // 2. Generate Bindings
    // We manually describe the C functions we wrote in shim.c so bindgen knows what to look for.
    let bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", "
            struct my_struct {
                int id;
                char name[10];
            };
            void add_user(int user_id, const char *name);
            struct my_struct *find_user(int user_id);
            void delete_all();
        ")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // 3. Write bindings to file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}