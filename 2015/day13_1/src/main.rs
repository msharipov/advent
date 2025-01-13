use day13_1::{highest_happiness, parse_preferences};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let names = [
        "Alice", "Bob", "Carol", "David", "Eric", "Frank", "George", "Mallory",
    ];
    let preference_lines = input.trim().lines().collect::<Vec<&str>>();
    let preferences = parse_preferences(&preference_lines).unwrap();
    let highest = highest_happiness(&names, &preferences).unwrap();
    println!("The highest possible happiness is: {}", highest);
}
