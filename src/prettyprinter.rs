
use super::testcases::{TestResult};

pub fn format_test_result(test_result: &TestResult) -> String {
    format!("{}: {}", test_result.name, if test_result.success { "PASSED" } else { "FAILED"} )
}

pub fn pretty_print_results(test_results: &Vec<TestResult>) {
    println!("Test results:");
    println!("");
    for test_result in test_results {
        println!("{}", format_test_result(&test_result));
    }
    let fail_count: isize = test_results.iter().map(|test_result| !test_result.success as isize).sum();
    println!("");
    if fail_count > 0 {
        println!("{} tests failed", fail_count);
    } else {
        println!("All tests passed");
    }
}

#[test]
fn format_successful_test_result_correctly() {
    let successful_test_result = TestResult { name: String::from("test 1"), success: true };
    let result_string = format_test_result(&successful_test_result);
    assert_eq!("test 1: PASSED", result_string);
}

#[test]
fn format_failed_test_result_correctly() {
    let failed_test_result = TestResult { name: String::from("test 2"), success: false };
    let result_string = format_test_result(&failed_test_result);
    assert_eq!("test 2: FAILED", result_string);
}
