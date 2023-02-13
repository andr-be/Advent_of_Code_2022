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
    let mut p1_solution_huw = 0;        
    let mut p2_solution_huw = 0;

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

        // Huw Solution
        if (p1_s >= p2_s && p1_e <= p2_e) || ( p2_s >= p1_s && p2_e <= p1_e) {
            p1_solution_huw += 1;
        }

        if ( p1_s <= p2_s && p1_e > p2_s ) || ( p2_s <= p1_s && p2_e > p1_s ) {
            p2_solution_huw +=1;
        }
    }   
    println!("HUW SOLUTION\tP1: {p1_solution_huw}\tP2: {p2_solution_huw}");
}
