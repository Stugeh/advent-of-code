// Day 1: Calorie Counting ---

// Elves have inventories of diffenrent kinds of food.

// they take turns to write the amount of calories in each of their food items in a text file.
// a Blank line denotes the end of an inventory.
// Get the elf with the most Calories

use std::{error::Error, fs};

pub fn count_calories() -> Result<u32, Box<dyn Error>> {
    let mut max = 0;

    let lines: Vec<String> = fs::read_to_string("../input-files/day01-input")?
        .lines()
        .map(String::from)
        .collect();

    let mut total: u32 = 0;

    for line in lines {
        if line.is_empty() {
            if total > max {
                max = total
            }
            total = 0;
        } else {
            total += line.parse::<u32>().unwrap();
        }
    }

    Ok(max)
}
