use std::fs::read_to_string;

use day2_2::{compute_checksum_2, parse_lines};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: Vec<_> = input.trim().lines().collect();
    let numbers = parse_lines(&input).unwrap();
    let checksum = compute_checksum_2(&numbers).unwrap();
    println!("Checksum: {checksum}");
}
