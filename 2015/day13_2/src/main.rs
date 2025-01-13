use day13_2::{highest_happiness, parse_preferences};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let names = [
        "me", "Alice", "Bob", "Carol", "David", "Eric", "Frank", "George", "Mallory",
    ];
    let preference_lines = input.trim().lines().collect::<Vec<&str>>();
    let mut preferences = parse_preferences(&preference_lines).unwrap();
    for name in &names[1..] {
        preferences.insert(("me".to_string(), name.to_string()), 0);
        preferences.insert((name.to_string(), "me".to_string()), 0);
    }
    let highest = highest_happiness(&names, &preferences).unwrap();
    println!("The highest possible happiness is: {}", highest);
}
