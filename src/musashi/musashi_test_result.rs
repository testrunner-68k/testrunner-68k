
use super::simulation_event::SimulationEvent;
use super::super::testcases::TestResult;

#[derive(Clone, Debug, PartialEq)]
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
            messages.push(format!("{}", event));
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
