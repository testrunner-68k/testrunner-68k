
#![allow(nonstandard_style)]
#![allow(dead_code)]

use std::ptr;
use std::sync::Mutex;

include!(concat!(env!("OUT_DIR"), "/musashi.bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/musashi_rust_wrapper.bindings.rs"));

pub struct Context {
    pub memory: Vec<u8>,
    pub emulation_state: Vec<u8>,
}

lazy_static! {
    static ref musashi_core_lock: Mutex<bool> = Mutex::new(true);
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

        newContext.init();
        newContext
    }

    pub fn init(&mut self) {

        let _musashi_core_lock_acquired = musashi_core_lock.lock();

        unsafe {
            m68k_init();
            m68k_set_cpu_type(M68K_CPU_TYPE_68000 as u32);
            m68k_get_context(self.emulation_state.as_mut_ptr() as *mut std::ffi::c_void);
        }
    }

    pub fn reset(&mut self) {

        let _musashi_core_lock_acquired = musashi_core_lock.lock();

        unsafe {
            m68k_set_context(self.emulation_state.as_mut_ptr() as *mut std::ffi::c_void);
            wrapped_m68k_pulse_reset(self as *mut Context as *mut std::ffi::c_void);
            m68k_get_context(self.emulation_state.as_mut_ptr() as *mut std::ffi::c_void);
        }
    }

    pub fn run(&mut self, cycles: i32) {

        let _musashi_core_lock_acquired = musashi_core_lock.lock();

        unsafe {
            m68k_set_context(self.emulation_state.as_mut_ptr() as *mut std::ffi::c_void);
            let cycles_used = wrapped_m68k_execute(self as *mut Context as *mut std::ffi::c_void, cycles);
            println!("cycles used: {}", cycles_used);
            m68k_get_context(self.emulation_state.as_mut_ptr() as *mut std::ffi::c_void);
        }
    }

    pub fn read_register(&self, reg: m68k_register_t) -> u32 {
        unsafe {
            m68k_get_reg(self.emulation_state.as_ptr() as *mut std::ffi::c_void, reg as u32)
        }
    }
}

#[no_mangle]
pub extern fn rust_m68k_read_memory_8(context: *mut Context, address: u32) -> RustM68KReadResult {
    unsafe {
        RustM68KReadResult { success: true, value: (*context).read_memory_8(address) as u32 }
    }
}

#[no_mangle]
pub extern fn rust_m68k_read_memory_16(context: *mut Context, address: u32) -> RustM68KReadResult {
    unsafe {
        RustM68KReadResult { success: true, value: (*context).read_memory_16(address) as u32 }
    }
}

#[no_mangle]
pub extern fn rust_m68k_read_memory_32(context: *mut Context, address: u32) -> RustM68KReadResult {
    unsafe {
        RustM68KReadResult { success: true, value: (*context).read_memory_32(address) as u32 }
    }
}

#[no_mangle]
pub extern fn rust_m68k_write_memory_8(context: *mut Context, address: u32, value: u32) -> RustM68KWriteResult {
    unsafe {
        (*context).write_memory_8(address, value as u8);
        RustM68KWriteResult { success: true }
    }
}

#[no_mangle]
pub extern fn rust_m68k_write_memory_16(context: *mut Context, address: u32, value: u32) -> RustM68KWriteResult {
    unsafe {
        (*context).write_memory_16(address, value as u16);
        RustM68KWriteResult { success: true }
    }
}

#[no_mangle]
pub extern fn rust_m68k_write_memory_32(context: *mut Context, address: u32, value: u32) -> RustM68KWriteResult {
    unsafe {
        (*context).write_memory_32(address, value as u32);
        RustM68KWriteResult { success: true }
    }
}

#[no_mangle]
pub extern fn rust_m68k_instruction_hook(_context: *mut Context) -> RustM68KInstructionHookResult {
    unsafe {
        let pc = m68k_get_reg(ptr::null_mut(), m68k_register_t_M68K_REG_PC);

        if pc == 0xf0fff0u32 {
            println!("End of test invoked");
            RustM68KInstructionHookResult { success: false }
        } else {
            RustM68KInstructionHookResult { success: true }
        }
    }
}

#[test]
fn run_musashi() {

    let mut ctx = Context::new();

    ctx.write_memory_32(0, 0xf000);
    ctx.write_memory_32(4, 0x1000);

    ctx.write_memory_16(0x1000, 0x7005);   // MOVEQ #5,d0
    ctx.write_memory_16(0x1002, 0x4ef9);   // JUMP $f0fff0
    ctx.write_memory_32(0x1004, 0xf0fff0); // <address>

    ctx.reset();
    ctx.run(1024);

    let d0 = ctx.read_register(m68k_register_t_M68K_REG_D0);
    assert_eq!(5u32, d0);
}