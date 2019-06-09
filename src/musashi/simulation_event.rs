use std::fmt;

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
    BusError,
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
            SimulationEvent::BusError => write!(f, "Bus error encountered"),
            SimulationEvent::Print { message } => write!(f, "{}", message.to_string()),
        }
    }
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
    assert_eq!("Bus error encountered", format!("{}", SimulationEvent::BusError));
    assert_eq!("smurf", format!("{}", SimulationEvent::Print { message: String::from("smurf") } ));
}
