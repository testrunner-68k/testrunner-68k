
extern crate ansi_term;

extern crate clap;

#[macro_use]
extern crate lazy_static;

use clap::{App, Arg};

use amiga_hunk_parser::HunkParser;

mod amigahunk;
mod musashi;
mod prettyprinter;
mod testcases;

use amigahunk::get_test_cases;
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
        .get_matches();

    let source_file = matches.value_of("INPUT").unwrap();

    #[cfg(windows)]
    {
        let _enabled = ansi_term::enable_ansi_support();
    }

    let hunks = HunkParser::parse_file(source_file).unwrap();
    let test_cases = get_test_cases(&hunks);

    let musashi_test_results = run_test_cases(&hunks, &test_cases);
    let test_results = musashi_test_results_to_test_results(&musashi_test_results);

    pretty_print_results(&test_results);

    std::process::exit( if successful(&test_results) { 0 } else { 1 });
}

