#![allow(nonstandard_style)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/musashi.bindings.rs"));

#[no_mangle]
pub extern fn m68k_read_memory_8(address: u32) -> u32 {
    0
}

#[no_mangle]
pub extern fn m68k_read_memory_16(address: u32) -> u32 {
    0
}

#[no_mangle]
pub extern fn m68k_read_memory_32(address: u32) -> u32 {
    0
}

#[no_mangle]
pub extern fn m68k_write_memory_8(address: u32, value: u32) {
}

#[no_mangle]
pub extern fn m68k_write_memory_16(address: u32, value: u32) {
}

#[no_mangle]
pub extern fn m68k_write_memory_32(address: u32, value: u32) {
}


#[test]
fn start_musashi() {
    unsafe { m68k_init(); }
}

