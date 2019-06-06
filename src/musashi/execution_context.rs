#![allow(nonstandard_style)]
#![allow(dead_code)]

use std::ptr;

include!(concat!(env!("OUT_DIR"), "/musashi.bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/musashi_rust_wrapper.bindings.rs"));

pub struct ExecutionContext<'a> {
    pub memory: &'a mut Vec<u8>,
}

impl<'a> ExecutionContext<'a> {

    fn read_memory_8(&self, address: u32) -> u8 {
        self.memory[address as usize]
    }

    fn read_memory_16(&self, address: u32) -> u16 {
        ((self.read_memory_8(address + 0) as u16) << 8)
        | (self.read_memory_8(address + 1) as u16)
    }

    fn read_memory_32(&self, address: u32) -> u32 {
        ((self.read_memory_8(address + 0) as u32) << 24)
        | ((self.read_memory_8(address + 1) as u32) << 16)
        | ((self.read_memory_8(address + 2) as u32) << 8)
        | (self.read_memory_8(address + 3) as u32)
    }

    fn write_memory_8(&mut self, address: u32, value: u8) {
        self.memory[address as usize] = value;
    }

    fn write_memory_16(&mut self, address: u32, value: u16) {
        self.write_memory_8(address + 0, (value >> 8) as u8);
        self.write_memory_8(address + 1, value as u8);
    }

    fn write_memory_32(&mut self, address: u32, value: u32) {
        self.write_memory_8(address + 0, (value >> 24) as u8);
        self.write_memory_8(address + 1, (value >> 16) as u8);
        self.write_memory_8(address + 2, (value >> 8) as u8);
        self.write_memory_8(address + 3, value as u8);
    }

    pub fn new(memory: &mut Vec<u8>) -> ExecutionContext {
        ExecutionContext {
            memory: memory,
        }
    }
}

#[no_mangle]
extern fn rust_m68k_read_memory_8(execution_context: *mut ExecutionContext, address: u32) -> RustM68KReadResult {
    unsafe {
        RustM68KReadResult { continue_simulation: true, value: (*execution_context).read_memory_8(address) as u32 }
    }
}

#[no_mangle]
extern fn rust_m68k_read_memory_16(execution_context: *mut ExecutionContext, address: u32) -> RustM68KReadResult {
    unsafe {
        RustM68KReadResult { continue_simulation: true, value: (*execution_context).read_memory_16(address) as u32 }
    }
}

#[no_mangle]
extern fn rust_m68k_read_memory_32(execution_context: *mut ExecutionContext, address: u32) -> RustM68KReadResult {
    unsafe {
        RustM68KReadResult { continue_simulation: true, value: (*execution_context).read_memory_32(address) as u32 }
    }
}

#[no_mangle]
extern fn rust_m68k_write_memory_8(execution_context: *mut ExecutionContext, address: u32, value: u32) -> RustM68KWriteResult {
    unsafe {
        (*execution_context).write_memory_8(address, value as u8);
        RustM68KWriteResult { continue_simulation: true }
    }
}

#[no_mangle]
extern fn rust_m68k_write_memory_16(execution_context: *mut ExecutionContext, address: u32, value: u32) -> RustM68KWriteResult {
    unsafe {
        (*execution_context).write_memory_16(address, value as u16);
        RustM68KWriteResult { continue_simulation: true }
    }
}

#[no_mangle]
extern fn rust_m68k_write_memory_32(execution_context: *mut ExecutionContext, address: u32, value: u32) -> RustM68KWriteResult {
    unsafe {
        (*execution_context).write_memory_32(address, value as u32);
        RustM68KWriteResult { continue_simulation: true }
    }
}

#[no_mangle]
extern fn rust_m68k_instruction_hook(_execution_context: *mut ExecutionContext) -> RustM68KInstructionHookResult {
    unsafe {
        let pc = m68k_get_reg(ptr::null_mut(), m68k_register_t_M68K_REG_PC);

        if pc == 0xf0fff0u32 {
            println!("End of test invoked");
            RustM68KInstructionHookResult { continue_simulation: false }
        } else {
            RustM68KInstructionHookResult { continue_simulation: true }
        }
    }
}

