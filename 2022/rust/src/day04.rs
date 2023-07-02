// Day 4: Camp Cleanup
//
// input file format:
//
// 2-4,6-8
// ...
// 2-3,4-5
//
// Part 1: Find how many times one of the ranges in a pair  fully contains the other.
// Part 2: Find how many times the ranges overlap at all

pub fn get_consumed_ranges_count(input: Vec<String>) {
    let mut consumed_ranges = 0;

    for line in input {
        let range_strings: Vec<&str> = line.split([',', '-']).collect();
        let range1: Vec<i32> = vec![
            range_strings[0]
                .parse()
                .unwrap_or_else(|err| panic!("{}", err)),
            range_strings[1]
                .parse()
                .unwrap_or_else(|err| panic!("{}", err)),
        ];

        let range2: Vec<i32> = vec![
            range_strings[2]
                .parse()
                .unwrap_or_else(|err| panic!("{}", err)),
            range_strings[3]
                .parse()
                .unwrap_or_else(|err| panic!("{}", err)),
        ];

        if range1[0] <= range2[1] && range1[1] >= range2[0] {
            consumed_ranges += 1;
        }
    }
    println!("{}", consumed_ranges);
}
