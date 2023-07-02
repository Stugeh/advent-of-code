// DAY 3:
// Rucksack has 2 compartments.
//
// Items of a type have to be in the same compartment.
//
// One type of item has been put into both compartments by mistake.
//
// Every different character in the input string is a unique item type (case sensitive).
//
// Rucksacks have an equal amount of items in each compartment, a line represents one rucksack.
//
// First half of line is compartment 1 second half compartment 2
//
// a-z priorities: 1 to 26, A-Z priorities: 27-52
//
// Sum the total priorities of incorrectly organized items

// Breaks with special characters
fn get_str_halfs(input_string: String) -> (String, String) {
    let slice1 = &input_string[..input_string.len() / 2];
    let slice2 = &input_string[input_string.len() / 2..];
    (slice1.to_owned(), slice2.to_owned())
}

fn get_char_anomaly(half1: String, half2: String) -> char {
    for chr in half1.chars() {
        if half2.contains(chr) {
            return chr;
        }
    }

    panic!("didn't find duplicate char")
}

fn get_char_priority(ch: char) -> i32 {
    let lowercase_offset = 96;
    let uppercase_offset = 38;

    match ch {
        'a'..='z' => (ch as i32) - lowercase_offset,
        'A'..='Z' => (ch as i32) - uppercase_offset,
        _ => panic!("invalid character"),
    }
}

fn analyze_sacks_for_duplicates(group: &[String]) -> char {
    let sack1 = group.first().unwrap_or_else(|| panic!("AAAAAA!"));
    for chr in sack1.chars() {
        if group[1].contains(chr) && group[2].contains(chr) {
            return chr;
        }
    }
    panic!("No duplicates found")
}

pub fn organize_rucksacks(input: Vec<String>) {
    let mut total1 = 0;
    let mut total2 = 0;

    //PART 1:
    for line in &input {
        let (half1, half2) = get_str_halfs(line.clone());
        let incorrect_char = get_char_anomaly(half1, half2);
        total1 += get_char_priority(incorrect_char);
    }

    // PART 2:
    let groups = input.chunks(3);
    for group in groups {
        let badge = analyze_sacks_for_duplicates(group);
        total2 += get_char_priority(badge);
    }

    println!("part 1: {}", total1);
    println!("part 2: {}", total2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_priority() {
        assert_eq!(get_char_priority('b'), 2);
        assert_eq!(get_char_priority('B'), 28);
    }

    #[test]
    fn check_string_halving() {
        assert_eq!(
            get_str_halfs("ABCDEFGH".to_owned()),
            ("ABCD".to_owned(), "EFGH".to_owned())
        );
    }

    #[test]
    fn check_duplicate_finding() {
        assert_eq!(get_char_anomaly("ABC".to_owned(), "aBc".to_owned()), 'B');
    }
}
