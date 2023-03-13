/// Advent of Code 2022
/// Day 11 - Parts 1 & 2 - "Monkey Keep Away!"
/// andr-be 2023

#[derive(Clone, Debug)]
enum Oper {
    Add,
    Mult,
}

impl Oper {
    fn _do(&self, val: i32, arg: i32) -> i32 {
        match self {
            Oper::Add => val + arg,
            Oper::Mult => val * arg,
        }
    }
}

#[derive(Clone, Debug)]
struct Action {
    op: Oper,
    div: i32,
    pass: i32,
    fail: i32,
} 

impl Action {
    fn new() -> Action {
        Action {
            op: Oper::Add, div: 0, pass: 0, fail: 0,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: i32,
    items: Vec<i32>,
    action: Action,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            id: 0, items: vec![], action: Action::new(),
        }
    }
}
/// receievs a long string and splits it into a vec of a vec of 7 strings
fn parse_input(input: String) -> Vec<String> {
    let vec_of_all_lines: Vec<String> = input.clone().lines().map(|i| i.to_string()).collect();
    let blocks = vec_of_all_lines.chunks(7).map(|c| c.to_owned().join("\n")).collect();
    blocks
}

/// receives a string of 7 lines and returns a vector of 7 lines
fn parse_block(input: String) -> Vec<(usize, String)> {
    return input.clone().lines().map(|i| i.to_string()).enumerate().collect();
}

/// receives a block of 7 enumerated lines and turns it into a monkey
fn parse_lines(input: Vec<(usize, String)>) -> Monkey {
    let mut monkey = Monkey::new();
    let mut action = Action::new();
    
    for line in input {
        println!("line: {:?}", line);
        match line.0 % 7 {
            0 => monkey.id = parse_id(&line.1),
            1 => monkey.items = parse_items(&line.1),
            2 => action.op = parse_op(&line.1),
            3 => action.div = parse_div(&line.1),
            4 => action.pass = parse_pass(&line.1),
            5 => action.fail = parse_pass(&line.1),
            6 => monkey.action = action.clone(),
            _ => { }
        }
    }
    monkey
}

/// grabs the monkey number from the first line of the input
fn parse_id(input: &str) -> i32 {
    input[7..8].parse().unwrap()
}
/// turns a string of comma separated numbers into a vec of i32s
fn parse_items(input: &str) -> Vec<i32> {
    todo!()
}
/// parses a string into a mathematical operation enum
fn parse_op(input: &str) -> Oper {
    todo!()
}
/// parses a divisor number for the operation
fn parse_div(input: &str) -> i32 {
    todo!()
}
/// works which monkey should be passed to
fn parse_pass(input: &str) -> i32 {
    todo!()
}

fn main() {
    // read test input and split it into blocks
    let input = std::fs::read_to_string("src/test_input.txt").unwrap();
    let mut blocks = parse_input(input);

    // generate the monkeys
    let monkey_3 = parse_lines(parse_block(blocks.pop().unwrap()));
    let monkey_2 = parse_lines(parse_block(blocks.pop().unwrap()));
    let monkey_1 = parse_lines(parse_block(blocks.pop().unwrap()));
    let monkey_0 = parse_lines(parse_block(blocks.pop().unwrap()));

    // go through each of the monkeys' moves
    todo!()
    
    //  
}

#[test]
fn add_test() {
    let op = Oper::Add;
    assert_eq!(op._do(2, 3), 5);
}

#[test]
fn mult_test() {
    let op = Oper::Mult;
    assert_eq!(op._do(2, 3), 6);
}
