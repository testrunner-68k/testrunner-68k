
#[derive(Clone, Debug, PartialEq)]
pub enum SimulationEvent {
	Passed,
	Failed,
	TimedOut,
	IllegalInstruction,
    Print { message: String },
}
