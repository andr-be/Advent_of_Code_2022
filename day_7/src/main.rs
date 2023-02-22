/// Advent of Code 2022 - Day 7 - Parts 1 & 2
/// This code is going to serve as my first implementation of a non-binary tree.
/// 
/// My approach is going to be to build the tree structure based on an input parse of the string
/// each time 'dir' is referenced I will add a new child node
/// each time a file is listed I will add its value to the current node value
/// each time a cd command is given, I will traverse the tree to the parent node
///
/// After all of the input has been processed, I iterate through the list backwards and update
/// the size of the directory's parents by summing them. This ensures that I don't accidentally
/// add the value of a child too soon, before it has had its value updated.
///
/// AP1:
///     achieved by summing all of the updated directories with < 100,000 size
///     
/// AP2: 
///     achieved by first calculating the remaining_space of the drive.
///     we take this remaining space away from the required_space (3,000,000) 
///     this gives us the minimum possible deletion we can do to install the update
///     we then add all of the directories with sizes above to_be_deleted to a new vector
///     this vector is sorted by size order and the first element is reported

use std::path::Path;

// STRUCTS
#[derive(PartialEq, Clone)]
struct Directory {
    name: String,
    size: usize,
    depth: usize,
}
#[derive(Debug, Default)]
struct ArenaTree<T> 
where
    T: PartialEq
{
    arena: Vec<Node<T>>,
}
#[derive(Debug)]
struct Node<T>
where 
    T: PartialEq
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

// METHODS 
impl<T> Node<T>
where
    T: PartialEq
{
    fn new(idx:usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

impl<T> ArenaTree<T>
where
    T: PartialEq
{
    fn node(&mut self, val: T) -> usize {
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }

    fn depth_to_target(&self, idx: usize, target: &T) -> Option<usize> {
        // already here? QED depth = 0
        if target == &self.arena[idx].val {
            return Some(0);
        }
        // if not, try all children
        for p in &self.arena[idx].children {
            if let Some(x) = self.depth_to_target(*p, &target) {
                return Some(1 + x);
            }
        }
        // if it can't be found, return None
        None
    }
}

fn main() {
    // read the file input and split it into an iterator over lines 
    let input = read_input(Path::new("src/input.txt"));
    let lines = input.lines();

    // instantiate the file system tree structure
    let mut filesystem: ArenaTree<Directory> = ArenaTree{ arena: vec![] };
    
    // declare the root and set it as the current directory
    let root = Directory { name: "/".to_string(), size: 0, depth: 0 };
    let mut current_dir = ArenaTree::node(&mut filesystem, root);

    // list the problem parameters
    let drive_max = 100_000;
    let drive_size = 70_000_000;
    let required_space = 30_000_000;

    // iterate through every line of input and map the drive
    for line in lines {
        match &line[0..4] {
            "$ ls" => { },    // do nothing
            "$ cd" => current_dir = change_directory(&mut filesystem, current_dir, &line[5..]),
            "dir " => create_directory(&mut filesystem, current_dir, &line[4..]),
              _    => add_file(&mut filesystem, current_dir, line),         
        };
    }

    // solve the problems 
    let answer_part_1 = part_1(&mut filesystem, drive_max);
    let answer_part_2 = &part_2(&mut filesystem, drive_size, required_space)[0];
    println!("ANSWER PART 1:\t{}\nANSWER PART 2:\tDELETE DIR {} - {}", 
        answer_part_1,
        answer_part_2.name, 
        answer_part_2.size
    )
}

/// take a path and return a .txt as a String
fn read_input(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(_) => String::from(""),
    }
}

/// change the current directory to one of its children
fn change_directory(f: &mut ArenaTree<Directory>, c: usize, l: &str) -> usize {
    match l {
        ".." => f.arena[c].parent.unwrap(),
        _ => {
            for dir in &f.arena[c].children {
                if f.arena[*dir].val.name == l {
                    return f.arena[*dir].idx
                }
            }
            0
        }
    }
}

/// create a new directory node, child of current node
fn create_directory(f: &mut ArenaTree<Directory>, c: usize, l: &str) {
    // TODO: turn this entire function into instantiation methods on structs
    let new_dir = Directory { name: l.to_string(), size: 0, depth: 0 };
    let new_dir_idx = ArenaTree::node(f, new_dir);
    f.arena[new_dir_idx].parent = Some(c);
    f.arena[c].children.push(new_dir_idx);
    f.arena[new_dir_idx].val.depth = f.depth_to_target(0, &f.arena[new_dir_idx].val).unwrap();
}

/// add the size of a file to the current directory's size 
fn add_file(f: &mut ArenaTree<Directory>, c: usize, l: &str) {
    let file_size = l
        .split_once(" ")
        .unwrap()
        .0
        .parse::<usize>()
        .unwrap();
    f.arena[c].val.size += file_size;
}

/// solve the first part of the problem
fn part_1(f: &mut ArenaTree<Directory>, max: usize) -> usize {
    let mut sum: usize = 0;
    for dir in (0..f.arena.len()).rev() {
        if f.arena[dir].children.len() > 0 {
            let children = f.arena[dir].children.clone();
            for id in children {
                f.arena[dir].val.size += f.arena[id].val.size;
            }
        }
        if f.arena[dir].val.size < max {
            sum += f.arena[dir].val.size;
        }
    }
    sum
}

/// solve the second part of the problem
fn part_2(f: &mut ArenaTree<Directory>, drive: usize, required: usize) -> Vec<Directory> {
    let mut vec: Vec<Directory> = vec![];
    let size_of_root = f.arena[0].val.size;
    let remaining_space = drive - size_of_root;
    let delete = required - remaining_space;
    for i in 0..f.arena.len() {
        if f.arena[i].val.size > delete {
            let clone = f.arena[i].val.clone();
            vec.push(clone);
        }
    }
    vec.sort_unstable_by_key(|a| a.size);
    vec
}
