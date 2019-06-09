#![allow(nonstandard_style)]
#![allow(dead_code)]

use std::ptr;

use super::simulation_event::SimulationEvent;
use super::memory_subsystem::MemorySubSystem;

include!(concat!(env!("OUT_DIR"), "/musashi.bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/musashi_rust_wrapper.bindings.rs"));

pub struct ExecutionContext {
    pub memory: MemorySubSystem,
    pub success: Option<bool>,
    pub events: Vec<SimulationEvent>,
}

impl ExecutionContext {

    fn read_memory_8(&self, address: u32) -> Option<u8> {
        self.memory.read_memory_8(address)
    }

    fn read_memory_16(&self, address: u32) -> Option<u16> {
        self.memory.read_memory_16(address)
    }

    fn read_memory_32(&self, address: u32) -> Option<u32> {
        self.memory.read_memory_32(address)
    }

    fn write_memory_8(&mut self, address: u32, value: u8) -> bool {
        self.memory.write_memory_8(address, value)
    }

    fn write_memory_16(&mut self, address: u32, value: u16) -> bool {
        self.memory.write_memory_16(address, value)
    }

    fn write_memory_32(&mut self, address: u32, value: u32) -> bool {
        self.memory.write_memory_32(address, value)
    }

    pub fn new(memory: &Vec<u8>) -> ExecutionContext {
        ExecutionContext {
            memory: MemorySubSystem::new(memory),
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
        let result = (*execution_context).read_memory_8(address);
        match result {
            Some(value) => RustM68KReadResult { continue_simulation: true, value: value as u32 },
            None => {
                (*execution_context).events.push( SimulationEvent::BusError );
                (*execution_context).success = Some(false);
                RustM68KReadResult { continue_simulation: false, value: 0u32 }
            }
        }
    }
}

#[no_mangle]
extern fn rust_m68k_read_memory_16(execution_context: *mut ExecutionContext, address: u32) -> RustM68KReadResult {
    unsafe {
        let result = (*execution_context).read_memory_16(address);
        match result {
            Some(value) => RustM68KReadResult { continue_simulation: true, value: value as u32 },
            None => {
                (*execution_context).events.push( SimulationEvent::BusError );
                (*execution_context).success = Some(false);
                RustM68KReadResult { continue_simulation: false, value: 0u32 }
            }
        }
    }
}

#[no_mangle]
extern fn rust_m68k_read_memory_32(execution_context: *mut ExecutionContext, address: u32) -> RustM68KReadResult {
    unsafe {
        let result = (*execution_context).read_memory_32(address);
        match result {
            Some(value) => RustM68KReadResult { continue_simulation: true, value: value as u32 },
            None => {
                (*execution_context).events.push( SimulationEvent::BusError );
                (*execution_context).success = Some(false);
                RustM68KReadResult { continue_simulation: false, value: 0u32 }
            }
        }
    }
}

#[no_mangle]
extern fn rust_m68k_write_memory_8(execution_context: *mut ExecutionContext, address: u32, value: u32) -> RustM68KWriteResult {
    unsafe {
        let result = (*execution_context).write_memory_8(address, value as u8);
        if result {
            RustM68KWriteResult { continue_simulation: true }
        } else {
            (*execution_context).events.push( SimulationEvent::BusError );
            (*execution_context).success = Some(false);
            RustM68KWriteResult { continue_simulation: false }
        }
    }
}

#[no_mangle]
extern fn rust_m68k_write_memory_16(execution_context: *mut ExecutionContext, address: u32, value: u32) -> RustM68KWriteResult {
    unsafe {
        let result = (*execution_context).write_memory_16(address, value as u16);
        if result {
            RustM68KWriteResult { continue_simulation: true }
        } else {
            (*execution_context).events.push( SimulationEvent::BusError );
            (*execution_context).success = Some(false);
            RustM68KWriteResult { continue_simulation: false }
        }
    }
}

#[no_mangle]
extern fn rust_m68k_write_memory_32(execution_context: *mut ExecutionContext, address: u32, value: u32) -> RustM68KWriteResult {
    unsafe {
        let result = (*execution_context).write_memory_32(address, value as u32);
        if result {
            RustM68KWriteResult { continue_simulation: true }
        } else {
            (*execution_context).events.push( SimulationEvent::BusError );
            (*execution_context).success = Some(false);
            RustM68KWriteResult { continue_simulation: false }
        }
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

#[no_mangle]
extern fn rust_m68k_exception_privilege_violation_hook(execution_context: *mut ExecutionContext) -> RustM68KInstructionHookResult {
    unsafe {
        (*execution_context).events.push(SimulationEvent::PrivilegeViolation);
        (*execution_context).success = Some(false);
        RustM68KInstructionHookResult { continue_simulation: false }
    }
}

#[no_mangle]
extern fn rust_m68k_exception_1010_hook(execution_context: *mut ExecutionContext) -> RustM68KInstructionHookResult {
    unsafe {
        (*execution_context).events.push(SimulationEvent::LineAException);
        (*execution_context).success = Some(false);
        RustM68KInstructionHookResult { continue_simulation: false }
    }
}

#[no_mangle]
extern fn rust_m68k_exception_1111_hook(execution_context: *mut ExecutionContext) -> RustM68KInstructionHookResult {
    unsafe {
        (*execution_context).events.push(SimulationEvent::LineFException);
        (*execution_context).success = Some(false);
        RustM68KInstructionHookResult { continue_simulation: false }
    }
}

#[no_mangle]
extern fn rust_m68k_exception_address_error_hook(execution_context: *mut ExecutionContext, address: u32, write: bool, function_code: u32) -> RustM68KInstructionHookResult {
    unsafe {
        (*execution_context).events.push(SimulationEvent::AddressError { address: address, write: write, function_code: function_code });
        (*execution_context).success = Some(false);
        RustM68KInstructionHookResult { continue_simulation: false }
    }
}
