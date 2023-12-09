// --- Day 1: Trebuchet?! ---
// Calibration document is scrambled.
//
// Each line should contain a specific calibration value that needs to be recovered.
// Calibration value on each line can be found by combining the first digit and the last digit to form a single two-digit number.
//
// For example:
// 1abc2        12
// pqr3stu8vwx  38
// a1b2c3d4e5f  15
// treb7uchet   77
// total = 142
//
// Get total.
//
//--- Part Two ---
// It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight,
// and nine also count as valid "digits".

pub fn solution(input: Vec<String>) {
    let mut total: i32 = 0;

    for input_line in &input {
        let line_bytes = input_line.as_bytes();
        let last_idx = input_line.len() - 1;
        let mut idx_offset = 0;
        let mut last_num = '_';
        let mut first_num = '_';

        while last_num == '_' || first_num == '_' {
            if last_num == '_' {
                let ch = line_bytes[last_idx - idx_offset] as char;
                if ch.is_numeric() {
                    last_num = ch;
                }
            }
            if first_num == '_' {
                let ch = line_bytes[idx_offset] as char;
                if ch.is_numeric() {
                    first_num = ch;
                }
            }

            idx_offset += 1;
        }

        let result = format!("{}{}", first_num, last_num);

        total += result.parse::<i32>().unwrap_or(0);
    }

    println!("{}", total);
}
