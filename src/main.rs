use std::env;

use amiga_hunk_parser::{Hunk, HunkParser};

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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: testrunner-m68k <executable name>");
        return;
    }

    let hunks = HunkParser::parse_file(&args[1]).unwrap();
//    dbg!(&hunks);
    let hunk_layout = layout_hunks(&hunks);
}
