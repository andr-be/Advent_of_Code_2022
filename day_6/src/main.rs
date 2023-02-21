/// Advent of Code 2022 - Day 6 - Parts 1 & 2 Solutions
/// andr-be 2023
/// this is an exceptionally elegant solution
use std::{collections::HashMap, path::Path};

fn read_input(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(_) => String::from(""),
    }
}

fn find_answer(path: &Path, window_size: usize) -> usize {
    let windows = read_input(path).chars().collect::<Vec<char>>();
    let windows = windows.windows(window_size).enumerate();

    for (slice_no, window) in windows {
        let mut hash_map = HashMap::new();
        for c in window {
            hash_map.insert(c, slice_no);
        }
        if hash_map.len() == window_size {
            return slice_no + window_size;
        }
    }
    0
}

fn main() {
    let path = Path::new("src/input.txt");
    let part_1 = 4;
    let part_2 = 14;

    println!("Part 1: {}", find_answer(path, part_1));
    println!("Part 2: {}", find_answer(path, part_2));
}
