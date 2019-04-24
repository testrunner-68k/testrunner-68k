#![allow(nonstandard_style)]
#![allow(dead_code)]

use std::ptr;
use std::sync::Mutex;

include!(concat!(env!("OUT_DIR"), "/musashi.bindings.rs"));

lazy_static! {
    static ref memory: Mutex<Vec<u8>> = Mutex::new(vec!(0u8; 1024*1024));
}

fn read_memory_byte(address: u32) -> u32 {
    memory.lock().unwrap()[address as usize] as u32
}

fn write_memory_byte(address: u32, value: u32) {
    memory.lock().unwrap()[address as usize] = value as u8
}

#[no_mangle]
pub extern fn m68k_read_memory_8(address: u32) -> u32 {
    read_memory_byte(address)
}

#[no_mangle]
pub extern fn m68k_read_memory_16(address: u32) -> u32 {
    (read_memory_byte(address + 0) << 8)
    | read_memory_byte(address + 1)
}

#[no_mangle]
pub extern fn m68k_read_memory_32(address: u32) -> u32 {
    (read_memory_byte(address + 0) << 24)
    | (read_memory_byte(address + 1) << 16)
    | (read_memory_byte(address + 2) << 8)
    | read_memory_byte(address + 3)
}

#[no_mangle]
pub extern fn m68k_write_memory_8(address: u32, value: u32) {
    write_memory_byte(address + 0, value);
}

#[no_mangle]
pub extern fn m68k_write_memory_16(address: u32, value: u32) {
    write_memory_byte(address + 0, value >> 8);
    write_memory_byte(address + 1, value);
}

#[no_mangle]
pub extern fn m68k_write_memory_32(address: u32, value: u32) {
    write_memory_byte(address + 0, value >> 24);
    write_memory_byte(address + 1, value >> 16);
    write_memory_byte(address + 2, value >> 8);
    write_memory_byte(address + 3, value);
}

#[test]
fn run_musashi() {
    unsafe {
        m68k_init();
        m68k_write_memory_32(0, 0xf000);
        m68k_write_memory_32(4, 0x1000);

        m68k_write_memory_16(0x1000, 0x7005);   // MOVEQ #5,d0
        m68k_write_memory_16(0x1002, 0x60fe);   // BRA.S *

        m68k_pulse_reset();
        m68k_execute(1000); // Test failure during execution

        // // let mut context: Vec<u8> = Vec::new();
        // // context.reserve(m68k_context_size() as usize);
        // // m68k_get_context(&context);

        // let d0 = m68k_get_reg(ptr::null_mut(), m68k_register_t_M68K_REG_D0);
        // dbg!(&d0);
        // //assert_eq!(5u32, d0);
    }
}
