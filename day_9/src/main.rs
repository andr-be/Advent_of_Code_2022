use std::collections::HashSet;
use std::fmt::Display;
use std::fs;

/// Advent of Code 2022: Day 9: Parts 1 & 2
/// andr-be 2023
///
/// The exponentially increasing difficulty of these challenges both
/// excites and frustrates me in equal measure. One can only hope I'm 
/// not wasting my time here.
/// 
/// UPDATE: My problem was assuming this was within a grid; I'm a goddamn idiot
///         it was unbounded problem...

const ROPE_L: f64 = 2.0;

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Right => write!(f, "RIGHT"),
            Direction::Left => write!(f, "LEFT"),
            Direction::Up => write!(f, "UP"),
            Direction::Down => write!(f, "DOWN"),
        }
    }
}

struct Knot {
    x_pos: i32,
    y_pos: i32,
    history: Vec<(i32, i32)>
}

impl Knot {
    fn new(x_pos: i32, y_pos: i32) -> Knot {
        let mut new_knot = Knot {
            x_pos,
            y_pos,
            history: Vec::new(),
        };
        new_knot.history.push((x_pos, y_pos));
        new_knot
    }

    fn move_one(&mut self, dir: &Direction) {
        self.history.push((self.x_pos, self.y_pos));
        match dir {
            Direction::Right =>  self.x_pos += 1,
            Direction::Left =>   self.x_pos -= 1,
            Direction::Up =>     self.y_pos += 1,
            Direction::Down =>   self.y_pos -= 1,
        }
    }

    fn follow(&mut self, head: &Knot) {
        let x_diff: i32 = head.x_pos - self.x_pos;
        let y_diff: i32 = head.y_pos - self.y_pos;

        let diff_mag: f64 = (x_diff * x_diff + y_diff * y_diff) as f64;
        let diff_mag = diff_mag.sqrt();
        self.history.push((self.x_pos, self.y_pos));

        if diff_mag >= ROPE_L {
            match x_diff {
                d if d.is_positive() => self.x_pos += 1,
                d if d.is_negative() => self.x_pos -= 1,
                _ => { }
            };
            match y_diff {
                d if d.is_positive() => self.y_pos += 1,
                d if d.is_negative() => self.y_pos -= 1,
                _ => { }
            };
        }
    }


}

fn main() {
    // generate two knots, head and tail
    let mut head = Knot::new(0,0);
    let mut k_1 = Knot::new(0,0);
    let mut k_2 = Knot::new(0,0);
    let mut k_3 = Knot::new(0,0);
    let mut k_4 = Knot::new(0,0);
    let mut k_5 = Knot::new(0,0);
    let mut k_6 = Knot::new(0,0);
    let mut k_7 = Knot::new(0,0);
    let mut k_8 = Knot::new(0,0);
    let mut tail = Knot::new(0,0);
    
    // parse the input into an iterable
    // let input = fs::read_to_string("src/test_input.txt").unwrap();
    let input = fs::read_to_string("src/input.txt").unwrap();

    let input = input
        .lines()
        .map(|t| t
            .split_whitespace()
            .collect::<Vec<&str>>()
        );

    let mut answer_part_1: HashSet<(i32, i32)> = HashSet::new();
    let mut answer_part_2: HashSet<(i32, i32)> = HashSet::new();

    for mut turn in input {
        let qty: i32 = turn
            .pop()          // grab 
            .unwrap()       // pull value
            .parse()        // convert to i32
            .unwrap_or(0);  // if it doesn't work, return 0 

        let dir = match turn.pop().unwrap() {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!(),
        };

        println!("\n=== {} {} ===", dir, qty);
        for _ in 0..qty {
            head.move_one(&dir);
            k_1.follow(&head);
            k_2.follow(&k_1);
            k_3.follow(&k_2);
            k_4.follow(&k_3);
            k_5.follow(&k_4);
            k_6.follow(&k_5);
            k_7.follow(&k_6);
            k_8.follow(&k_7);
            tail.follow(&k_8);
            answer_part_1.insert((k_1.x_pos, k_1.y_pos));
            answer_part_2.insert((tail.x_pos, tail.y_pos));
        }
    }
    
    println!("ANSWER PART 1: {}\nANSWER PART 2: {}\nFROM {} MOVES", 
        answer_part_1.len(),
        answer_part_2.len(),
        tail.history.len(),
    );
}


