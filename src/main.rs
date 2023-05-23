use std::{io, fs};
use std::env;
use elves::Elf;

mod elves;

fn main() {
    let cla: Vec<String> = env::args().collect();

    let file_contents = fs::read_to_string(&cla[1]).unwrap();
    let mut cur_elf = Elf::new();
    let mut food = String::new();
    let mut single_return = false;


    let mut elves = Vec::new();

    for char in file_contents.chars() {
        match char {
            '\n' => {
                if !single_return {
                    single_return = true;
                    println!("Current value of food: {}", food);
                    let food_item: u32 = food.trim().parse().expect("Needed a number, got ");
                    cur_elf.add_food_item(food_item);
                    food = String::new();
                } else {
                    elves.push(cur_elf);
                    cur_elf = Elf::new();
                    food = String::new();
                    single_return = false;
                }
            },
            '0'..='9' => { 
                food.push(char);
                single_return = false;
            },
            _ => panic!("Inconsolable exception while parsing file contents"),
        }
    }
    
    // read file
    // parse file
    // get result

    elves.sort_by(|a, b| b.get_total().cmp(&a.get_total()));

    let most = &elves[0..=2];
    
    let mut sum = 0;

    for elf in most {
        println!("Summing {} and {}", sum, (*elf).get_total());
        sum = sum + (*elf).get_total();
    }

    println!("The most calories carried by a single elf is {}", sum);

}
