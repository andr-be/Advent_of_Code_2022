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

#[derive(PartialEq)]
struct Directory {
    name: String,
    size: usize,
    depth: usize,
}

fn read_input(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(_) => String::from(""),
    }
}

fn main() {
    let path = Path::new("src/input.txt");

    let input = read_input(path);

    let lines = input.lines();
    let mut lines_processed = 0;

    // instantiate the file system tree structure
    let mut filesystem: ArenaTree<Directory> = ArenaTree{ arena: Vec::new() };
    
    // declare the root and set the current dir to it as you create it
    let root = Directory { name: "/".to_string(), size: 0, depth: 0 };
    let mut current_dir = ArenaTree::node(&mut filesystem, root);

    let drive_size = 70_000_000;
    let required_space = 30_000_000;

    for line in lines {
        match &line[0..4] {
            "$ cd" => {     // change current directory
                let drive_name = &line[5..];
                match drive_name {
                    ".." => {
                        current_dir = filesystem.arena[current_dir].parent.unwrap();
                    }
                    _ => { 
                        for dir in &filesystem.arena[current_dir].children {
                            if filesystem.arena[*dir].val.name == drive_name {
                                current_dir = filesystem.arena[*dir].idx;
                            }
                        }
                    }
                }
            },

            "$ ls" => {     /* do nothing */    },

            "dir " => {     // create a new directory node, child of current node
                let new_dir = Directory { name: line[4..].to_string(), size: 0, depth: 0 };
                let new_dir_idx = ArenaTree::node(&mut filesystem, new_dir);
                
                filesystem.arena[new_dir_idx].parent = Some(current_dir);
                
                filesystem.arena[current_dir].children.push(new_dir_idx);
                
                filesystem.arena[new_dir_idx].val.depth = filesystem
                    .depth_to_target(
                        0, &filesystem.arena[new_dir_idx].val
                    ).unwrap();
            }, 
            _ => {      // add line.split_at().1 to current.size
                let file_size = line
                    .split_once(" ")
                    .unwrap()
                    .0
                    .parse::<usize>()
                    .unwrap();
                filesystem.arena[current_dir].val.size += file_size;
            },         
        };
        lines_processed += 1;
    }
    println!("{}\nLINES PROCESSED: {}", path.to_str().unwrap(), lines_processed);
    
    let mut answer_part_1 = 0;


    for dir in (0..filesystem.arena.len()).rev() {
        if filesystem.arena[dir].children.len() > 0 {
            let children = filesystem.arena[dir].children.clone();
            for id in children {
                filesystem.arena[dir].val.size += filesystem.arena[id].val.size;
            }
        }
        if filesystem.arena[dir].val.size < 100_000 {
            answer_part_1 += filesystem.arena[dir].val.size;
        }
    }
    println!("ANSWER PART 1:\t{}\n", answer_part_1);

    // PART 2
    let mut answer_part_2 = Vec::new();
    let size_of_root = filesystem.arena[0].val.size;
    let remaining_space = drive_size - size_of_root;
    let to_be_deleted = required_space - remaining_space;

    println!("SIZE OF {} directory: {:7}\nREMAINING SPACE: {:7}\nTO BE DELETED: {:7}", 
        filesystem.arena[0].val.name, 
        size_of_root,
        remaining_space,
        to_be_deleted,
    );

    for i in 0..filesystem.arena.len() {
        if filesystem.arena[i].val.size > to_be_deleted {
            answer_part_2.push(&filesystem.arena[i].val);
        }
    }
    answer_part_2.sort_unstable_by_key(|a| a.size);
    println!("\nANSWER PART 2:\tDELETE DIR {} - {}", 
        answer_part_2[0].name, 
        answer_part_2[0].size)
}

