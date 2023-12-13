use std::fs;

pub mod day_01;
pub mod day_02;
// pub mod day_03;
// pub mod day_04;

fn get_input_lines(day: &str) -> Vec<String> {
    let input_lines = fs::read_to_string(format!("../inputs/day_{}", day));

    if let Ok(input_lines) = input_lines {
        input_lines.lines().map(String::from).collect()
    } else {
        Vec::new()
    }
}

fn main() {
    let input = get_input_lines("02test");
    // day_01::solution(input);
    day_02::solution(input)
}
