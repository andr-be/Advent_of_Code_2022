/// 1. read the lines
///     if the first char is '$', perform the command in the next section
///     1.1. read the command
///         if the command is 'ls', all the following lines until the next $ are files and
///         directories in our current dir
///         if the command is 'cd', we've entered a drive and should change our current dir
///


use std::path::Path;

enum CommandType {
    ChangeDirectory,
    ListFiles,
}

enum LineType {
    Command,
    Data,
}

struct Directory{
    name: String,
    parent: String,
    subs: Vec<String>,
    size: usize,
}

impl Directory {
    fn new(name: &str, parent: &Directory) -> Directory {
        Directory {
            name: String::from(name),
            parent: parent.name.clone(),
            subs: Vec::new(),
            size: 0,
        }
    }

    fn add_sub(&mut self, dir: &Directory) {
        self.subs.push(dir.name.clone());
    }
}

fn main() {
    let problem = Path::new("src/input.txt");

    let input: String = read_input(problem);
    
    let lines = input.lines();
    println!("{} chars", input.len());

    let mut current_dir = Directory {
        name: "/".to_string(),
        parent: "".to_string(),
        subs: Vec::new(),
        size: 0,
    };
    let mut all_directories: Vec<&Directory> = Vec::new();
    all_directories.push(&current_dir);

    for line in lines {
        match read_line_type(line) {
            LineType::Command => { 
                match read_command_type(line) {
                    CommandType::ChangeDirectory => { 
                        match &get_new_dir(line)[..] {
                            ".." => {
                                println!("{line}\tmoving up a level");
                            },
                             _   => {
                                println!("{line}\tchanging current directory to {}", get_new_dir(line));
                                let new_dir = Directory::new(&get_new_dir(line), &current_dir);
                                current_dir = new_dir;
                            },

                        }
                    },
                    CommandType::ListFiles => { println!("{line}\tlisting files belonging to {}", current_dir.name) },
                } 
            },
            LineType::Data => { 
                println!("{}:\t{line}", current_dir.name.clone());
                let data_size = line.split_once(" ").unwrap();
                match data_size.0 {
                    "dir" => { 
                      current_dir.subs.push(data_size.1.to_string())  
                    },
                    _ => {
                        let data_size = data_size.0.parse::<usize>().unwrap();
                        current_dir.size += data_size;
                    }
                }
            }
        }
    }
}

fn read_line_type(line: &str) -> LineType {
    match &line[0..1] {
        "$" => LineType::Command,
         _  => LineType::Data,
    }
}

fn read_command_type(line: &str) -> CommandType {
    match &line[2..4] {
        "cd" => CommandType::ChangeDirectory,
        "ls" => CommandType::ListFiles,
         _   => panic!(),
    }
}

fn get_new_dir(line: &str) -> String {
    String::from(&line[5..])
}

fn read_input(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(_) => String::from(""),
    }
}
