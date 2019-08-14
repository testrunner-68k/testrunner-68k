use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum OperationSize {
    Byte,
    Word,
    LongWord
}

impl fmt::Display for OperationSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn fmt_sr_bitflags(sr: u16) -> String {
    format!("T={} S={} IPL={} X={} N={} Z={} V={} C={}", (sr >> 15) & 1, (sr >> 13) & 1, (sr >> 8) & 7, (sr >> 4) & 1, (sr >> 3) & 1, (sr >> 2) & 1, (sr >> 1) & 1, (sr >> 0) & 1)
}

fn fmt_sr(sr: u16) -> String {
    format!("SR = {:04X} [{}]", sr, fmt_sr_bitflags(sr))
}

fn fmt_dn(index: usize, value: u32) -> String {
    format!("D{} = {:08X}", index, value)
}

fn fmt_an(index: usize, value: u32) -> String {
    format!("A{} = {:08X}", index, value)
}

fn fmt_pc(value: u32) -> String {
    format!("PC = {:08X}", value)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Registers {
    pub dn : Vec<u32>,
    pub an : Vec<u32>,
    pub pc : u32,
    pub sr : u16
}

impl fmt::Display for Registers {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Registers:\n")?;
    
        for i in 0..8 {
            write!(f, "\t{}\n", fmt_dn(i, self.dn[i]))?;
        }
        for i in 0..8 {
            write!(f, "\t{}\n", fmt_an(i, self.an[i]))?;
        }

        write!(f, "\t{}\n", fmt_pc(self.pc))?;
        write!(f, "\t{}\n", fmt_sr(self.sr))?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SimulationEvent {
	Passed { registers: Option<Registers> },
	Failed { registers: Option<Registers> },
	TimedOut { registers: Option<Registers> },
    PrivilegeViolation { registers: Option<Registers> },
    LineAException { registers: Option<Registers> },
    LineFException { registers: Option<Registers> },
	IllegalInstruction { registers: Option<Registers> },
    AddressError { address: u32, write: bool, function_code: u32, registers: Option<Registers> },
    BusError { address: u32, write: bool, operation_size: OperationSize, registers: Option<Registers> },
    Print { message: String, registers: Option<Registers> },
}

fn write_registers(f: &mut fmt::Formatter, registers: &Option<Registers>) -> fmt::Result {
    match registers {
        Some(registers) => write!(f, "{}", registers),
        None => Ok(())
    }
}

impl fmt::Display for SimulationEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimulationEvent::Passed { registers } => { write!(f, "******************** Test passed ********************\n")?; write_registers(f, registers)?; Ok(()) }
            SimulationEvent::Failed { registers } => { write!(f, "******************** Test failed ********************\n")?; write_registers(f, registers)?; Ok(()) }
            SimulationEvent::TimedOut { registers } => { write!(f, "******************** Test timed out ********************\n")?; write_registers(f, registers)?; Ok(()) }
            SimulationEvent::PrivilegeViolation { registers } => { write!(f, "******************** Privilege violation ********************\n")?; write_registers(f, registers)?; Ok(()) }
            SimulationEvent::LineAException { registers } => { write!(f, "******************** Line-A exception ********************\n")?; write_registers(f, registers)?; Ok(()) }
            SimulationEvent::LineFException { registers } => { write!(f, "******************** Line-F exception ********************\n")?; write_registers(f, registers)?; Ok(()) }
            SimulationEvent::IllegalInstruction { registers } => { write!(f, "******************** Illegal instruction encountered ********************\n")?; write_registers(f, registers)?; Ok(()) }
            SimulationEvent::AddressError { address, write, function_code, registers } => { write!(f, "******************** Address error encountered ********************\nAccess address: 0x{:x}\nAccess type: {}\nFunction code: {}\n", address, if *write { "Write" } else { "Read" }, function_code)?; write_registers(f, registers)?; Ok(()) }
            SimulationEvent::BusError { address, write, operation_size, registers } => { write!(f, "******************** Bus error encountered ********************\nAccess address: 0x{:x}\nAccess type: {}\nSize: {}\n", address, if *write { "Write" } else { "Read" }, *operation_size)?; write_registers(f, registers)?; Ok(()) }
            SimulationEvent::Print { message, registers } => { write!(f, "{}\n", message.to_string())?; write_registers(f, registers)?; Ok(()) }
        }
    }
}

