
#[derive(Debug, Clone)]
pub enum SimulationEvent {
	Passed,
	Failed,
	TimedOut,
	IllegalInstruction,
    Print { message: String },
}
