
pub struct MemorySubSystem {
    pub memory: Vec<u8>,
}

impl MemorySubSystem {

    pub fn read_memory_8(&self, address: u32) -> Option<u8> {
        if address < (self.memory.len() as u32) {
            Some(self.memory[address as usize])
        } else {
            None
        }
    }

    pub fn read_memory_16(&self, address: u32) -> Option<u16> {
        if address < (self.memory.len() as u32) && (address + 1) < (self.memory.len() as u32) {
            Some(((self.memory[(address + 0) as usize] as u16) << 8)
                | (self.memory[(address + 1) as usize] as u16))
        } else {
            None
        }
    }

    pub fn read_memory_32(&self, address: u32) -> Option<u32> {
        if address < (self.memory.len() as u32) && (address + 3) < (self.memory.len() as u32) {
            Some(((self.memory[(address + 0) as usize] as u32) << 24)
                | ((self.memory[(address + 1) as usize] as u32) << 16)
                | ((self.memory[(address + 2) as usize] as u32) << 8)
                | (self.memory[(address + 3) as usize] as u32))
        } else {
            None
        }
    }

    pub fn write_memory_8(&mut self, address: u32, value: u8) -> bool {
        if address < (self.memory.len() as u32) {
            self.memory[address as usize] = value;
            true
        } else {
            false
        }
    }

    pub fn write_memory_16(&mut self, address: u32, value: u16) -> bool {
        if address < (self.memory.len() as u32) && (address + 1) < (self.memory.len() as u32) {
            self.memory[(address + 0) as usize] = (value >> 8) as u8;
            self.memory[(address + 1) as usize] = value as u8;
            true
        } else {
            false
        }
    }

    pub fn write_memory_32(&mut self, address: u32, value: u32) -> bool {
        if address < (self.memory.len() as u32) && (address + 3) < (self.memory.len() as u32) {
            self.memory[(address + 0) as usize] = (value >> 24) as u8;
            self.memory[(address + 1) as usize] = (value >> 16) as u8;
            self.memory[(address + 2) as usize] = (value >> 8) as u8;
            self.memory[(address + 3) as usize] = value as u8;
            true
        } else {
            false
        }
    }

    pub fn new(original_memory: &Vec<u8>) -> MemorySubSystem {
        MemorySubSystem { memory: original_memory.to_vec() }
    }
}

#[test]
fn test_reads_from_memory_subsystem() {

    let mut mem = vec!(0u8; 1024);
    mem[3] = 4;
    mem[20] = 0x20;
    mem[21] = 0xc0;
    mem[373] = 0x6c;
    mem[374] = 0x12;
    mem[375] = 0x34;
    mem[376] = 0x56;
    mem[377] = 0x78;
    mem[378] = 0x9a;

    let memory = MemorySubSystem::new(&mem);

    assert_eq!(Some(4u8), memory.read_memory_8(3));
    assert_eq!(Some(0x20c0u16), memory.read_memory_16(20));
    assert_eq!(Some(0x12345678u32), memory.read_memory_32(374));

    assert_eq!(None, memory.read_memory_8(10003));
    assert_eq!(None, memory.read_memory_16(10020));
    assert_eq!(None, memory.read_memory_32(100374));

    assert_eq!(None, memory.read_memory_8(0xffffffffu32));
    assert_eq!(None, memory.read_memory_16(0xfffffffeu32));
    assert_eq!(None, memory.read_memory_32(0xfffffffeu32));
}

#[test]
fn test_writes_to_memory_subsystem() {

    let mut mem = vec!(0u8; 1024);
    mem[3] = 4;
    mem[20] = 0x20;
    mem[21] = 0xc0;
    mem[373] = 0x6c;
    mem[374] = 0x12;
    mem[375] = 0x34;
    mem[376] = 0x56;
    mem[377] = 0x78;
    mem[378] = 0x9a;

    let mut memory = MemorySubSystem::new(&mem);

    assert_eq!(true, memory.write_memory_8(3, 7));
    assert_eq!(Some(7u8), memory.read_memory_8(3));
    assert_eq!(Some(7u16), memory.read_memory_16(2));

    assert_eq!(true, memory.write_memory_16(20, 0x1234u16));
    assert_eq!(Some(0x1234u16), memory.read_memory_16(20));

    assert_eq!(true, memory.write_memory_32(370, 0x31517191u32));
    assert_eq!(Some(0x31517191u32), memory.read_memory_32(370));
    assert_eq!(Some(0x12345678u32), memory.read_memory_32(374));

    assert_eq!(false, memory.write_memory_8(10003, 0u8));
    assert_eq!(false, memory.write_memory_16(10020, 0u16));
    assert_eq!(false, memory.write_memory_32(100374, 0u32));

    assert_eq!(false, memory.write_memory_8(0xffffffffu32, 0u8));
    assert_eq!(false, memory.write_memory_16(0xfffffffeu32, 0u16));
    assert_eq!(false, memory.write_memory_32(0xfffffffeu32, 0u32));
}
