use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: testrunner-m68k <executable name>");
        return;
    }

    let result = amiga_hunk_parser::HunkParser::parse_file(&args[1]).unwrap();
}
