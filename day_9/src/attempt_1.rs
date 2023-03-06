/// Advent of Code 2022: Day 9
/// andr-be 2023
///
/// This was an attempt to solve Day 9's AOC challenge.
/// Ultimately I'm unable to diagnose the problems with it; it solves the 
/// test case but cannot solve the full case for Part 1. Debugging has proved
/// to be exceptionally difficult due to the nature of the answer.


use std::{path::Path, collections::HashSet};

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x_pos: usize, y_pos: usize) -> Position {
        Position { x: x_pos, y: y_pos }
    }

    fn follow(head: &Position, tail: &Position) -> Option<Position> {
        let x_dif: isize = head.x as isize - tail.x as isize;
        let y_dif: isize = head.y as isize - tail.y as isize;

        let x_mov: isize;
        let y_mov: isize;
        let mag = x_dif.abs() + y_dif.abs();
        println!("mag: {mag}, x_dif: {x_dif}, y_dif: {y_dif}");

        if mag < 3 {
            x_mov = x_dif / 2;
            y_mov = y_dif / 2;
        } 
        else 
        {
            if head.x > tail.x { x_mov = 1; }
            else if head.x == tail.x { x_mov = 0;}
            else { x_mov = -1; }
            
            if head.y > tail.y { y_mov = 1; }
            else if head.y == tail.y { y_mov = 0; }
            else { y_mov = -1; }
        }
        Some(Position::new(
                (tail.x as isize + x_mov) as usize, 
                (tail.y as isize + y_mov) as usize,
            )
        )
    }
}

struct Knot {
    knot_type: KnotType,
    position: Position,
    history: Vec<Position>,
}

impl Knot {
    fn new(k_type: KnotType, x_pos: usize, y_pos: usize) -> Knot {
        let mut new_knot = Knot {
            knot_type: k_type,
            position: Position::new(x_pos, y_pos),
            history: Vec::new(),
        };
        new_knot.history.push(Position::new(0, 0));
        new_knot
    }

    fn move_head(&mut self, tail: &mut Knot, movement: Move) {
        for _ in 0..movement.magnitude{
            match movement.direction {
                Direction::Up => if self.position.y < 4 {self.position.y += 1},
                Direction::Down => if self.position.y > 0 {self.position.y -= 1},
                Direction::Right => if self.position.x < 5 {self.position.x += 1},
                Direction::Left => if self.position.x > 0 {self.position.x -= 1},
            };
            println!("head position is now: {}, {}", self.position.x, self.position.y);
            self.move_tail(tail);
            println!("tail position is now: {}, {}\n---", tail.position.x, tail.position.y);
        }
    }

    fn move_tail(&mut self, tail: &mut Knot) {
        let tail_pos = &tail.position;
        let head_pos = &self.position;
        tail.position = Position::follow(head_pos, tail_pos).unwrap();
        tail.history.push(tail.position.clone());
    }
}

struct Move {
    direction: Direction,
    magnitude: usize,
}

impl Move {
    fn parse(input: &str) -> Move {
        let dir = match &input[0..1] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "R" => Direction::Right,
                "L" => Direction::Left,
                _ => panic!()
            };
        let mag: usize = str::parse(&input[2..]).unwrap_or(0);
            return Move { direction: dir, magnitude: mag }
        }
        
}

enum KnotType {
    Head,
    Tail,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    // initialise the scenario parameters
    let mut h = Knot::new(KnotType::Head, 0, 0);
    let mut t = Knot::new(KnotType::Tail, 0, 0);
    
    // parse the input
    let input = read_input(Path::new("src/input.txt"));
    let lines = input.lines();

    // simulate the outcome
    for line in lines {
        println!("\n{}", line);
        let movement = Move::parse(line);
        println!("{:?}, {:?}", movement.direction, movement.magnitude);
        h.move_head(&mut t, movement);
    }
    // report the result
    println!("{}", t.history.len());
    let mut answer_part_1 = HashSet::new();
    for position in t.history {
        answer_part_1.insert(position);
    }
    println!("{}", answer_part_1.len());
    for answer in answer_part_1 {
        println!("{:?}", answer);
    }
}

fn read_input(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(_) => String::from(""),
    }
}
