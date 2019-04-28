extern crate bindgen;

use std::env;
use std::env::var;
use std::path::PathBuf;

fn path_relative_to_cargo_manifest_dir(path: &String) -> String {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    format!("{}/{}", manifest_dir, path)
}

fn musashi_lib() -> (String, String) {
    let folder = if cfg!(debug_assertions) { "win32-msvc-debug-default" } else { "win32-msvc-release-default" };
    (path_relative_to_cargo_manifest_dir(&format!("t2-output/{}", folder)), "musashi".to_string())
}

fn main() {
    let (musashi_dir, musashi_name) = musashi_lib();
    println!("cargo:rustc-link-search={}", musashi_dir);
    println!("cargo:rustc-link-lib={}", musashi_name);

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