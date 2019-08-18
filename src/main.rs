
extern crate ansi_term;

extern crate clap;

extern crate xml;

#[macro_use]
extern crate lazy_static;

use clap::{App, Arg};

use amiga_hunk_parser::HunkParser;

mod amigahunk;
mod junit_result_writer;
mod musashi;
mod prettyprinter;
mod testcases;

use amigahunk::get_test_cases;
use junit_result_writer::write_test_results;
use musashi::runner::run_test_cases;
use musashi::musashi_test_result::musashi_test_results_to_test_results;
use prettyprinter::pretty_print_results;
use testcases::TestResult;

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
        .arg(Arg::with_name("JUNIT_RESULTS_FILE")
            .long("junit")
            .short("j")
            .value_name("JUnit results file")
            .help("If specified, test results will be written in JUnit XML format to this file")
            .takes_value(true))
        .get_matches();

    let source_file = matches.value_of("INPUT").unwrap();
    let junit_results_file = matches.value_of("JUNIT_RESULTS_FILE");

    #[cfg(windows)]
    {
        let _enabled = ansi_term::enable_ansi_support();
    }

    let hunks = HunkParser::parse_file(source_file).unwrap();
    let test_cases = get_test_cases(&hunks);

    let musashi_test_results = run_test_cases(&hunks, &test_cases);
    let test_results = musashi_test_results_to_test_results(&musashi_test_results);

    pretty_print_results(&test_results);
    if !junit_results_file.is_none() {
        write_test_results(&test_results, junit_results_file.unwrap());
    }

    std::process::exit( if successful(&test_results) { 0 } else { 1 });
}

