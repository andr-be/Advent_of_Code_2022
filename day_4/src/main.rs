/// Advent of Code 2022 - Day 4 - Parts 1 & 2 Solutions
/// andr-be 2023

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
    let lines = input.lines();
    let mut p1_solution = 0;        
    let mut p2_solution = 0;

    for pair in lines {
        // split the pair
        let mut split = pair.split(",");
        
        // split the first range 
        let p1 = split.next().unwrap();
        let mut p1 = p1.split("-");
        
        // split the second range 
        let p2 = split.next().unwrap();
        let mut p2 = p2.split("-");

        // get the starts and ends of the ranges
        let p1_min = p1.next().unwrap().parse::<usize>().unwrap();
        let p1_max = p1.next().unwrap().parse::<usize>().unwrap() + 1;
        
        let p2_min = p2.next().unwrap().parse::<usize>().unwrap();
        let p2_max = p2.next().unwrap().parse::<usize>().unwrap() + 1;

        // Huw's Inequality-Based Solution
        if (p1_min >= p2_min && p1_max <= p2_max) 
        || ( p2_min >= p1_min && p2_max <= p1_max) {
            p1_solution += 1;
        }
        if ( p1_min <= p2_min && p1_max > p2_min ) 
        || ( p2_min <= p1_min && p2_max > p1_min ) {
            p2_solution +=1;
        }
    }   
    println!("SOLUTION\tP1: {p1_solution}\tP2: {p2_solution}");
}
