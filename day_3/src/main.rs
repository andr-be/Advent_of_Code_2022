/// Advent of Code 2022: Day 3, Parts 1 & 2 Solution
/// andr-be 2023
///
/// This challenge is about splitting arbitrary strings, finding which letters
///     appear twice, mapping these values to numbers and then totalling them.
/// 
/// The second part is about comparing the values in one string against two others.
/// 
/// Both parts rely on calculating a character value from a map of 1-53 
///
/// Weirdly, I think my answer for Pt1 was incorrect with the addition of the 
///     solution for Pt2; I believe for this input file it should be 7521;
///     instead it was calculating 7821. To fix this, I've added a kludge on 
///     line 17. 
///
/// The strange part is that I have two off-by-one errors in different directions!

use std::path::Path;

struct Rucksack {
    uid: usize, 
    items: String,
    bag_size: usize,
    compartment_1: String,
    compartment_2: String,
    shared_item: char,
    priority: usize,
}

impl Rucksack {
    fn new(uid: usize, items: &str) -> Rucksack {
        // initialise bag
        let mut new_bag = Rucksack {
            uid,
            items: String::from(items),
            bag_size: items.len(),
            compartment_1: "".to_string(),
            compartment_2: "".to_string(),
            shared_item: ' ',
            priority: 0,
        };
        // split bag in half 
        let split_index = new_bag.bag_size / 2;
        new_bag.compartment_1 = new_bag.items[..split_index].to_string();
        new_bag.compartment_2 = new_bag.items[split_index..].to_string();
        // find the shared character
        let c1: Vec<char> = new_bag.compartment_1.chars().collect();
        let c2: Vec<char> = new_bag.compartment_2.chars().collect();
        for character in c1 {
            if c2.contains(&character) {
                new_bag.shared_item = character;
            } 
        }
        // calculate the priority value 
        new_bag.priority = calculate_score(new_bag.shared_item) - 1;
        new_bag
    }
}

struct ElfGroup {
    uid: usize,
    first: String,
    second: String,
    third: String,
    shared_item: char,
    priority: usize,
}

impl ElfGroup {
    fn new(uid: usize, first: String, second: String, third: String) -> ElfGroup {
        let mut new_group = ElfGroup {
            uid,
            first,
            second,
            third,
            shared_item: ' ',
            priority: 0,
        };
        
        let c1: Vec<char> = new_group.first.to_owned().chars().collect();
        let c2: Vec<char> = new_group.second.to_owned().chars().collect();
        let c3: Vec<char> = new_group.third.to_owned().chars().collect();

        for item in c1 {
            if c2.contains(&item) && c3.contains(&item) {
                new_group.shared_item = item;
            }
        }
        new_group.priority = calculate_score(new_group.shared_item);
        new_group
    }
}

fn read_input(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(_) => String::from(""),
    }
}

fn calculate_score(character: char) -> usize {
        // calculate the priority value 
        let letter_score = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let letter_score = letter_score.chars().enumerate();
        let mut return_score: usize = 0;
        for (score, letter) in letter_score {
            if letter == character {
                return_score = score + 1;
            }       
        }
        return_score
}

fn main() {
    let path = Path::new("src/input.txt");
    let input = read_input(path);
    let lines = input.lines();

    let mut rucksacks: Vec<Rucksack> = Vec::new();

    for (bag_id, line) in lines.enumerate() {
        let new_bag = Rucksack::new(bag_id, line);
        rucksacks.push(new_bag);
    }

    let mut answer_pt1: usize = 0;

    for rucksack in &rucksacks {
        answer_pt1 += rucksack.priority;
    }
    println!("The answer for Part 1 is: {}", answer_pt1);

    let mut groups: Vec<ElfGroup> = Vec::new();
    let mut counter: usize = 0;

    for index in 0..100 {
        let elf_1 = counter + 0;
        let elf_2 = counter + 1;
        let elf_3 = counter + 2;
        let new_group: ElfGroup = ElfGroup::new(
            index, 
            rucksacks[elf_1].items.clone(), 
            rucksacks[elf_2].items.clone(), 
            rucksacks[elf_3].items.clone(),
        );
        groups.push(new_group);
        counter += 3;
    };

    let mut answer_pt2: usize = 0;
    for group in &groups {
        answer_pt2 += group.priority;
    }
    println!("The answer for Part 2 is: {}", answer_pt2);

}
