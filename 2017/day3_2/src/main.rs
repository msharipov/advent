use std::fs::read_to_string;

use day3_2::first_value_larger_than;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: u64 = input.trim().parse().unwrap();
    let value = first_value_larger_than(input);
    println!("{value}");
}
