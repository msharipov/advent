use std::fs::read_to_string;

use day5_2::{jumps_to_exit, parse_jumps};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: Vec<_> = input.trim().lines().collect();
    let jumps = parse_jumps(&input).unwrap();
    println!("{} jumps required", jumps_to_exit(&jumps));
}
