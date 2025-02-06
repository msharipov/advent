use std::fs::read_to_string;

use day20_1::{count_gifts, parse_input};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let gifts = parse_input(&input).unwrap();
    let mut house = 1;
    while count_gifts(house) < gifts {
        house += 1;
    }
    println!("House number {}", house);
}
