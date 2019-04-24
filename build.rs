extern crate bindgen;

use std::env;
use std::env::var;
use std::path::PathBuf;

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/t2-output/win32-msvc-debug-default", manifest_dir);
    println!("cargo:rustc-link-lib=musashi");

    let bindings = bindgen::Builder::default()
        .header("musashi/m68k.h")
        .blacklist_function("m68k_read_memory_8")
        .blacklist_function("m68k_read_memory_16")
        .blacklist_function("m68k_read_memory_32")
        .blacklist_function("m68k_write_memory_8")
        .blacklist_function("m68k_write_memory_16")
        .blacklist_function("m68k_write_memory_32")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("musashi.bindings.rs"))
        .expect("Couldn't write bindings!");
}