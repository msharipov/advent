use std::fs::read_to_string;

use day10_2::{first_bot_with_numbers, parse_instructions};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let (initial, transfers) = parse_instructions(&input).unwrap();
    let bot = first_bot_with_numbers(&initial, &transfers, 17, 61);
    println!("Bot #{bot}");
}
