use std::fs::read_to_string;

use day1_1::{parse_moves, taxicab_distance, walk};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let moves = parse_moves(input[0]).unwrap();
    let distance = taxicab_distance((0, 0), walk(&moves));
    println!("{distance} blocks away");
}
