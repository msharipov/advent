use day15_2::{best_score, parse_ingredients};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let ingredients = parse_ingredients(&input).unwrap();
    let best_score = best_score(&ingredients, 100, None);
    println!("The best possible score is: {}", best_score);
}
