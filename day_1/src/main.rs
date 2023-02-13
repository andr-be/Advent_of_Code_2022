// Advent of Code: Day 1 Solution (Parts 1 & 2) - andr-be 2023 
// This program will solve AoC:D1:P1&2 with any valid input file 

use std::path::Path;

// Not sure if all of these need to be derived, but it works.
// TODO: Work out which of these are necessary
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
struct Elf {
    // This struct contains information about the elves with food in their bags
    number: u32,    // unique elf idenfier, ascending order based on the order they appear in the file
    calories: u32,  // the total number of calories they are carrying with them
}

// a generator function that creates a new Elf from 2 input variables
impl Elf {
    fn record_elf(number: u32, calories: u32) -> Elf {
        Elf {
            number,
            calories,
        }
    }
}

fn main() {
    // create a Path variable to the location of the file
    let path = Path::new("/home/andr-be/rust_programs/advent_of_code_2022/day_1/input");
    // read the file as input
    let data = read_input(path);
    // convert the string into an iterator of strings separated by \n (???)
    let data = data.lines();

    // initialise our variables
    let mut elf_uid: u32 = 1;
    let mut calories: u32 = 0;
    let mut max_cal: u32 = 0;
    let mut max_elf: u32 = 0;
    
    // elf_heap is literally exactly what it says it is
    let mut elf_heap = Vec::new(); 

    // this double-if iterates through every line 
    for line in data {
        // if it's not blank, add the number to the total
        if ! line.is_empty() {
            calories += line.parse::<u32>().unwrap();
        } else {
            // if it is, it creates a record of the elf with the current totals 
            let count = Elf::record_elf(elf_uid, calories);
            // check to see if their calorie total is bigger than the current maximum 
            if count.calories > max_cal {
                // update the max if it is 
                max_cal = count.calories;
                max_elf = count.number;
                if max_elf > 1 {
                    println!("New max! Elf {} with {} calories!", count.number, count.calories);
                }
            }
            // chuck the record onto the heap, increment the uid and reset the total
            elf_heap.push(count);
            elf_uid += 1;
            calories = 0;
        }
    }
    
    // report the answer for Part 1 
    println!("The maximum number of calories are carried by Elf {max_elf}, carrying {max_cal} calories.");
    
    // sort the list in descending order on the calories field 
    elf_heap.sort_by_key(|x| std::cmp::Reverse(x.calories));

    // grab the top 3 elves, list their achievements, tot up their caloric totals
    let mut total = 0;
    for elf_podium in 0..3 {
        println!("Elf {} is carrying {:?} calories.", 
            elf_heap[elf_podium].number, 
            elf_heap[elf_podium].calories
        );
        total += elf_heap[elf_podium].calories;
    }
    // report the answer for Part 2 
    println!("The total amount of calories carried by the top 3 elves is: {total}")
}

fn read_input(file: &Path) -> String {
    // when passed a path to a file, read the file as a string
    // there is no error catching >:)
    std::fs::read_to_string(file).unwrap()
}


