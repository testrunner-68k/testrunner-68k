
use super::simulation_event::SimulationEvent;
use super::super::testcases::TestResult;

#[derive(Debug)]
pub struct MusashiTestResult {
    pub name: String,
    pub success: bool,
    pub events: Vec<SimulationEvent>,
}

pub fn musashi_test_results_to_test_results(musashi_test_results: &Vec<MusashiTestResult>) -> Vec<TestResult> {

    let mut test_results: Vec<TestResult> = Vec::new();
    for musashi_test_result in musashi_test_results {

        let mut messages: Vec<String> = Vec::new();

        for event in musashi_test_result.events.iter() {
            match event {
                SimulationEvent::Passed => messages.push(String::from("Test passed")),
                SimulationEvent::Failed => messages.push(String::from("Test failed")),
                SimulationEvent::TimedOut => messages.push(String::from("Test timed out")),
                SimulationEvent::IllegalInstruction => messages.push(String::from("Illegal instruction encountered")),
                SimulationEvent::Print { message } => messages.push(message.to_string()),
            }
        }

        let test_result = TestResult {
            name: musashi_test_result.name.clone(),
            success: musashi_test_result.success,
            messages: messages
            };

        test_results.push(test_result);
    }

    test_results
}