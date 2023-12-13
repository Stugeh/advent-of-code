// --- Day 2: Cube Conundrum ---
// Cubes are either red, green or blue.
// In a bag there are a random number of cubes of each color.
//
// Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag.
// He'll do this a few times per game.
//
// Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).
//
// For example, the record of a few games might look like this:
//
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
//
// What is the sum of the IDs of games possible with 12 red cubes, 13 green cubes, and 14 blue cubes?

fn fuck(val: &mut u16, nums: &mut String) {
    let new_num = nums.parse::<u16>().unwrap();
    if new_num > *val {
        *val = new_num
    }
    *nums = String::from("0");
}

pub fn solution(input: Vec<String>) {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    let mut possible_games: Vec<u16> = vec![];

    for line in input.iter() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut id = 0;

        let mut nums = String::from("0");
        let line = line;

        for ch in line.chars() {
            match ch.is_numeric() {
                true => nums.push(ch),
                false => match ch {
                    'r' => fuck(&mut red, &mut nums),
                    'g' => fuck(&mut green, &mut nums),
                    'b' => fuck(&mut blue, &mut nums),
                    ':' => {
                        id = nums.parse::<u16>().unwrap();
                        nums = String::from("0");
                    }
                    _ => {}
                },
            }
        }

        if red > red_max || green > green_max || blue > blue_max {
            continue;
        }

        possible_games.push(id);
    }

    let total = possible_games.iter().sum::<u16>();
    println!("{total}");
}

