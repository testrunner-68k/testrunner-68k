#[macro_use]
extern crate lazy_static;

use std::env;

use amiga_hunk_parser::HunkParser;

mod amigahunk;
mod musashi;
mod testcases;

use amigahunk::get_test_cases;

use musashi::run_test_cases;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: testrunner-m68k <executable name>");
        return;
    }

    let hunks = HunkParser::parse_file(&args[1]).unwrap();
//    dbg!(&hunks);
    let test_cases = get_test_cases(&hunks);
    let test_results = run_test_cases(&hunks, &test_cases);
    dbg!(&test_results);
}

