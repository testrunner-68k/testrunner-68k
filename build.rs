extern crate bindgen;

use std::env;
use std::env::var;
use std::path::PathBuf;

fn path_relative_to_cargo_manifest_dir(path: &String) -> String {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    format!("{}/{}", manifest_dir, path)
}

fn musashi_lib() -> (String, String) {

    let target_os = env::var("CARGO_CFG_TARGET_OS");

    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };

    let platform_toolset = match target_os.as_ref().map(|x| &**x) {
        Ok("windows") => "win32-msvc",
        Ok("linux") => "linux-gcc",
        Ok(target_os_string) => panic!("Unsupported target OS: {}", target_os_string),
        Err(_) => panic!()
    };

    let folder = format!("t2-output/{}-{}-default", platform_toolset, profile);
    (path_relative_to_cargo_manifest_dir(&folder), "musashi".to_string())
}

fn generate_musashi_bindings() {
    
    let bindings = bindgen::Builder::default()
        .header("musashi/m68k.h")
        .blacklist_function("m68k_read_memory_8")
        .blacklist_function("m68k_read_memory_16")
        .blacklist_function("m68k_read_memory_32")
        .blacklist_function("m68k_write_memory_8")
        .blacklist_function("m68k_write_memory_16")
        .blacklist_function("m68k_write_memory_32")
        .blacklist_function("m68k_instruction_callback")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("musashi.bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn generate_musashi_rust_wrapper_bindings() {
    
    let bindings = bindgen::Builder::default()
        .header("musashi/musashi_rust_wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("musashi_rust_wrapper.bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    let (musashi_dir, musashi_name) = musashi_lib();
    println!("cargo:rustc-link-search={}", musashi_dir);
    println!("cargo:rustc-link-lib={}", musashi_name);

    generate_musashi_bindings();
    generate_musashi_rust_wrapper_bindings();
}