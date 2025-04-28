use std::fs::read_to_string;

use day18_1::{count_safe, generate_floor, parse_row};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>()[0];
    let floor = generate_floor(&parse_row(input).unwrap(), 40);
    let count = count_safe(&floor);
    println!("{count} safe tiles");
}

