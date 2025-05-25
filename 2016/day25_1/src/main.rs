use std::fs::read_to_string;

use day25_1::{first_valid_a_value, parse_instructions};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let instructions = parse_instructions(&input).unwrap();
    let lowest = first_valid_a_value(&instructions).unwrap();
    println!("a = {lowest}");
}
