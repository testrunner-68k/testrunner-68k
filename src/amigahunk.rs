
use amiga_hunk_parser::{Hunk, HunkType};

use super::testcases::TestCase;

fn get_test_cases_for_hunk(hunk: &Hunk) -> Vec<TestCase> {
    let mut test_cases = Vec::new();
    match hunk.hunk_type {
        HunkType::Code | HunkType::Data => {
            match hunk.symbols.as_ref() {
                Some(symbols) => {
                    test_cases = symbols.iter().filter(|symbol| symbol.name.starts_with("test_")).map(|symbol| TestCase { name: symbol.name.clone() }).collect();
                },
                None => {}
            }
        },
        HunkType::Bss => {}
    }

    return test_cases;
}

pub fn get_test_cases(hunks: &Vec<Hunk>) -> Vec<TestCase> {
    let mut test_cases = Vec::new();
    for hunk_index in 0..hunks.len() {
        let hunk = &hunks[hunk_index];
        test_cases.append(&mut get_test_cases_for_hunk(hunk));
    }
    return test_cases;
}

#[cfg(test)]
use amiga_hunk_parser::HunkParser;

#[test]
fn test_cases_enumerate_successfully() {
    let hunks = HunkParser::parse_file("testdata/test.test_cases.amiga.exe").unwrap();
    let test_cases = get_test_cases(&hunks);
    assert_eq!(3, test_cases.len());
    assert_eq!("test_Module1_Case1", test_cases[0].name);
    assert_eq!("test_Module2_Case2", test_cases[1].name);
    assert_eq!("test_Module1_Case2", test_cases[2].name);
}