#[test]
fn test_fmt_sr_bitflags() {
    // Test individual bits
    assert_eq!("T=1 S=0 IPL=0 X=0 N=0 Z=0 V=0 C=0", fmt_sr_bitflags(0x8000));
    assert_eq!("T=0 S=1 IPL=0 X=0 N=0 Z=0 V=0 C=0", fmt_sr_bitflags(0x2000));
    assert_eq!("T=0 S=0 IPL=0 X=1 N=0 Z=0 V=0 C=0", fmt_sr_bitflags(0x0010));
    assert_eq!("T=0 S=0 IPL=0 X=0 N=1 Z=0 V=0 C=0", fmt_sr_bitflags(0x0008));
    assert_eq!("T=0 S=0 IPL=0 X=0 N=0 Z=1 V=0 C=0", fmt_sr_bitflags(0x0004));
    assert_eq!("T=0 S=0 IPL=0 X=0 N=0 Z=0 V=1 C=0", fmt_sr_bitflags(0x0002));
    assert_eq!("T=0 S=0 IPL=0 X=0 N=0 Z=0 V=0 C=1", fmt_sr_bitflags(0x0001));

    // Test IPL field
    assert_eq!("T=0 S=0 IPL=7 X=0 N=0 Z=0 V=0 C=0", fmt_sr_bitflags(0x0700));
    assert_eq!("T=0 S=0 IPL=5 X=0 N=0 Z=0 V=0 C=0", fmt_sr_bitflags(0x0500));
    assert_eq!("T=0 S=0 IPL=2 X=0 N=0 Z=0 V=0 C=0", fmt_sr_bitflags(0x0200));

    // Test all bits cleared
    assert_eq!("T=0 S=0 IPL=0 X=0 N=0 Z=0 V=0 C=0", fmt_sr_bitflags(0x0000));

    // Test all used bits cleared, all unused bits set
    assert_eq!("T=0 S=0 IPL=0 X=0 N=0 Z=0 V=0 C=0", fmt_sr_bitflags(0x58e0));

    // Test a typical value
    assert_eq!("T=0 S=1 IPL=7 X=0 N=0 Z=0 V=0 C=0", fmt_sr_bitflags(0x2700));
}

#[test]
fn test_fmt_sr() {
    assert_eq!("SR = 2700 [T=0 S=1 IPL=7 X=0 N=0 Z=0 V=0 C=0]", fmt_sr(0x2700));
}

#[test]
fn test_fmt_dn() {
    assert_eq!("D3 = FEDCBA98", fmt_dn(3, 0xfedcba98u32));
    assert_eq!("D0 = 00000001", fmt_dn(0, 1));
}

#[test]
fn test_fmt_an() {
    assert_eq!("A3 = FEDCBA98", fmt_an(3, 0xfedcba98u32));
    assert_eq!("A0 = 00000001", fmt_an(0, 1));
}

#[test]
fn test_fmt_pc() {
    assert_eq!("PC = FEDCBA98", fmt_pc(0xfedcba98u32));
    assert_eq!("PC = 00000001", fmt_pc(1));
}

