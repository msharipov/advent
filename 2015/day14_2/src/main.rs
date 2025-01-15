use day14_2::{parse_reindeer, race_winner_points};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let reindeer = parse_reindeer(&input).unwrap();
    println!(
        "The winner accrued {} points.",
        race_winner_points(&reindeer, 2503).unwrap()
    );
}
