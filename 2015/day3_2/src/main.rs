use day3_2::{count_distinct_houses, parse_moves};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    println!(
        "{} houses visited",
        count_distinct_houses(&parse_moves(&input))
    );
}
