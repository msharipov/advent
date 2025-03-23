use std::fs::read_to_string;

use day1_2::{hq_location, parse_moves, taxicab_distance};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let moves = parse_moves(input[0]).unwrap();
    let distance = taxicab_distance((0, 0), hq_location(&moves).unwrap());
    println!("{distance} blocks away");
}
