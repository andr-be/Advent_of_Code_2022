/// Advent of Code 2022
/// Day 11 - Parts 1 & 2 - "Monkey Keep Away!"
/// andr-be 2023

// create the graph structure that will hold the different nodes
struct MonkeyBox<T> {
    nodes: Vec<Node<T>>,
}

// create the node that will hold the monkey struct
struct Node<T> {
    id: i32,
    val: T,
}

// create the monkey struct that holds the items
#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    items: Vec<usize>,
    action: Action,
    p_true: i32,
    p_false: i32,
    p_1_count: i32,
}

// create the action which processes the item
#[derive(Debug, PartialEq, Clone)]
struct Action {
    op: (OpType, i32),
    test: i32,
}

// create the enum that tells the action how to process
#[derive(Debug, PartialEq, Clone)]
enum OpType {
    Addi,
    Multi,
    Square,
}

impl<T> MonkeyBox<T> {
    /// self function that adds a new node
    fn add_monkey(&mut self, monkey: T) {
        let new_node = Node::new(self.nodes.len() as i32, monkey);
        self.nodes.push(new_node);
    }
}

impl MonkeyBox<Monkey> {
    fn inspect(&mut self, id: usize) {
        let mut monkey = self.nodes[id].val.clone();
        println!("Pre: {:?}", monkey.items);
        monkey.items = monkey
            .items
            .iter()
            .map(|i| monkey.action.perform(*i as i32) as usize)
            .collect();
        println!("Post Inspect: {:?}", monkey.items);
        self.nodes[id].val = monkey.clone();
        for x in 0..monkey.items.len() {
            match monkey.items[x] % monkey.action.test as usize {
                0 => self.move_item(id, monkey.p_true as usize),
                _ => self.move_item(id, monkey.p_false as usize),
            }
        }
    }
    fn move_item(&mut self, id: usize, send_to: usize) {
        self.nodes[id].val.p_1_count += 1;
        let item = self.nodes[id].val.items.remove(0);
        self.nodes[send_to].val.items.push(item);
        println!("Monkey {id} throws {item} to Monkey {send_to}!");
    }
}

impl<T> Node<T> {
    /// encapsulate the Monkey in a node
    fn new(id: i32, val: T) -> Node<T> {
        Node { id, val }
    }
}

impl Monkey {
    fn generate(block: String) -> Self {
        let mut block = block.lines();

        let mut items: Vec<usize> = Vec::new();
        let mut action: Action = Action::new('+', 0, 0);
        let mut p_true: i32 = 0;
        let mut p_false: i32 = 0;

        for x in 0..7 {
            let line = block.next();
            match x {
                1 => items = Self::parse_items(line),      // parse the items
                2 => action.op = Self::parse_action(line), // parse the action
                3 => action.test = Self::grab_last_as_int(line), // parse the test
                4 => p_true = Self::grab_last_as_int(line), // parse the true pass
                5 => p_false = Self::grab_last_as_int(line), // parse the fail pass
                _ => {}
            }
        }

        Monkey::new(items, action, p_true, p_false)
    }

    fn new(items: Vec<usize>, action: Action, p_true: i32, p_false: i32) -> Self {
        Monkey {
            items,
            action,
            p_true,
            p_false,
            p_1_count: 0,
        }
    }

    fn parse_items(line: Option<&str>) -> Vec<usize> {
        line.unwrap()[18..]
            .split(",")
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect()
    }

    fn parse_action(line: Option<&str>) -> (OpType, i32) {
        let line: Vec<&str> = line.unwrap()[18..].split_whitespace().collect();
        let mut op = match line[1] {
            "+" => OpType::Addi,
            "*" => OpType::Multi,
            _ => panic!(),
        };
        let arg: i32 = match line[2].parse() {
            Ok(int) => int,
            Err(_) => {
                op = OpType::Square;
                0
            }
        };
        (op, arg)
    }

    fn grab_last_as_int(line: Option<&str>) -> i32 {
        let line: Vec<&str> = line.unwrap().split_whitespace().collect();
        let ret: i32 = line[line.len() - 1].parse().unwrap_or(0);
        ret
    }
}

impl Action {
    fn new(op: char, op_no: i32, test: i32) -> Action {
        let new_op = match op {
            '+' => OpType::Addi,
            '*' => OpType::Multi,
            _ => panic!(),
        };
        Action {
            op: (new_op, op_no),
            test,
        }
    }

    fn perform(&self, item: i32) -> i32 {
        let new_x = match self.op {
            (OpType::Addi, x) => item + x,
            (OpType::Multi, x) => item * x,
            (OpType::Square, _) => item * item,
        };
        new_x / 3
    }
}

fn main() {
    // turn input into a string
    // let input = std::fs::read_to_string("src/test_input.txt").unwrap();
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    // split the input into blocks of 7
    let blocks = split_into_blocks(input);

    // initialise the monkeybox
    let mut mbox = MonkeyBox { nodes: vec![] };

    // process each block into a new monkey
    for block in blocks {
        mbox.add_monkey(Monkey::generate(block))
    }
    // cycle through every monkey
    for r in 0..10 {
        println!("=== ROUND {r:5} === ");
        round(&mut mbox);
    }
}

fn split_into_blocks(input: String) -> Vec<String> {
    let vec_lines: Vec<String> = input.lines().map(|i| i.to_string()).collect();
    let blocks = vec_lines
        .chunks(7)
        .map(|c| c.to_owned().join("\n"))
        .collect();
    blocks
}

/// for every monkey in the box, perform inspect
fn round(mb: &mut MonkeyBox<Monkey>) {
    for x in 0..mb.nodes.len() {
        println!("Monkey: {x}");
        mb.inspect(x);
        println!("\n");
    }
    for x in 0..mb.nodes.len() {
        println!(
            "MONKEY {:?} is holding {:?}\nand inspected {} total items\n",
            mb.nodes[x].id, mb.nodes[x].val.items, mb.nodes[x].val.p_1_count
        );
    }
}

#[test]
fn split() {
    let input = std::fs::read_to_string("src/test_input.txt").unwrap();
    let blocks = split_into_blocks(input);
    assert_eq!(4, blocks.len());
}
#[test]
fn items() {
    let input = "  Starting items: 79, 98";
    let result = Monkey::parse_items(Some(input));
    assert_eq!(result, vec![79, 98])
}
#[test]
fn operation() {
    let input = "  Operation: new = old * 19";
    let result = Monkey::parse_action(Some(input));
    assert_eq!(result, (OpType::Multi, 19));
}
#[test]
fn div() {
    let input = "  Test: divisible by 23";
    let result = Monkey::grab_last_as_int(Some(input));
    assert_eq!(result, 23);
}
#[test]
fn tpass() {
    let input = "    If true: throw to monkey 2";
    let result = Monkey::grab_last_as_int(Some(input));
    assert_eq!(result, 2);
}
#[test]
fn fpass() {
    let input = "    If false: throw to monkey 3";
    let result = Monkey::grab_last_as_int(Some(input));
    assert_eq!(result, 3);
}
#[test]
fn build_monkey() {
    let input = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\n";
    let dummy = Monkey {
        items: vec![79, 98],
        action: Action {
            op: (OpType::Multi, 19),
            test: 23,
        },
        p_true: 2,
        p_false: 3,
        p_1_count: 0,
    };
    let result = Monkey::generate(input.to_string());
    assert_eq!(result, dummy);
}
