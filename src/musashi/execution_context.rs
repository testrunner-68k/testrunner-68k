#![allow(nonstandard_style)]
#![allow(dead_code)]

use std::ptr;

use super::simulation_event::SimulationEvent;

include!(concat!(env!("OUT_DIR"), "/musashi.bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/musashi_rust_wrapper.bindings.rs"));

pub struct ExecutionContext<'a> {
    pub memory: &'a mut Vec<u8>,
    pub success: Option<bool>,
    pub events: Vec<SimulationEvent>,
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
            success: None,
            events: Vec::new(),
        }
    }

    pub fn run(&mut self, cycles: i32) -> (bool, Vec<SimulationEvent>) {

        unsafe {
            wrapped_m68k_pulse_reset(self as *mut ExecutionContext as *mut std::ffi::c_void);
            let _cycles_used = wrapped_m68k_execute(self as *mut ExecutionContext as *mut std::ffi::c_void, cycles);

            if self.success == None {
                self.events.push(SimulationEvent::TimedOut);
                self.success = Some(false);
            }

            (self.success.unwrap(), self.events.to_vec())
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
extern fn rust_m68k_instruction_hook(execution_context: *mut ExecutionContext) -> RustM68KInstructionHookResult {
    unsafe {
        let pc = m68k_get_reg(ptr::null_mut(), m68k_register_t_M68K_REG_PC);

        if pc == 0xf0fff0u32 {
            let d0 = m68k_get_reg(ptr::null_mut(), m68k_register_t_M68K_REG_D0);
            let success = d0 != 0;
            (*execution_context).events.push( if success { SimulationEvent::Passed } else { SimulationEvent::Failed } );
            (*execution_context).success = Some(success);
            RustM68KInstructionHookResult { continue_simulation: false }
        } else {
            RustM68KInstructionHookResult { continue_simulation: true }
        }
    }
}

#[no_mangle]
extern fn rust_m68k_exception_illegal_hook(execution_context: *mut ExecutionContext) -> RustM68KInstructionHookResult {
    unsafe {
        (*execution_context).events.push(SimulationEvent::IllegalInstruction);
        (*execution_context).success = Some(false);
        RustM68KInstructionHookResult { continue_simulation: false }
    }
}
