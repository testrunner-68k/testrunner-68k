
use amiga_hunk_parser::{Hunk, HunkType, HunkParser};

use super::testcases::TestCase;

// Compute start address for each hunk
pub fn layout_hunks(hunks: &Vec<Hunk>) -> Vec<u32> {

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

fn get_test_cases_for_hunk(hunk: &Hunk, start_address: &u32) -> Vec<TestCase> {
    let mut test_cases = Vec::new();
    match hunk.hunk_type {
        HunkType::Code | HunkType::Data => {
            match hunk.symbols.as_ref() {
                Some(symbols) => {
                    test_cases = symbols.iter().filter(|symbol| symbol.name.starts_with("test_")).map(|symbol| TestCase { name: symbol.name.clone(), entrypoint: start_address + symbol.offset }).collect();
                },
                None => {}
            }
        },
        HunkType::Bss => {}
    }

    return test_cases;
}

pub fn get_test_cases(hunks: &Vec<Hunk>, hunk_layout: &Vec<u32>) -> Vec<TestCase> {
    let mut test_cases = Vec::new();
    for hunk_index in 0..hunks.len() {
        let hunk = &hunks[hunk_index];
        let start_address = &hunk_layout[hunk_index];
        test_cases.append(&mut get_test_cases_for_hunk(hunk, start_address));
    }
    return test_cases;
}

#[test]
fn test_cases_enumerate_successfully() {
    let hunks = HunkParser::parse_file("testdata/test.test_cases.amiga.exe").unwrap();
    let hunk_layout = layout_hunks(&hunks);
    let test_cases = get_test_cases(&hunks, &hunk_layout);
    assert_eq!(3, test_cases.len());
    assert_eq!("test_Module1_Case1", test_cases[0].name);
    assert_eq!("test_Module2_Case2", test_cases[1].name);
    assert_eq!("test_Module1_Case2", test_cases[2].name);
}