use std::{env, fs};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

fn get_input_lines(day: &String) -> Vec<String> {
    let input_lines = fs::read_to_string(format!("../input-files/day{}-input", *day));

    if let Ok(input_lines) = input_lines {
        input_lines.lines().map(String::from).collect()
    } else {
        Vec::new()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args[1].clone();
    let input = get_input_lines(&day);

    match day.as_str() {
        "01" => day01::count_calories(input),
        "02" => day02::get_score(input),
        "03" => day03::organize_rucksacks(input),
        "04" => day04::get_consumed_ranges_count(input),
        &_ => {}
    };
}
