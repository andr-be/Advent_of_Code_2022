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
        let p1_s = p1.next().unwrap().parse::<usize>().unwrap();
        let p1_e = p1.next().unwrap().parse::<usize>().unwrap() + 1;
        let p2_s = p2.next().unwrap().parse::<usize>().unwrap();
        let p2_e = p2.next().unwrap().parse::<usize>().unwrap() + 1;
        
        // construct the range 
        let p1_range = [p1_s..p1_e];
        let p2_range = [p2_s..p2_e];

        // convert ranges into vectors 
        let mut p1_numbers: Vec<usize> = Vec::new();
        let mut p2_numbers: Vec<usize> = Vec::new();
        for number in p1_range {
            for sub in number {
                p1_numbers.push(sub);
            }
        }
        for number in p2_range {
            for sub in number {
                p2_numbers.push(sub);
            }
        }
        println!("\n{pair}");

        let mut not_contained_1 = 0;
        let mut not_contained_2 = 0;

        for number in &p1_numbers {
            if ! p2_numbers.contains(&number) {
                not_contained_1 += 1;
            }
        }
        println!("Total P1 not contained in P2: {not_contained_1}");

        for number in &p2_numbers {
            if ! p1_numbers.contains(&number) {
                not_contained_2 += 1;
            }
        }
        println!("Total P2 not contained in P1: {not_contained_2}");

        if not_contained_1 == 0 || not_contained_2 == 0 {
            p1_solution += 1;
        }
        
        if not_contained_1 < p1_numbers.len() || not_contained_2 < p2_numbers.len() {
            p2_solution += 1;
        }
    }   

    println!("P1 ANSWER: {p1_solution}\nP2 ANSWER: {p2_solution}");
}
