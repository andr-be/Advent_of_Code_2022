use std::{path::Path, fmt};

/// Advent of Code 2022 - Day 8 - Parts 1 & 2
///
/// I believe this solution will involve some kind of depth search?
///
/// Part 1 was solved at 1679
/// Part 2 is not something I've been able to solve yet.
///
struct Forest {
    trees: Vec<Tree>
}

impl Forest {
    fn new() -> Forest {
        Forest {
            trees: Vec::new(),
        }
    }
    fn add_tree(&mut self, mut tree: Tree) {
        tree.id = self.trees.len();
        self.trees.push(tree);
    }
    fn height_from_pos(&self, x_pos: usize, y_pos: usize) -> usize {
        for tree in &self.trees {
            if tree.x_pos == x_pos && tree.y_pos == y_pos {
                return tree.height
            }
        }
        panic!("COULDN'T LOCATE TREE AT THAT LOCATION")
    }
}

struct Tree {
    id: usize,
    height: usize,
    x_pos: usize,
    y_pos: usize,
    visible: bool,
    beauty: usize,
}

impl Tree {
    fn new(height: usize, x_pos: usize, y_pos: usize) -> Tree {
        let t = Tree {
            id: 999_999,
            height,
            x_pos,
            y_pos,
            visible: false,
            beauty: 0,
        };
        t
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {:4}\th: {:1}\tx,y: ({:3},{:3})\tv: {}\tb: {}", 
            self.id, 
            self.height, 
            self.x_pos, 
            self.y_pos,
            self.visible,
            self.beauty,
        )
    }
}

fn main() {
    // let path: &Path = Path::new("src/input.txt");
    let path: &Path = Path::new("src/input.txt");
    let input: String = read_input(path);
    let lines = input.lines();

    let mut forest: Forest = Forest::new();
    let mut current_line: usize = 0;
    
    for line in lines {
        let letters = line.char_indices();
        for letter in letters {
            forest.add_tree(
                Tree::new(
                    letter.1.to_digit(10).unwrap() as usize, 
                    letter.0, 
                    current_line
                )
            );

        }
        current_line += 1;
    }

    let tree_count: usize = forest.trees.len();
    let plot_width: usize = forest.trees[tree_count-1].x_pos + 1;
    let plot_height: usize = forest.trees[tree_count-1].y_pos + 1;

    println!("TOTAL TREES: {tree_count}\nWIDTH: {plot_width}\tHEIGHT: {plot_height}");
    
    for i in 0..tree_count {
        if forest.trees[i].x_pos == 0 
        || forest.trees[i].x_pos == plot_width - 1
        || forest.trees[i].y_pos == 0 
        || forest.trees[i].y_pos == plot_height - 1
        {
            forest.trees[i].visible = true;
        }
    }
    
    for i in 0..forest.trees.len() {
        if i % 1000 == 0 {
            println!("CHECKING TREE {i}");
        };
        let mut obs_left: Option<(usize, usize)> = Default::default();
        let mut obs_top: Option<(usize, usize)> = Default::default();
        let mut obs_right: Option<(usize, usize)> = Default::default();
        let mut obs_bottom: Option<(usize, usize)> = Default::default();

        let trees_visible_l: usize;
        let trees_visible_r: usize;
        let trees_visible_t: usize;
        let trees_visible_b: usize;
    
        if forest.trees[i].visible == false {
            let x_pos = forest.trees[i].x_pos;
            let y_pos = forest.trees[i].y_pos;
            let height = forest.trees[i].height;
            
            // check left
            for x in (0..x_pos).rev() {
                if forest.height_from_pos(x, y_pos) >= height && obs_left == None {
                    obs_left = Some((x, y_pos));
                    break;
                }
            }
            // check right
            for x in x_pos+1..plot_width {
                if forest.height_from_pos(x, y_pos) >= height && obs_right == None {
                    obs_right = Some((x, y_pos));
                    break;
                }
            }
            // check up
            for y in (0..y_pos).rev() {
                if forest.height_from_pos(x_pos, y) >= height && obs_top == None {
                    obs_top = Some((x_pos, y));
                    break;
                }
            }
            // check down
            for y in y_pos+1..plot_height {
                if forest.height_from_pos(x_pos, y) >= height && obs_bottom == None {
                    obs_bottom = Some((x_pos, y));
                    break;
                } 
            }

            if obs_top == None || obs_right == None || obs_left == None || obs_bottom == None {
                forest.trees[i].visible = true;
            }
            
            match obs_left {
                Some((x, _)) => trees_visible_l = x_pos - x,
                None => trees_visible_l = x_pos,
            }
            match obs_right {
                Some((x, _)) => trees_visible_r = x - x_pos,
                None => trees_visible_r = plot_width - x_pos - 1,
            }
            match obs_top {
                Some((_, y)) => trees_visible_t = y_pos - y,
                None => trees_visible_t = y_pos,
            }
            match obs_bottom {
                Some((_, y)) => trees_visible_b = y - y_pos,
                None => trees_visible_b = plot_height - y_pos - 1,
            }

            forest.trees[i].beauty = trees_visible_l * trees_visible_r * trees_visible_t * trees_visible_b;
        }
    }
    let mut answer_pt_1 = 0;
    for tree in &forest.trees {
        match tree.visible {
            true => answer_pt_1 += 1,
            false => { },
        }
    }
    println!("\nANSWER PART 1: {} TREES ARE VISIBLE", answer_pt_1);

    forest.trees.sort_unstable_by_key(|a| a.beauty);
    forest.trees.reverse();
    // assert!(forest.trees[0].beauty > 7980);
    println!("\nANSWER PART 2: {} is the highest scenic score\n{}", forest.trees[0].beauty, forest.trees[0]);
}

fn read_input(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(_) => String::from(""),
    }
}

