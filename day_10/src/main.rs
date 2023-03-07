/// Advent of Code 2022 Day 10 - Parts 1 & 2
/// Cathode Ray Tube
///
/// Simulating an adder register like in assembly
/// P1: 13220
/// P2: RUAKHBEK
///
/// I think for Part 2 I'm going to make a completely different program that registers each cycle
/// as an event; these events will then be made to happen in the correct order

const SCREEN_WIDTH: i32 = 40;

enum Action {
    NoOp,
    Addx,
}

impl Action {
    fn parse_action(line: &str) -> Action {
        match &line[0..4] {
            "noop" => Action::NoOp,
            "addx" => Action::Addx,
            _ => panic!()
        }
    }
}

struct Register {
    x: i32
}

impl Register {
    fn new(init: i32) -> Register {
        Register { x: init }
    }

    fn add(&mut self, x: i32) {
        self.x += x;
    }
}

fn main() {
    let raw = std::fs::read_to_string("src/input.txt").unwrap();
    let mut input = raw.lines();
    
    let mut x_reg = Register::new(1);
    let mut cycle: i32 = 1;
    let mut answer_part_1: i32 = 0;
    let mut answer_part_2 = String::new();

    while cycle <= 240 {
        let line = input.next().unwrap();
        match Action::parse_action(line) {
            Action::NoOp => {
                println!("C: {cycle:3} -->> NO OP\t\tX_REG: {}", x_reg.x);
                answer_part_1 += part_1_sum(&cycle, &x_reg.x);
                answer_part_2 += &part_2_render(&cycle, &x_reg.x);
            }
            Action::Addx => {
                let addi: i32 = line[5..].parse().unwrap();
                answer_part_1 += part_1_sum(&cycle, &x_reg.x);
                answer_part_2 += &part_2_render(&cycle, &x_reg.x);
                println!("C: {cycle:3} -->> ADDX {:3} (1/2)\tX_REG: {}", addi, x_reg.x);
                cycle += 1;
                
                answer_part_1 += part_1_sum(&cycle, &x_reg.x);
                x_reg.add(addi);
                answer_part_2 += &part_2_render(&cycle, &x_reg.x);
                println!("C: {cycle:3} -->> ADDX {:3} (2/2)\tX_REG: {}", addi, x_reg.x);
            }
        }
        cycle += 1;
    }
    println!("ANSWER PART 1: {}", answer_part_1);
    println!("ANSWER PART 2:\n{}", answer_part_2);
}

fn part_1_sum(cycle: &i32, x: &i32) -> i32 {
    let sample_c = [20, 60, 100, 140, 180, 220];
    if sample_c.contains(cycle) {
        return cycle * x
    };
    0
}

fn part_2_render(cycle: &i32, x: &i32) -> String {
    let mut ret = String::new();
    let mapped_cycle: i32 = cycle.clone() % SCREEN_WIDTH;
    let x_rng = (x-1)..=(x+1);
    if x_rng.contains(&mapped_cycle) {
        ret += "# "
    } else {
        ret += ". "
    };
    if mapped_cycle == 39 {
        ret += "\n"
    };
    ret
}
