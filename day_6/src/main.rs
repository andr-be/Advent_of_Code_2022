/// Advent of Code 2022 - Day 6 - Parts 1 & 2 Solutions
/// andr-be 2023
///
use std::{path::Path, collections::HashMap};

fn read_input(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(_) => String::from(""),
    }
}

fn main() {
    let path = Path::new("src/input.txt");

    let input = read_input(path);

    let character_vec: Vec<char> = input.chars().collect();
    println!("Input Length: {}", character_vec.len());

    let window_size = 14;
    let windows = character_vec.windows(window_size).enumerate();
    
    for (slice_no, window) in windows {
        let mut hash_map = HashMap::new();
        for c in window {
            hash_map.insert(c, slice_no);
        }
        if hash_map.len() == window_size {
            println!("Unique string of length: {window_size} found in slice {slice_no}");
            println!("First pattern match character occurs at index: {}/{}", 
                slice_no + window_size, 
                character_vec.len()
            );
            break
        } else {
            if slice_no % 128 == 0 {
                println!("{slice_no} slices checked");
            }
        }
    }
}
