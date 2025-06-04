use std::fs::read_to_string;

use day3_1::{Offset, manhattan_distance, offset};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: u64 = input.trim().parse().unwrap();
    let position = offset(input.try_into().unwrap());
    let steps = manhattan_distance(&Offset { right: 0, up: 0 }, &position);
    println!("{steps} steps");
}
