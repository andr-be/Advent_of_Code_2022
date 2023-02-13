/// Advent of Code 2022 - Day 4 - Parts 1 & 2 Solutions
/// andr-be 2023
///
///

use std::path::Path;

fn read_input(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(_) => String::from(""),
    }
}

fn main() {
    let problem = Path::new("src/input.txt");
    let input = read_input(problem);

    
}
