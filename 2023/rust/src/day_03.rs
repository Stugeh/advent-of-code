// --- Day 3: Gear Ratios ---
// Gondolas are broken.
// Engine part is missing.
// Add up all the part numbers in the schematic to figure out which.
//
// Number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
//
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
//
// What is the sum of all of the part numbers in the engine schematic?

use std::char;

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
}
impl Coords {
    fn new(x: usize, y: usize) -> Coords {
        Coords { x, y }
    }
}
#[derive(Debug)]
struct Bounds {
    start: Coords,
    end: Coords,
}

impl Bounds {
    fn new() -> Bounds {
        Bounds {
            start: Coords::new(0, 0),
            end: Coords::new(0, 0),
        }
    }
}

// Trait for getting non-overflowing indexes;
trait IdxIncrementers {
    fn try_add_one(self, max: usize) -> Self;
    fn try_sub_one(self) -> Self;
}

impl IdxIncrementers for usize {
    fn try_add_one(self, max: usize) -> usize {
        if self == max {
            return self;
        }
        self + 1
    }

    fn try_sub_one(self) -> usize {
        match self.checked_sub(1) {
            Some(start) => start,
            None => self,
        }
    }
}

pub fn solution(input: Vec<String>) {
    let char_matrix: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let mut answer = 0;

    for (y, line) in char_matrix.iter().enumerate() {
        let mut num = String::from("");
        let mut num_bounds = Bounds::new();

        for (x, char) in line.iter().enumerate() {
            if char.is_numeric() {
                if num.len() == 0 {
                    // First number in a row. Add starting bound.
                    num_bounds.start = Coords::new(x, y);
                }
                num.push(*char);
            }

            // Line ends or next character is not a number.
            // Start Checking surrounding characters for parts.
            if x == line.len() - 1 && num.len() > 0 || (num.len() > 0 && !char.is_numeric()) {
                num_bounds.end = Coords::new(x - 1, y);
                if check_surrounding_chars(&num_bounds, &char_matrix) {
                    answer += num.parse().unwrap_or(0);
                }
                // Reset for next number/line
                num = String::from("");
                num_bounds = Bounds::new();
            }
        }
    }

    println!("{answer}");
}

fn check_surrounding_chars(num_bounds: &Bounds, char_matrix: &Vec<Vec<char>>) -> bool {
    let start_y = num_bounds.start.y;
    // Get the x range of the surrounding digits
    let start_x = num_bounds.start.x.try_sub_one();
    let end_x = num_bounds.end.x.try_add_one(char_matrix[start_y].len() - 1);
    let x_range = start_x..=end_x;

    let row_above = char_matrix.get(start_y.try_sub_one());
    let row_below = char_matrix.get(start_y.try_add_one(char_matrix.len() - 1));

    let left_char = char_matrix[start_y][start_x];
    let right_char = char_matrix[start_y][end_x];

    row_above.map_or(false, |row| vec_has_part(row[x_range.clone()].to_vec()))
        || row_below.map_or(false, |row| vec_has_part(row[x_range].to_vec()))
        || left_char != '.'
        || right_char != '.'
}

fn vec_has_part(row: Vec<char>) -> bool {
    row.iter()
        .filter(|ch| **ch != '.')
        .any(|ch| !ch.is_numeric())
}
