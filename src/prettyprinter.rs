
use ansi_term::Colour::*;

use super::testcases::{TestResult};

pub fn format_test_result(test_result: &TestResult) -> String {
    if test_result.success {
        format!("{}: {}", test_result.name, if test_result.success { Green.paint("PASSED").to_string() } else { Red.paint("FAILED").to_string() } )
    } else {
        let mut result_string = String::from("");
        for message in &test_result.messages {
            result_string = result_string + &message;
            result_string = result_string + "\n";
        }
        result_string = result_string + &format!("{}: {}", test_result.name, if test_result.success { Green.paint("PASSED").to_string() } else { Red.paint("FAILED").to_string() } );
        result_string
    }
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
        println!("{}", Red.paint(format!("{} tests failed", fail_count)).to_string());
    } else {
        println!("{}", Green.paint("All tests passed").to_string());
    }
}

#[test]
fn format_successful_test_result_correctly() {
    let successful_test_result = TestResult { name: String::from("test 1"), success: true, messages: vec!(String::from("test message 1"), String::from("test message 2")) };
    let result_string = format_test_result(&successful_test_result);
    assert_eq!(format!("test 1: {}", Green.paint("PASSED").to_string()), result_string);
}

#[test]
fn format_failed_test_result_correctly() {
    let failed_test_result = TestResult { name: String::from("test 2"), success: false, messages: vec!(String::from("test message 1"), String::from("test message 2")) };
    let result_string = format_test_result(&failed_test_result);
    assert_eq!(format!("test message 1\ntest message 2\ntest 2: {}", Red.paint("FAILED").to_string()), result_string);
}
