use std::fs::read_to_string;

use day21_2::{apply_operations, invert_operations, parse_instructions};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let ops = parse_instructions(&input).unwrap();
    let ops = invert_operations(&ops);
    let final_str = apply_operations("fbgdceah", &ops).unwrap();
    println!("Unscrambled string: {final_str}");
}
