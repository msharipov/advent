use std::fs::read_to_string;

use day11_1::{least_steps_to_finish, parse_floors};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let floors = parse_floors(&input).unwrap();
    let steps = least_steps_to_finish(&floors).unwrap();
    println!("{steps} steps needed");
}

