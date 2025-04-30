use std::fs::read_to_string;

use day21_1::{apply_operations, parse_instructions};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let ops = parse_instructions(&input).unwrap();
    let final_str = apply_operations("abcdefgh", &ops).unwrap();
    println!("Final result: {final_str}");
}
