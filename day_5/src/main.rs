/// Advent of Code 2022 - Day 5 - Parts 1 & 2
/// andr-be 2023
/// 
/// In its current iteration, this program solves just Part 2 of Day 5.
/// However, in order to get it to solve Part 1; you just need to change 
/// line 84 from push_front to push_back!

use std::{path::Path, collections::VecDeque};

fn read_input(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(_) => String::from(""),
    }
}

fn main() {
    let path = Path::new("src/input");

    let input = read_input(path);

    let mut lines = input.lines();

    let crate_section = 8;
    
    let bay_1 = VecDeque::new();
    let bay_2 = VecDeque::new();
    let bay_3 = VecDeque::new();
    let bay_4 = VecDeque::new();
    let bay_5 = VecDeque::new();
    let bay_6 = VecDeque::new();
    let bay_7 = VecDeque::new();
    let bay_8 = VecDeque::new();
    let bay_9 = VecDeque::new();

    let mut bay_list: Vec<VecDeque<char>> = 
    [bay_1, bay_2, bay_3, bay_4, bay_5, bay_6, bay_7, bay_8, bay_9].into();

    for _ in 0..crate_section {
        let this_line = lines.next().unwrap();
        let this_line = this_line.chars().enumerate();
        for (position, character) in this_line {
            match character {
                ' ' | ']' | '[' => {},
                _ =>    { match position { 
                                            1  => bay_list[0].push_back(character),
                                            5  => bay_list[1].push_back(character),
                                            9  => bay_list[2].push_back(character),
                                            13 => bay_list[3].push_back(character),
                                            17 => bay_list[4].push_back(character),
                                            21 => bay_list[5].push_back(character),
                                            25 => bay_list[6].push_back(character),
                                            29 => bay_list[7].push_back(character),
                                            33 => bay_list[8].push_back(character),
                                            _  => {},
                                        }
                        }      
            }
        }
    }
    
    let mut repeats: usize = 0;
    let mut pickup: usize = 1;
    let mut dropoff: usize = 1;
    lines.next();
    lines.next();

    for (_, line) in lines.enumerate() {
            println!("{line}");
            let words = line.split(" ").map(|s| s.to_string()).enumerate();
            for (pos, word) in words {
                if pos == 1 {
                    repeats = word.parse::<usize>().unwrap();
                } else if pos == 3 {
                    pickup = word.parse::<usize>().unwrap() - 1;
                } else if pos == 5 {
                    dropoff = word.parse::<usize>().unwrap() - 1;
                }
            }
            let mut crane: VecDeque<char> = VecDeque::new();
            for _ in 0..repeats {
                let lifted = bay_list[pickup].pop_front().unwrap();
                println!("LIFTING {lifted} FROM {pickup}");
                crane.push_front(lifted);
                // crane.push_back(lifted); solves part 1
            }
            for item in crane {
                bay_list[dropoff].push_front(item);
                println!("DROPPING {item} ON {dropoff}");
            }
        }
        for mut bay in bay_list {
            print!("{}", bay.pop_front().unwrap_or_else(|| ' '))
        }
        print!("\n");
    }
