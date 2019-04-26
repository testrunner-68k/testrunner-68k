#![allow(nonstandard_style)]
#![allow(dead_code)]

use std::ptr;
use std::sync::Mutex;

use amiga_hunk_parser::Hunk;

use super::testcases::{TestCase, TestResult};

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

// Compute start address for each hunk
pub fn layout_hunks(hunks: &Vec<Hunk>) -> Vec<u32> {

    let mut layout_hunks = Vec::new();

    let mut start_address = 0u32;

    for hunk_index in 0..hunks.len() {

        let hunk = &hunks[hunk_index];
        layout_hunks.push(start_address);
        start_address = ((start_address + (hunk.alloc_size as u32)) + 3) & 0xfffffffc;
    }

    dbg!(&layout_hunks);

    return layout_hunks;
}

pub fn run_test_case(_hunks: &Vec<Hunk>, _test_case: &TestCase) -> TestResult {
    TestResult { name: "hello".to_string(), success: true }
}

pub fn run_test_cases(hunks: &Vec<Hunk>, test_cases: &Vec<TestCase>) -> Vec<TestResult> {

    let test_results: Vec<TestResult> = Vec::new();

    for test_case in test_cases {
        test_results.push(run_test_case(&hunks, &test_case));
    }

    test_results
}

#[test]
fn run_musashi() {
    unsafe {
        m68k_init();
        m68k_set_cpu_type(M68K_CPU_TYPE_68000 as u32);

        m68k_write_memory_32(0, 0xf000);
        m68k_write_memory_32(4, 0x1000);

        m68k_write_memory_16(0x1000, 0x7005);   // MOVEQ #5,d0
        m68k_write_memory_16(0x1002, 0x60fe);   // BRA.S *

        m68k_pulse_reset();
        m68k_execute(1024);

        let d0 = m68k_get_reg(ptr::null_mut(), m68k_register_t_M68K_REG_D0);
        dbg!(&d0);
        assert_eq!(5u32, d0);
    }
}
