
#![allow(nonstandard_style)]
#![allow(dead_code)]

use super::execution_context::ExecutionContext;
use super::musashi_core_lock::MUSASHI_CORE_LOCK;
use super::simulation_event::SimulationEvent;

include!(concat!(env!("OUT_DIR"), "/musashi.bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/musashi_rust_wrapper.bindings.rs"));

pub struct Context {
    pub memory: Vec<u8>,
    pub emulation_state: Vec<u8>,
}

impl Context {

    pub fn read_memory_8(&self, address: u32) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_memory_16(&self, address: u32) -> u16 {
        ((self.read_memory_8(address + 0) as u16) << 8)
        | (self.read_memory_8(address + 1) as u16)
    }

    pub fn read_memory_32(&self, address: u32) -> u32 {
        ((self.read_memory_8(address + 0) as u32) << 24)
        | ((self.read_memory_8(address + 1) as u32) << 16)
        | ((self.read_memory_8(address + 2) as u32) << 8)
        | (self.read_memory_8(address + 3) as u32)
    }

    pub fn write_memory_8(&mut self, address: u32, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn write_memory_16(&mut self, address: u32, value: u16) {
        self.write_memory_8(address + 0, (value >> 8) as u8);
        self.write_memory_8(address + 1, value as u8);
    }

    pub fn write_memory_32(&mut self, address: u32, value: u32) {
        self.write_memory_8(address + 0, (value >> 24) as u8);
        self.write_memory_8(address + 1, (value >> 16) as u8);
        self.write_memory_8(address + 2, (value >> 8) as u8);
        self.write_memory_8(address + 3, value as u8);
    }

    pub fn new() -> Context {
        let mut newContext = Context {
            memory: vec!(0u8; 1024*1024),
            emulation_state: vec!(0u8; unsafe { m68k_context_size() } as usize),
        };

        newContext.init_emulation_state();
        newContext
    }

    fn init_emulation_state(&mut self) {

        let _musashi_core_lock_acquired = MUSASHI_CORE_LOCK.lock();

        unsafe {
            m68k_init();
            m68k_set_cpu_type(M68K_CPU_TYPE_68000 as u32);
            m68k_get_context(self.emulation_state.as_mut_ptr() as *mut std::ffi::c_void);
        }
    }

    pub fn run(&mut self, cycles: i32) -> (bool, Vec<SimulationEvent>) {

        let mut execution_context = ExecutionContext::new(&self.memory);

        let _musashi_core_lock_acquired = MUSASHI_CORE_LOCK.lock();

        unsafe {
            m68k_set_context(self.emulation_state.as_mut_ptr() as *mut std::ffi::c_void);
            let (success, events) = execution_context.run(cycles);
            m68k_get_context(self.emulation_state.as_mut_ptr() as *mut std::ffi::c_void);
            (success, events)
        }
    }

    pub fn read_register(&self, reg: m68k_register_t) -> u32 {
        unsafe {
            m68k_get_reg(self.emulation_state.as_ptr() as *mut std::ffi::c_void, reg)
        }
    }
}

#[test]
fn run_musashi() {

    let mut ctx = Context::new();

    ctx.write_memory_32(0, 0xf000);
    ctx.write_memory_32(4, 0x1000);

    ctx.write_memory_16(0x1000, 0x7005);   // MOVEQ #5,d0
    ctx.write_memory_16(0x1002, 0x4eb9);   // JSR $f0fff0
    ctx.write_memory_32(0x1004, 0xf0fff0); // <address>

    let (success, events) = ctx.run(1024);

    assert_eq!(true, success);
    assert_eq!(events, vec!(SimulationEvent::Passed { registers: None }));
}