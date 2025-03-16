use std::fs::read_to_string;

use day19_2::{count_inversions, parse_replacements, parse_sequence};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let mut input = input.trim().lines();
    let repl = parse_replacements(&mut input).unwrap();
    let mut seq = parse_sequence(&mut input).unwrap();
    let count = count_inversions(&mut seq, &repl).unwrap();
    println!("{count} steps needed");
}
