use day14_1::{parse_reindeer, race_winner};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let reindeer = parse_reindeer(&input).unwrap();
    let winner = race_winner(&reindeer, 2503).unwrap();
    println!("The winner traveled {} km.", winner.position(2503));
}
