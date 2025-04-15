use std::fs::read_to_string;

use day11_2::{least_steps_to_finish, parse_floors};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let mut floors = parse_floors(&input).unwrap();
    floors[0].insert(day11_2::Part::RTG(6));
    floors[0].insert(day11_2::Part::RTG(7));
    floors[0].insert(day11_2::Part::Chip(6));
    floors[0].insert(day11_2::Part::Chip(7));
    let steps = least_steps_to_finish(&floors).unwrap();
    println!("{steps} steps needed");
}
