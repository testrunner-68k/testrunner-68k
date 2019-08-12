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
pub enum SimulationEvent {
	Passed,
	Failed,
	TimedOut,
    PrivilegeViolation,
    LineAException,
    LineFException,
	IllegalInstruction,
    AddressError { address: u32, write: bool, function_code: u32 },
    BusError { address: u32, write: bool, operation_size: OperationSize },
    Print { message: String },
}

impl fmt::Display for SimulationEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimulationEvent::Passed => write!(f, "Test passed"),
            SimulationEvent::Failed => write!(f, "Test failed"),
            SimulationEvent::TimedOut => write!(f, "Test timed out"),
            SimulationEvent::PrivilegeViolation => write!(f, "Privilege violation"),
            SimulationEvent::LineAException => write!(f, "Line-A exception"),
            SimulationEvent::LineFException => write!(f, "Line-F exception"),
            SimulationEvent::IllegalInstruction => write!(f, "Illegal instruction encountered"),
            SimulationEvent::AddressError { address, write, function_code } => write!(f, "Address error encountered, access address: 0x{:x}, {}, function code: {}", address, if *write { "write" } else { "read" }, function_code),
            SimulationEvent::BusError { address, write, operation_size } => write!(f, "Bus error encountered, access address: 0x{:x}, {}, size: {}", address, if *write { "write" } else { "read" }, *operation_size),
            SimulationEvent::Print { message } => write!(f, "{}", message.to_string()),
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
fn test_simulation_event_to_string() {
    assert_eq!("Test passed", format!("{}", SimulationEvent::Passed));
    assert_eq!("Test failed", format!("{}", SimulationEvent::Failed));
    assert_eq!("Test timed out", format!("{}", SimulationEvent::TimedOut));
    assert_eq!("Privilege violation", format!("{}", SimulationEvent::PrivilegeViolation));
    assert_eq!("Line-A exception", format!("{}", SimulationEvent::LineAException));
    assert_eq!("Line-F exception", format!("{}", SimulationEvent::LineFException));
    assert_eq!("Illegal instruction encountered", format!("{}", SimulationEvent::IllegalInstruction));
    assert_eq!("Address error encountered, access address: 0x11337755, read, function code: 2", format!("{}", SimulationEvent::AddressError { address: 0x11337755u32, write: false, function_code: 2 } ));
    assert_eq!("Bus error encountered, access address: 0x123456, write, size: LongWord", format!("{}", SimulationEvent::BusError { address: 0x123456u32, write: true, operation_size: OperationSize::LongWord } ));
    assert_eq!("smurf", format!("{}", SimulationEvent::Print { message: String::from("smurf") } ));
}
