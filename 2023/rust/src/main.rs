use std::fs;

pub mod day_01;
// pub mod day_02;
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
    let input = get_input_lines("01");
    // let args: Vec<String> = env::args().collect();
    //
    // let day = args[1].clone();
    //
    // match day.as_str() {
    //     "01" => day_01::solution(input),
    //     // "02" => day02::get_score(input),
    //     // "03" => day03::organize_rucksacks(input),
    //     // "04" => day04::get_consumed_ranges_count(input),
    //     &_ => {}
    // };
    day_01::solution(input);
}
