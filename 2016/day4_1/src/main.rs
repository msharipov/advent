use std::fs::read_to_string;

use day4_1::parse_rooms;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.lines().collect::<Vec<_>>();
    let count = parse_rooms(&input)
        .unwrap()
        .iter()
        .filter(|room| room.is_valid())
        .map(|room| room.sector())
        .sum::<u32>();
    println!("The sum of sector IDs is {count}.")
}
