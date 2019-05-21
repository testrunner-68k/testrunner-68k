#[macro_use]
extern crate lazy_static;

extern crate clap;
use clap::{App, Arg};

use amiga_hunk_parser::HunkParser;

mod amigahunk;
mod musashi;
mod testcases;

use amigahunk::get_test_cases;

use musashi::run_test_cases;

use testcases::TestResult;

fn pretty_print_results(test_results: &Vec<TestResult>) {
    for test_result in test_results {
        println!("{}: {}", test_result.name, test_result.success);
    }
    let success_count: isize = test_results.iter().map(|test_result| test_result.success as isize).sum();
    let total_count = test_results.len();
    println!("{} of {} tests passed", success_count, total_count);
}

fn successful(test_results: &Vec<TestResult>) -> bool {
    let first_failed_test_result = test_results.iter().find(|test_result| test_result.success == false);
    first_failed_test_result.is_none()
}


fn main() {

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("INPUT")
            .help("File with test code")
            .required(true)
            .index(1))
        .get_matches();

    let source_file = matches.value_of("INPUT").unwrap();

    let hunks = HunkParser::parse_file(source_file).unwrap();
    let test_cases = get_test_cases(&hunks);
    let test_results = run_test_cases(&hunks, &test_cases);
    pretty_print_results(&test_results);

    std::process::exit( if successful(&test_results) { 0 } else { 1 });
}

