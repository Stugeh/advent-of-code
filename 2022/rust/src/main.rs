pub mod day01;

fn main() {
    let max_cals = day01::count_calories();
    if let Ok(max_cals) = max_cals {
        println!("{}", max_cals);
    }
}
