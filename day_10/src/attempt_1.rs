/// Advent of Code 2022 Day 10 - Parts 1 & 2
/// Cathode Ray Tube
///
/// Simulating an adder register like in assembly
/// P1: 13220
/// P2: UNFATHOMABLE
///
/// I think for Part 2 I'm going to make a completely different program that registers each cycle
/// as an event; these events will then be made to happen in the correct order

enum Action {
    NoOp,
    Addx,
}

fn main() {
    // let raw = std::fs::read_to_string("src/input.txt").unwrap();
    let raw = std::fs::read_to_string("src/test_input.txt").unwrap();
    let input = raw.lines();
    
    let mut x_reg: i32 = 1;
    let mut cycle: i32 = 1;
    let mut answer_part_1: i32 = 0;
    let mut answer_part_2 = String::new(); 
    
    answer_part_1 += report(&cycle, &x_reg);
    answer_part_2 += &render(&cycle, &x_reg);

    for line in input {
        cycle += 1;
        answer_part_1 += report(&cycle, &x_reg);
        answer_part_2 += &render(&cycle, &x_reg);
        let action = match &line[0..4] {
            "noop" => Action::NoOp,
            "addx" => Action::Addx,
            _ => panic!()
        };
        match action {
            Action::NoOp => continue,
            Action::Addx => {
                let addi: i32 = line[5..].parse().unwrap();
                x_reg += addi;
                cycle += 1;
            }
        }
        answer_part_1 += report(&cycle, &x_reg);
        answer_part_2 += &render(&cycle, &x_reg);
    }
    println!("ANSWER_PART_1: {answer_part_1}");
    println!("ANSWER_PART_2:\n{}", answer_part_2)
}

fn report(cycle: &i32, x_reg: &i32) -> i32 {
    // takes in a cycle no and x_reg total and returns a value of
    // signal strength if the cycle matches one of the sample pts
    let sample_points = [20, 60, 100, 140, 180, 220];
    if sample_points.contains(cycle){
        println!("adding {cycle} * {x_reg} to ap1: {}", cycle * x_reg);
        return x_reg * cycle
    }
    0
}

fn render(cycle: &i32, x_reg: &i32) -> String {
    let mut ret = String::new();
    let sprite_range = (x_reg - 1)..=(x_reg + 1);
    if sprite_range.contains(cycle) {
        ret += "#"
    } else {
        ret += "."
    };
    if cycle % 40 == 0 {
        ret += "\n"
    };
    println!("{ret}");
    ret
}
