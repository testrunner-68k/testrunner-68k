use std::env;

use amiga_hunk_parser::{Hunk, HunkType, HunkParser};

// Compute start address for each hunk
fn layout_hunks(hunks: &Vec<Hunk>) -> Vec<u32> {

    let mut layout_hunks = Vec::new();

    let mut start_address = 0u32;

    for hunk_index in 0..hunks.len() {

        let hunk = &hunks[hunk_index];
        layout_hunks.push(start_address);
        start_address = ((start_address + (hunk.alloc_size as u32)) + 3) & 0xfffffffc;
    }

    dbg!(&layout_hunks);

    return layout_hunks;
}

#[derive(Debug)]
struct TestCase {
    name: String,
    entrypoint: u32,
}

fn get_test_cases_for_hunk(hunk: &Hunk, start_address: &u32) -> Vec<TestCase> {
    let mut test_cases = Vec::new();
    match hunk.hunk_type {
        HunkType::Code | HunkType::Data => {
            match hunk.symbols {
                Some(symbols) => {
                    test_cases = symbols.iter().filter(|symbol| symbol.name.starts_with("test_")).map(|symbol| TestCase { name: symbol.name, entrypoint: start_address + symbol.offset }).collect();
                }
            }
        }
    }

    return test_cases;
}

fn get_test_cases(hunks: &Vec<Hunk>, hunk_layout: &Vec<u32>) -> Vec<TestCase> {
    let mut test_cases = Vec::new();
    for hunk_index in 0..hunks.len() {
        let hunk = &hunks[hunk_index];
        let start_address = &hunk_layout[hunk_index];
        test_cases.append(&mut get_test_cases_for_hunk(hunk, start_address));
    }
    return test_cases;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: testrunner-m68k <executable name>");
        return;
    }

    let hunks = HunkParser::parse_file(&args[1]).unwrap();
//    dbg!(&hunks);
    let hunk_layout = layout_hunks(&hunks);
    let test_cases = get_test_cases(&hunks, &hunk_layout);
}