#[test]
fn test_formatting_of_registers() {
    let registers = Registers {
        dn: vec!(0, 0xffffffff, 2, 3, 4, 5, 6, 7),
        an: vec!(7, 6, 5, 4, 0x30720, 2, 1, 0),
        pc: 0x12345,
        sr: 0x2700
    };

    let formatted_registers = format!("{}", registers);

    let expected_result = "Registers:
\tD0 = 00000000
\tD1 = FFFFFFFF
\tD2 = 00000002
\tD3 = 00000003
\tD4 = 00000004
\tD5 = 00000005
\tD6 = 00000006
\tD7 = 00000007
\tA0 = 00000007
\tA1 = 00000006
\tA2 = 00000005
\tA3 = 00000004
\tA4 = 00030720
\tA5 = 00000002
\tA6 = 00000001
\tA7 = 00000000
\tPC = 00012345
\tSR = 2700 [T=0 S=1 IPL=7 X=0 N=0 Z=0 V=0 C=0]
";
    assert_eq!(expected_result, formatted_registers);
}

#[test]
fn test_simulation_event_to_string() {
    let registers = Registers {
        dn: vec!(0, 0xffffffff, 2, 3, 4, 5, 6, 7),
        an: vec!(7, 6, 5, 4, 0x30720, 2, 1, 0),
        pc: 0x12345,
        sr: 0x2700
    };
    let formatted_registers = "Registers:
\tD0 = 00000000
\tD1 = FFFFFFFF
\tD2 = 00000002
\tD3 = 00000003
\tD4 = 00000004
\tD5 = 00000005
\tD6 = 00000006
\tD7 = 00000007
\tA0 = 00000007
\tA1 = 00000006
\tA2 = 00000005
\tA3 = 00000004
\tA4 = 00030720
\tA5 = 00000002
\tA6 = 00000001
\tA7 = 00000000
\tPC = 00012345
\tSR = 2700 [T=0 S=1 IPL=7 X=0 N=0 Z=0 V=0 C=0]
";

    // Test printing without registers
    assert_eq!("******************** Test passed ********************\n", format!("{}", SimulationEvent::Passed { registers: None }));

    // Test printing with registers
    assert_eq!(format!("******************** Test passed ********************\n{}", formatted_registers), format!("{}", SimulationEvent::Passed { registers: Some(registers.clone()) }));
    assert_eq!(format!("******************** Test failed ********************\n{}", formatted_registers), format!("{}", SimulationEvent::Failed { registers: Some(registers.clone()) }));
    assert_eq!(format!("******************** Test timed out ********************\n{}", formatted_registers), format!("{}", SimulationEvent::TimedOut { registers: Some(registers.clone()) }));
    assert_eq!(format!("******************** Privilege violation ********************\n{}", formatted_registers), format!("{}", SimulationEvent::PrivilegeViolation { registers: Some(registers.clone()) }));
    assert_eq!(format!("******************** Line-A exception ********************\n{}", formatted_registers), format!("{}", SimulationEvent::LineAException { registers: Some(registers.clone()) }));
    assert_eq!(format!("******************** Line-F exception ********************\n{}", formatted_registers), format!("{}", SimulationEvent::LineFException { registers: Some(registers.clone()) }));
    assert_eq!(format!("******************** Illegal instruction encountered ********************\n{}", formatted_registers), format!("{}", SimulationEvent::IllegalInstruction { registers: Some(registers.clone()) }));
    assert_eq!(format!("******************** Address error encountered ********************\nAccess address: 0x11337755\nAccess type: Read\nFunction code: 2\n{}", formatted_registers), format!("{}", SimulationEvent::AddressError { address: 0x11337755u32, write: false, function_code: 2, registers: Some(registers.clone()) } ));
    assert_eq!(format!("******************** Bus error encountered ********************\nAccess address: 0x123456\nAccess type: Write\nSize: LongWord\n{}", formatted_registers), format!("{}", SimulationEvent::BusError { address: 0x123456u32, write: true, operation_size: OperationSize::LongWord, registers: Some(registers.clone()) } ));
    assert_eq!(format!("smurf\n{}", formatted_registers), format!("{}", SimulationEvent::Print { message: String::from("smurf"), registers: Some(registers.clone()) } ));
}
