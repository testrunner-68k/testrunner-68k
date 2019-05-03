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

fn read_memory_8(address: u32) -> u32 {
    memory.lock().unwrap()[address as usize] as u32
}

fn read_memory_16(address: u32) -> u32 {
    (read_memory_8(address + 0) << 8)
    | read_memory_8(address + 1)
}

fn read_memory_32(address: u32) -> u32 {
    (read_memory_8(address + 0) << 24)
    | (read_memory_8(address + 1) << 16)
    | (read_memory_8(address + 2) << 8)
    | read_memory_8(address + 3)
}

fn write_memory_8(address: u32, value: u32) {
    memory.lock().unwrap()[address as usize] = value as u8
}

fn write_memory_16(address: u32, value: u32) {
    write_memory_8(address + 0, value >> 8);
    write_memory_8(address + 1, value);
}

fn write_memory_32(address: u32, value: u32) {
    write_memory_8(address + 0, value >> 24);
    write_memory_8(address + 1, value >> 16);
    write_memory_8(address + 2, value >> 8);
    write_memory_8(address + 3, value);
}

#[no_mangle]
pub extern fn m68k_read_memory_8(address: u32) -> u32 {
    read_memory_8(address)
}

#[no_mangle]
pub extern fn m68k_read_memory_16(address: u32) -> u32 {
    read_memory_16(address)
}

#[no_mangle]
pub extern fn m68k_read_memory_32(address: u32) -> u32 {
    read_memory_32(address)
}

#[no_mangle]
pub extern fn m68k_write_memory_8(address: u32, value: u32) {
    write_memory_8(address, value);
}

#[no_mangle]
pub extern fn m68k_write_memory_16(address: u32, value: u32) {
    write_memory_16(address, value);
}

#[no_mangle]
pub extern fn m68k_write_memory_32(address: u32, value: u32) {
    write_memory_32(address, value);
}

// Compute start address for each hunk
pub fn layout_hunks(hunks: &Vec<Hunk>, start_address: u32) -> Vec<u32> {

    let mut layout_hunks = Vec::new();

    let mut hunk_start_address = start_address;

    for hunk_index in 0..hunks.len() {

        let hunk = &hunks[hunk_index];
        layout_hunks.push(hunk_start_address);
        hunk_start_address = ((hunk_start_address + (hunk.alloc_size as u32)) + 3) & 0xfffffffc;
    }

    return layout_hunks;
}

fn initialize_emulator_memory(memory_size: u32) {
    memory.lock().unwrap().resize(0, 0);
    memory.lock().unwrap().resize(memory_size as usize, 0);
}

fn load_hunk_into_emulator_memory(hunk: &Hunk, hunk_start_address: u32) {
    if !hunk.code_data.is_none() {
        let code_data = &hunk.code_data.as_ref().unwrap();
        for offset in 0..code_data.len() {
            write_memory_8(hunk_start_address + (offset as u32), code_data[offset] as u32);
        }
    }
}

fn load_hunks_into_emulator_memory(hunks: &Vec<Hunk>, hunk_layout: &Vec<u32>) {
    for i in 0..hunks.len() {
        let hunk = &hunks[i];
        let hunk_start_address = hunk_layout[i];
        load_hunk_into_emulator_memory(&hunk, hunk_start_address);
    }
}

fn get_function_start_address(hunks: &Vec<Hunk>, hunk_layout: &Vec<u32>, test_case_name: &String) -> u32{
    for i in 0..hunks.len() {
        let hunk = &hunks[i];
        if !hunk.symbols.is_none() {
            for symbol in hunk.symbols.as_ref().unwrap().iter() {
                if symbol.name == *test_case_name {
                    return hunk_layout[i] + symbol.offset;
                }
            }
        }
    }

    panic!("Symbol {} not found", test_case_name);
}

fn setup_emulator_init_and_trampoline(stack_ptr: u32, program_done_ptr: u32, test_function_start: u32) {
    write_memory_16(program_done_ptr, 0x60fe);           // BRA.S *
    write_memory_32(stack_ptr, program_done_ptr);
    write_memory_32(0, stack_ptr);
    write_memory_32(4, test_function_start);
}

fn run_emulator_test() {

    unsafe {
        m68k_init();
        m68k_set_cpu_type(M68K_CPU_TYPE_68000 as u32);
        m68k_pulse_reset();
        m68k_execute(1024);
    }
}

fn clear_emulator_test_result() {
}

fn get_emulator_test_result(test_case_name: &String) -> TestResult {
    unsafe {
        let d0 = m68k_get_reg(ptr::null_mut(), m68k_register_t_M68K_REG_D0);
        TestResult { name: test_case_name.clone(), success: d0 != 0 }
    }
}

pub fn run_test_case(hunks: &Vec<Hunk>, test_case: &TestCase) -> TestResult {

    let memory_size = (1024 * 1024) as u32;
    let stack_size = 4096u32;

    let memory_area_start = 1024u32;
    let _memory_area_end = memory_size - stack_size;

    let program_done_ptr = memory_size - 16;
    let stack_ptr = program_done_ptr - 4;

    let hunk_layout = layout_hunks(&hunks, memory_area_start);

    clear_emulator_test_result();
    initialize_emulator_memory(memory_size);
    load_hunks_into_emulator_memory(&hunks, &hunk_layout);
    let test_function_start = get_function_start_address(&hunks, &hunk_layout, &test_case.name);
    setup_emulator_init_and_trampoline(stack_ptr, program_done_ptr, test_function_start);
    run_emulator_test();
    get_emulator_test_result(&test_case.name)
}

pub fn run_test_cases(hunks: &Vec<Hunk>, test_cases: &Vec<TestCase>) -> Vec<TestResult> {

    let mut test_results: Vec<TestResult> = Vec::new();

    for test_case in test_cases {
        test_results.push(run_test_case(&hunks, &test_case));
    }

    test_results
}

#[test]
use serial_test_derive::serial;

#[test]
#[serial]
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
        assert_eq!(5u32, d0);
    }
}

#[test]
use amiga_hunk_parser::HunkParser;

#[test]
#[serial]
fn run_successful_test() {
    let hunks = HunkParser::parse_file("testdata/test.successful_test_case.amiga.exe").unwrap();
    let test_case = TestCase { name: "test_TestModule_successfulCase".to_string() };
    let test_result = run_test_case(&hunks, &test_case);
    assert_eq!(true, test_result.success)
}

#[test]
#[serial]
fn run_failed_test() {
    let hunks = HunkParser::parse_file("testdata/test.failed_test_case.amiga.exe").unwrap();
    let test_case = TestCase { name: "test_TestModule_failedCase".to_string() };
    let test_result = run_test_case(&hunks, &test_case);
    assert_eq!(false, test_result.success)
}